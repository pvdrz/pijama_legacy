use std::include_str;

use pijama_ast::{
    self,
    node::{Block, Branch, Expression as Expr, Node, Statement as Stat},
    ty::{Ty, TyAnnotation},
};
use pijama_common::{BinOp::*, Literal, Local, UnOp};
use pijama_parser::{parse, ParsingResult};

use crate::util::DummyLoc;

fn block_into_iter<'a>(block: Block<'a>) -> impl Iterator<Item = Node<'a>> {
    block.nodes.into_iter().chain(Some(Node::Expr(*block.expr)))
}

#[test]
fn name() -> ParsingResult<()> {
    let input = include_str!("name.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Local(Local::Name("x")).loc()),
        result.next().unwrap(),
        "single letter"
    );
    assert_eq!(
        Node::Expr(Expr::Local(Local::Name("foo")).loc()),
        result.next().unwrap(),
        "word"
    );
    assert_eq!(
        Node::Expr(Expr::Local(Local::Name("foo_bar")).loc()),
        result.next().unwrap(),
        "snake case"
    );
    Ok(())
}

#[test]
fn single_comment() -> ParsingResult<()> {
    let input = include_str!("single_comment.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Local(Local::Name("foo_bar")).loc()),
        result.next().unwrap(),
        "snake case"
    );
    Ok(())
}

#[test]
fn consecutive_comments() -> ParsingResult<()> {
    let input = include_str!("consecutive_comments.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Local(Local::Name("foo_bar")).loc()),
        result.next().unwrap(),
        "snake case"
    );
    Ok(())
}

#[test]
fn literal() -> ParsingResult<()> {
    let input = include_str!("literal.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(0)).loc()),
        result.next().unwrap(),
        "integer"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(-1)).loc()),
        result.next().unwrap(),
        "negative integer"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(14142)).loc()),
        result.next().unwrap(),
        "large integer in different bases"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(14142)).loc()),
        result.next().unwrap(),
        "large integer in different bases"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(14142)).loc()),
        result.next().unwrap(),
        "large integer in different bases"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(14142)).loc()),
        result.next().unwrap(),
        "large integer in different bases"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Number(14142)).loc()),
        result.next().unwrap(),
        "large integer in different bases"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Bool(true)).loc()),
        result.next().unwrap(),
        "true"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Bool(false)).loc()),
        result.next().unwrap(),
        "false"
    );
    assert_eq!(
        Node::Expr(Expr::Literal(Literal::Unit).loc()),
        result.next().unwrap(),
        "unit"
    );
    Ok(())
}

#[test]
fn binary_op() -> ParsingResult<()> {
    let input = include_str!("bin_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Add,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(Expr::Local(Local::Name("b")).loc()),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "simple"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Add,
                Box::new(
                    Expr::BinaryOp(
                        Add,
                        Box::new(Expr::Local(Local::Name("a")).loc()),
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(Expr::Local(Local::Name("c")).loc()),
            )
            .loc()
        ),
        result.next().unwrap(),
        "left-associative"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Add,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Add,
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc()
        ),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}

#[test]
fn unary_op() -> ParsingResult<()> {
    let input = include_str!("un_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::UnaryOp(UnOp::Neg, Box::new(Expr::Local(Local::Name("x")).loc())).loc(),),
        result.next().unwrap(),
        "minus"
    );
    assert_eq!(
        Node::Expr(Expr::UnaryOp(UnOp::Not, Box::new(Expr::Local(Local::Name("x")).loc())).loc(),),
        result.next().unwrap(),
        "not"
    );
    assert_eq!(
        Node::Expr(
            Expr::UnaryOp(
                UnOp::Not,
                Box::new(
                    Expr::UnaryOp(UnOp::Not, Box::new(Expr::Local(Local::Name("x")).loc())).loc(),
                ),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "double"
    );
    assert_eq!(
        Node::Expr(Expr::UnaryOp(UnOp::Not, Box::new(Expr::Local(Local::Name("x")).loc())).loc()),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}

#[test]
fn logic_op() -> ParsingResult<()> {
    let input = include_str!("logic_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                And,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(Expr::Local(Local::Name("b")).loc()),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "simple"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Or,
                Box::new(
                    Expr::BinaryOp(
                        And,
                        Box::new(Expr::Local(Local::Name("a")).loc()),
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(Expr::Local(Local::Name("c")).loc()),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "left-associative"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                And,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Or,
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc()
        ),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}

#[test]
fn bit_op() -> ParsingResult<()> {
    let input = include_str!("bit_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                BitAnd,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(Expr::Local(Local::Name("b")).loc()),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "simple"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                BitXor,
                Box::new(
                    Expr::BinaryOp(
                        BitOr,
                        Box::new(
                            Expr::BinaryOp(
                                BitAnd,
                                Box::new(Expr::Local(Local::Name("a")).loc()),
                                Box::new(Expr::Local(Local::Name("b")).loc()),
                            )
                            .loc(),
                        ),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
                Box::new(Expr::Local(Local::Name("d")).loc()),
            )
            .loc()
        ),
        result.next().unwrap(),
        "left-associative"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                BitXor,
                Box::new(
                    Expr::BinaryOp(
                        BitAnd,
                        Box::new(Expr::Local(Local::Name("a")).loc()),
                        Box::new(
                            Expr::BinaryOp(
                                BitOr,
                                Box::new(Expr::Local(Local::Name("b")).loc()),
                                Box::new(Expr::Local(Local::Name("c")).loc()),
                            )
                            .loc(),
                        ),
                    )
                    .loc(),
                ),
                Box::new(Expr::Local(Local::Name("d")).loc()),
            )
            .loc()
        ),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}

#[test]
fn assign() -> ParsingResult<()> {
    let input = include_str!("let_bind.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Stat(
            Stat::Assign(
                TyAnnotation {
                    item: Local::Name("x").loc(),
                    ty: Ty::Missing.loc(),
                },
                Expr::Local(Local::Name("y")).loc(),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "simple"
    );
    assert_eq!(
        Node::Stat(
            Stat::Assign(
                TyAnnotation {
                    item: Local::Name("x").loc(),
                    ty: Ty::Missing.loc(),
                },
                Expr::BinaryOp(
                    Add,
                    Box::new(Expr::Local(Local::Name("y")).loc()),
                    Box::new(Expr::Local(Local::Name("z")).loc()),
                )
                .loc(),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "bind to bin op"
    );
    assert_eq!(
        Node::Stat(
            Stat::Assign(
                TyAnnotation {
                    item: Local::Name("x").loc(),
                    ty: Ty::Int.loc(),
                },
                Expr::Local(Local::Name("y")).loc(),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "type binding"
    );
    assert_eq!(
        Node::Stat(
            Stat::Assign(
                TyAnnotation {
                    item: Local::Name("foo").loc(),
                    ty: Ty::Missing.loc(),
                },
                Expr::AnonFn(
                    vec![TyAnnotation {
                        item: Local::Name("x").loc(),
                        ty: Ty::Int.loc(),
                    }],
                    TyAnnotation {
                        item: Block {
                            nodes: Default::default(),
                            expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                        },
                        ty: Ty::Missing.loc(),
                    },
                )
                .loc(),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "bind to nameless function"
    );
    Ok(())
}

#[test]
fn cond() -> ParsingResult<()> {
    let input = include_str!("cond.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::Cond(
                Branch {
                    cond: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                    body: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("y")).loc()),
                    },
                },
                vec![],
                Block {
                    nodes: Default::default(),
                    expr: Box::new(Expr::Local(Local::Name("z")).loc()),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "simple blocks"
    );
    assert_eq!(
        Node::Expr(
            Expr::Cond(
                Branch {
                    cond: Block {
                        nodes: vec![Node::Expr(Expr::Local(Local::Name("u")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Local(Local::Name("v")).loc()),
                    },
                    body: Block {
                        nodes: vec![Node::Expr(Expr::Local(Local::Name("w")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                },
                vec![],
                Block {
                    nodes: vec![Node::Expr(Expr::Local(Local::Name("y")).loc())]
                        .into_iter()
                        .collect(),
                    expr: Box::new(Expr::Local(Local::Name("z")).loc()),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "long blocks"
    );
    Ok(())
}
#[test]
fn elif() -> ParsingResult<()> {
    let input = include_str!("elif.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::Cond(
                Branch {
                    cond: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                    body: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("y")).loc()),
                    },
                },
                vec![Branch {
                    cond: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("a")).loc()),
                    },
                    body: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("b")).loc()),
                    },
                },],
                Block {
                    nodes: Default::default(),
                    expr: Box::new(Expr::Local(Local::Name("z")).loc()),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "simple blocks"
    );
    assert_eq!(
        Node::Expr(
            Expr::Cond(
                Branch {
                    cond: Block {
                        nodes: vec![Node::Expr(Expr::Local(Local::Name("u")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Local(Local::Name("v")).loc()),
                    },
                    body: Block {
                        nodes: vec![Node::Expr(Expr::Local(Local::Name("w")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                },
                vec![
                    Branch {
                        cond: Block {
                            nodes: vec![Node::Expr(Expr::Local(Local::Name("a")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Local(Local::Name("b")).loc()),
                        },
                        body: Block {
                            nodes: vec![Node::Expr(Expr::Local(Local::Name("c")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Local(Local::Name("d")).loc()),
                        },
                    },
                    Branch {
                        cond: Block {
                            nodes: vec![Node::Expr(Expr::Local(Local::Name("e")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Local(Local::Name("f")).loc()),
                        },
                        body: Block {
                            nodes: vec![Node::Expr(Expr::Local(Local::Name("g")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Local(Local::Name("h")).loc()),
                        },
                    },
                ],
                Block {
                    nodes: vec![Node::Expr(Expr::Local(Local::Name("y")).loc())]
                        .into_iter()
                        .collect(),
                    expr: Box::new(Expr::Local(Local::Name("z")).loc()),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "long blocks"
    );
    Ok(())
}

#[test]
fn call() -> ParsingResult<()> {
    let input = include_str!("call.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Call(Box::new(Expr::Local(Local::Name("x")).loc()), vec![]).loc()),
        result.next().unwrap(),
        "nullary call"
    );
    assert_eq!(
        Node::Expr(
            Expr::Call(
                Box::new(Expr::Local(Local::Name("x")).loc()),
                vec![Expr::Local(Local::Name("y")).loc()],
            )
            .loc(),
        ),
        result.next().unwrap(),
        "unary call"
    );
    assert_eq!(
        Node::Expr(
            Expr::Call(
                Box::new(Expr::Local(Local::Name("x")).loc()),
                vec![
                    Expr::Local(Local::Name("y")).loc(),
                    Expr::Local(Local::Name("z")).loc()
                ],
            )
            .loc(),
        ),
        result.next().unwrap(),
        "binary call"
    );
    assert_eq!(
        Node::Expr(
            Expr::Call(
                Box::new(
                    Expr::BinaryOp(
                        Add,
                        Box::new(Expr::Local(Local::Name("x")).loc()),
                        Box::new(Expr::Local(Local::Name("y")).loc()),
                    )
                    .loc(),
                ),
                vec![Expr::Local(Local::Name("z")).loc()],
            )
            .loc(),
        ),
        result.next().unwrap(),
        "complex call"
    );
    Ok(())
}

#[test]
fn fn_def() -> ParsingResult<()> {
    let input = include_str!("fn_def.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Stat(
            Stat::FnDef(
                Local::Name("foo").loc(),
                vec![TyAnnotation {
                    item: Local::Name("x").loc(),
                    ty: Ty::Int.loc(),
                }],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                    ty: Ty::Missing.loc(),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "unary def"
    );
    assert_eq!(
        Node::Stat(
            Stat::FnDef(
                Local::Name("foo").loc(),
                vec![],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(
                            Expr::Call(Box::new(Expr::Local(Local::Name("foo")).loc()), vec![])
                                .loc(),
                        ),
                    },
                    ty: Ty::Unit.loc(),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "recursive def"
    );
    assert_eq!(
        Node::Stat(
            Stat::FnDef(
                Local::Name("foo").loc(),
                vec![
                    TyAnnotation {
                        item: Local::Name("x").loc(),
                        ty: Ty::Int.loc(),
                    },
                    TyAnnotation {
                        item: Local::Name("y").loc(),
                        ty: Ty::Int.loc(),
                    },
                ],
                TyAnnotation {
                    item: Block {
                        nodes: vec![Node::Expr(Expr::Local(Local::Name("x")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Local(Local::Name("y")).loc()),
                    },
                    ty: Ty::Missing.loc(),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "long body"
    );
    assert_eq!(
        Node::Expr(
            Expr::AnonFn(
                vec![TyAnnotation {
                    item: Local::Name("x").loc(),
                    ty: Ty::Int.loc(),
                }],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                    ty: Ty::Missing.loc(),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "nameless"
    );
    assert_eq!(
        Node::Expr(
            Expr::AnonFn(
                vec![TyAnnotation {
                    item: Local::Name("x").loc(),
                    ty: Ty::Int.loc(),
                }],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Local(Local::Name("x")).loc()),
                    },
                    ty: Ty::Missing.loc(),
                },
            )
            .loc(),
        ),
        result.next().unwrap(),
        "nameless with space"
    );
    Ok(())
}

#[test]
fn precedence() -> ParsingResult<()> {
    let input = include_str!("precedence.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Add,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Mul,
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "mul precedes add"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                BitAnd,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Add,
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "add precedes bitwise and"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Eq,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        BitAnd,
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "bitwise and precedes equal"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                And,
                Box::new(Expr::Local(Local::Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Eq,
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "equal precedes and"
    );
    Ok(())
}

#[test]
fn cmp_and_shift() -> ParsingResult<()> {
    let input = include_str!("cmp_and_shift.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Lt,
                Box::new(
                    Expr::BinaryOp(
                        Shl,
                        Box::new(Expr::Local(Local::Name("a")).loc()),
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(
                    Expr::BinaryOp(
                        Shl,
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                        Box::new(Expr::Local(Local::Name("d")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc()
        ),
        result.next().unwrap(),
        "left shift"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Gt,
                Box::new(
                    Expr::BinaryOp(
                        Shr,
                        Box::new(Expr::Local(Local::Name("a")).loc()),
                        Box::new(Expr::Local(Local::Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(
                    Expr::BinaryOp(
                        Shr,
                        Box::new(Expr::Local(Local::Name("c")).loc()),
                        Box::new(Expr::Local(Local::Name("d")).loc()),
                    )
                    .loc(),
                ),
            )
            .loc()
        ),
        result.next().unwrap(),
        "right shift"
    );
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Shr,
                Box::new(
                    Expr::BinaryOp(
                        Shr,
                        Box::new(Expr::Local(Local::Name("a")).loc()),
                        Box::new(
                            Expr::BinaryOp(
                                Gt,
                                Box::new(Expr::Local(Local::Name("b")).loc()),
                                Box::new(Expr::Local(Local::Name("c")).loc()),
                            )
                            .loc(),
                        ),
                    )
                    .loc(),
                ),
                Box::new(Expr::Local(Local::Name("d")).loc()),
            )
            .loc()
        ),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}
