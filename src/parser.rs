use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alpha1, char, digit1, line_ending, multispace0, multispace1, space0, space1,
};
use nom::combinator::{all_consuming, cut, map, map_opt, opt, peek, recognize, verify};
use nom::error::{convert_error, ParseError, VerboseError};
use nom::multi::{separated_list, separated_nonempty_list};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::ast::{BinOp, Literal, Name, Node, UnOp};
use crate::ty::{Binding, Ty};
use crate::{LangError, LangResult};

pub fn parse<'a>(input: &'a str) -> LangResult<Vec<Node<'a>>> {
    let result: IResult<&str, Vec<Node>, VerboseError<&str>> =
        all_consuming(surrounded(block0, multispace0))(input);
    match result {
        Ok((_, block)) => Ok(block),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            Err(LangError::Parse(convert_error(input, e)))
        }
        _ => Err(LangError::Parse(String::new())),
    }
}

fn surrounded<I, O, O2, E: ParseError<I>>(
    content: impl Fn(I) -> IResult<I, O, E>,
    delimiter: impl Fn(I) -> IResult<I, O2, E> + Copy,
) -> impl Fn(I) -> IResult<I, O, E> {
    delimited(delimiter, content, delimiter)
}

fn in_brackets<'a, O, E: ParseError<&'a str>>(
    content: impl Fn(&'a str) -> IResult<&'a str, O, E>,
) -> impl Fn(&'a str) -> IResult<&'a str, O, E> {
    delimited(char('('), content, char(')'))
}

fn base_ty<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Ty, E> {
    alt((
        map(tag("Bool"), |_| Ty::Bool),
        map(tag("Int"), |_| Ty::Int),
        map(tag("Unit"), |_| Ty::Unit),
        delimited(char('('), ty, char(')')),
    ))(input)
}

fn ty<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Ty, E> {
    let (rem, t1) = base_ty(input)?;
    if let (rem, Some(t2)) = opt(preceded(surrounded(tag("->"), space1), ty))(rem)? {
        Ok((rem, Ty::Arrow(Box::new(t1), Box::new(t2))))
    } else {
        Ok((rem, t1))
    }
}

fn binding<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Binding, E> {
    map(
        separated_pair(name, surrounded(char(':'), space0), ty),
        |(name, ty)| Binding { name, ty },
    )(input)
}

const KEYWORDS: &[&str] = &[
    "fn", "do", "end", "if", "else", "true", "false", "unit", "Bool", "Int", "Unit", "rec",
];

fn name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Name<'a>, E> {
    map(
        verify(recognize(separated_nonempty_list(char('_'), alpha1)), |s| {
            !KEYWORDS.contains(s)
        }),
        Name,
    )(input)
}

fn bin_op<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BinOp, E> {
    use BinOp::*;
    alt((
        map(char('+'), |_| Plus),
        map(char('-'), |_| Minus),
        map(char('*'), |_| Times),
        map(char('/'), |_| Divide),
        map(char('%'), |_| Modulo),
        map(tag("<="), |_| LessThanOrEqual),
        map(tag(">="), |_| GreaterThanOrEqual),
        map(char('<'), |_| LessThan),
        map(char('>'), |_| GreaterThan),
        map(tag("=="), |_| Equal),
        map(tag("!="), |_| NotEqual),
        map(tag("&&"), |_| And),
        map(tag("||"), |_| Or),
        map(char('&'), |_| BitAnd),
        map(char('|'), |_| BitOr),
        map(char('^'), |_| BitXor),
    ))(input)
}

fn un_op<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, UnOp, E> {
    alt((
        map(char('!'), |_| UnOp::Not),
        map(char('-'), |_| UnOp::Minus),
    ))(input)
}

fn literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Literal, E> {
    alt((
        map(tag("true"), |_| Literal::Bool(true)),
        map(tag("false"), |_| Literal::Bool(false)),
        map(tag("unit"), |_| Literal::Unit),
        map(number, |n| Literal::Number(n)),
    ))(input)
}

fn number<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    map_opt(
        pair(opt(char('-')), digit1),
        |(sign, digits): (Option<char>, &str)| {
            let mut number = digits.parse::<i128>().ok()?;
            if sign.is_some() {
                number *= -1;
            }
            Some(number)
        },
    )(input)
}

fn node<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    let (mut input, mut node) = base_node(input)?;
    while let (rem, Some((op, node2))) =
        opt(pair(surrounded(bin_op, space0), cut(base_node)))(input)?
    {
        input = rem;
        node = Node::BinaryOp(op, Box::new(node), Box::new(node2));
    }
    Ok((input, node))
}

fn base_node<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    alt((
        (in_brackets(surrounded(node, space0))),
        map(literal, Node::Literal),
        preceded(peek(tag("if")), cut(cond)),
        preceded(peek(tuple((tag("fn"), space1, tag("rec")))), cut(func_rec)),
        preceded(peek(tag("fn")), cut(func)),
        preceded(
            peek(name),
            cut(alt((let_bind, call, map(name, Node::Name)))),
        ),
        preceded(peek(alt((tag("-"), tag("!")))), cut(un_oper)),
    ))(input)
}

fn call<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        pair(
            name,
            surrounded(
                in_brackets(surrounded(
                    separated_list(delimited(space0, char(','), multispace0), node),
                    multispace0,
                )),
                space0,
            ),
        ),
        |(name, args)| Node::Call(name, args),
    )(input)
}

fn cond<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        terminated(
            tuple((
                delimited(
                    pair(tag("if"), multispace1),
                    block1,
                    delimited(multispace0, tag("do"), multispace1),
                ),
                block0,
                opt(preceded(
                    delimited(multispace0, tag("else"), multispace1),
                    block0,
                )),
            )),
            pair(multispace0, tag("end")),
        ),
        |(if_block, do_block, else_block)| {
            Node::Cond(if_block, do_block, else_block.unwrap_or_default())
        },
    )(input)
}

fn let_bind<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        separated_pair(name, tuple((space0, char('='), space0)), node),
        |(name, node)| Node::LetBind(name, Box::new(node)),
    )(input)
}

fn un_oper<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(separated_pair(un_op, space0, base_node), |(un_op, node)| {
        Node::UnaryOp(un_op, Box::new(node))
    })(input)
}

fn func<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        tuple((
            pair(tag("fn"), space1),
            name,
            tuple((space0, char('('), multispace0)),
            separated_list(tuple((space0, char(','), multispace0)), binding),
            tuple((multispace0, char(')'), space0)),
            tuple((tag("do"), multispace1)),
            block0,
            tuple((multispace0, tag("end"))),
        )),
        |(_, name, _, args, _, _, body, _)| Node::FnDef(name, args, body, None),
    )(input)
}

fn func_rec<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        tuple((
            preceded(tuple((tag("fn"), space1, tag("rec"), space1)), name),
            surrounded(
                in_brackets(surrounded(
                    separated_list(delimited(space0, char(','), multispace0), binding),
                    multispace0,
                )),
                space0,
            ),
            preceded(pair(char(':'), space0), ty),
            delimited(
                delimited(space0, tag("do"), multispace1),
                block0,
                pair(multispace0, tag("end")),
            ),
        )),
        |(name, args, ty, body)| Node::FnDef(name, args, body, Some(ty)),
    )(input)
}

fn block0<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<Node>, E> {
    separated_list(line_ending, preceded(multispace0, node))(input)
}

fn block1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<Node>, E> {
    separated_nonempty_list(line_ending, preceded(multispace0, node))(input)
}
