use nom::{error::ParseError, IResult};

use nom::{character::complete::space0, combinator::map, sequence::separated_pair};

use crate::ast::Node;
use crate::parser::name::name;

use super::{fn_def::args, node};

pub fn call<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(separated_pair(name, space0, args(node)), |(name, args)| {
        Node::Call(name, args)
    })(input)
}
