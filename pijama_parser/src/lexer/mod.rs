use logos::{Logos, SpannedIter};

use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

use pijama_common::location::{Located, Location};

mod raw;

use raw::RawToken;

#[derive(Debug, Clone)]

pub enum LexError {
    Internal,
    Custom(&'static str),
}

pub struct Lexer<'a> {
    inner: SpannedIter<'a, RawToken<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn from_input(input: &'a str) -> Self {
        Lexer {
            inner: RawToken::lexer(input).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, Token<'a>, usize), Located<LexError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (raw, span) = self.inner.next()?;
        Some(
            Token::try_from(raw)
                .map(|token| (span.start, token, span.end))
                .map_err(|err| Location::new(span.start, span.end).with_content(err)),
        )
    }
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Newline,
    Int(i64),
    Ident(&'a str),
    Kword(Keyword),
    Op(Operator),
    Sym(Symbol),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Token::Newline => write!(f, "\\n"),
            Token::Int(int) => write!(f, "{}", int),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Kword(kw) => write!(f, "{}", kw),
            Token::Op(op) => write!(f, "{}", op),
            Token::Sym(sym) => write!(f, "{}", sym),
        }
    }
}

impl<'a> TryFrom<RawToken<'a>> for Token<'a> {
    type Error = LexError;

    fn try_from(raw: RawToken<'a>) -> Result<Self, Self::Error> {
        match raw {
            RawToken::Newline => Ok(Token::Newline),
            RawToken::Int(int) => Ok(Token::Int(int)),
            RawToken::Ident(ident) => Ok(Token::Ident(ident)),
            RawToken::Fn => Ok(Token::Kword(Keyword::Fn)),
            RawToken::If => Ok(Token::Kword(Keyword::If)),
            RawToken::Do => Ok(Token::Kword(Keyword::Do)),
            RawToken::End => Ok(Token::Kword(Keyword::End)),
            RawToken::Else => Ok(Token::Kword(Keyword::Else)),
            RawToken::Elif => Ok(Token::Kword(Keyword::Elif)),
            RawToken::True => Ok(Token::Kword(Keyword::True)),
            RawToken::False => Ok(Token::Kword(Keyword::False)),
            RawToken::Unit => Ok(Token::Kword(Keyword::Unit)),
            RawToken::IntTy => Ok(Token::Kword(Keyword::IntTy)),
            RawToken::BoolTy => Ok(Token::Kword(Keyword::BoolTy)),
            RawToken::UnitTy => Ok(Token::Kword(Keyword::UnitTy)),
            RawToken::Print => Ok(Token::Kword(Keyword::Print)),
            RawToken::Add => Ok(Token::Op(Operator::Add)),
            RawToken::Sub => Ok(Token::Op(Operator::Sub)),
            RawToken::Mul => Ok(Token::Op(Operator::Mul)),
            RawToken::Div => Ok(Token::Op(Operator::Div)),
            RawToken::Rem => Ok(Token::Op(Operator::Rem)),
            RawToken::BitAnd => Ok(Token::Op(Operator::BitAnd)),
            RawToken::BitOr => Ok(Token::Op(Operator::BitOr)),
            RawToken::BitXor => Ok(Token::Op(Operator::BitXor)),
            RawToken::Shr => Ok(Token::Op(Operator::Shr)),
            RawToken::Shl => Ok(Token::Op(Operator::Shl)),
            RawToken::Not => Ok(Token::Op(Operator::Not)),
            RawToken::And => Ok(Token::Op(Operator::And)),
            RawToken::Or => Ok(Token::Op(Operator::Or)),
            RawToken::Eq => Ok(Token::Op(Operator::Eq)),
            RawToken::Neq => Ok(Token::Op(Operator::Neq)),
            RawToken::Gt => Ok(Token::Op(Operator::Gt)),
            RawToken::Lt => Ok(Token::Op(Operator::Lt)),
            RawToken::Gte => Ok(Token::Op(Operator::Gte)),
            RawToken::Lte => Ok(Token::Op(Operator::Lte)),
            RawToken::Assign => Ok(Token::Op(Operator::Assign)),
            RawToken::Colon => Ok(Token::Op(Operator::Colon)),
            RawToken::Arrow => Ok(Token::Op(Operator::Arrow)),
            RawToken::LParen => Ok(Token::Sym(Symbol::LParen)),
            RawToken::RParen => Ok(Token::Sym(Symbol::RParen)),
            RawToken::Comma => Ok(Token::Sym(Symbol::Comma)),
            RawToken::Error => Err(LexError::Internal),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Fn,
    If,
    Do,
    End,
    Else,
    Elif,
    True,
    False,
    Unit,
    IntTy,
    BoolTy,
    UnitTy,
    Print,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Keyword::Fn => write!(f, "fn"),
            Keyword::If => write!(f, "if"),
            Keyword::Do => write!(f, "do"),
            Keyword::End => write!(f, "end"),
            Keyword::Else => write!(f, "else"),
            Keyword::Elif => write!(f, "elif"),
            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),
            Keyword::Unit => write!(f, "unit"),
            Keyword::IntTy => write!(f, "Int"),
            Keyword::BoolTy => write!(f, "Bool"),
            Keyword::UnitTy => write!(f, "Unit"),
            Keyword::Print => write!(f, "print"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shr,
    Shl,
    Not,
    And,
    Or,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Assign,
    Colon,
    Arrow,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Rem => write!(f, "%"),
            Operator::BitAnd => write!(f, "&"),
            Operator::BitOr => write!(f, "|"),
            Operator::BitXor => write!(f, "^"),
            Operator::Shr => write!(f, ">>"),
            Operator::Shl => write!(f, "<<"),
            Operator::Not => write!(f, "!"),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Eq => write!(f, "=="),
            Operator::Neq => write!(f, "!="),
            Operator::Gt => write!(f, ">"),
            Operator::Lt => write!(f, "<"),
            Operator::Gte => write!(f, ">="),
            Operator::Lte => write!(f, "<="),
            Operator::Assign => write!(f, "="),
            Operator::Colon => write!(f, ":"),
            Operator::Arrow => write!(f, "->"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    LParen,
    RParen,
    Comma,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Symbol::LParen => write!(f, "("),
            Symbol::RParen => write!(f, ")"),
            Symbol::Comma => write!(f, ","),
        }
    }
}
