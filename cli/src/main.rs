use std::{env, fs};
use std::error::Error;

use simple_logger::SimpleLogger;

use document::Document;
use document::meta_data::DocumentMetaData;
use document::style::DocumentStyles;
use export::ExportType;
use layout::options::LayoutOptions;
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

    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Expected file path as first argument");

    // TODO Use a proper command line parser
    let styles = if args.len() > 2 {
        let style_file_path = args.get(2).unwrap();
        let style_src = fs::read_to_string(style_file_path).expect("Could not read style file");
        style::parse(&style_src)?
    } else {
        DocumentStyles::new()
    };

    // TODO Read meta data file (if any) first -> otherwise create default meta data
    // TODO Use meta data file to fetch encoding to use to read script and style files for this document
    let script = fs::read_to_string(file_path).expect("Could not read script file");
    let document_structure = parse_document_structure(&script)?;
    let document = Document {
        meta_data: DocumentMetaData::default(), // TODO Read meta data from config file
        structure: document_structure,
        styles,
    };

    let layout_options = LayoutOptions::default();
    let document_layout = layout::layout(&document, layout_options)?;

    export::export(document_layout, ExportType::PDF)?;

    Ok(())
}
