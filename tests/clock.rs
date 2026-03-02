use nexus::interpreter::{Interpreter, Value};
use nexus::lang::parser::parser;
use nexus::lang::typecheck::TypeChecker;

fn prepare_test_source(src: &str) -> String {
    let s = src.replace("let main = fn ()", "pub let __test = fn ()");
    format!("{}\nlet main = fn () -> unit do\n  return ()\nend\n", s)
}

fn check(src: &str) -> Result<(), String> {
    let src = prepare_test_source(src);
    let p = parser().parse(src.as_str()).map_err(|e| format!("{:?}", e))?;
    let mut checker = TypeChecker::new();
    checker.check_program(&p).map_err(|e| e.message)
}

fn run(src: &str) -> Result<Value, String> {
    let src = prepare_test_source(src);
    let p = parser().parse(src.as_str()).map_err(|e| format!("{:?}", e))?;
    let mut checker = TypeChecker::new();
    checker.check_program(&p).map_err(|e| e.message)?;
    let mut interpreter = Interpreter::new(p);
    interpreter.run_function("__test", vec![])
}

#[test]
fn clock_now_returns_positive_value() {
    let src = r#"
import { Clock }, * as clk from nxlib/stdlib/clock.nx

let main = fn () -> bool require { PermClock } do
  inject clk.system_handler do
    let t = Clock.now()
    return t > 0
  end
end
"#;
    assert_eq!(run(src).unwrap(), Value::Bool(true));
}

#[test]
fn clock_sleep_does_not_crash() {
    let src = r#"
import { Clock }, * as clk from nxlib/stdlib/clock.nx

let main = fn () -> bool require { PermClock } do
  inject clk.system_handler do
    Clock.sleep(ms: 10)
    return true
  end
end
"#;
    assert_eq!(run(src).unwrap(), Value::Bool(true));
}

#[test]
fn clock_requires_perm_clock() {
    let src = r#"
import { Clock }, * as clk from nxlib/stdlib/clock.nx

let main = fn () -> i64 do
  inject clk.system_handler do
    return Clock.now()
  end
end
"#;
    assert!(
        check(src).is_err(),
        "Clock.now without PermClock should fail typechecking"
    );
}
