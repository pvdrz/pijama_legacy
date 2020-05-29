use crate::{test_type, util::DummyLoc};

use pijama::LangError;

use pijama_ast::ty::{Ty, TyError};

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
        expected: Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool)),
        found: Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)).loc()
    }))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);
