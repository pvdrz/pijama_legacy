use std::include_str;

use pijama_ast::{
    self,
    ty::{Ty, TyAnnotation},
    BinOp::*,
    Branch,
    Node::*,
    UnOp,
};

use pijama::{parser::parse, LangResult};

use crate::util::DummyLoc;

#[test]
fn name() -> LangResult<'static, ()> {
    let input = include_str!("name.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Name(pijama_ast::Name("x")).loc(),
        Name(pijama_ast::Name("foo")).loc(),
        Name(pijama_ast::Name("foo_bar")).loc(),
    ];

    assert_eq!(expected[0], result[0], "single letter");
    assert_eq!(expected[1], result[1], "word");
    assert_eq!(expected[2], result[2], "snake case");
    Ok(())
}

#[test]
fn single_comment() -> LangResult<'static, ()> {
    let input = include_str!("single_comment.pj");
    let result = parse(input)?.content;
    let expected = vec![Name(pijama_ast::Name("foo_bar")).loc()];

    assert_eq!(expected[0], result[0], "snake case");
    Ok(())
}

#[test]
fn literal() -> LangResult<'static, ()> {
    let input = include_str!("literal.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Literal(pijama_ast::Literal::Number(0)).loc(),
        Literal(pijama_ast::Literal::Number(-1)).loc(),
        Literal(pijama_ast::Literal::Number(14142)).loc(),
        Literal(pijama_ast::Literal::Number(14142)).loc(),
        Literal(pijama_ast::Literal::Number(14142)).loc(),
        Literal(pijama_ast::Literal::Number(14142)).loc(),
        Literal(pijama_ast::Literal::Number(14142)).loc(),
        Literal(pijama_ast::Literal::Bool(true)).loc(),
        Literal(pijama_ast::Literal::Bool(false)).loc(),
        Literal(pijama_ast::Literal::Unit).loc(),
    ];

    assert_eq!(expected[0], result[0], "integer");
    assert_eq!(expected[1], result[1], "negative integer");
    assert_eq!(
        expected[2..7],
        result[2..7],
        "large integer in different bases"
    );
    assert_eq!(expected[7], result[7], "true");
    assert_eq!(expected[8], result[8], "false");
    assert_eq!(expected[9], result[9], "unit");
    Ok(())
}

#[test]
fn binary_op() -> LangResult<'static, ()> {
    let input = include_str!("bin_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            Add,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(Name(pijama_ast::Name("b")).loc()),
        )
        .loc(),
        BinaryOp(
            Add,
            Box::new(
                BinaryOp(
                    Add,
                    Box::new(Name(pijama_ast::Name("a")).loc()),
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                )
                .loc(),
            ),
            Box::new(Name(pijama_ast::Name("c")).loc()),
        )
        .loc(),
        BinaryOp(
            Add,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(
                BinaryOp(
                    Add,
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "left-associative");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}

#[test]
fn unary_op() -> LangResult<'static, ()> {
    let input = include_str!("un_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        UnaryOp(UnOp::Neg, Box::new(Name(pijama_ast::Name("x")).loc())).loc(),
        UnaryOp(UnOp::Not, Box::new(Name(pijama_ast::Name("x")).loc())).loc(),
        UnaryOp(
            UnOp::Not,
            Box::new(UnaryOp(UnOp::Not, Box::new(Name(pijama_ast::Name("x")).loc())).loc()),
        )
        .loc(),
        UnaryOp(UnOp::Not, Box::new(Name(pijama_ast::Name("x")).loc())).loc(),
    ];

    assert_eq!(expected[0], result[0], "minus");
    assert_eq!(expected[1], result[1], "not");
    assert_eq!(expected[2], result[2], "double");
    assert_eq!(expected[3], result[3], "brackets");
    Ok(())
}

#[test]
fn logic_op() -> LangResult<'static, ()> {
    let input = include_str!("logic_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            And,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(Name(pijama_ast::Name("b")).loc()),
        )
        .loc(),
        BinaryOp(
            Or,
            Box::new(
                BinaryOp(
                    And,
                    Box::new(Name(pijama_ast::Name("a")).loc()),
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                )
                .loc(),
            ),
            Box::new(Name(pijama_ast::Name("c")).loc()),
        )
        .loc(),
        BinaryOp(
            And,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(
                BinaryOp(
                    Or,
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "left-associative");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}

#[test]
fn bit_op() -> LangResult<'static, ()> {
    let input = include_str!("bit_op.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            BitAnd,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(Name(pijama_ast::Name("b")).loc()),
        )
        .loc(),
        BinaryOp(
            BitXor,
            Box::new(
                BinaryOp(
                    BitOr,
                    Box::new(
                        BinaryOp(
                            BitAnd,
                            Box::new(Name(pijama_ast::Name("a")).loc()),
                            Box::new(Name(pijama_ast::Name("b")).loc()),
                        )
                        .loc(),
                    ),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
            Box::new(Name(pijama_ast::Name("d")).loc()),
        )
        .loc(),
        BinaryOp(
            BitXor,
            Box::new(
                BinaryOp(
                    BitAnd,
                    Box::new(Name(pijama_ast::Name("a")).loc()),
                    Box::new(
                        BinaryOp(
                            BitOr,
                            Box::new(Name(pijama_ast::Name("b")).loc()),
                            Box::new(Name(pijama_ast::Name("c")).loc()),
                        )
                        .loc(),
                    ),
                )
                .loc(),
            ),
            Box::new(Name(pijama_ast::Name("d")).loc()),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "left-associative");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}

#[test]
fn let_bind() -> LangResult<'static, ()> {
    let input = include_str!("let_bind.pj");
    let result = parse(input)?.content;
    let expected = vec![
        LetBind(
            TyAnnotation {
                item: pijama_ast::Name("x").loc(),
                ty: Ty::Missing.loc(),
            },
            Box::new(Name(pijama_ast::Name("y")).loc()),
        )
        .loc(),
        LetBind(
            TyAnnotation {
                item: pijama_ast::Name("x").loc(),
                ty: Ty::Missing.loc(),
            },
            Box::new(
                BinaryOp(
                    Add,
                    Box::new(Name(pijama_ast::Name("y")).loc()),
                    Box::new(Name(pijama_ast::Name("z")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
        LetBind(
            TyAnnotation {
                item: pijama_ast::Name("x").loc(),
                ty: Ty::Int.loc(),
            },
            Box::new(Name(pijama_ast::Name("y")).loc()),
        )
        .loc(),
        LetBind(
            TyAnnotation {
                item: pijama_ast::Name("foo").loc(),
                ty: Ty::Missing.loc(),
            },
            Box::new(
                FnDef(
                    None,
                    vec![TyAnnotation {
                        item: pijama_ast::Name("x").loc(),
                        ty: Ty::Int.loc(),
                    }],
                    TyAnnotation {
                        item: vec![Name(pijama_ast::Name("x")).loc()].loc(),
                        ty: Ty::Missing.loc(),
                    },
                )
                .loc(),
            ),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple");
    assert_eq!(expected[1], result[1], "bind to bin op");
    assert_eq!(expected[2], result[2], "type binding");
    assert_eq!(expected[3], result[3], "bind to nameless function");
    Ok(())
}

#[test]
fn cond() -> LangResult<'static, ()> {
    let input = include_str!("cond.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Cond(
            Branch {
                cond: vec![Name(pijama_ast::Name("x")).loc()].loc(),
                body: vec![Name(pijama_ast::Name("y")).loc()].loc(),
            },
            vec![],
            vec![Name(pijama_ast::Name("z")).loc()].loc(),
        )
        .loc(),
        Cond(
            Branch {
                cond: vec![
                    Name(pijama_ast::Name("u")).loc(),
                    Name(pijama_ast::Name("v")).loc(),
                ]
                .loc(),
                body: vec![
                    Name(pijama_ast::Name("w")).loc(),
                    Name(pijama_ast::Name("x")).loc(),
                ]
                .loc(),
            },
            vec![],
            vec![
                Name(pijama_ast::Name("y")).loc(),
                Name(pijama_ast::Name("z")).loc(),
            ]
            .loc(),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "simple blocks");
    assert_eq!(expected[1], result[1], "long blocks");
    Ok(())
}

#[test]
fn elif() -> LangResult<'static, ()> {
    let input = include_str!("elif.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Cond(
            Branch {
                cond: vec![Name(pijama_ast::Name("x")).loc()].loc(),
                body: vec![Name(pijama_ast::Name("y")).loc()].loc(),
            },
            vec![Branch {
                cond: vec![Name(pijama_ast::Name("a")).loc()].loc(),
                body: vec![Name(pijama_ast::Name("b")).loc()].loc(),
            }],
            vec![Name(pijama_ast::Name("z")).loc()].loc(),
        )
        .loc(),
        Cond(
            Branch {
                cond: vec![
                    Name(pijama_ast::Name("u")).loc(),
                    Name(pijama_ast::Name("v")).loc(),
                ]
                .loc(),
                body: vec![
                    Name(pijama_ast::Name("w")).loc(),
                    Name(pijama_ast::Name("x")).loc(),
                ]
                .loc(),
            },
            vec![
                Branch {
                    cond: vec![
                        Name(pijama_ast::Name("a")).loc(),
                        Name(pijama_ast::Name("b")).loc(),
                    ]
                    .loc(),
                    body: vec![
                        Name(pijama_ast::Name("c")).loc(),
                        Name(pijama_ast::Name("d")).loc(),
                    ]
                    .loc(),
                },
                Branch {
                    cond: vec![
                        Name(pijama_ast::Name("e")).loc(),
                        Name(pijama_ast::Name("f")).loc(),
                    ]
                    .loc(),
                    body: vec![
                        Name(pijama_ast::Name("g")).loc(),
                        Name(pijama_ast::Name("h")).loc(),
                    ]
                    .loc(),
                },
            ],
            vec![
                Name(pijama_ast::Name("y")).loc(),
                Name(pijama_ast::Name("z")).loc(),
            ]
            .loc(),
        )
        .loc(),
    ];
    assert_eq!(expected[0], result[0], "simple blocks");
    assert_eq!(expected[1], result[1], "long blocks");
    Ok(())
}

#[test]
fn call() -> LangResult<'static, ()> {
    let input = include_str!("call.pj");
    let result = parse(input)?.content;
    let expected = vec![
        Call(Box::new(Name(pijama_ast::Name("x")).loc()), vec![]).loc(),
        Call(
            Box::new(Name(pijama_ast::Name("x")).loc()),
            vec![Name(pijama_ast::Name("y")).loc()],
        )
        .loc(),
        Call(
            Box::new(Name(pijama_ast::Name("x")).loc()),
            vec![
                Name(pijama_ast::Name("y")).loc(),
                Name(pijama_ast::Name("z")).loc(),
            ],
        )
        .loc(),
        Call(
            Box::new(
                BinaryOp(
                    Add,
                    Box::new(Name(pijama_ast::Name("x")).loc()),
                    Box::new(Name(pijama_ast::Name("y")).loc()),
                )
                .loc(),
            ),
            vec![Name(pijama_ast::Name("z")).loc()],
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "nullary call");
    assert_eq!(expected[1], result[1], "unary call");
    assert_eq!(expected[2], result[2], "binary call");
    assert_eq!(expected[3], result[3], "complex callee");
    Ok(())
}

#[test]
fn fn_def() -> LangResult<'static, ()> {
    let input = include_str!("fn_def.pj");
    let result = parse(input)?.content;
    let expected = vec![
        FnDef(
            Some(pijama_ast::Name("foo").loc()),
            vec![],
            TyAnnotation {
                item: vec![].loc(),
                ty: Ty::Missing.loc(),
            },
        )
        .loc(),
        FnDef(
            Some(pijama_ast::Name("foo").loc()),
            vec![TyAnnotation {
                item: pijama_ast::Name("x").loc(),
                ty: Ty::Int.loc(),
            }],
            TyAnnotation {
                item: vec![Name(pijama_ast::Name("x")).loc()].loc(),
                ty: Ty::Missing.loc(),
            },
        )
        .loc(),
        FnDef(
            Some(pijama_ast::Name("foo").loc()),
            vec![],
            TyAnnotation {
                item: vec![Call(Box::new(Name(pijama_ast::Name("foo")).loc()), vec![]).loc()].loc(),
                ty: Ty::Unit.loc(),
            },
        )
        .loc(),
        FnDef(
            Some(pijama_ast::Name("foo").loc()),
            vec![
                TyAnnotation {
                    item: pijama_ast::Name("x").loc(),
                    ty: Ty::Int.loc(),
                },
                TyAnnotation {
                    item: pijama_ast::Name("y").loc(),
                    ty: Ty::Int.loc(),
                },
            ],
            TyAnnotation {
                item: vec![
                    Name(pijama_ast::Name("x")).loc(),
                    Name(pijama_ast::Name("y")).loc(),
                ]
                .loc(),
                ty: Ty::Missing.loc(),
            },
        )
        .loc(),
        FnDef(
            None,
            vec![TyAnnotation {
                item: pijama_ast::Name("x").loc(),
                ty: Ty::Int.loc(),
            }],
            TyAnnotation {
                item: vec![Name(pijama_ast::Name("x")).loc()].loc(),
                ty: Ty::Missing.loc(),
            },
        )
        .loc(),
        FnDef(
            None,
            vec![TyAnnotation {
                item: pijama_ast::Name("x").loc(),
                ty: Ty::Int.loc(),
            }],
            TyAnnotation {
                item: vec![Name(pijama_ast::Name("x")).loc()].loc(),
                ty: Ty::Missing.loc(),
            },
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "nullary def");
    assert_eq!(expected[1], result[1], "unary def");
    assert_eq!(expected[2], result[2], "recursive def");
    assert_eq!(expected[3], result[3], "long body");
    assert_eq!(expected[4], result[4], "nameless");
    assert_eq!(expected[5], result[5], "nameless with space");
    Ok(())
}

#[test]
fn precedence() -> LangResult<'static, ()> {
    let input = include_str!("precedence.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            Add,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(
                BinaryOp(
                    Mul,
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
        BinaryOp(
            BitAnd,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(
                BinaryOp(
                    Add,
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
        BinaryOp(
            Eq,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(
                BinaryOp(
                    BitAnd,
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
        BinaryOp(
            And,
            Box::new(Name(pijama_ast::Name("a")).loc()),
            Box::new(
                BinaryOp(
                    Eq,
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
    ];
    assert_eq!(expected[0], result[0], "mul precedes add");
    assert_eq!(expected[1], result[1], "add precedes bitwise and");
    assert_eq!(expected[2], result[2], "bitwise and precedes equal");
    assert_eq!(expected[3], result[3], "equal precedes and");
    Ok(())
}

#[test]
fn cmp_and_shift() -> LangResult<'static, ()> {
    let input = include_str!("cmp_and_shift.pj");
    let result = parse(input)?.content;
    let expected = vec![
        BinaryOp(
            Lt,
            Box::new(
                BinaryOp(
                    Shl,
                    Box::new(Name(pijama_ast::Name("a")).loc()),
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                )
                .loc(),
            ),
            Box::new(
                BinaryOp(
                    Shl,
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                    Box::new(Name(pijama_ast::Name("d")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
        BinaryOp(
            Gt,
            Box::new(
                BinaryOp(
                    Shr,
                    Box::new(Name(pijama_ast::Name("a")).loc()),
                    Box::new(Name(pijama_ast::Name("b")).loc()),
                )
                .loc(),
            ),
            Box::new(
                BinaryOp(
                    Shr,
                    Box::new(Name(pijama_ast::Name("c")).loc()),
                    Box::new(Name(pijama_ast::Name("d")).loc()),
                )
                .loc(),
            ),
        )
        .loc(),
        BinaryOp(
            Shr,
            Box::new(
                BinaryOp(
                    Shr,
                    Box::new(Name(pijama_ast::Name("a")).loc()),
                    Box::new(
                        BinaryOp(
                            Gt,
                            Box::new(Name(pijama_ast::Name("b")).loc()),
                            Box::new(Name(pijama_ast::Name("c")).loc()),
                        )
                        .loc(),
                    ),
                )
                .loc(),
            ),
            Box::new(Name(pijama_ast::Name("d")).loc()),
        )
        .loc(),
    ];

    assert_eq!(expected[0], result[0], "left shift");
    assert_eq!(expected[1], result[1], "right shift");
    assert_eq!(expected[2], result[2], "brackets");
    Ok(())
}
