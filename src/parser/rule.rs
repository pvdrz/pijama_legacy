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

pub enum OrRule<A, B> {
    Left(A),
    Right(B),
}

impl<'a, A, B> Rule<'a> for OrRule<A, B>  where A: Rule<'a>, B: Rule<'a> {
    fn lookahead(parser: &Parser<'a>) -> bool {
        A::lookahead(parser) || B::lookahead(parser)
    }

    fn consume(parser: &mut Parser<'a>) -> ParseResult<Self> {
        if A::lookahead(parser) {
            A::consume(parser).map(Self::Left)
        } else {
            B::consume(parser).map(Self::Right)
        }
    }

    fn apply(parser: &mut Parser<'a>) -> Option<ParseResult<Self>> {
        if A::lookahead(parser) {
            Some(A::consume(parser).map(Self::Left))
        } else if B::lookahead(parser) {
            Some(B::consume(parser).map(Self::Right))
        } else {
            None
        }
    }

}
