use serde::{Deserialize, Serialize};

// use this as the default value
// alternatively, make it go through env vars
// better yet, go through CLI, then env, then here

// let CURRENT_VERSION = 1;

// all `Schema` implementing Types should have
// a method called `from_data` that takes in a string of data
// and returns some type of Schema
pub trait Schema {
    fn from_data(data: String);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaV1 {
    key1: String,
    key2: Vec<String>,
}

// run this only on things that implement the Schmea trait
// per this: https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
// fn get_schema_from_data(schema &impl Schema) -> &impl Schema {

// }

impl Schema for SchemaV1 {
    fn from_data(data: String) -> Self {
        let result: Self = serde_json::from_str(&data).unwrap();
        Box::new(result)
    }
}

pub fn get_latest_schema(data: String) -> SchemaV1 {
    let schema: Box<dyn Schema> = SchemaV1::from_data(data);
    Box::new(schema)
}

// want some way of...
// getting JUST the struct, but not instantiated...
// So there should be a common schema type
// that can have some shared behavior...
// probably the "validation" behavior

// pub fn match_schema() -> Schema {
//     match version {
//         1 => SchemaV1,
//         _ => Err,
//     }
// }
