use crate::{
    ast::{BinOp, Block, Literal, Located, Location, Name, Node, UnOp},
    mir::Term,
    ty::{expect_ty, ty_check, Binding, Ty, TyResult},
};

pub fn lower_blk<'a>(blk: Located<Block<'a>>) -> TyResult<Located<Term<'a>>> {
    let mut terms = blk.content.into_iter().rev().map(lower_node);
    if let Some(term) = terms.next() {
        let mut term = term?;
        for prev_term in terms {
            let prev_term = prev_term?;
            let next_term = Box::new(term);

            let loc = prev_term.loc;
            let content = if let Term::Let(name, value, _) = prev_term.content {
                Term::Let(name, value, next_term)
            } else {
                Term::Seq(Box::new(prev_term), next_term)
            };
            term = Located::new(content, loc);
        }
        Ok(term)
    } else {
        Ok(Located::new(Term::Lit(Literal::Unit), blk.loc))
    }
}

fn lower_node(node: Located<Node<'_>>) -> TyResult<Located<Term<'_>>> {
    let loc = node.loc;
    let term = match node.content {
        Node::Name(name) => Ok(Located::new(Term::Var(name), loc)),
        Node::Cond(if_blk, do_blk, el_blk) => lower_cond(loc, if_blk, do_blk, el_blk),
        Node::Literal(lit) => Ok(Located::new(Term::Lit(lit), loc)),
        Node::Call(name, args) => lower_call(loc, name, args),
        Node::BinaryOp(bin_op, node1, node2) => lower_binary_op(loc, bin_op, *node1, *node2),
        Node::UnaryOp(un_op, node) => lower_unary_op(loc, un_op, *node),
        Node::LetBind(name, opt_ty, node) => lower_let_bind(loc, name, opt_ty, *node),
        Node::FnDef(opt_name, binds, body, opt_ty) => {
            lower_fn_def(loc, opt_name, binds, body, opt_ty)
        }
        Node::FnRecDef(name, binds, body, ty) => lower_fn_rec_def(loc, name, binds, body, ty),
    }?;
    Ok(term)
}

fn lower_cond<'a>(
    loc: Location,
    if_blk: Located<Block<'a>>,
    do_blk: Located<Block<'a>>,
    el_blk: Located<Block<'a>>,
) -> TyResult<Located<Term<'a>>> {
    Ok(Located::new(
        Term::Cond(
            Box::new(lower_blk(if_blk)?),
            Box::new(lower_blk(do_blk)?),
            Box::new(lower_blk(el_blk)?),
        ),
        loc,
    ))
}

fn lower_call<'a>(
    loc: Location,
    name: Located<Name<'a>>,
    args: Block<'a>,
) -> TyResult<Located<Term<'a>>> {
    let mut term = Located::new(Term::Var(name.content), loc);
    for node in args {
        term = Located::new(Term::App(Box::new(term), Box::new(lower_node(node)?)), loc);
    }
    Ok(term)
}

fn lower_binary_op<'a>(
    loc: Location,
    bin_op: BinOp,
    node1: Located<Node<'a>>,
    node2: Located<Node<'a>>,
) -> TyResult<Located<Term<'a>>> {
    Ok(Located::new(
        Term::BinaryOp(
            bin_op,
            Box::new(lower_node(node1)?),
            Box::new(lower_node(node2)?),
        ),
        loc,
    ))
}

fn lower_unary_op(
    loc: Location,
    un_op: UnOp,
    node: Located<Node<'_>>,
) -> TyResult<Located<Term<'_>>> {
    Ok(Located::new(
        Term::UnaryOp(un_op, Box::new(lower_node(node)?)),
        loc,
    ))
}

fn lower_let_bind<'a>(
    loc: Location,
    name: Located<Name<'a>>,
    opt_ty: Option<Located<Ty>>,
    node: Located<Node<'a>>,
) -> TyResult<Located<Term<'a>>> {
    let term = lower_node(node)?;

    if let Some(ty) = opt_ty {
        let term_ty = ty_check(&term)?;
        expect_ty(ty.content, term_ty)?;
    }

    Ok(Located::new(
        Term::Let(
            name,
            Box::new(term),
            Box::new(Located::new(
                Term::Lit(Literal::Unit),
                Location::new(loc.end, loc.end),
            )),
        ),
        loc,
    ))
}

fn lower_fn_def<'a>(
    loc: Location,
    opt_name: Option<Located<Name<'a>>>,
    binds: Vec<Located<Binding<'a>>>,
    body: Located<Block<'a>>,
    opt_ty: Option<Located<Ty>>,
) -> TyResult<Located<Term<'a>>> {
    let mut term = lower_blk(body)?;

    let opt_ty = opt_ty.map(|ty| {
        let mut ty = ty.content;
        for bind in binds.iter().rev() {
            ty = Ty::Arrow(Box::new(bind.content.ty.clone()), Box::new(ty));
        }
        ty
    });

    for bind in binds.into_iter().rev() {
        term = Located::new(Term::Abs(bind.content, Box::new(term)), loc);
    }

    if let Some(ty) = opt_ty {
        let term_ty = ty_check(&term)?;
        expect_ty(ty, term_ty)?;
    }

    if let Some(name) = opt_name {
        term = Located::new(
            Term::Let(
                name,
                Box::new(term),
                Box::new(Located::new(
                    Term::Lit(Literal::Unit),
                    Location::new(loc.end, loc.end),
                )),
            ),
            loc,
        );
    }

    Ok(term)
}

fn lower_fn_rec_def<'a>(
    loc: Location,
    name: Located<Name<'a>>,
    binds: Vec<Located<Binding<'a>>>,
    body: Located<Block<'a>>,
    ty: Located<Ty>,
) -> TyResult<Located<Term<'a>>> {
    let mut term = lower_blk(body)?;

    let mut ty = ty.content;

    for bind in binds.into_iter().rev() {
        let loc = term.loc;
        ty = Ty::Arrow(Box::new(bind.content.ty.clone()), Box::new(ty));
        term = Located::new(Term::Abs(bind.content, Box::new(term)), loc);
    }

    term = Located::new(
        Term::Fix(Box::new(Located::new(
            Term::Abs(
                Binding {
                    name: name.content,
                    ty,
                },
                Box::new(term),
            ),
            loc,
        ))),
        loc,
    );

    Ok(Located::new(
        Term::Let(
            name,
            Box::new(term),
            Box::new(Located::new(
                Term::Lit(Literal::Unit),
                Location::new(loc.end, loc.end),
            )),
        ),
        loc,
    ))
}
