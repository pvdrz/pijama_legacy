use super::*;
use crate::parser::{rule::{Rule, OrRule}, ASTNode, ParseErrorKind, ParseResult, Parser};

pub type Node<'a> = OrRule<Atom<'a>, Seq<'a>>;

impl<'a> Into<ASTNode<'a>> for Node<'a> {
    fn into(self) -> ASTNode<'a> {
        match self {
            OrRule::Left(Atom(name)) => ASTNode::Atom(name),
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
            parser.consume_space();
        }
        RBracket::apply(parser).unwrap_or_else(|| parser.error(ParseErrorKind::BracketMismatch))?;
        Ok(Seq(nodes))
    }
}

pub struct Atom<'a>(&'a str);

impl<'a> Rule<'a> for Atom<'a> {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|b| b.is_ascii_alphanumeric())
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        let begin = parser.index;
        while {
            parser.advance();
            if parser.check_next(|&b| b'-' == b) {
                parser.advance();
            }
            Self::lookahead(parser)
        } {}
        let bytes = parser.text.get(begin..parser.index).unwrap();
        Ok(Atom(std::str::from_utf8(bytes).unwrap()))
    }
}
