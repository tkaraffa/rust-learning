use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: env::Args = env::args();
    let config = Config::from_args(args).unwrap_or_else(|err| {
        eprintln!("oops: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
