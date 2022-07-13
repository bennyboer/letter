use script::parse_document_structure;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Expected file path as first argument");
    let script = fs::read_to_string(file_path).expect("Could not read script file");

    parse_document_structure(&script);
}
