use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::{Error, Result, Value};

pub fn from_jsonld(path: impl AsRef<Path>) -> Value {
    // Open file in read-only mode with buffer.
    let file = File::open(path).expect("Could not open file.");
    let reader = BufReader::new(file);

    // Read the JSON contents as an instance of `serde_json::Value`.
    let content: Value = serde_json::from_reader(reader).expect("Could not parse JSON data.");
    content
}
