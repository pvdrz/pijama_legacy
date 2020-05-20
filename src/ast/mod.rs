use std::fmt::{Debug, Display, Formatter, Result};

use crate::ty::{Binding, Ty};

mod location;
mod visitor;

pub use location::*;

use visitor::NodeVisitor;

pub type Block<'a> = Vec<Located<Node<'a>>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Name<'a>(pub &'a str);

impl<'a> Display for Name<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shr,
    Shl,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl<'a> Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use BinOp::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Rem => write!(f, "%"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            BitAnd => write!(f, "&"),
            BitOr => write!(f, "|"),
            BitXor => write!(f, "^"),
            Shr => write!(f, ">>"),
            Shl => write!(f, "<<"),
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Lte => write!(f, "<="),
            Gte => write!(f, ">="),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnOp {
    Neg,
    Not,
}

impl<'a> Display for UnOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use UnOp::*;
        match self {
            Not => write!(f, "!"),
            Neg => write!(f, "-"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Literal {
    Bool(bool),
    Unit,
    Number(i128),
}

impl Into<Literal> for i128 {
    fn into(self) -> Literal {
        Literal::Number(self)
    }
}

impl Into<Literal> for bool {
    fn into(self) -> Literal {
        Literal::Bool(self)
    }
}

impl<'a> Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Literal::*;
        match self {
            Bool(b) => write!(f, "{}", b),
            Unit => write!(f, "unit"),
            Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    BinaryOp(BinOp, Box<Located<Node<'a>>>, Box<Located<Node<'a>>>),
    UnaryOp(UnOp, Box<Located<Node<'a>>>),
    LetBind(
        Located<Name<'a>>,
        Option<Located<Ty>>,
        Box<Located<Node<'a>>>,
    ),
    Cond(Located<Block<'a>>, Located<Block<'a>>, Located<Block<'a>>),
    FnDef(
        Option<Located<Name<'a>>>,
        Vec<Located<Binding<'a>>>,
        Located<Block<'a>>,
        Option<Located<Ty>>,
    ),
    Call(Box<Located<Node<'a>>>, Block<'a>),
    Literal(Literal),
    Name(Name<'a>),
}


/// Checks if a function is recursive or not.
pub struct RecursionChecker<'a> {
    /// Name of the target function
    name: Name<'a>,
    /// Stores if the function is recursive or not during the visiting.
    is_rec: bool,
    /// Stores if the target name is being shadowed in the current scope. It represents the top of
    /// the stack
    is_shadowed: bool,
    /// Stores the shadowing status in the upper scopes.
    stack: Vec<bool>,
}

impl<'a> RecursionChecker<'a> {
    /// Runs the recursion check with the target function's name and body.
    pub fn run(name: Name<'a>, body: &Block<'a>) -> bool {
        let mut this = RecursionChecker {
            name,
            is_rec: false,
            is_shadowed: false,
            stack: Vec::new(),
        };
        this.visit_block(body);
        // Sanity check. There should be only one scope, the original one, after visiting the body
        // function.
        assert!(this.stack.is_empty(), "Someone forgot to pop a scope from the stack");
        this.is_rec
    }

    /// Push a new scope into the stack.
    ///
    /// The new scope has the same shadowed status as the old scope because names are preserved
    /// when creating a new scope.
    fn push_scope(&mut self) {
        self.stack.push(self.is_shadowed)
    }

    /// Pops a scope from the stack.
    ///
    /// This function panics if there are no more scopes in the stack. Which should be impossible
    /// because the stack always starts as non-empty and we should only pop newly added scopes from
    /// the stack.
    fn pop_scope(&mut self) {
        self.is_shadowed = self.stack.pop().expect("there are no more scopes in the stack");
    }
}

impl<'a> NodeVisitor<'a> for RecursionChecker<'a> {
    fn visit_name(&mut self, name: &Name<'a>) {
        // The function is recursive if its name is not shadowed in the current scope and we found
        // it is somewhere inside its body.
        if !self.is_shadowed && *name == self.name {
            self.is_rec = true;
        }
        // Keep visiting
        self.super_name(name);
    }

    fn visit_let_bind(
        &mut self,
        name: &Located<Name<'a>>,
        opt_ty: &Option<Located<Ty>>,
        body: &Located<Node<'a>>,
    ) {
        // If the binding binds the target name, the latter is being shadowed in the current scope.
        if name.content == self.name {
            self.is_shadowed = true;
        }
        // Keep visiting
        self.super_let_bind(name, opt_ty, body);
    }

    fn visit_fn_def(
        &mut self,
        opt_name: &Option<Located<Name<'a>>>,
        args: &Vec<Located<Binding<'a>>>,
        body: &Located<Block<'a>>,
        opt_ty: &Option<Located<Ty>>,
    ) {
        // If the function definition binds the target name, the latter is being shadowed in the
        // current scope.
        //
        // FIXME: what happens if this function is recursive too?
        match opt_name {
            Some(name) if name.content == self.name => {
                self.is_shadowed = true;
            }
            _ => {}
        };
        // Push a new scope into the stack
        self.push_scope();
        // Keep visiting
        self.super_fn_def(opt_name, args, body, opt_ty);
        // Pop the scope after visiting the function
        self.pop_scope();
    }
}
