use std::include_str;

use pijama_ast::{
    self,
    node::{
        BinOp::*, Block, Branch, Expression as Expr, Literal, Name, Node, Statement as Stat, UnOp,
    },
    ty::{Ty, TyAnnotation},
};
use pijama_driver::LangResult;
use pijama_parser::parse;

use crate::util::DummyLoc;

fn block_into_iter<'a>(block: Block<'a>) -> impl Iterator<Item = Node<'a>> {
    block.nodes.into_iter().chain(Some(Node::Expr(*block.expr)))
}

#[test]
fn name() -> LangResult<()> {
    let input = include_str!("name.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Name(Name("x")).loc()),
        result.next().unwrap(),
        "single letter"
    );
    assert_eq!(
        Node::Expr(Expr::Name(Name("foo")).loc()),
        result.next().unwrap(),
        "word"
    );
    assert_eq!(
        Node::Expr(Expr::Name(Name("foo_bar")).loc()),
        result.next().unwrap(),
        "snake case"
    );
    Ok(())
}

#[test]
fn single_comment() -> LangResult<()> {
    let input = include_str!("single_comment.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Name(Name("foo_bar")).loc()),
        result.next().unwrap(),
        "snake case"
    );
    Ok(())
}

#[test]
fn consecutive_comments() -> LangResult<()> {
    let input = include_str!("consecutive_comments.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Name(Name("foo_bar")).loc()),
        result.next().unwrap(),
        "snake case"
    );
    Ok(())
}

#[test]
fn literal() -> LangResult<()> {
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
fn binary_op() -> LangResult<()> {
    let input = include_str!("bin_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Add,
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(Expr::Name(Name("b")).loc()),
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
                        Box::new(Expr::Name(Name("a")).loc()),
                        Box::new(Expr::Name(Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(Expr::Name(Name("c")).loc()),
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
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Add,
                        Box::new(Expr::Name(Name("b")).loc()),
                        Box::new(Expr::Name(Name("c")).loc()),
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
fn unary_op() -> LangResult<()> {
    let input = include_str!("un_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::UnaryOp(UnOp::Neg, Box::new(Expr::Name(Name("x")).loc())).loc(),),
        result.next().unwrap(),
        "minus"
    );
    assert_eq!(
        Node::Expr(Expr::UnaryOp(UnOp::Not, Box::new(Expr::Name(Name("x")).loc())).loc(),),
        result.next().unwrap(),
        "not"
    );
    assert_eq!(
        Node::Expr(
            Expr::UnaryOp(
                UnOp::Not,
                Box::new(Expr::UnaryOp(UnOp::Not, Box::new(Expr::Name(Name("x")).loc())).loc(),),
            )
            .loc(),
        ),
        result.next().unwrap(),
        "double"
    );
    assert_eq!(
        Node::Expr(Expr::UnaryOp(UnOp::Not, Box::new(Expr::Name(Name("x")).loc())).loc()),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}

#[test]
fn logic_op() -> LangResult<()> {
    let input = include_str!("logic_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                And,
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(Expr::Name(Name("b")).loc()),
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
                        Box::new(Expr::Name(Name("a")).loc()),
                        Box::new(Expr::Name(Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(Expr::Name(Name("c")).loc()),
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
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Or,
                        Box::new(Expr::Name(Name("b")).loc()),
                        Box::new(Expr::Name(Name("c")).loc()),
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
fn bit_op() -> LangResult<()> {
    let input = include_str!("bit_op.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                BitAnd,
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(Expr::Name(Name("b")).loc()),
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
                                Box::new(Expr::Name(Name("a")).loc()),
                                Box::new(Expr::Name(Name("b")).loc()),
                            )
                            .loc(),
                        ),
                        Box::new(Expr::Name(Name("c")).loc()),
                    )
                    .loc(),
                ),
                Box::new(Expr::Name(Name("d")).loc()),
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
                        Box::new(Expr::Name(Name("a")).loc()),
                        Box::new(
                            Expr::BinaryOp(
                                BitOr,
                                Box::new(Expr::Name(Name("b")).loc()),
                                Box::new(Expr::Name(Name("c")).loc()),
                            )
                            .loc(),
                        ),
                    )
                    .loc(),
                ),
                Box::new(Expr::Name(Name("d")).loc()),
            )
            .loc()
        ),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}

#[test]
fn assign() -> LangResult<()> {
    let input = include_str!("let_bind.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Stat(
            Stat::Assign(
                TyAnnotation {
                    item: Name("x").loc(),
                    ty: Ty::Missing.loc(),
                },
                Expr::Name(Name("y")).loc(),
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
                    item: Name("x").loc(),
                    ty: Ty::Missing.loc(),
                },
                Expr::BinaryOp(
                    Add,
                    Box::new(Expr::Name(Name("y")).loc()),
                    Box::new(Expr::Name(Name("z")).loc()),
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
                    item: Name("x").loc(),
                    ty: Ty::Int.loc(),
                },
                Expr::Name(Name("y")).loc(),
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
                    item: Name("foo").loc(),
                    ty: Ty::Missing.loc(),
                },
                Expr::AnonFn(
                    vec![TyAnnotation {
                        item: Name("x").loc(),
                        ty: Ty::Int.loc(),
                    }],
                    TyAnnotation {
                        item: Block {
                            nodes: Default::default(),
                            expr: Box::new(Expr::Name(Name("x")).loc()),
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
fn cond() -> LangResult<()> {
    let input = include_str!("cond.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::Cond(
                Branch {
                    cond: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
                    },
                    body: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("y")).loc()),
                    },
                },
                vec![],
                Block {
                    nodes: Default::default(),
                    expr: Box::new(Expr::Name(Name("z")).loc()),
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
                        nodes: vec![Node::Expr(Expr::Name(Name("u")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Name(Name("v")).loc()),
                    },
                    body: Block {
                        nodes: vec![Node::Expr(Expr::Name(Name("w")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
                    },
                },
                vec![],
                Block {
                    nodes: vec![Node::Expr(Expr::Name(Name("y")).loc())]
                        .into_iter()
                        .collect(),
                    expr: Box::new(Expr::Name(Name("z")).loc()),
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
fn elif() -> LangResult<()> {
    let input = include_str!("elif.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::Cond(
                Branch {
                    cond: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
                    },
                    body: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("y")).loc()),
                    },
                },
                vec![Branch {
                    cond: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("a")).loc()),
                    },
                    body: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("b")).loc()),
                    },
                },],
                Block {
                    nodes: Default::default(),
                    expr: Box::new(Expr::Name(Name("z")).loc()),
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
                        nodes: vec![Node::Expr(Expr::Name(Name("u")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Name(Name("v")).loc()),
                    },
                    body: Block {
                        nodes: vec![Node::Expr(Expr::Name(Name("w")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
                    },
                },
                vec![
                    Branch {
                        cond: Block {
                            nodes: vec![Node::Expr(Expr::Name(Name("a")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Name(Name("b")).loc()),
                        },
                        body: Block {
                            nodes: vec![Node::Expr(Expr::Name(Name("c")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Name(Name("d")).loc()),
                        },
                    },
                    Branch {
                        cond: Block {
                            nodes: vec![Node::Expr(Expr::Name(Name("e")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Name(Name("f")).loc()),
                        },
                        body: Block {
                            nodes: vec![Node::Expr(Expr::Name(Name("g")).loc())]
                                .into_iter()
                                .collect(),
                            expr: Box::new(Expr::Name(Name("h")).loc()),
                        },
                    },
                ],
                Block {
                    nodes: vec![Node::Expr(Expr::Name(Name("y")).loc())]
                        .into_iter()
                        .collect(),
                    expr: Box::new(Expr::Name(Name("z")).loc()),
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
fn call() -> LangResult<()> {
    let input = include_str!("call.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(Expr::Call(Box::new(Expr::Name(Name("x")).loc()), vec![]).loc()),
        result.next().unwrap(),
        "nullary call"
    );
    assert_eq!(
        Node::Expr(
            Expr::Call(
                Box::new(Expr::Name(Name("x")).loc()),
                vec![Expr::Name(Name("y")).loc()],
            )
            .loc(),
        ),
        result.next().unwrap(),
        "unary call"
    );
    assert_eq!(
        Node::Expr(
            Expr::Call(
                Box::new(Expr::Name(Name("x")).loc()),
                vec![Expr::Name(Name("y")).loc(), Expr::Name(Name("z")).loc()],
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
                        Box::new(Expr::Name(Name("x")).loc()),
                        Box::new(Expr::Name(Name("y")).loc()),
                    )
                    .loc(),
                ),
                vec![Expr::Name(Name("z")).loc()],
            )
            .loc(),
        ),
        result.next().unwrap(),
        "complex call"
    );
    Ok(())
}

#[test]
fn fn_def() -> LangResult<()> {
    let input = include_str!("fn_def.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Stat(
            Stat::FnDef(
                Name("foo").loc(),
                vec![TyAnnotation {
                    item: Name("x").loc(),
                    ty: Ty::Int.loc(),
                }],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
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
                Name("foo").loc(),
                vec![],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(
                            Expr::Call(Box::new(Expr::Name(Name("foo")).loc()), vec![]).loc(),
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
                Name("foo").loc(),
                vec![
                    TyAnnotation {
                        item: Name("x").loc(),
                        ty: Ty::Int.loc(),
                    },
                    TyAnnotation {
                        item: Name("y").loc(),
                        ty: Ty::Int.loc(),
                    },
                ],
                TyAnnotation {
                    item: Block {
                        nodes: vec![Node::Expr(Expr::Name(Name("x")).loc())]
                            .into_iter()
                            .collect(),
                        expr: Box::new(Expr::Name(Name("y")).loc()),
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
                    item: Name("x").loc(),
                    ty: Ty::Int.loc(),
                }],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
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
                    item: Name("x").loc(),
                    ty: Ty::Int.loc(),
                }],
                TyAnnotation {
                    item: Block {
                        nodes: Default::default(),
                        expr: Box::new(Expr::Name(Name("x")).loc()),
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
fn precedence() -> LangResult<()> {
    let input = include_str!("precedence.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Add,
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Mul,
                        Box::new(Expr::Name(Name("b")).loc()),
                        Box::new(Expr::Name(Name("c")).loc()),
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
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Add,
                        Box::new(Expr::Name(Name("b")).loc()),
                        Box::new(Expr::Name(Name("c")).loc()),
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
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        BitAnd,
                        Box::new(Expr::Name(Name("b")).loc()),
                        Box::new(Expr::Name(Name("c")).loc()),
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
                Box::new(Expr::Name(Name("a")).loc()),
                Box::new(
                    Expr::BinaryOp(
                        Eq,
                        Box::new(Expr::Name(Name("b")).loc()),
                        Box::new(Expr::Name(Name("c")).loc()),
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
fn cmp_and_shift() -> LangResult<()> {
    let input = include_str!("cmp_and_shift.pj");
    let mut result = block_into_iter(parse(input)?);
    assert_eq!(
        Node::Expr(
            Expr::BinaryOp(
                Lt,
                Box::new(
                    Expr::BinaryOp(
                        Shl,
                        Box::new(Expr::Name(Name("a")).loc()),
                        Box::new(Expr::Name(Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(
                    Expr::BinaryOp(
                        Shl,
                        Box::new(Expr::Name(Name("c")).loc()),
                        Box::new(Expr::Name(Name("d")).loc()),
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
                        Box::new(Expr::Name(Name("a")).loc()),
                        Box::new(Expr::Name(Name("b")).loc()),
                    )
                    .loc(),
                ),
                Box::new(
                    Expr::BinaryOp(
                        Shr,
                        Box::new(Expr::Name(Name("c")).loc()),
                        Box::new(Expr::Name(Name("d")).loc()),
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
                        Box::new(Expr::Name(Name("a")).loc()),
                        Box::new(
                            Expr::BinaryOp(
                                Gt,
                                Box::new(Expr::Name(Name("b")).loc()),
                                Box::new(Expr::Name(Name("c")).loc()),
                            )
                            .loc(),
                        ),
                    )
                    .loc(),
                ),
                Box::new(Expr::Name(Name("d")).loc()),
            )
            .loc()
        ),
        result.next().unwrap(),
        "brackets"
    );
    Ok(())
}
