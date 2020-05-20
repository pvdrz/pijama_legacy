use pijama::{
    ast::{Block, Located, Location, Name, Node},
    ty::{Binding, Ty},
};

pub trait DummyLoc: std::fmt::Debug + Sized {
    fn loc(self) -> Located<Self> {
        Located::new(self, Location::new(0, 0))
    }
}

impl<'a> DummyLoc for Node<'a> {}
impl DummyLoc for Ty {}
impl<'a> DummyLoc for Binding<'a> {}
impl<'a> DummyLoc for Block<'a> {}
impl<'a> DummyLoc for Name<'a> {}
