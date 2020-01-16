use crate::parser::{rule::Rule, ParseResult, Parser};

pub struct LBracket;

impl<'a> Rule<'a> for LBracket {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|&b| b == b'(')
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        parser.advance();
        Ok(Self)
    }
}

pub struct RBracket;

impl<'a> Rule<'a> for RBracket {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|&b| b == b')')
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        parser.advance();
        Ok(Self)
    }
}
