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
    let script = fs::read_to_string(file_path).expect("Could not read script file");

    let document_structure = parse_document_structure(&script)?;

    println!("##########");
    println!("# OUTPUT #");
    println!("##########");
    println!("{}", document_structure.fmt_pretty());

    Ok(())
}
