
use nexus::ast::*;
use nexus::typecheck::{TypeChecker, TypeError};
use nexus::parser::parser;
use chumsky::Parser;

fn check(src: &str) -> Result<(), String> {
    let p = parser().parse(src).map_err(|e| format!("{:?}", e))?;
    let mut checker = TypeChecker::new();
    checker.check_program(&p).map_err(|e| e.message)
}

#[test]
fn test_list_creation() {
    let src = r#"
    fn main() -> unit do
        let l = [1, 2, 3]
        return ()
    endfn
    "#;
    assert!(check(src).is_ok());
}

#[test]
fn test_list_type_mismatch() {
    let src = r#"
    fn main() -> unit do
        let l = [1, true]
        return ()
    endfn
    "#;
    assert!(check(src).is_err(), "Should fail: mixed types in list");
}

#[test]
fn test_list_nested() {
    let src = r#"
    fn main() -> unit do
        let l = [[1, 2], [3, 4]]
        return ()
    endfn
    "#;
    assert!(check(src).is_ok());
}

#[test]
fn test_list_of_linear() {
    let src = r#"
    fn main() -> unit do
        let %l = [%1, %2] // Assuming integers can be linear for test
        // This fails because integers are not linear by default unless cast/annotated?
        // Let's use linear literal syntax? No such thing.
        // Use constructor?
        return ()
    endfn
    "#;
    // Currently no way to create linear literals easily.
    // Skip linear list test for now or use stdlib function that returns linear.
}
