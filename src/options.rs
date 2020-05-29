use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pijama", about = "The Pijama compiler")]
pub struct Options {
    #[structopt(name = "INPUT", help = "Path to the input file.")]
    pub path: String,

    #[structopt(long = "--overflow-check", help = "Execution panics on integer overflow")]
    pub overflow_check: bool
}
