use crate::{test_type, util::DummyLoc};

use pijama::{
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    wrong_type_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);

test_type!(
    wrong_return_type_fn_int_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Bool,
        found: Ty::Int.loc()
    }))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);
