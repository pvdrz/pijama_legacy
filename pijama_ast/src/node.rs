//! Nodes of Pijama's AST.
//!
//! The purpose of the types in this module is to represent the syntax of Pijama as faithfully as
//! possible. Some types here are used through Pijama's different internal representations.
use std::{collections::VecDeque, fmt::Debug};

use pijama_common::{
    location::{Located, Location},
    BinOp, Literal, Local, Primitive, UnOp,
};

use crate::ty::TyAnnotation;

/// A block is a sequence of nodes terminating in an expression.
#[derive(Debug, Eq, PartialEq)]
pub struct Block<'a> {
    /// Nodes of the block.
    pub nodes: VecDeque<Node<'a>>,
    /// Terminating expression of the block.
    pub expr: Box<Located<Expression<'a>>>,
}

/// A Node in the AST that encapsulates the different expressions and statements that Pijama's
/// syntax supports.
#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    /// A statement.
    Stat(Located<Statement<'a>>),
    /// An expression.
    Expr(Located<Expression<'a>>),
}

impl<'a> Node<'a> {
    /// Return the location of the node.
    pub fn loc(&self) -> Location {
        match *self {
            Node::Stat(Located { loc, .. }) | Node::Expr(Located { loc, .. }) => loc,
        }
    }
}

/// An AST node that performs an action.
#[derive(Debug, Eq, PartialEq)]
pub enum Statement<'a> {
    /// Statement containing an assignment.
    Assign(TyAnnotation<Located<Local<'a>>>, Located<Expression<'a>>),
    /// Statement containing a function definition.
    FnDef(
        Located<Local<'a>>,
        Vec<TyAnnotation<Located<Local<'a>>>>,
        TyAnnotation<Block<'a>>,
    ),
}

/// An AST node that produces a value.
#[derive(Debug, Eq, PartialEq)]
pub enum Expression<'a> {
    /// Expression containing a binary operation.
    BinaryOp(
        BinOp,
        Box<Located<Expression<'a>>>,
        Box<Located<Expression<'a>>>,
    ),
    /// Expression containing a unary operation.
    UnaryOp(UnOp, Box<Located<Expression<'a>>>),
    /// Expression containing a conditional.
    Cond(Branch<'a>, Vec<Branch<'a>>, Block<'a>),
    /// Expression containing an anonymous function.
    AnonFn(
        Vec<TyAnnotation<Located<Local<'a>>>>,
        TyAnnotation<Block<'a>>,
    ),
    /// Expression containing a function call.
    Call(Box<Located<Expression<'a>>>, Vec<Located<Expression<'a>>>),
    /// Expression containing a literal.
    Literal(Literal),
    /// Expression containing a local.
    Local(Local<'a>),
    /// Expression containing a primitive function.
    PrimFn(Primitive),
}

/// Encapsulates a conditional statement in Pijama's syntax. It is used to represent both `if` and
/// `elif` branches.
#[derive(Debug, Eq, PartialEq)]
pub struct Branch<'a> {
    /// The condition of the branch.
    pub cond: Block<'a>,
    /// The body of the branch that is evaluated if the condition is true.
    pub body: Block<'a>,
}
