mod schema;
use clap::Parser;
use std::fs;

use schema::{get_schema_by_version, read_data};

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Path in which to process files
//     in: String,
// }

fn main() {
    // let args = Args::parse();
    // let input_directory = args.in;
    let input_directory = "test";
    let version: usize = 1;

    let input_directory = fs::read_dir(input_directory).unwrap();
    for path in input_directory {
        let data = read_data(path.unwrap().path());
        let schema = get_schema_by_version(data, version);
        println!("{}", schema.get_data());
    }
}

/*
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

*/
