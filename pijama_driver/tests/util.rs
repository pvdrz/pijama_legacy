use pijama_common::location::{Located, Location};

use std::fmt::Debug;

pub const fn dummy_loc() -> Location {
    Location::new(0, 0)
}

pub trait DummyLoc: Debug + Sized {
    fn loc(self) -> Located<Self> {
        Located::new(self, dummy_loc())
    }
}

impl<T: Debug + Sized> DummyLoc for T {}
