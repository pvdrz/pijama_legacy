use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
    },
};
use structopt::StructOpt;

use pijama_driver::{LangError, LangErrorKind};

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

pub fn display_error(input: &str, path: &str, error: &LangError) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let mut files = SimpleFiles::new();

    let file_id = files.add(path, input);

    let loc = error.loc();
    let msg = match error.kind() {
        LangErrorKind::Ty(_) => "Type error",
        LangErrorKind::Parse(_) => "Parsing error",
        LangErrorKind::Lower(_) => "Lowering error",
    };

    let diagnostic = Diagnostic::error()
        .with_message(msg)
        .with_labels(vec![
            Label::primary(file_id, loc.start..loc.end).with_message(error.to_string())
        ]);

    emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}
