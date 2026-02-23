use std::fs;
use std::path::PathBuf;

pub mod parser;

pub fn read_fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    fs::read_to_string(path).expect(&format!("Failed to read fixture: {}", name))
}
