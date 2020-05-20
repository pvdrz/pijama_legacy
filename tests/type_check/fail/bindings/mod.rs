use crate::test_type;

use pijama::{
    ast::{Located, Location},
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    bind_bool_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located {
            content: Ty::Bool,
            loc: Location { start: 0, end: 0 }
        }
    }))
);
