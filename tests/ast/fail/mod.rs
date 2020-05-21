use crate::{test_type, util::DummyLoc};

use pijama::{ty::TyError, LangError};

test_type!(
    detect_indirect_recursion,
    Err(LangError::Ty(TyError::Missing(().loc())))
);

test_type!(
    detect_recursion_after_shadowing,
    Err(LangError::Ty(TyError::Missing(().loc())))
);

test_type!(
    detect_recursion_inside_functions,
    Err(LangError::Ty(TyError::Missing(().loc())))
);
