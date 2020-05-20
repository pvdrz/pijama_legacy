use pijama::{mir, parser, ty, ty::Ty, LangResult};

mod fail;
mod pass;

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    Ok(ty::ty_check(&mir)?.content)
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
            assert_eq!(ty, $pattern, "{:#?}", ty);
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
                assert_eq!(ty, $pattern,
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
