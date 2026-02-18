use chumsky::Parser;
use nexus::parser;
use nexus::typecheck::TypeChecker;

fn check_code(src: &str) -> Result<(), String> {
    let parser = parser::parser();
    let program = parser.parse(src).map_err(|e| format!("{:?}", e))?;

    let mut checker = TypeChecker::new();
    checker.check_program(&program).map_err(|e| e.message)
}

#[test]
fn test_ffi_declaration() {
    let src = r#"
    import external "math.wasm"
    pub external add_i64 : (a: i64, b: i64) -> i64 = "add"
    
    fn main() -> unit effect { IO } do
      let x = add_i64(a: 1, b: 2)
      // perform print(val: i64_to_string(val: x))
      return ()
    endfn
    "#;
    match check_code(src) {
        Ok(_) => (),
        Err(e) => panic!("Type check failed: {}", e),
    }
}

#[test]
fn test_ffi_effectful() {
    let src = r#"
    import external "time.wasm"
    pub external get_time : () -> float effect { IO } = "get_time"
    
    fn main() -> unit effect { IO } do
      let t = perform get_time()
      // perform print(val: float_to_string(val: t))
      return ()
    endfn
    "#;
    match check_code(src) {
        Ok(_) => (),
        Err(e) => panic!("Type check failed: {}", e),
    }
}

#[test]
fn test_ffi_mismatch() {
    let src = r#"
    pub external foo : (a: i64) -> i64 = "foo"
    fn main() -> unit do
      let x = foo(a: true)
    endfn
    "#;
    assert!(check_code(src).is_err());
}
