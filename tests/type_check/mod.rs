use pijama::{parser, mir, LangResult};
use pijama::ty::Ty;

mod error;

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    ty::ty_check(&mir)
}