use pijama_common::location::{Located, Location};

use std::fmt::Debug;

use pijama_tycheck::{TyError, TyErrorKind};

pub const fn dummy_loc() -> Location {
    Location::new(0, 0)
}

pub trait DummyLoc<Output = Located<Self>>: Debug + Sized {
    fn loc(self) -> Output;
}

impl DummyLoc<TyError> for TyErrorKind {
    fn loc(self) -> TyError {
        TyError::new(self, dummy_loc())
    }
}

impl<T: Debug + Sized> DummyLoc<Located<T>> for T {
    fn loc(self) -> Located<T> {
        Located::new(self, dummy_loc())
    }
}
