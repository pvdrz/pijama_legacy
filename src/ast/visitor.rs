use crate::{
    ast::{BinOp, Block, Literal, Located, Name, Node, UnOp},
    ty::{Binding, Ty},
};

/// Trait to traverse the AST.
///
/// This trait should be used when you need to traverse the AST and you are only interested in
/// particular elements of it or do not want to write the code necessary to traverse the AST by
/// yourself.
///
/// There are two kinds of methods here: - The `visit_<foo>` methods: where the code specific to
/// your visiting resides.  - The `super_<foo>` methods: that destructure each component and take
/// care of the actual visiting.
///
/// The `visit_<foo>` methods are the ones that should be modified. You should always call the
/// corresponding `super_<foo>` method inside your implementation of `visit_<foo>` to guarantee
/// that your visitor will visit the whole AST.
///
/// You should never implement `super_<foo>` unless you want to modify what to visit inside a
/// component.
///
/// Every update to the `Node` type should be reflected here too. Otherwise, it might end up
/// breaking all the processes that use this trait to traverse the AST.
pub trait NodeVisitor<'a> {
    fn super_block(&mut self, block: &Block<'a>) {
        for node in block {
            self.visit_node(&node);
        }
    }

    fn super_node(&mut self, node: &Located<Node<'a>>) {
        match &node.content {
            Node::BinaryOp(op, node1, node2) => {
                self.visit_binary_op(&op, node1.as_ref(), node2.as_ref())
            }
            Node::UnaryOp(op, node) => self.visit_unary_op(&op, node.as_ref()),
            Node::LetBind(name, opt_ty, node) => self.visit_let_bind(&name, &opt_ty, node.as_ref()),
            Node::Cond(if_blk, do_blk, el_blk) => self.visit_cond(&if_blk, &do_blk, &el_blk),
            Node::FnDef(opt_name, args, body, opt_ty) => {
                self.visit_fn_def(&opt_name, &args, &body, &opt_ty)
            }
            Node::Call(func, args) => self.visit_call(func.as_ref(), &args),
            Node::Literal(literal) => self.visit_literal(&literal),
            Node::Name(name) => self.visit_name(&name),
        }
    }

    fn super_binary_op(
        &mut self,
        _op: &BinOp,
        node1: &Located<Node<'a>>,
        node2: &Located<Node<'a>>,
    ) {
        self.visit_node(node1);
        self.visit_node(node2);
    }

    fn super_unary_op(&mut self, _op: &UnOp, node: &Located<Node<'a>>) {
        self.visit_node(node);
    }

    fn super_let_bind(
        &mut self,
        name: &Located<Name<'a>>,
        _opt_ty: &Option<Located<Ty>>,
        node: &Located<Node<'a>>,
    ) {
        self.visit_name(&name.content);
        self.visit_node(node);
    }

    fn super_cond(
        &mut self,
        if_blk: &Located<Block<'a>>,
        do_blk: &Located<Block<'a>>,
        el_blk: &Located<Block<'a>>,
    ) {
        self.visit_block(&if_blk.content);
        self.visit_block(&do_blk.content);
        self.visit_block(&el_blk.content);
    }

    fn super_fn_def(
        &mut self,
        opt_name: &Option<Located<Name<'a>>>,
        _args: &Vec<Located<Binding<'a>>>,
        body: &Located<Block<'a>>,
        _opt_ty: &Option<Located<Ty>>,
    ) {
        if let Some(name) = opt_name {
            self.visit_name(&name.content);
        }

        self.visit_block(&body.content);
    }

    fn super_call(&mut self, func: &Located<Node<'a>>, args: &Block<'a>) {
        self.visit_node(func);
        self.visit_block(args);
    }

    fn super_literal(&mut self, _literal: &Literal) {}

    fn super_name(&mut self, _name: &Name<'a>) {}

    fn visit_block(&mut self, block: &Block<'a>) {
        self.super_block(block);
    }

    fn visit_node(&mut self, node: &Located<Node<'a>>) {
        self.super_node(node)
    }

    fn visit_binary_op(
        &mut self,
        op: &BinOp,
        node1: &Located<Node<'a>>,
        node2: &Located<Node<'a>>,
    ) {
        self.super_binary_op(op, node1, node2);
    }

    fn visit_unary_op(&mut self, op: &UnOp, node: &Located<Node<'a>>) {
        self.super_unary_op(op, node);
    }

    fn visit_let_bind(
        &mut self,
        name: &Located<Name<'a>>,
        opt_ty: &Option<Located<Ty>>,
        node: &Located<Node<'a>>,
    ) {
        self.super_let_bind(name, opt_ty, node);
    }

    fn visit_cond(
        &mut self,
        if_blk: &Located<Block<'a>>,
        do_blk: &Located<Block<'a>>,
        el_blk: &Located<Block<'a>>,
    ) {
        self.super_cond(if_blk, do_blk, el_blk);
    }

    fn visit_fn_def(
        &mut self,
        opt_name: &Option<Located<Name<'a>>>,
        args: &Vec<Located<Binding<'a>>>,
        body: &Located<Block<'a>>,
        opt_ty: &Option<Located<Ty>>,
    ) {
        self.super_fn_def(opt_name, args, body, opt_ty);
    }

    fn visit_call(&mut self, func: &Located<Node<'a>>, args: &Block<'a>) {
        self.super_call(func, args)
    }

    fn visit_literal(&mut self, literal: &Literal) {
        self.super_literal(literal);
    }

    fn visit_name(&mut self, name: &Name<'a>) {
        self.super_name(name);
    }
}
