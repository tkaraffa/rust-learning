mod process;
mod schema;
use clap::Parser;
use std::fs;

use process::ThreadPool;
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

fn main() -> Result<(), String> {
    let args = Args::parse();
    let input_directory = args.input_directory;
    let schema_version: usize = args.schema_version;

    let pool = ThreadPool::new(50);

    let input_directory = fs::read_dir(input_directory)
        .expect("Something went wrong when reading from {input_directory");

    for path in input_directory {
        pool.execute(move || {
            let path_str = path
                .expect("Something went wrong when reading from {input_directory}.")
                .path();
            let json = fs::read_to_string(&path_str)
                .expect("Something went wrong when reading file {path_str}.");
            let _schema =
                get_schema_by_version(&json, &schema_version).expect("Something went wrong");

            println!("Schema processed for {:?}!", &path_str);
        });
    }
    Ok(())
}

/*

other thoughts:
* i think there's not a way to get string rep of the structure itself (with types)
* so, if we want to have a "write this to avro", we'd have to also keep a string rep
    of the schema as a method
    * kind of sucks


so, main idea is

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
