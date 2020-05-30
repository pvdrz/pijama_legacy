//! Diverse checks that need to be done before lowering.
use crate::{
    ty::{Ty, TyAnnotation},
    visitor::{BlockRef, NodeVisitor},
    Block, Located, Name, Node,
};

/// Checks if a function is recursive or not.
pub struct RecursionChecker<'a> {
    /// Name of the target function
    name: Name<'a>,
    /// Stores if the function is recursive or not in each step of the traversal.
    is_rec: bool,
    /// Stores if the target name is being shadowed in the current scope. It represents the top of
    /// the stack
    is_shadowed: bool,
    /// Stores the shadowing status in the upper scopes.
    stack: Vec<bool>,
}

impl<'a> RecursionChecker<'a> {
    /// Runs the recursion check with the target function's name and body.
    pub fn run(name: Name<'a>, body: BlockRef<'a, '_>) -> bool {
        let mut this = RecursionChecker {
            name,
            is_rec: false,
            is_shadowed: false,
            stack: Vec::new(),
        };
        this.visit_block(body);
        // Sanity check. There should be only one scope after visiting the body function. the
        // original one
        assert!(
            this.stack.is_empty(),
            "Someone forgot to pop a scope from the stack"
        );
        this.is_rec
    }

    /// Push a new scope into the stack.
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
    fn visit_name(&mut self, name: &Name<'a>) {
        // The function is recursive if its name is not shadowed in the current scope and we found
        // it is somewhere inside its body.
        if !self.is_shadowed && *name == self.name {
            self.is_rec = true;
        }
        // Keep visiting
        self.super_name(name);
    }

    fn visit_let_bind(
        &mut self,
        name: &Located<Name<'a>>,
        opt_ty: &Option<Located<Ty>>,
        body: &Located<Node<'a>>,
    ) {
        // If the binding binds the target name, the latter is being shadowed in the current scope.
        if name.content == self.name {
            self.is_shadowed = true;
        }
        // Keep visiting
        self.super_let_bind(name, opt_ty, body);
    }

    fn visit_fn_def(
        &mut self,
        opt_name: &Option<Located<Name<'a>>>,
        args: &[Located<TyAnnotation<'a>>],
        body: &Located<Block<'a>>,
        opt_ty: &Option<Located<Ty>>,
    ) {
        // If the function definition binds the target name, the latter is being shadowed in the
        // current scope.
        match opt_name {
            Some(name) if name.content == self.name => {
                self.is_shadowed = true;
            }
            _ => {}
        };
        // Keep visiting
        self.super_fn_def(opt_name, args, body, opt_ty);
    }

    fn visit_block(&mut self, block: BlockRef<'a, '_>) {
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
