use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Name<'a>(pub &'a str);

#[derive(Debug)]
pub enum Ty {
    Bool,
    Int,
    Unit,
    Arrow(Box<Ty>, Box<Ty>),
}

impl<'a> fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Ty::*;
        match self {
            Bool => write!(f, "Bool"),
            Int => write!(f, "Int"),
            Unit => write!(f, "Unit"),
            Arrow(t1, t2) => write!(f, "({} -> {})", t1, t2),
        }
    }
}

#[derive(Debug)]
pub struct Binding<'a> {
    pub name: Name<'a>,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl<'a> fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinOp::*;
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Times => write!(f, "*"),
            Divide => write!(f, "/"),
            Modulo => write!(f, "%"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Equal => write!(f, "=="),
            NotEqual => write!(f, "!="),
            LessThan => write!(f, "<"),
            GreaterThan => write!(f, ">"),
            LessThanOrEqual => write!(f, "<="),
            GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Minus,
    Not,
}

impl<'a> fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnOp::*;
        match self {
            Not => write!(f, "!"),
            Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    True,
    False,
    Unit,
    Number(i128),
}

impl Literal {
    fn is_bool(&self) -> bool {
        match self {
            Literal::True | Literal::False => true,
            _ => false,
        }
    }
}

impl Into<Literal> for i128 {
    fn into(self) -> Literal {
        Literal::Number(self)
    }
}

impl Into<Literal> for bool {
    fn into(self) -> Literal {
        if self {
            Literal::True
        } else {
            Literal::False
        }
    }
}

impl<'a> fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Literal::*;
        match self {
            True => write!(f, "true"),
            False => write!(f, "false"),
            Unit => write!(f, "unit"),
            Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug)]
pub enum Node<'a> {
    BinaryOp(BinOp, Box<Node<'a>>, Box<Node<'a>>),
    UnaryOp(UnOp, Box<Node<'a>>),
    LetBind(Name<'a>, Box<Node<'a>>),
    Cond(Vec<Node<'a>>, Vec<Node<'a>>, Vec<Node<'a>>),
    FnDef(Name<'a>, Vec<Binding<'a>>, Option<Ty>, Vec<Node<'a>>),
    Call(Name<'a>, Vec<Node<'a>>),
    Literal(Literal),
    Name(Name<'a>),
}
