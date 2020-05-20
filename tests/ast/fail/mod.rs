use crate::test_type;

use pijama::{ty::TyError, LangError};

test_type!(
    detect_indirect_recursion,
    Err(LangError::Ty(TyError::Missing(_)))
);

test_type!(
    detect_recursion_after_shadowing,
    Err(LangError::Ty(TyError::Missing(_)))
);

test_type!(
    detect_recursion_inside_functions,
    Err(LangError::Ty(TyError::Missing(_)))
);
