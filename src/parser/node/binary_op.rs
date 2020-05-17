//! Parsers for binary operations.
//!
//! The entry-point for this module is the [`binary_op`] parser. To parse binary operations
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
//! eliminate the infinite loop
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
//! The [`binary_op`] and the `binary_op_<n>` parsers in this module corresponds to each
//! one of the rules above. The [`node`] and [`base_node`] parsers are in the supermodule.
//!
//! ## Modifying the precedence of operators
//!
//! If you need to change the precedence of one operator. First check if it is enough to move the
//! operator to a different level in the [`bin_op`] module. If that's not the case, a new
//! precedence level might be needed. For that you need to:
//!
//! - Add a new `bin_op_n` parser in [`bin_op`] displacing by 1 all the `bin_op_m` parsers with
//! `m > n` and add the new operator to the desired level.
//! - Add a new `binary_op_n` parser in this module displacing the `binary_op_m` parser with the
//! largest `m` by 1 and adding a new `binary_op_m` similar to the other ones.
//!
//! [`ty`]: crate::parser::ty
//! [`binary_op`]: crate::parser::node::binary_op::binary_op()
//! [`node`]: crate::parser::node::node
//! [`base_node`]: crate::parser::node::base_node
//! [`bin_op`]: crate::parser::bin_op
use nom::{error::ParseError, IResult};

use nom::{
    combinator::{cut, opt},
    sequence::pair,
};

use crate::ast::Node;
use crate::parser::{bin_op::*, node::base_node};

pub fn binary_op<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    let (mut input, mut node) = binary_op_1(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_1, cut(binary_op_1)))(input)? {
        input = rem;
        node = Node::BinaryOp(op, Box::new(node), Box::new(node2));
    }
    Ok((input, node))
}

fn binary_op_1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    let (mut input, mut node) = binary_op_2(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_2, cut(binary_op_2)))(input)? {
        input = rem;
        node = Node::BinaryOp(op, Box::new(node), Box::new(node2));
    }
    Ok((input, node))
}

fn binary_op_2<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    let (mut input, mut node) = binary_op_3(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_3, cut(binary_op_3)))(input)? {
        input = rem;
        node = Node::BinaryOp(op, Box::new(node), Box::new(node2));
    }
    Ok((input, node))
}

fn binary_op_3<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    let (mut input, mut node) = binary_op_4(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_4, cut(binary_op_4)))(input)? {
        input = rem;
        node = Node::BinaryOp(op, Box::new(node), Box::new(node2));
    }
    Ok((input, node))
}

fn binary_op_4<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    let (mut input, mut node) = base_node(input)?;
    while let (rem, Some((op, node2))) = opt(pair(bin_op_5, cut(base_node)))(input)? {
        input = rem;
        node = Node::BinaryOp(op, Box::new(node), Box::new(node2));
    }
    Ok((input, node))
}
