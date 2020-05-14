use pijama::ty::Ty;
use pijama::{mir, parser, ty, LangResult};

mod fail;
mod pass;

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    ty::ty_check(&mir)
}

#[macro_export]
macro_rules! test_type {
    ($name:ident, $pattern:pat) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!(stringify!($name), ".pj"));
            let ty = crate::type_check::type_check(input);
            assert!(matches!(ty, $pattern));
        }
    }
}
