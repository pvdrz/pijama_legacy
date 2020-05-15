use pijama::ty::Ty;
use pijama::{mir, parser, ty, LangResult};

mod fail;
mod pass;

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    Ok(ty::ty_check(&mir)?)
}

#[macro_export]
macro_rules! test_type {
    ($name:ident, $pattern:pat) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!(stringify!($name), ".pj"));
            let ty = crate::type_check::type_check(input);
            assert!(matches!(ty, $pattern), "{:#?}", ty);
        }
    };
}

#[macro_export]
macro_rules! test_type_with_placeholder {
    ($name:ident, $pattern:pat, $placeholder:tt, $( $replacement:tt ),*) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!(stringify!($name), ".pj"));
            let replacements = [
                $(
                    stringify!($replacement),
                )*
            ];
            for replacement in &replacements {
                let input = input.replace(stringify!($placeholder), replacement);
                let ty = crate::type_check::type_check(&input);
                assert!(matches!(ty, $pattern), "failed with replacement {}\n{:#?}", replacement, ty);
            }
        }
    };
}
