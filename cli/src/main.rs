use document::meta_data::DocumentMetaData;
use document::Document;
use script::parse_document_structure;
use simple_logger::SimpleLogger;
use std::error::Error;
use std::{env, fs};

const BANNER: &str = "\
-------------------
  | _ _|__|_ _  _  
  |(/_ |  | (/_|
-------------------";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    println!("{}", BANNER);
    println!("v{}\n", VERSION);

    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Expected file path as first argument");

    // TODO Read meta data file (if any) first -> otherwise create default meta data
    // TODO Use meta data file to fetch encoding to use to read script and style files for this document
    let script = fs::read_to_string(file_path).expect("Could not read script file");

    let document_structure = parse_document_structure(&script)?;
    let document = Document {
        meta_data: DocumentMetaData::default(),
        structure: document_structure,
    };

    typeset::typeset(&document)?;

    Ok(())
}
