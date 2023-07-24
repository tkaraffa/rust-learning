mod schema;
use clap::Parser;
use std::fs;

use process_files::process_file;
// use schema::get_latest_schema;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path in which to process files
    path: String,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(args.path).unwrap();
    for path in paths {
        let result = process_file(path.unwrap().path());

        println!("{:?}", result);
    }
}
