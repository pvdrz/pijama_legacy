//! Parsers for recursive function definitions.
//!
//! The entry point for this module is the [`fn_def`] function. Function definitions are parsed following the
//! rule
//!
//! ```abnf
//! fn_rec_def = "fn" "rec" name "(" (binding ("," binding)*)? ")" ":" ty "do" block1 "end"
//! ```
//!
//! In contrast with the [`fn_def`] parser, the return type binding here is mandatory. Most of the
//! code in this module has the same logic as the one in the [`fn_def`] module.
//!
//! [`fn_def`]: super::fn_def
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0, space1},
    combinator::map,
    sequence::{separated_pair, terminated, tuple},
};
use nom_locate::position;

use crate::{
    ast::{Located, Location, Name, Node},
    parser::{
        helpers::surrounded,
        name::name,
        node::fn_def::{args, fn_body},
        ty::{binding, colon_ty},
        IResult, Span,
    },
};

/// Parses a [`FnDef`].
///
/// The spacing works the same as with function definitions module.
pub fn fn_rec_def(input: Span) -> IResult<Located<Node>> {
    map(
        tuple((
            fn_rec_name,
            surrounded(args(binding), space0),
            terminated(colon_ty, multispace0),
            fn_body,
        )),
        |(name, args, ty, body)| {
            let loc1 = name.loc;
            let loc2 = body.loc;
            Located::new(
                Node::FnRecDef(name.content, args.content, body.content, ty),
                loc1 + loc2,
            )
        },
    )(input)
}

/// Parses the name of a recursive function in a definition.
///
/// This parser requires that the name is preceded by `"fn"`, at least one space, `"rec"` and at
/// least another space.
fn fn_rec_name(input: Span) -> IResult<Located<Located<Name>>> {
    map(
        separated_pair(
            position,
            tuple((tag("fn"), space1, tag("rec"), space1)),
            name,
        ),
        |(span, name)| {
            let loc1 = Location::from(span);
            let loc2 = name.loc;
            Located::new(name, loc1 + loc2)
        },
    )(input)
}
