use std::collections::HashMap;

use crate::ir::Symbol;

#[derive(Debug, Clone, Copy)]
struct ScopeId(usize);

#[derive(Default)]
struct Scope<'a> {
    parent: Option<ScopeId>,
    table: HashMap<&'a str, Symbol>,
}

impl<'a> Scope<'a> {
    fn parent(&self) -> Option<ScopeId> {
        self.parent
    }

    fn define(&mut self, name: &'a str, symbol: Symbol) {
        self.table.insert(name, symbol);
    }

    fn resolve(&self, name: &'a str) -> Option<&Symbol> {
        self.table.get(name)
    }
}

pub struct ScopeArena<'a> {
    count: usize,
    scopes: Vec<Scope<'a>>,
    curr: ScopeId,
}

impl<'a> Default for ScopeArena<'a> {
    fn default() -> Self {
        let mut scopes = ScopeArena {
            count: 0,
            scopes: Vec::new(),
            curr: ScopeId(0),
        };
        scopes.curr = scopes.nest_with_parent(None);
        scopes
    }
}

impl<'a> ScopeArena<'a> {
    fn new_id(&mut self) -> ScopeId {
        let id = ScopeId(self.count);
        self.count += 1;
        debug_assert_eq!(self.count, self.scopes.len());
        id
    }

    fn nest_with_parent(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let scope = Scope {
            parent,
            table: HashMap::new(),
        };
        self.scopes.push(scope);
        self.new_id()
    }

    fn get(&self, id: ScopeId) -> &Scope<'a> {
        self.scopes.get(id.0).expect("Invalid ScopeId")
    }

    fn get_mut(&mut self, id: ScopeId) -> &mut Scope<'a> {
        self.scopes.get_mut(id.0).expect("Invalid ScopeId")
    }

    fn curr(&self) -> &Scope<'a> {
        self.get(self.curr)
    }

    fn curr_mut(&mut self) -> &mut Scope<'a> {
        self.get_mut(self.curr)
    }

    pub fn define(&mut self, name: &'a str, symbol: Symbol) {
        self.curr_mut().define(name, symbol);
    }

    pub fn resolve(&self, name: &'a str) -> Option<&Symbol> {
        let scope = self.curr();
        scope
            .resolve(name)
            .or_else(|| scope.parent().and_then(|id| self.get(id).resolve(name)))
    }

    pub fn up(&mut self) {
        self.curr = self.curr().parent().unwrap_or(self.curr);
    }

    pub fn down(&mut self) {
        self.curr = self.nest_with_parent(Some(self.curr));
    }
}
