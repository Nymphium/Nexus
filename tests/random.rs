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
fn random_range_returns_in_bounds_value() {
    let src = r#"
import { Random }, * as rng from nxlib/stdlib/random.nx

let main = fn () -> bool require { PermRandom } do
  inject rng.system_handler do
    let n = Random.range(min: 10, max: 20)
    if n >= 10 then
      return n < 20
    else
      return false
    end
  end
end
"#;
    assert_eq!(run(src).unwrap(), Value::Bool(true));
}

#[test]
fn random_range_requires_perform() {
    let src = r#"
import { Random }, * as rng from nxlib/stdlib/random.nx

let main = fn () -> i64 do
  inject rng.system_handler do
    let n = Random.range(min: 0, max: 10)
    return n
  end
end
"#;
    assert!(
        check(src).is_err(),
        "random.range without PermRandom should fail typechecking"
    );
}
