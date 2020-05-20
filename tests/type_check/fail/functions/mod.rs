use crate::test_type;

use pijama::{
    ast::Located,
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    wrong_type_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located { content: Ty::Bool, ..}
    }))
);

test_type!(
    wrong_return_type_fn_int_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Arrow(box Ty::Int, box Ty::Bool),
        found: Located { content: Ty::Arrow(box Ty::Int, box Ty::Int), ..}
    }))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located { content: Ty::Bool, ..}
    }))
);

test_type!(
    wrong_return_type_anon_fn_int_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Arrow(box Ty::Int, box Ty::Bool),
        found: Located { content: Ty::Arrow(box Ty::Int, box Ty::Int), ..}
    }))
);
