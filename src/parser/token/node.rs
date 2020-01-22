use super::*;
use crate::parser::{
    rule::{OrRule, Rule},
    ASTNode, ParseErrorKind, ParseResult, Parser,
};

pub type Node<'a> = OrRule<Atom<'a>, Seq<'a>>;

impl<'a> Into<ASTNode<'a>> for Node<'a> {
    fn into(self) -> ASTNode<'a> {
        match self {
            OrRule::Left(atom) => atom.into(),
            OrRule::Right(Seq(nodes)) => ASTNode::Seq(nodes.into_iter().map(Self::into).collect()),
        }
    }
}

pub struct Seq<'a>(Vec<Node<'a>>);

impl<'a> Rule<'a> for Seq<'a> {
    fn lookahead(parser: &Parser<'a>) -> bool {
        LBracket::lookahead(parser)
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        let mut nodes = Vec::new();
        LBracket::consume(parser)?;
        parser.consume_space();
        while let Some(node) = Node::apply(parser) {
            nodes.push(node?);
            if !(parser.consume_space() || RBracket::lookahead(parser)) {
                let chr = parser.curr_char().unwrap();
                return parser.error(ParseErrorKind::UnexpectedChar(chr))
            }
        }
        RBracket::apply(parser).unwrap_or_else(|| parser.error(ParseErrorKind::BracketMismatch))?;
        Ok(Seq(nodes))
    }
}

pub type Atom<'a> = OrRule<OrRule<Name<'a>, Number<'a>>, Operator<'a>>;

impl<'a> Into<ASTNode<'a>> for Atom<'a> {
    fn into(self) -> ASTNode<'a> {
        match self {
            OrRule::Left(OrRule::Left(Name(s))) | OrRule::Left(OrRule::Right(Number(s))) | OrRule::Right(Operator(s)) => ASTNode::Atom(s)
        }
    }
}

pub struct Name<'a>(&'a str);

impl<'a> Rule<'a> for Name<'a> {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|b| b.is_ascii_alphabetic())
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        let begin = parser.index;
        while {
            parser.advance();
            if parser.check_next(|&b| b'-' == b) {
                parser.advance();
            }
            parser.check_next(|b| b.is_ascii_alphanumeric())
        } {}
        let bytes = parser.text.get(begin..parser.index).unwrap();
        Ok(Name(std::str::from_utf8(bytes).unwrap()))
    }
}

pub struct Number<'a>(&'a str);

impl<'a> Rule<'a> for Number<'a> {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|b| b.is_ascii_digit() || *b == b'-')
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        let begin = parser.index;
        while {
            parser.advance();
            parser.check_next(|b| b.is_ascii_digit())
        } {}
        let bytes = parser.text.get(begin..parser.index).unwrap();
        Ok(Number(std::str::from_utf8(bytes).unwrap()))
    }
}

pub struct Operator<'a>(&'a str);

impl<'a> Rule<'a> for Operator<'a> {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|&b| b == b'+' || b == b'-' || b == b'*' || b == b'/' || b == b'=')
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        let begin = parser.index;
        parser.advance();
        let bytes = parser.text.get(begin..parser.index).unwrap();
        Ok(Operator(std::str::from_utf8(bytes).unwrap()))
    }
}
