use std::{fmt::Debug, hash::Hash, collections::HashMap};

use pijama_common::{location::Location, generator::Generator};

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

pub struct Store<Id> {
    gen_id: Generator<Id>,
    locations: HashMap<Id, Location>,
    type_info: HashMap<Id, TypeInfo>,
}

impl<Id: Debug + Hash + Eq + Copy> Store<Id> {
    pub fn new_id(&mut self) -> Id {
        self.gen_id.gen()
    }

    pub fn insert_location(&mut self, id: Id, loc: Location) {
        assert!(
            self.locations.insert(id, loc).is_none(),
            "Overwrote location for {:?}",
            id
        );
    }
    pub fn insert_type_info(&mut self, id: Id, type_info: TypeInfo) {
        assert!(
            self.type_info.insert(id, type_info).is_none(),
            "Overwrote type information for {:?}",
            id
        );
    }

    pub fn get_location(&self, id: Id) -> Option<Location> {
        self.locations.get(&id).cloned()
    }

    pub fn get_type_info(&self, id: Id) -> Option<&TypeInfo> {
        self.type_info.get(&id)
    }

    pub fn types_mut<'ty>(&'ty mut self) -> impl Iterator<Item = &'ty mut Ty> {
        self.type_info.values_mut().map(|info| &mut info.ty)
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

    pub fn local(&mut self) -> &mut Store<LocalId> {
        &mut self.local
    }

    pub fn term(&mut self) -> &mut Store<TermId> {
        &mut self.term
    }
}

