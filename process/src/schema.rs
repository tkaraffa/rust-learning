use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Error, ErrorKind};

// use this as the default value
// alternatively, make it go through env vars
// better yet, go through CLI, then env, then here

pub const CURRENT_VERSION: usize = 1;

// all `Schema` implementing Types should have
// a method called `from_data` that takes in a string of data
// and returns some type of Schema
pub fn get_schema_by_version(data: &String, version: &usize) -> Result<serde_json::Value, Error> {
    match version {
        2 => SchemaV2::from_data(data)?.data_to_object(),
        1 => SchemaV1::from_data(data)?.data_to_object(),
        // eventually there should be a custom error, but use this generic one for now
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("Error Invalid Schema: V{version}"),
        )),
    }
}

trait Schema {
    type SchemaType;
    fn from_data<'a>(data: &'a String) -> Result<Self::SchemaType, Error>
    where
        Self: Sized,
        <Self as Schema>::SchemaType: Deserialize<'a>,
        <Self as Schema>::SchemaType: Serialize,
    {
        // might want to break out into own parts
        // still probably wants its definition here, to get the correct type
        // but maybe doesn't want to json!() it into text quite yet
        let schema: Self::SchemaType = serde_json::from_str(data)?;
        Ok(schema)
        // Ok(serde_json::json!(schema))
    }
    fn data_to_object(&self) -> Result<serde_json::Value, Error>
    where
        Self: Serialize,
    {
        Ok(serde_json::json!(&self))
    }
    fn data_to_string(&self) -> Result<String, Error>
    where
        Self: Serialize,
    {
        let data: String = serde_json::to_string(&self)?;
        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaV2 {
    key1: String,
    key2: Vec<String>,
}

impl Schema for SchemaV2 {
    type SchemaType = Self;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaV1 {
    key1: String,
    key2: String,
}

impl Schema for SchemaV1 {
    type SchemaType = Self;
}
