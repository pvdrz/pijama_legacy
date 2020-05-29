use std::include_str;

use pijama::LangError;

use crate::run;

#[test]
fn consecutive_comments() {
    let input = include_str!("consecutive_comments.pj");
    let err = run(input).unwrap_err();
    assert!(matches!(err, LangError::Parse(_)))
}
