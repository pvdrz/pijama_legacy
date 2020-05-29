use crate::test_type;
use pijama::ty::Ty;

// Non-recursive functions
test_type!(
    fn_from_int_to_int,
    Ok(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
test_type!(
    fn_from_int_to_int_with_type,
    Ok(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
test_type!(
    anon_fn_from_int_to_int,
    Ok(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)))
);
