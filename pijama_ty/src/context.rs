use std::{collections::HashMap, fmt::Debug, hash::Hash};

use pijama_common::location::Location;

use crate::Ty;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LocalId(usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TermId(usize);

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub ty: Ty,
    pub loc: Location,
}

pub trait ContextExt<Id: Debug + Hash + Eq + Copy> {
    fn new_id(&mut self) -> Id;
    fn insert_location(&mut self, id: Id, loc: Location);
    fn insert_type_info(&mut self, id: Id, info: TypeInfo);
    fn get_location(&self, id: Id) -> Option<Location>;
    fn get_type_info(&self, id: Id) -> Option<&TypeInfo>;
}

pub struct Store<Id> {
    gen_id: Generator<Id>,
    locations: HashMap<Id, Location>,
    type_info: HashMap<Id, TypeInfo>,
}

impl<Id: Debug + Hash + Eq + Copy> ContextExt<Id> for Store<Id> {
    fn new_id(&mut self) -> Id {
        self.gen_id.gen()
    }

    fn insert_location(&mut self, id: Id, loc: Location) {
        assert!(
            self.locations.insert(id, loc).is_none(),
            "Overwrote location for {:?}",
            id
        );
    }
    fn insert_type_info(&mut self, id: Id, info: TypeInfo) {
        assert!(
            self.type_info.insert(id, info).is_none(),
            "Overwrote type information for {:?}",
            id
        );
    }

    fn get_location(&self, id: Id) -> Option<Location> {
        self.locations.get(&id).cloned()
    }

    fn get_type_info(&self, id: Id) -> Option<&TypeInfo> {
        self.type_info.get(&id)
    }
}

pub struct Context {
    local: Store<LocalId>,
    term: Store<TermId>,
    ty_gen: Generator<Ty>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            local: Store {
                gen_id: Generator::new(LocalId),
                locations: HashMap::default(),
                type_info: HashMap::default(),
            },
            term: Store {
                gen_id: Generator::new(TermId),
                locations: HashMap::default(),
                type_info: HashMap::default(),
            },
            ty_gen: Generator::new(Ty::Var),
        }
    }

    pub fn new_ty(&mut self) -> Ty {
        self.ty_gen.gen()
    }

    pub fn iter_mut_local_types(&mut self) -> impl Iterator<Item = (LocalId, &mut Ty)> {
        self.local
            .type_info
            .iter_mut()
            .map(|(id, info)| (*id, &mut info.ty))
    }

    pub fn iter_mut_term_types(&mut self) -> impl Iterator<Item = (TermId, &mut Ty)> {
        self.term
            .type_info
            .iter_mut()
            .map(|(id, info)| (*id, &mut info.ty))
    }
}

impl ContextExt<TermId> for Context {
    fn new_id(&mut self) -> TermId {
        self.term.new_id()
    }

    fn insert_location(&mut self, id: TermId, loc: Location) {
        self.term.insert_location(id, loc)
    }

    fn insert_type_info(&mut self, id: TermId, info: TypeInfo) {
        self.term.insert_type_info(id, info)
    }

    fn get_location(&self, id: TermId) -> Option<Location> {
        self.term.get_location(id)
    }

    fn get_type_info(&self, id: TermId) -> Option<&TypeInfo> {
        self.term.get_type_info(id)
    }
}

impl ContextExt<LocalId> for Context {
    fn new_id(&mut self) -> LocalId {
        self.local.new_id()
    }

    fn insert_location(&mut self, id: LocalId, loc: Location) {
        self.local.insert_location(id, loc)
    }

    fn insert_type_info(&mut self, id: LocalId, info: TypeInfo) {
        self.local.insert_type_info(id, info)
    }

    fn get_location(&self, id: LocalId) -> Option<Location> {
        self.local.get_location(id)
    }

    fn get_type_info(&self, id: LocalId) -> Option<&TypeInfo> {
        self.local.get_type_info(id)
    }
}

struct Generator<T> {
    count: usize,
    f: fn(usize) -> T,
}

impl<T> Generator<T> {
    pub fn new(f: fn(usize) -> T) -> Self {
        Self { count: 0, f }
    }

    pub fn gen(&mut self) -> T {
        let item = (self.f)(self.count);
        self.count += 1;
        item
    }
}
