use super::{ParseResult, Parser};

pub(super) trait Rule<'a>: Sized {
    fn lookahead(parser: &Parser<'a>) -> bool;

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self>;

    fn apply(parser: &mut Parser<'a>) -> Option<ParseResult<Self>> {
        if Self::lookahead(parser) {
            Some(Self::consume(parser))
        } else {
            None
        }
    }
}
