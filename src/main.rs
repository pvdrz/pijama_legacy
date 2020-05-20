use std::{env::args, fs::read_to_string};

use pijama::{display_error, run};

fn main() {
    let mut args = args();
    args.next().unwrap();
    let path = args.next().expect("no path to source code");
    let input = read_to_string(&path).unwrap();
    match run(&input) {
        Ok(term) => println!("{}", term),
        Err(e) => display_error(&input, &path, &e),
    }
}
