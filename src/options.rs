use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pijama", about = "The Pijama compiler")]
pub struct Options {
    #[structopt(name = "INPUT", help = "Path to the input file.")]
    pub path: String,
    #[structopt(flatten)]
    pub machine_opts: MachineOptions,
}

#[derive(Debug, StructOpt)]
pub struct MachineOptions {
    #[structopt(
        long = "--overflow-check",
        help = "Execution panics on integer overflow"
    )]
    // If the flag is not passed, the default value is `false`.
    pub overflow_check: bool,
}
