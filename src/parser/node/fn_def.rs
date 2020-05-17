use nom::{error::ParseError, IResult};

use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, space0, space1},
    combinator::{map, opt},
    multi::separated_list,
    sequence::{delimited, pair, preceded, terminated, tuple},
};

use crate::ast::{Block, Name, Node};
use crate::parser::{
    block::block0,
    helpers::{in_brackets, surrounded},
    name::name,
    ty::{binding, type_binding},
};

pub fn fn_def<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        tuple((
            fn_name,
            surrounded(args(binding), space0),
            terminated(opt(type_binding), multispace0),
            fn_body,
        )),
        |(name, args, opt_ty, body)| Node::FnDef(name, args, body, opt_ty),
    )(input)
}

pub fn args<'a, O, E: ParseError<&'a str>>(
    content: impl Fn(&'a str) -> IResult<&'a str, O, E>,
) -> impl Fn(&'a str) -> IResult<&'a str, Vec<O>, E> {
    in_brackets(separated_list(
        delimited(space0, char(','), multispace0),
        content,
    ))
}

fn fn_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Name, E> {
    preceded(pair(tag("fn"), space1), name)(input)
}

pub fn fn_body<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    delimited(
        pair(tag("do"), multispace1),
        block0,
        pair(multispace0, tag("end")),
    )(input)
}
