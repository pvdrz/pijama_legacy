use crate::test_type;

use pijama_driver::LangErrorKind;
use pijama_hir::LowerErrorKind;

test_type!(
    detect_indirect_recursion,
    Err(&LangErrorKind::Lower(LowerErrorKind::RequiredTy))
);

test_type!(
    detect_recursion_after_shadowing,
    Err(&LangErrorKind::Lower(LowerErrorKind::RequiredTy))
);

test_type!(
    detect_recursion_after_shadowing_2,
    Err(&LangErrorKind::Lower(LowerErrorKind::RequiredTy))
);

test_type!(
    detect_recursion_inside_functions,
    Err(&LangErrorKind::Lower(LowerErrorKind::RequiredTy))
);
