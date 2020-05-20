use crate::test_type;
use pijama::{
    ast::{Located, Location},
    ty::TyError,
    LangError,
};

test_type!(
    unbounded,
    Err(LangError::Ty(TyError::Unbound(Located {
        content: "x".to_owned(),
        loc: Location { start: 0, end: 0 }
    })))
);
