use crate::test_type;

use pijama::{
    ast::{Located, Location},
    ty::{Ty, TyError},
    LangError,
};

test_type!(
    wrong_type_cond_input,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Bool,
        found: Located {
            content: Ty::Int,
            loc: Location { start: 0, end: 0 }
        }
    }))
);
test_type!(
    mixed_types_cond_result,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Bool,
        found: Located {
            content: Ty::Int,
            loc: Location { start: 0, end: 0 }
        }
    }))
);
