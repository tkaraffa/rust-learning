mod schema;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize,Deserialize};
use schema::{get_latest_schema, Schema};

#[derive(Serialize,Deserialize,Debug)]
pub struct JSONTest {
    key1: String,
    key2: Vec<String>,
}

pub fn process_file(path: PathBuf) -> Box<dyn Schema> {
    let data = fs::read_to_string(path).unwrap();
    let schema = get_latest_schema(data);
    // let deserialized_data: JSONTest = serde_json::from_str(&data).unwrap();
    Box::new(schema) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() {
        let filename = PathBuf::from("./test/test.json");
        let result = process_file(filename);
        assert_eq!(
            result.key1, "value1"
        )
    }
}
