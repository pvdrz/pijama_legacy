use pijama_parser::parse;

use pijama_common::location::LocatedError;
use pijama_driver::LangResult;
use pijama_ty::Ty;
use pijama_tycheck::ty_check;

mod fail;
mod pass;

pub fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parse(input).map_err(LocatedError::kind_into)?;
    let (hir, ctx) = pijama_hir::lower_ast(ast).map_err(LocatedError::kind_into)?;
    Ok(ty_check(&hir, ctx)
        .map_err(LocatedError::kind_into)?
        .0
        .content)
}

/// Create a test with `$name` that type checks a file with `$name`.pj
/// in the same directory against the `$pattern`.
#[macro_export]
macro_rules! test_type {
    ($name:ident, $pattern:expr) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!(stringify!($name), ".pj"));
            let ty = crate::type_check::type_check(input);
            let ty2 = match &ty {
                Ok(ty) => Ok(ty),
                Err(err) => Err(err.kind()),
            };
            assert_eq!(ty2, $pattern, "{:#?}", ty);
        }
    };
}

/// Create a test with `$name` that type checks a file with `$name`.pj
/// in the same directory against the `$pattern`. It does this once for
/// each `$replacement` that will replace the `$placeholder` in the file.
#[macro_export]
macro_rules! test_type_with_placeholder {
    ($name:ident, $pattern:expr, $placeholder:tt, $( $replacement:tt ),+) => {
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
                let ty2 = match &ty {
                    Ok(ty) => Ok(ty),
                    Err(err) => Err(err.kind()),
                };
                assert_eq!(ty2, $pattern,
                    "failed with replacement {}\n{:#?}",
                    replacement, ty);
            }
        }
    };
}

/// Create a test with `$name` that type checks a file with `$name`.pj
/// in the same directory against the `$pattern`. It does this once for
/// each int binary operator that will replace the `$placeholder` in the file.
#[macro_export]
macro_rules! test_type_for_all_integer_binops {
    ($name:ident, $pattern:expr, $placeholder:tt) => {
        crate::test_type_with_placeholder!(
            $name,
            $pattern,
            $placeholder,
            /, *, +, -, &, |, ^, <<, >>);
    };
}

/// Create a test with `$name` that type checks a file with `$name`.pj
/// in the same directory against the `$pattern`. It does this once for
/// each comparision operator that will replace the `$placeholder` in the file.
#[macro_export]
macro_rules! test_type_for_all_comparision_binops {
    ($name:ident, $pattern:expr, $placeholder:tt) => {
        crate::test_type_with_placeholder!(
            $name,
            $pattern,
            $placeholder,
            <, >, <=, >=);
    };
}

/// Create a test with `$name` that type checks a file with `$name`.pj
/// in the same directory against the `$pattern`. It does this once for
/// each equality operator that will replace the `$placeholder` in the file.
#[macro_export]
macro_rules! test_type_for_all_equality_binops {
    ($name:ident, $pattern:expr, $placeholder:tt) => {
        crate::test_type_with_placeholder!(
            $name,
            $pattern,
            $placeholder,
            ==, !=);
    };
}

/// Create a test with `$name` that type checks a file with `$name`.pj
/// in the same directory against the `$pattern`. It does this once for
/// each logical operator that will replace the `$placeholder` in the file.
#[macro_export]
macro_rules! test_type_for_all_logical_binops {
    ($name:ident, $pattern:expr, $placeholder:tt) => {
        crate::test_type_with_placeholder!(
            $name,
            $pattern,
            $placeholder,
            &&, ||);
    };
}
