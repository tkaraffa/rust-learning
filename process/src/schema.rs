use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_json;

// use this as the default value
// alternatively, make it go through env vars
// better yet, go through CLI, then env, then here

pub const CURRENT_VERSION: usize = 1;

// all `Schema` implementing Types should have
// a method called `from_data` that takes in a string of data
// and returns some type of Schema
pub fn get_schema_by_version(data: String, version: usize) -> serde_json::Value {
    let schema = match version {
        1 => SchemaV1::from_data(data),
        _ => DefaultSchema::from_data(data),
    };
    schema
}

pub trait Schema {
    fn from_data(data: String) -> serde_json::Value
    where
        Self: Sized;
    fn get_data(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaV1 {
    pub key1: String,
    pub key2: Vec<String>,
}

impl Schema for SchemaV1 {
    fn from_data(data: String) -> serde_json::Value {
        let schema: Self = serde_json::from_str(&data).unwrap();
        serde_json::json!(schema)
    }
    fn get_data(&self) -> String {
        serde_json::to_string(&self).expect("oops")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultSchema {}

impl Schema for DefaultSchema {
    fn from_data(data: String) -> serde_json::Value {
        let schema: Self = serde_json::from_str(&data).unwrap();
        serde_json::json!(schema)
    }
    fn get_data(&self) -> String {
        serde_json::to_string(&self).expect("oops")
    }
}
