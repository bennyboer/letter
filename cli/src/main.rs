use script::parse_document_structure;
use std::error::Error;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Expected file path as first argument");
    let script = fs::read_to_string(file_path).expect("Could not read script file");

    let document_structure = parse_document_structure(&script)?;

    println!("##########");
    println!("# OUTPUT #");
    println!("##########");
    document_structure.pretty_print();

    Ok(())
}
