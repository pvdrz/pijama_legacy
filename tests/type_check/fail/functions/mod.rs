use crate::test_type;

use pijama::{
    ast::{Located, Location},
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    wrong_type_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located {
            content: Ty::Bool,
            loc: Location { start: 0, end: 0 }
        }
    }))
);

test_type!(
    wrong_return_type_fn_int_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool)),
        found: Located {
            content: Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)),
            loc: Location { start: 0, end: 0 }
        }
    }))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located {
            content: Ty::Bool,
            loc: Location { start: 0, end: 0 }
        }
    }))
);

test_type!(
    wrong_return_type_anon_fn_int_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool)),
        found: Located {
            content: Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)),
            loc: Location { start: 0, end: 0 }
        }
    }))
);
