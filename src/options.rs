use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pijama", about = "The Pijama compiler")]
pub struct Options {
    #[structopt(name = "INPUT", help = "Path to the input file.")]
    pub path: String,
}
