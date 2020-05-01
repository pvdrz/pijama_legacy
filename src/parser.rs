use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alpha1, char, digit1, line_ending, multispace0, multispace1, space0, space1,
};
use nom::combinator::{all_consuming, map, opt, recognize, verify};
use nom::multi::{many0, separated_list, separated_nonempty_list};
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::*;
use crate::ty::{Ty, Binding};

pub fn parse<'a>(input: &'a str) -> Option<Vec<Node<'a>>> {
    let result = all_consuming(tuple((multispace0, Node::parse_block0, multispace0)))(input);
    dbg!(result).ok().map(|(_, (_, node, _))| node)
}

const KEYWORDS: &[&'static str] = &[
    "fn", "do", "end", "if", "else", "true", "false", "unit", "Bool", "Int", "Unit",
];

impl<'a> Name<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            verify(
                recognize(tuple((alpha1, many0(tuple((char('_'), alpha1)))))),
                |s| !KEYWORDS.contains(s),
            ),
            |name| Name(name),
        )(input)
    }
}

impl Ty {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rem, ty) = Self::parse_base(input)?;
        if let (rem, Some((_, _, _, ty2))) =
            opt(tuple((space1, tag("->"), space1, Self::parse)))(rem)?
        {
            Ok((rem, Self::Arrow(Box::new(ty), Box::new(ty2))))
        } else {
            Ok((rem, ty))
        }
    }

    fn parse_base(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("Bool"), |_| Self::Bool),
            map(tag("Int"), |_| Self::Int),
            map(tag("Unit"), |_| Self::Unit),
            map(tuple((char('('), Self::parse, char(')'))), |(_, ty, _)| ty),
        ))(input)
    }
}

impl<'a> Binding<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((Name::parse, tuple((space0, char(':'), space0)), Ty::parse)),
            |(name, _, ty)| Binding { name, ty },
        )(input)
    }
}

impl BinOp {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(char('+'), |_| Self::Plus),
            map(char('-'), |_| Self::Minus),
            map(char('*'), |_| Self::Times),
            map(char('/'), |_| Self::Divide),
            map(char('%'), |_| Self::Modulo),
            map(tag("<="), |_| Self::LessThanOrEqual),
            map(tag(">="), |_| Self::GreaterThanOrEqual),
            map(char('<'), |_| Self::LessThan),
            map(char('>'), |_| Self::GreaterThan),
            map(tag("=="), |_| Self::Equal),
            map(tag("!="), |_| Self::NotEqual),
            map(tag("&&"), |_| Self::And),
            map(tag("||"), |_| Self::Or),
        ))(input)
    }
}

impl UnOp {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(char('!'), |_| Self::Not),
            map(char('-'), |_| Self::Minus),
        ))(input)
    }
}

impl Literal {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("true"), |_| Self::True),
            map(tag("false"), |_| Self::False),
            map(tag("unit"), |_| Self::Unit),
            map(
                tuple((opt(char('-')), digit1)),
                |(sign, digits): (Option<char>, &str)| {
                    let mut number = digits.parse().unwrap();
                    if sign.is_some() {
                        number *= -1;
                    }
                    Self::Number(number)
                },
            ),
        ))(input)
    }
}

impl<'a> Node<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (mut input, mut node) = Self::parse_base(input)?;
        while let (rem, Some((_, op, _, node2))) =
            opt(tuple((space0, BinOp::parse, space0, Self::parse_base)))(input)?
        {
            input = rem;
            node = Self::BinaryOp(op, Box::new(node), Box::new(node2));
        }
        Ok((input, node))
    }

    fn parse_base(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            Self::parse_let_bind,
            Self::parse_cond,
            Self::parse_fn,
            map(Literal::parse, |literal| Self::Literal(literal)),
            Self::parse_unary_op,
            Self::parse_call,
            map(Name::parse, |name| Self::Name(name)),
            map(
                tuple((char('('), space0, Self::parse, space0, char(')'))),
                |(_, _, node, _, _)| node,
            ),
        ))(input)
    }

    fn parse_call(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                Name::parse,
                tuple((space0, char('('), multispace0)),
                separated_list(tuple((space0, char(','), multispace0)), Self::parse),
                tuple((multispace0, char(')'), space0)),
            )),
            |(name, _, args, _)| Self::Call(name, args),
        )(input)
    }

    fn parse_cond(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                tuple((tag("if"), multispace1)),
                Self::parse_block1,
                tuple((multispace0, tag("do"), multispace1)),
                Self::parse_block0,
                opt(map(
                    tuple((multispace0, tag("else"), multispace1, Self::parse_block0)),
                    |(_, _, _, block)| block,
                )),
                tuple((multispace0, tag("end"))),
            )),
            |(_, if_block, _, do_block, else_block, _)| {
                Self::Cond(if_block, do_block, else_block.unwrap_or(Vec::new()))
            },
        )(input)
    }

    fn parse_let_bind(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((Name::parse, tuple((space0, char('='), space0)), Self::parse)),
            |(name, _, node)| Self::LetBind(name, Box::new(node)),
        )(input)
    }

    fn parse_unary_op(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((UnOp::parse, space0, Self::parse_base)),
            |(un_op, _, node)| Self::UnaryOp(un_op, Box::new(node)),
        )(input)
    }

    fn parse_fn(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                tuple((tag("fn"), space1)),
                Name::parse,
                tuple((space0, char('('), multispace0)),
                separated_list(tuple((space0, char(','), multispace0)), Binding::parse),
                tuple((multispace0, char(')'), space0)),
                opt(map(
                    tuple((char(':'), space0, Ty::parse, space0)),
                    |(_, _, ty, _)| ty,
                )),
                tuple((tag("do"), multispace1)),
                Self::parse_block0,
                tuple((multispace0, tag("end"))),
            )),
            |(_, name, _, args, _, opt_ty, _, body, _)| Self::FnDef(name, args, opt_ty, body),
        )(input)
    }

    fn parse_block0(input: &'a str) -> IResult<&'a str, Vec<Self>> {
        separated_list(
            line_ending,
            map(tuple((multispace0, Self::parse)), |(_, node)| node),
        )(input)
    }

    fn parse_block1(input: &'a str) -> IResult<&'a str, Vec<Self>> {
        separated_nonempty_list(
            line_ending,
            map(tuple((multispace0, Self::parse)), |(_, node)| node),
        )(input)
    }
}
