use crate::parser::{rule::Rule, ParseError, ParseResult, Parser};

pub struct Space;

impl<'a> Rule<'a> for Space {
    fn lookahead(parser: &Parser<'a>) -> bool {
        parser.check_next(|b| b.is_ascii_whitespace())
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        while {
            parser.advance();
            Self::lookahead(parser)
        } {}
        Ok(Self)
    }
}

pub struct EOF;

impl<'a> Rule<'a> for EOF {
    fn lookahead(parser: &Parser<'a>) -> bool {
        !parser.check_next(|_| true)
    }

    fn consume(_parser: &mut Parser<'a>) -> ParseResult<Self> {
        Err(ParseError::UnexpectedEOF)
    }
}
