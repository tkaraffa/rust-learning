use serde::{Deserialize, Serialize};
use serde_json;

// use this as the default value
// alternatively, make it go through env vars
// better yet, go through CLI, then env, then here

pub const CURRENT_VERSION: usize = 1;

// all `Schema` implementing Types should have
// a method called `from_data` that takes in a string of data
// and returns some type of Schema
pub fn get_schema_by_version(data: String, version: usize) -> Box<dyn Process> {
    match version {
        1 => Box::new(SchemaV1::from_data(data)),
        _ => Box::new(DefaultSchema::from_data(data)),
    }
}

pub trait Process {
    fn from_data(data: String) -> Self
    where
        Self: Sized;
    fn get_data(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaV1 {
    key1: String,
    key2: Vec<String>,
}

impl Process for SchemaV1 {
    fn from_data(data: String) -> Self {
        let schema: Self = serde_json::from_str(&data).unwrap();
        schema
    }
    fn get_data(&self) -> String {
        serde_json::to_string(&self).expect("oops")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultSchema {}

impl Process for DefaultSchema {
    fn from_data(data: String) -> Self {
        let schema: Self = serde_json::from_str(&data).unwrap();
        schema
    }
    fn get_data(&self) -> String {
        serde_json::to_string(&self).expect("oops")
    }
}
