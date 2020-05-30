//! Trait to traverse the AST.
use crate::{
    ty::TyAnnotation, BinOp, Block, Branch, Literal, Located, Name, Node, Primitive, UnOp,
};

/// Trait for the node visitor pattern.
///
/// This trait should be used when you need to traverse the AST and you are only interested in
/// particular elements of it or you do not want to write the code necessary to traverse the AST
/// yourself.
///
/// There are two kinds of methods:
/// - The `visit_<foo>` methods: where the code specific to
/// your visiting resides.
/// - The `super_<foo>` methods: that destructure each component and take
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
    /// Visits a Block.
    fn super_block(&mut self, block: &Block<'a>) {
        for node in block {
            self.visit_node(&node);
        }
    }

    /// Visits a Node.
    fn super_node(&mut self, node: &Located<Node<'a>>) {
        match &node.content {
            Node::BinaryOp(op, node1, node2) => {
                self.visit_binary_op(*op, node1.as_ref(), node2.as_ref())
            }
            Node::UnaryOp(op, node) => self.visit_unary_op(*op, node.as_ref()),
            Node::LetBind(annotation, node) => self.visit_let_bind(annotation, node.as_ref()),
            Node::Cond(if_branch, branches, el_blk) => self.visit_cond(if_branch, branches, el_blk),
            Node::FnDef(opt_name, args, body) => self.visit_fn_def(opt_name, args, body),
            Node::Call(func, args) => self.visit_call(func.as_ref(), &args),
            Node::Literal(literal) => self.visit_literal(literal),
            Node::Name(name) => self.visit_name(name),
            Node::PrimFn(primitive) => self.visit_prim_fn(*primitive),
        }
    }

    /// Visits a Node with a Binary operation.
    fn super_binary_op(
        &mut self,
        _op: BinOp,
        node1: &Located<Node<'a>>,
        node2: &Located<Node<'a>>,
    ) {
        self.visit_node(node1);
        self.visit_node(node2);
    }

    /// Visits a Node with a Unary operation.
    fn super_unary_op(&mut self, _op: UnOp, node: &Located<Node<'a>>) {
        self.visit_node(node);
    }

    /// Visits a Node with a Let binding.
    fn super_let_bind(&mut self, annotation: &TyAnnotation<Name<'a>>, node: &Located<Node<'a>>) {
        self.visit_name(&annotation.item.content);
        self.visit_node(node);
    }

    /// Visits a Node with a Conditional.
    fn super_cond(
        &mut self,
        if_branch: &Branch<'a>,
        branches: &[Branch<'a>],
        el_blk: &Located<Block<'a>>,
    ) {
        self.visit_branch(if_branch);

        for branch in branches {
            self.visit_branch(branch);
        }

        self.visit_block(&el_blk.content);
    }

    /// Visits a Node with a single Branch.
    fn super_branch(&mut self, branch: &Branch<'a>) {
        let cond = &branch.cond;
        let body = &branch.body;

        self.visit_block(&cond.content);
        self.visit_block(&body.content);
    }

    /// Visits a Node with a Function Definition.
    fn super_fn_def(
        &mut self,
        opt_name: &Option<Located<Name<'a>>>,
        _args: &[TyAnnotation<Name<'a>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        if let Some(name) = opt_name {
            self.visit_name(&name.content);
        }

        self.visit_block(&body.item.content);
    }

    /// Visits a Node with a Function Call.
    fn super_call(&mut self, func: &Located<Node<'a>>, args: &Block<'a>) {
        self.visit_node(func);
        self.visit_block(args);
    }

    /// Visits a Node with a Literal.
    fn super_literal(&mut self, _literal: &Literal) {}

    /// Vishts a Node with a Name.
    fn super_name(&mut self, _name: &Name<'a>) {}

    /// Visits a Node with a Primitive function.
    fn super_prim_fn(&mut self, _prim_fn: Primitive) {}

    /// Specifies how Blocks should be visited.
    fn visit_block(&mut self, block: &Block<'a>) {
        self.super_block(block);
    }

    /// Specifies how Nodes should be visited.
    fn visit_node(&mut self, node: &Located<Node<'a>>) {
        self.super_node(node)
    }

    /// Specifies how Binary operations should be visited.
    fn visit_binary_op(&mut self, op: BinOp, node1: &Located<Node<'a>>, node2: &Located<Node<'a>>) {
        self.super_binary_op(op, node1, node2);
    }

    /// Specifies how Unary operations should be visited.
    fn visit_unary_op(&mut self, op: UnOp, node: &Located<Node<'a>>) {
        self.super_unary_op(op, node);
    }

    /// Specifies how Let bindings should be visited.
    fn visit_let_bind(&mut self, annotation: &TyAnnotation<Name<'a>>, node: &Located<Node<'a>>) {
        self.super_let_bind(annotation, node);
    }

    /// Specifies how Conditionals should be visited.
    fn visit_cond(
        &mut self,
        if_branch: &Branch<'a>,
        branches: &[Branch<'a>],
        el_blk: &Located<Block<'a>>,
    ) {
        self.super_cond(if_branch, branches, el_blk);
    }

    /// Specifies how Branches should be visited.
    fn visit_branch(&mut self, branch: &Branch<'a>) {
        self.super_branch(branch);
    }

    /// Specifies how Function Definitions should be visited.
    fn visit_fn_def(
        &mut self,
        opt_name: &Option<Located<Name<'a>>>,
        args: &[TyAnnotation<Name<'a>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        self.super_fn_def(opt_name, args, body);
    }

    /// Specifies how Function Calls should be visited.
    fn visit_call(&mut self, func: &Located<Node<'a>>, args: &Block<'a>) {
        self.super_call(func, args)
    }

    /// Specifies how Literals should be visited.
    fn visit_literal(&mut self, literal: &Literal) {
        self.super_literal(literal);
    }

    /// Specifies how Names should be visited.
    fn visit_name(&mut self, name: &Name<'a>) {
        self.super_name(name);
    }

    /// Specifies how Pimitive Functions should be visited.
    fn visit_prim_fn(&mut self, prim_fn: Primitive) {
        self.super_prim_fn(prim_fn);
    }
}
