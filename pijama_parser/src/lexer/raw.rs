use logos::Logos;

use std::borrow::Cow;

#[derive(Logos, Debug, PartialEq)]
pub(super) enum RawToken<'a> {
    #[regex("(\n[ \t]*)")]
    #[regex(r"(#[^\n]*\n)")]
    Newline,
    #[regex(r"[0-9]+", |lex| lex_integer(lex.slice(), 10, false))]
    #[regex(r"-[0-9]+", |lex| lex_integer(lex.slice(), 10, true))]
    #[regex(r"0b[0-1]+", |lex| lex_integer(lex.slice(), 2, false))]
    #[regex(r"-0b[0-1]+", |lex| lex_integer(lex.slice(), 2, true))]
    #[regex(r"0o[0-7]+", |lex| lex_integer(lex.slice(), 8, false))]
    #[regex(r"-0o[0-7]+", |lex| lex_integer(lex.slice(), 8, true))]
    #[regex(r"0x[0-9a-f]+", |lex| lex_integer(lex.slice(), 16, false))]
    #[regex(r"-0x[0-9a-f]+", |lex| lex_integer(lex.slice(), 16, true))]
    Int(i64),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    Ident(&'a str),
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("do")]
    Do,
    #[token("end")]
    End,
    #[token("else")]
    Else,
    #[token("elif")]
    Elif,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("unit")]
    Unit,
    #[token("Int")]
    IntTy,
    #[token("Bool")]
    BoolTy,
    #[token("Unit")]
    UnitTy,
    #[token("print")]
    Print,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Rem,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("^")]
    BitXor,
    #[token(">>")]
    Shr,
    #[token("<<")]
    Shl,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token(">=")]
    Gte,
    #[token("<=")]
    Lte,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=")]
    Assign,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token(",")]
    Comma,
    #[error]
    #[regex(r"[ \t]+", logos::skip)]
    Error,
}

fn lex_integer(mut input: &str, radix: u32, is_neg: bool) -> Option<i64> {
    // Stores how many characters we need to remove from the string to keep just the digits.
    let mut offset: usize = is_neg.into();

    if let 2 | 8 | 16 = radix {
        offset += 2
    };
    // Keep just the digits
    input = &input[offset..];

    let digits = if is_neg {
        // Create a string with enough capacity for the number plus the sign to avoid unnecessary
        // allocations when prepending the sign This allows using the whole range of i64 numbers
        // without handling the i64::min() case ourselves
        let mut digits = String::with_capacity(input.len() + 1);
        digits.push('-');
        digits.push_str(input);
        Cow::from(digits)
    } else {
        Cow::from(input)
    };

    i64::from_str_radix(&digits, radix).ok()
}
