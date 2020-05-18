use nom::{error::ParseError, IResult};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0, space1},
    combinator::map,
    sequence::{preceded, terminated, tuple},
};

use crate::ast::{Name, Node};
use crate::parser::{
    helpers::surrounded,
    name::name,
    ty::{binding, colon_ty},
};

use super::fn_def::{args, fn_body};

pub fn fn_rec_def<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        tuple((
            fn_rec_name,
            surrounded(args(binding), space0),
            terminated(colon_ty, multispace0),
            fn_body,
        )),
        |(name, args, ty, body)| Node::FnRecDef(name, args, body, ty),
    )(input)
}

fn fn_rec_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Name, E> {
    preceded(tuple((tag("fn"), space1, tag("rec"), space1)), name)(input)
}
