use crate::test_type;
use pijama::{
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    wrong_type_fn_call_arg,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);

test_type!(
    wrong_return_type_fn_int_to_int,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Arrow(box Ty::Int, box Ty::Bool),
        found: Ty::Arrow(box Ty::Int, box Ty::Int),
    }))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);

test_type!(
    wrong_return_type_anon_fn_int_to_int,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Arrow(box Ty::Int, box Ty::Bool),
        found: Ty::Arrow(box Ty::Int, box Ty::Int),
    }))
);
