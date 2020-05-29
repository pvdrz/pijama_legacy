use crate::{test_type, util::DummyLoc};

use pijama::LangError;

use pijama_ast::ty::{Ty, TyError};

test_type!(
    bind_bool_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);
