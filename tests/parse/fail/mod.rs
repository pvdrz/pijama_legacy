use std::include_str;

use pijama::{run_with_machine, LangError, eval::CheckedMachine};

#[test]
fn consecutive_comments() {
    let input = include_str!("consecutive_comments.pj");
    let err = run_with_machine(input, CheckedMachine::default()).unwrap_err();
    assert!(matches!(err, LangError::Parse(_)))
}
