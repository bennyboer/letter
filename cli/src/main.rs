use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, fs};

use encoding_rs::{Encoding, UTF_8};
use encoding_rs_io::DecodeReaderBytesBuilder;
use simple_logger::SimpleLogger;

use document::meta_data::{DocumentEncoding, DocumentMetaData};
use document::style::DocumentStyles;
use document::Document;
use export::ExportType;
use layout::options::LayoutOptions;
use metadata::read_meta_data;
use script::parse_document_structure;

const BANNER: &str = "\
-------------------
  | _ _|__|_ _  _  
  |(/_ |  | (/_|
-------------------";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    println!("{}", BANNER);
    println!("      v{}\n", VERSION);

    // TODO Use a proper command line parser
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Expected file path as first argument");

    // TODO For now we simply search the folder of the script file for a meta data file with the name "metadata.toml"
    let meta_data_file_path = Path::new(file_path)
        .parent()
        .unwrap()
        .join("meta_data.toml");
    let meta_data = if meta_data_file_path.exists() {
        let meta_data_src = fs::read_to_string(meta_data_file_path)?;
        read_meta_data(&meta_data_src)?
    } else {
        DocumentMetaData::default()
    };

    println!("Meta data: {:#?}", meta_data);

    let styles = if args.len() > 2 {
        let style_file_path = args.get(2).unwrap();
        let style_src = read_to_string_with_encoding(style_file_path, &meta_data.encoding)
            .expect("Could not read style file");
        style::parse(&style_src)?
    } else {
        DocumentStyles::new()
    };

    let script = read_to_string_with_encoding(file_path, &meta_data.encoding)
        .expect("Could not read script file");
    let document_structure = parse_document_structure(&script)?;

    let document = Document {
        meta_data,
        structure: document_structure,
        styles,
    };

    let layout_options = LayoutOptions::default();
    let document_layout = layout::layout(&document, layout_options)?;

    export::export(document_layout, ExportType::PDF)?;

    Ok(())
}

fn read_to_string_with_encoding(
    path: &str,
    file_encoding: &DocumentEncoding,
) -> Result<String, Box<dyn Error>> {
    let encoding_name = file_encoding.name().as_bytes();
    let encoding = Encoding::for_label(encoding_name);
    if encoding.is_none() {
        println!("Could not find encoding for provided label '{}' in meta data. Falling back to 'utf-8' instead.", file_encoding.name());
    }

    let file = File::open(path)?;
    let decode_reader_bytes = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding.unwrap_or(UTF_8)))
        .build(file);
    let reader = BufReader::new(decode_reader_bytes);

    let mut result = String::new();
    for read_result in reader.lines() {
        if let Ok(line) = read_result {
            result.push_str(&line);
        }
    }

    Ok(result)
}
