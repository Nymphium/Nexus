use nexus::interpreter::{Interpreter, Value};
use nexus::lang::parser::parser;
use nexus::lang::typecheck::TypeChecker;

fn prepare_test_source(src: &str) -> String {
    let s = src.replace("let main = fn ()", "pub let __test = fn ()");
    format!("{}\nlet main = fn () -> unit do\n  return ()\nend\n", s)
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
fn result_from_exn_builds_err() {
    let src = r#"
import as result from nxlib/stdlib/result.nx

let main = fn () -> bool do
  let exn = RuntimeError(val: [=[boom]=])
  let r = result.from_exn(exn: exn)
  return result.is_err(res: r)
end
"#;
    assert_eq!(run(src).unwrap(), Value::Bool(true));
}

#[test]
fn result_to_exn_raises_and_is_catchable() {
    let src = r#"
import as result from nxlib/stdlib/result.nx

let main = fn () -> bool do
  let err: Result<i64, Exn> = Err(err: RuntimeError(val: [=[boom]=]))
  try
    let _ = result.to_exn(res: err)
    return false
  catch e ->
    return true
  end
end
"#;
    assert_eq!(run(src).unwrap(), Value::Bool(true));
}
