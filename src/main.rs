mod parse;

use parse::extract_doi;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let pdf_path = PathBuf::from(&args[0]);

    let doi = extract_doi(pdf_path).unwrap();
    println!("{}", doi)
}
