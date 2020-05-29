pub mod lir;
pub mod machine;
pub mod mir;
pub mod options;
pub mod parser;
pub mod ty;

use thiserror::Error;

use std::io::Write;

use pijama_ast::Location;

use machine::{arithmetic::OverflowArithmetic, Machine};
use mir::LowerError;
use parser::ParsingError;
use ty::TyError;

pub type LangResult<'a, T> = Result<T, LangError<'a>>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LangError<'a> {
    #[error("{0}")]
    Ty(#[from] TyError),
    #[error("{0}")]
    Parse(ParsingError<'a>),
    #[error("{0}")]
    Lower(#[from] LowerError),
}

impl<'a> From<ParsingError<'a>> for LangError<'a> {
    fn from(err: ParsingError<'a>) -> Self {
        LangError::Parse(err)
    }
}

pub fn display_error<'a>(input: &str, path: &str, error: &LangError<'a>) {
    use codespan_reporting::{
        diagnostic::{Diagnostic, Label},
        files::SimpleFiles,
        term::{
            emit,
            termcolor::{ColorChoice, StandardStream},
        },
    };

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let mut files = SimpleFiles::new();

    let file_id = files.add(path, input);

    let (msg, loc) = match &error {
        LangError::Ty(error) => ("Type error", error.loc()),
        LangError::Parse(error) => ("Parsing error", Location::from(error.span)),
        LangError::Lower(error) => ("Lowering error", error.loc()),
    };

    let diagnostic = Diagnostic::error()
        .with_message(msg)
        .with_labels(vec![
            Label::primary(file_id, loc.start..loc.end).with_message(error.to_string())
        ]);

    emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}

pub fn run(input: &str) -> LangResult<lir::Term> {
    run_with_machine(input, Machine::default())
}

pub fn run_with_machine<W: Write>(
    input: &str,
    mut machine: Machine<W, OverflowArithmetic>,
) -> LangResult<lir::Term> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    let _ty = ty::ty_check(&mir)?;
    let lir = lir::Term::from_mir(mir);
    let res = machine.evaluate(lir);
    Ok(res)
}
