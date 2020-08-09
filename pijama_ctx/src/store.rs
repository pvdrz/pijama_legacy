use std::{collections::HashMap, fmt::Debug, hash::Hash};

use pijama_common::location::Location;

use crate::{generator::Generator, ContextExt, TypeInfo};

pub(super) struct Store<Id> {
    pub(super) gen_id: Generator<Id>,
    pub(super) locations: HashMap<Id, Location>,
    pub(super) type_info: HashMap<Id, TypeInfo>,
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
