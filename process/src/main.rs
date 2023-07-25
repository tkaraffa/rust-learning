mod schema;
use clap::Parser;
use std::fs;

use schema::{get_schema_by_version, CURRENT_VERSION};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory from which to process files
    input_directory: String,

    /// Schema version to use for processing data
    #[arg(short, long, default_value_t = CURRENT_VERSION)]
    schema_version: usize,
}

fn main() {
    let args = Args::parse();
    let input_directory = args.input_directory;
    let schema_version: usize = args.schema_version;

    let input_directory = fs::read_dir(input_directory).unwrap();
    for path in input_directory {
        let path_str = path
            .expect("Something went wrong when reading from {input_directory}.")
            .path();
        let json = fs::read_to_string(path_str)
            .expect("Something went wrong when reading file {path_str}.");
        let schema = get_schema_by_version(json, schema_version);
        println!("{:?}", schema);
    }
}
/*
 THE SCHEMA TRAIT SHOULD BE A TYPE
 and there should be a ..."process" trait that uses that type

so, main idea is
* have some number of schemas that dictate how the JSON data should look
    * these might be parsed from YAML (into structs) or be structs themselves
* no matter what schema is used, have some way of:
    * reading JSON file(s) from a directory
    * loading those into the appropriate struct
    * validating the data (this might be in the above)
    * doing something? maybe
    * unloading into avro files
* should be able to read, process, and write files concurrently (using threads)

* should take command line args that dictate
    * input directory
    * input filetype
    * output directory
    * output filetype
    * which schema to use

* should handle if an invalid schema is used

*/
