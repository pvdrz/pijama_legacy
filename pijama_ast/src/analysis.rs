//! An assortment of checks that are done before lowering.
use crate::{
    node::{Block, Expression},
    ty::TyAnnotation,
    visitor::NodeVisitor,
};

use pijama_common::{location::Located, Local};

/// Checks if a function is recursive or not.
pub fn is_fn_def_recursive<'a>(name: Local<'a>, body: &Block<'a>) -> bool {
    let mut checker = RecursionChecker {
        name,
        is_rec: false,
        is_shadowed: false,
        stack: Vec::default(),
    };
    checker.visit_block(body);
    // Check to ensure that the there is only one scope after visiting the body.
    assert!(
        checker.stack.is_empty(),
        "Someone forgot to pop a scope from the stack"
    );
    checker.is_rec
}

/// Visitor that checks if a function is recursive or not.
struct RecursionChecker<'a> {
    /// Local of the target function
    name: Local<'a>,
    /// Stores if the function is recursive or not in each step of the traversal.
    is_rec: bool,
    /// Stores if the target name is being shadowed in the current scope. It represents the top of
    /// the stack
    is_shadowed: bool,
    /// Stores the shadowing status in the upper scopes.
    stack: Vec<bool>,
}

impl<'a> RecursionChecker<'a> {
    /// Push a new scope onto the stack.
    ///
    /// The new scope has the same shadowed status as the old scope because names are preserved
    /// when creating a new scope.
    fn push_scope(&mut self) {
        self.stack.push(self.is_shadowed)
    }

    /// Pops a scope from the stack.
    ///
    /// This function panics if there are no more scopes in the stack. Which should be impossible
    /// because the stack always starts as non-empty and we should only pop newly added scopes from
    /// the stack.
    fn pop_scope(&mut self) {
        self.is_shadowed = self
            .stack
            .pop()
            .expect("there are no more scopes in the stack");
    }
}

impl<'a> NodeVisitor<'a> for RecursionChecker<'a> {
    fn visit_local(&mut self, name: &Local<'a>) {
        // The function is recursive if its name is not shadowed in the current scope and we found
        // it somewhere inside its body.
        if !self.is_shadowed && *name == self.name {
            self.is_rec = true;
        }
        // Keep visiting
        self.super_local(name);
    }

    fn visit_assign(
        &mut self,
        annotation: &TyAnnotation<Located<Local<'a>>>,
        expr: &Located<Expression<'a>>,
    ) {
        // If the binding binds the target name, the latter is being shadowed in the current scope.
        if annotation.item.content == self.name {
            self.is_shadowed = true;
        }
        // Keep visiting
        self.super_assign(annotation, expr);
    }

    fn visit_fn_def(
        &mut self,
        name: &Located<Local<'a>>,
        args: &[TyAnnotation<Located<Local<'a>>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        if name.content == self.name {
            // If the function definition binds the target name, the latter is being shadowed in
            // the current scope.
            self.is_shadowed = true;
        } else {
            for arg in args {
                // If any of the arguments uses the same name as the target, the latter is being
                // shadowed in the current scope.
                if arg.item.content == self.name {
                    self.is_shadowed = true;
                    break;
                }
            }
        }
        // Keep visiting
        self.super_fn_def(name, args, body);
    }

    fn visit_anon_fn(
        &mut self,
        args: &[TyAnnotation<Located<Local<'a>>>],
        body: &TyAnnotation<Block<'a>>,
    ) {
        for arg in args {
            // If any of the arguments uses the same name as the target, the latter is being
            // shadowed in the current scope.
            if arg.item.content == self.name {
                self.is_shadowed = true;
                break;
            }
        }
        // Keep visiting
        self.super_anon_fn(args, body);
    }

    fn visit_block(&mut self, block: &Block<'a>) {
        // Entering a block means that we need to push a new scope into the stack because the
        // bindings done inside the block can only exist in that block.
        self.push_scope();
        // Keep visiting
        self.super_block(block);
        // Pop the scope after visiting the block because all the bindings inside the block are
        // discarded outside it.
        self.pop_scope();
    }
}
