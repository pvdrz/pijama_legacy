//! Parsers for binary operations.
//!
//! The entry point for this module is the [`binary_op`] parser. To parse binary operations
//! following the precedence levels stated in the [`bin_op`] module, we have a set of parsers
//! `binary_op_<n>` for each level of precedence `n` matching their `bin_op_<n>` counterparts in
//! the [`bin_op`] module.
//!
//! In the same way as in the [`ty`] module. The naive grammar for nodes including binary
//! operations is left-recursive:
//!
//! ```abnf
//! node = binary_op / unary_op / let_bind / cond / fn_def / fn_rec_def / call
//! binary_op = binary_op_1 bin_op_1 binary_op_1
//! binary_op_1 = binary_op_2 bin_op_2 binary_op_2
//! binary_op_2 = binary_op_3 bin_op_3 binary_op_3
//! binary_op_3 = binary_op_4 bin_op_4 binary_op_4
//! binary_op_4 = (node bin_op_5 node) / ("(" node ")")
//! ```
//! The first thing to do when parsing a node is parsing a node again. The only difference is that
//! here the chain is longer. Doing the same procedure as with the [`ty`] module. It is possible to
//! eliminate the infinite loop:
//!
//! ```abnf
//! node = binary_op
//! binary_op = binary_op_1 (bin_op_1 binary_op_1)*
//! binary_op_1 = binary_op_2 (bin_op_2 binary_op_2)*
//! binary_op_2 = binary_op_3 (bin_op_3 binary_op_3)*
//! binary_op_3 = binary_op_4 (bin_op_4 binary_op_4)*
//! binary_op_4 = base_node (bin_op_5 base_node)*
//! base_node = unary_op / let_bind / cond / fn_def / fn_rec_def / call / ("(" node ")")
//! ```
//!
//! The [`binary_op`] and the `binary_op_<n>` parsers in this module corresponds to each one of the
//! rules above. The [`node`] and [`base_node`] parsers are in the supermodule.
//!
//! Every binary operator here is considered to be left-associative, in contrast with the `->` for
//! in the [`ty`] module which is right-associative.
//!
//! The location of the returned binary operations matches the start of the first operand and the
//! end of the second.
//!
//! [`ty`]: crate::parser::ty
//! [`node`]: crate::parser::node::node
//! [`bin_op`]: crate::parser::bin_op
use nom::{
    combinator::{cut, opt},
    sequence::pair,
};

use pijama_ast::{Located, Node, Span};

use crate::parser::{bin_op::*, node::base_node, IResult};

/// Parses a [`Node::BinaryOp`].
pub fn binary_op(input: Span) -> IResult<Located<Node>> {
    let (mut input, mut node) = binary_op_1(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_1, cut(binary_op_1)))(input)? {
        input = rem;
        let loc = node.loc + node2.loc;
        node = Located::new(Node::BinaryOp(op, Box::new(node), Box::new(node2)), loc);
    }
    Ok((input, node))
}

fn binary_op_1(input: Span) -> IResult<Located<Node>> {
    let (mut input, mut node) = binary_op_2(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_2, cut(binary_op_2)))(input)? {
        input = rem;
        let loc = node.loc + node2.loc;
        node = Located::new(Node::BinaryOp(op, Box::new(node), Box::new(node2)), loc);
    }
    Ok((input, node))
}

fn binary_op_2(input: Span) -> IResult<Located<Node>> {
    let (mut input, mut node) = binary_op_3(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_3, cut(binary_op_3)))(input)? {
        input = rem;
        let loc = node.loc + node2.loc;
        node = Located::new(Node::BinaryOp(op, Box::new(node), Box::new(node2)), loc);
    }
    Ok((input, node))
}

fn binary_op_3(input: Span) -> IResult<Located<Node>> {
    let (mut input, mut node) = binary_op_4(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_4, cut(binary_op_4)))(input)? {
        input = rem;
        let loc = node.loc + node2.loc;
        node = Located::new(Node::BinaryOp(op, Box::new(node), Box::new(node2)), loc);
    }
    Ok((input, node))
}

fn binary_op_4(input: Span) -> IResult<Located<Node>> {
    let (mut input, mut node) = base_node(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_5, cut(base_node)))(input)? {
        input = rem;
        let loc = node.loc + node2.loc;
        node = Located::new(Node::BinaryOp(op, Box::new(node), Box::new(node2)), loc);
    }
    Ok((input, node))
}
