use structopt::StructOpt;

use std::fs::read_to_string;

use pijama::{display_error, Options};
use pijama_driver::run;

fn main() {
    let options = Options::from_args();

    let input = match read_to_string(&options.path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match run(&input, options.machine_opts.overflow_check) {
        Ok(()) => (),
        Err(err) => display_error(&input, &options.path, &err),
    }
}
