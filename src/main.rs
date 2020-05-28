use std::fs::read_to_string;

use pijama::{display_error, options::Options, run};

use structopt::StructOpt;

fn main() {
    let options = Options::from_args();

    let input = match read_to_string(&options.path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("{}", err);
            return ();
        }
    };

    match run(&input) {
        Ok(term) => println!("{}", term),
        Err(err) => display_error(&input, &options.path, &err),
    }
}
