use clap::Parser;
/// Write text into a file
#[derive(Parser, Debug)]
#[clap(author = "me")]
struct Args {
    /// name of file
    filename: String,
    /// text to write
    text: String,
}

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let filename = args.filename;
    let text = args.text;

    make_file(&filename, &text)
}

fn make_file(filename: &String, text: &String) -> std::io::Result<()> {
    let filetext = text.as_bytes();
    let mut file = File::create(filename)?;
    file.write(filetext)?;
    Ok(())
}
