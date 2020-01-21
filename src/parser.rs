mod result;
mod rule;
mod token;

pub use result::*;
use rule::*;
use token::*;

#[derive(Debug)]
pub enum ASTNode<'a> {
    Atom(&'a str),
    Seq(Vec<ASTNode<'a>>),
}

pub fn parse<'a>(text: &'a str) -> ParseResult<Vec<ASTNode<'a>>> {
    Parser {
        index: 0,
        text: text.as_bytes(),
    }
    .collect()
}

struct Parser<'a> {
    index: usize,
    text: &'a [u8],
}

impl<'a> Parser<'a> {
    fn curr_char(&self) -> Option<char> {
        self.text.get(self.index).map(|&b| b.into())
    }

    fn check_next<F: Fn(&u8) -> bool>(&self, f: F) -> bool {
        self.text.get(self.index).map(f).unwrap_or(false)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn consume_space(&mut self) -> bool {
        Space::apply(self).is_some()
    }

    fn error<T>(&self, kind: ParseErrorKind) -> ParseResult<T> {
        Err(ParseError::new(self.index, kind))
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = ParseResult<ASTNode<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_space();
        if !EOF::lookahead(self) {
            if let Some(node) = Node::apply(self) {
                Some(node.map(Node::into))
            } else {
                let chr = self.curr_char().unwrap();
                Some(self.error(ParseErrorKind::UnexpectedChar(chr)))
            }
        } else {
            None
        }
    }
}
