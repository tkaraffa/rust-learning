use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io;

// use this as the default value
// alternatively, make it go through env vars
// better yet, go through CLI, then env, then here

pub const CURRENT_VERSION: usize = 1;

// all `Schema` implementing Types should have
// a method called `from_data` that takes in a string of data
// and returns some type of Schema
pub fn get_schema_by_version(data: String, version: usize) -> Result<serde_json::Value, io::Error> {
    match version {
        2 => SchemaV2::from_data(data),
        1 => SchemaV1::from_data(data),
        _ => Ok(DefaultSchema::from_data(data)).expect_err("Nooo"),
    }
}

pub trait Schema {
    fn from_data(data: String) -> Result<serde_json::Value, io::Error>
    where
        Self: Sized;
    fn get_data(&self) -> Result<String, io::Error>;
}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaV2 {
    key1: String,
    key2: String,
}

impl Schema for SchemaV2 {
    fn from_data(data: String) -> Result<serde_json::Value, io::Error> {
        let schema: Self = serde_json::from_str(&data)?;
        Ok(serde_json::json!(schema))
    }
    fn get_data(&self) -> Result<String, io::Error> {
        let data: String = serde_json::to_string(&self)?;
        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaV1 {
    pub key1: String,
    pub key2: Vec<String>,
}

impl Schema for SchemaV1 {
    fn from_data(data: String) -> Result<serde_json::Value, io::Error> {
        let schema: Self = serde_json::from_str(&data)?;
        Ok(serde_json::json!(schema))
    }
    fn get_data(&self) -> Result<String, io::Error> {
        let data: String = serde_json::to_string(&self)?;
        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultSchema {}

impl Schema for DefaultSchema {
    fn from_data(data: String) -> Result<serde_json::Value, io::Error> {
        let schema: Self = serde_json::from_str(&data)?;
        Ok(serde_json::json!(schema))
    }
    fn get_data(&self) -> Result<String, io::Error> {
        let data: String = serde_json::to_string(&self)?;
        Ok(data)
    }
}
