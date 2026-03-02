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
fn test_to_string_runtime_error() {
    let src = r#"
import { to_string } from nxlib/stdlib/exn.nx

let main = fn () -> string do
  let e: Exn = RuntimeError(val: [=[boom]=])
  return to_string(exn: e)
end
"#;
    assert_eq!(
        run(src).unwrap(),
        Value::String("RuntimeError: boom".to_string())
    );
}

#[test]
fn test_to_string_invalid_index() {
    let src = r#"
import { to_string } from nxlib/stdlib/exn.nx

let main = fn () -> string do
  let e: Exn = InvalidIndex(val: 42)
  return to_string(exn: e)
end
"#;
    assert_eq!(
        run(src).unwrap(),
        Value::String("InvalidIndex: 42".to_string())
    );
}

#[test]
fn test_backtrace_returns_frames() {
    let src = r#"
import { backtrace } from nxlib/stdlib/exn.nx

let inner = fn () -> unit effect { Exn } do
  raise RuntimeError(val: [=[oops]=])
  return ()
end

let outer = fn () -> [string] do
  try
    inner()
  catch e ->
    return backtrace(exn: e)
  end
  return []
end

let main = fn () -> [string] do
  return outer()
end
"#;
    let result = run(src).unwrap();
    // The backtrace should be a non-empty Cons list containing "inner"
    let mut frames = Vec::new();
    let mut current = result;
    loop {
        match &current {
            Value::Variant(name, args) if name == "Cons" && args.len() == 2 => {
                if let Value::String(s) = &args[0] {
                    frames.push(s.clone());
                }
                current = args[1].clone();
            }
            Value::Variant(name, _) if name == "Nil" => break,
            _ => break,
        }
    }
    assert!(
        frames.contains(&"inner".to_string()),
        "backtrace should contain 'inner', got: {:?}",
        frames
    );
}
