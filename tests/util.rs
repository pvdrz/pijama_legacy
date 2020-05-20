use pijama::ast::{Located, Location};

use std::fmt::Debug;

pub trait DummyLoc: Debug + Sized {
    fn loc(self) -> Located<Self> {
        Located::new(self, Location::new(0, 0))
    }
}

impl<T: Debug + Sized> DummyLoc for T {}
