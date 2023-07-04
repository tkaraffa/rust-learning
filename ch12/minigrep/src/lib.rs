use std::env;
use std::error::Error;
use std::fs;

/// Hold configuration for program run.
pub struct Config {
    string: String,
    file: String,
    ignore_case: bool,
}

impl Config {
    /// Create a new Config from CLI arguments
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let string: String = match args.next() {
            Some(arg) => arg,
            None => return Err("No text provided."),
        };
        let file: String = match args.next() {
            Some(arg) => arg,
            None => return Err("No file provided."),
        };
        let _check: String = match args.next() {
            Some(_arg) => return Err("Too many args!"),
            None => String::from("OK"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            string,
            file,
            ignore_case,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.string, &contents)
    } else {
        search(&config.string, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(string: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(string))
        .collect()
}

pub fn search_case_insensitive<'a>(string: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(string))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
