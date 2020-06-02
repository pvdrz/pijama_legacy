//! Trait to traverse the AST.
use crate::{
    location::Located,
    node::{BinOp, Block, Branch, Expression, Literal, Name, Node, Primitive, Statement, UnOp},
    ty::TyAnnotation,
};

/// Trait for the node visitor pattern.
///
/// This trait should be used when you need to traverse the AST and you are only interested in
/// particular elements of it or you do not want to write the code necessary to traverse the AST
/// yourself.
///
/// There are two kinds of methods:
/// - The `visit_<foo>` methods: where the code specific to your visiting resides.
/// - The `super_<foo>` methods: that destructure each component and take care of the actual
/// visiting.
///
/// Most of the time, the `visit_<foo>` methods are the ones that should be implemented. It is
/// important that the corresponding `super_<foo>` method is called at the end of the
/// implementation of `visit_<foo>` to guarantee that your visitor will visit the whole AST.
///
/// You should never implement `super_<foo>` unless you want to modify what to visit inside an AST
/// item.
///
/// Every update to the `Node` type should be reflected here too. Otherwise, it might end up
/// breaking all the processes that use this trait to traverse the AST.
pub trait NodeVisitor<'a> {
    /// Destructures a block to visit its children.
    fn super_block(&mut self, block: &Block<'a>) {
        for node in &block.nodes {
            self.visit_node(node);
        }
        self.visit_expression(block.expr.as_ref());
    }
    /// Destructures a node to visit its children.
    fn super_node(&mut self, node: &Node<'a>) {
        match node {
            Node::Stat(stat) => self.visit_statement(&stat),
            Node::Expr(expr) => self.visit_expression(&expr),
        }
    }
    /// Destructures a statement to visit its children.
    fn super_statement(&mut self, stat: &Located<Statement<'a>>) {
        match &stat.content {
            Statement::Assign(annotation, expr) => self.visit_assign(annotation, expr),
            Statement::FnDef(name, args, body) => self.visit_fn_def(name, args, body),
        }
    }
    /// Destructures an expression to visit its children.
    fn super_expression(&mut self, expr: &Located<Expression<'a>>) {
        match &expr.content {
            Expression::BinaryOp(op, expr1, expr2) => {
                self.visit_binary_op(*op, expr1.as_ref(), expr2.as_ref())
            }
            Expression::UnaryOp(op, expr) => self.visit_unary_op(*op, expr.as_ref()),
            Expression::Cond(if_branch, branches, el_blk) => {
                self.visit_cond(if_branch, branches, el_blk)
            }
            Expression::AnonFn(args, body) => self.visit_anon_fn(args, body),
            Expression::Call(func, args) => self.visit_call(func.as_ref(), &args),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Name(name) => self.visit_name(name),
            Expression::PrimFn(primitive) => self.visit_prim_fn(*primitive),
        }
    }
    /// Destructures a binary operation to visit its children.
    fn super_binary_op(
        &mut self,
        _op: BinOp,
        expr1: &Located<Expression<'a>>,
        expr2: &Located<Expression<'a>>,
    ) {
        self.visit_expression(expr1);
        self.visit_expression(expr2);
    }
    /// Destructures a unary operation to visit its children.
    fn super_unary_op(&mut self, _op: UnOp, expr: &Located<Expression<'a>>) {
        self.visit_expression(expr);
    }
    /// Destructures an assignment to visit its children.
    fn super_assign(
        &mut self,
        annotation: &TyAnnotation<Located<Name<'a>>>,
        expr: &Located<Expression<'a>>,
    ) {
        self.visit_name(&annotation.item.content);
        self.visit_expression(expr);
    }
    /// Destructures a conditional to visit its children.
    fn super_cond(&mut self, if_branch: &Branch<'a>, branches: &[Branch<'a>], el_blk: &Block<'a>) {
        self.visit_branch(if_branch);

        for branch in branches {
            self.visit_branch(branch);
        }

        self.visit_block(el_blk);
    }
    /// Destructures a branch to visit its children.
    fn super_branch(&mut self, branch: &Branch<'a>) {
        self.visit_block(&branch.cond);
        self.visit_block(&branch.body);
    }
    /// Destructures a function definition to visit its children.
    fn super_fn_def(
        &mut self,
        name: &Located<Name<'a>>,
        args: &[TyAnnotation<Located<Name<'a>>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        self.visit_name(&name.content);
        for ann in args {
            self.visit_name(&ann.item.content);
        }
        self.visit_block(&body.item);
    }
    /// Destructures an anonymous function to visit its children.
    fn super_anon_fn(
        &mut self,
        args: &[TyAnnotation<Located<Name<'a>>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        for ann in args {
            self.visit_name(&ann.item.content);
        }
        self.visit_block(&body.item);
    }
    /// Destructures a function call to visit its children.
    fn super_call(&mut self, func: &Located<Expression<'a>>, args: &[Located<Expression<'a>>]) {
        self.visit_expression(func);
        for expr in args {
            self.visit_expression(expr);
        }
    }
    /// Destructures a literal to visit its children.
    fn super_literal(&mut self, _literal: &Literal) {}
    /// Destructures a name to visit its children.
    fn super_name(&mut self, _name: &Name<'a>) {}
    /// Destructures a primitive function to visit its children.
    fn super_prim_fn(&mut self, _prim: Primitive) {}
    /// Specifies how blocks should be visited.
    fn visit_block(&mut self, block: &Block<'a>) {
        self.super_block(block);
    }
    /// Specifies how nodes should be visited.
    fn visit_node(&mut self, node: &Node<'a>) {
        self.super_node(node)
    }
    /// Specifies how statements should be visited.
    fn visit_statement(&mut self, stat: &Located<Statement<'a>>) {
        self.super_statement(stat)
    }
    /// Specifies how expressions should be visited.
    fn visit_expression(&mut self, expr: &Located<Expression<'a>>) {
        self.super_expression(expr)
    }
    /// Specifies how binary operations should be visited.
    fn visit_binary_op(
        &mut self,
        op: BinOp,
        expr1: &Located<Expression<'a>>,
        expr2: &Located<Expression<'a>>,
    ) {
        self.super_binary_op(op, expr1, expr2);
    }
    /// Specifies how unary operations should be visited.
    fn visit_unary_op(&mut self, op: UnOp, expr: &Located<Expression<'a>>) {
        self.super_unary_op(op, expr);
    }
    /// Specifies how assignments should be visited.
    fn visit_assign(
        &mut self,
        annotation: &TyAnnotation<Located<Name<'a>>>,
        expr: &Located<Expression<'a>>,
    ) {
        self.super_assign(annotation, expr);
    }
    /// Specifies how conditionals should be visited.
    fn visit_cond(&mut self, if_branch: &Branch<'a>, branches: &[Branch<'a>], el_blk: &Block<'a>) {
        self.super_cond(if_branch, branches, el_blk);
    }
    /// Specifies how branches should be visited.
    fn visit_branch(&mut self, branch: &Branch<'a>) {
        self.super_branch(branch);
    }
    /// Specifies how function definitions should be visited.
    fn visit_fn_def(
        &mut self,
        name: &Located<Name<'a>>,
        args: &[TyAnnotation<Located<Name<'a>>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        self.super_fn_def(name, args, body);
    }
    /// Specifies how anonymous functions should be visited.
    fn visit_anon_fn(
        &mut self,
        args: &[TyAnnotation<Located<Name<'a>>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        self.super_anon_fn(args, body);
    }
    /// Specifies how function calls should be visited.
    fn visit_call(&mut self, func: &Located<Expression<'a>>, args: &[Located<Expression<'a>>]) {
        self.super_call(func, args)
    }
    /// Specifies how literals should be visited.
    fn visit_literal(&mut self, literal: &Literal) {
        self.super_literal(literal);
    }
    /// Specifies how names should be visited.
    fn visit_name(&mut self, name: &Name<'a>) {
        self.super_name(name);
    }
    /// Specifies how pimitive functions should be visited.
    fn visit_prim_fn(&mut self, prim: Primitive) {
        self.super_prim_fn(prim);
    }
}
