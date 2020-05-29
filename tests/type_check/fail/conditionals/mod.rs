use crate::{test_type, util::DummyLoc};

use pijama::LangError;

use pijama_ast::ty::{Ty, TyError};

test_type!(
    wrong_type_cond_input,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Bool,
        found: Ty::Int.loc()
    }))
);
test_type!(
    mixed_types_cond_result,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Bool,
        found: Ty::Int.loc()
    }))
);
