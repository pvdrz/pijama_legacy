#![feature(box_patterns)]

use thiserror::Error;

use machine::Machine;
use parser::ParseError;
use ty::TyError;

pub mod ast;
pub mod lir;
pub mod machine;
pub mod mir;
pub mod parser;
pub mod ty;

pub type LangResult<'a, T> = Result<T, LangError<'a>>;

#[derive(Error, Debug)]
pub enum LangError<'a> {
    #[error("Type error: {0}")]
    Ty(#[from] TyError),
    #[error("Parse error: {0}")]
    Parse(ParseError<'a>),
}

impl<'a> From<ParseError<'a>> for LangError<'a> {
    fn from(err: ParseError<'a>) -> Self {
        LangError::Parse(err)
    }
}
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub fn display_error<'a>(input: &str, path:&str, error: LangError<'a>) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let mut files = SimpleFiles::new();

    let file_id = files.add(path, input);

    match error {
        LangError::Ty(error) => eprintln!("{}", error),
        LangError::Parse(error) => {
            let span = error.span;
            let begin = span.location_offset();
            let diagnostic = Diagnostic::error()
                .with_message("Parsing failed")
                .with_labels(vec![
                    Label::primary(file_id, begin..input.len()).with_message(format!("{}", error))
                ]);

            codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
                .unwrap();
        }
    }
}

pub fn run(input: &str) -> LangResult<lir::Term> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    let _ty = ty::ty_check(&mir)?;
    let lir = lir::Term::from_mir(mir);
    let res = Machine::default().evaluate(lir);
    Ok(res)
}
