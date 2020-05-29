use crate::{test_type, util::DummyLoc};

use pijama::{
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    bind_bool_to_int,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);
