use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, path::PathBuf};

// use this as the default value
// alternatively, make it go through env vars
// better yet, go through CLI, then env, then here

// let CURRENT_VERSION = 1;

// all `Schema` implementing Types should have
// a method called `from_data` that takes in a string of data
// and returns some type of Schema
pub fn read_data(path: PathBuf) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_schema_by_version(data: String, version: usize) -> Box<dyn Schema> {
    match version {
        1 => Box::new(SchemaV1::from_data(data)),
        _ => Box::new(DefaultSchema::from_data(data)),
    }
}

pub trait Schema {
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

impl Schema for SchemaV1 {
    fn from_data(data: String) -> Self
    where
        Self: Sized,
    {
        let schema: SchemaV1 = serde_json::from_str(&data).unwrap();
        schema
    }

    fn get_data(&self) -> String
    where
        Self: Sized,
    {
        String::from(self.key1.clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultSchema {}

impl Schema for DefaultSchema {
    fn from_data(data: String) -> Self
    where
        Self: Sized,
    {
        let schema: DefaultSchema = serde_json::from_str(&data).unwrap();
        schema
    }
    fn get_data(&self) -> String {
        String::from("no")
    }
}
