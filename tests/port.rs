use chumsky::Parser;
use nexus::interpreter::{Interpreter, Value};
use nexus::lang::parser::parser;
use nexus::lang::typecheck::TypeChecker;

fn run(src: &str) -> Result<Value, String> {
    let p = parser().parse(src).map_err(|e| format!("{:?}", e))?;
    let mut checker = TypeChecker::new();
    checker.check_program(&p).map_err(|e| e.message)?;
    let mut interpreter = Interpreter::new(p);
    interpreter.run_function("main", vec![])
}

#[test]
fn test_port_basic() {
    let src = r#"
    import { print } from nxlib/stdlib/stdio.nx
    port Logger do
      fn log(msg: string) -> unit effect { Console }
    endport

    let main = fn () -> unit effect { Console } do
      let stdout_logger = handler Logger do
        fn log(msg: string) -> unit effect { Console } do
          print(val: msg)
          return ()
        endfn
      endhandler

      inject stdout_logger do
        Logger.log(msg: [=[test message]=])
      endinject
      return ()
    endfn
    "#;
    let res = run(src);
    assert!(res.is_ok(), "Execution failed: {:?}", res.err());
}

#[test]
fn test_port_redefinition_wins() {
    let src = r#"
    import { print } from nxlib/stdlib/stdio.nx
    import { i64_to_string } from nxlib/stdlib/string.nx
    port Adder do
      fn add_one(n: i64) -> i64
    endport

    let main = fn () -> unit effect { Console } do
      let normal_adder = handler Adder do
        fn add_one(n: i64) -> i64 do
          return n + 1
        endfn
      endhandler

      let weird_adder = handler Adder do
        fn add_one(n: i64) -> i64 do
          return n + 2
        endfn
      endhandler

      inject weird_adder do
        let result = Adder.add_one(n: 10)
        let msg = i64_to_string(val: result)
        print(val: msg)
      endinject
      return ()
    endfn
    "#;
    let res = run(src);
    assert!(res.is_ok(), "Execution failed: {:?}", res.err());
}
