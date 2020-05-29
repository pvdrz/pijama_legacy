use crate::{test_type, util::DummyLoc};

use pijama::LangError;

use pijama_ast::ty::TyError;

test_type!(
    unbounded,
    Err(LangError::Ty(TyError::Unbound("x".to_owned().loc())))
);
