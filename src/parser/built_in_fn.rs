//! Parsers for built in functions.
//!
//! The entry point for this module is the [`built_in_fn`] function. All built in functions are
//! registered in the [`BUILT_IN_FNS`] constant.
use nom::{error::ParseError, IResult};

use nom::{
    character::complete::{alpha1, char},
    combinator::{map, recognize, verify},
    multi::separated_nonempty_list,
};

use crate::ast::{BuiltInFn, BUILT_IN_FNS};

/// Parser for [`BuiltInFn`]s.
pub fn built_in_fn<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BuiltInFn, E> {
    map(
        verify(
            recognize(separated_nonempty_list(char('_'), alpha1)),
            |s: &str| BUILT_IN_FNS.contains_key(s),
        ),
        |s| *BUILT_IN_FNS.get(s).unwrap(),
    )(input)
}
