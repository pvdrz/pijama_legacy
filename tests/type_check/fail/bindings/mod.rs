use crate::test_type;

use pijama::{
    ast::Located,
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    bind_bool_to_int,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Located { content: Ty::Bool, ..}
    }))
);
