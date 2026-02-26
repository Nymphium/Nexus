use chumsky::Parser;
use nexus::lang::parser::parser;
use nexus::lang::typecheck::TypeChecker;

fn check(src: &str) -> Result<(), String> {
    let p = parser().parse(src).map_err(|e| format!("{:?}", e))?;
    let mut checker = TypeChecker::new();
    checker.check_program(&p).map_err(|e| e.message)
}

#[test]
fn test_effect_propagation() {
    // f has Console effect. g calls f, so g must have Console effect.
    let src = r#"
    type Console = {} // Dummy type for effect

    let f = fn () -> unit effect { Console } do
        return ()
    endfn

    let g = fn () -> unit effect { Console } do
        f()
    endfn

    let main = fn () -> unit effect { Console } do
        g()
    endfn
    "#;
    assert!(check(src).is_ok());
}

#[test]
fn test_call_pure_from_impure() {
    let src = r#"
    type Console = {}
    let pure_fn = fn () -> unit do return () endfn
    let impure_fn = fn () -> unit effect { Console } do
        pure_fn() // Should be allowed without return ()
    endfn
    let main = fn () -> unit effect { Console } do
        impure_fn()
        return ()
    endfn
    "#;
    assert!(check(src).is_ok());
}

#[test]
fn test_try_catch_removes_exn() {
    let src = r#"
    import { print } from nxlib/stdlib/stdio.nx
    import { i64_to_string } from nxlib/stdlib/string.nx
    exception Oops(string)

    let risky = fn () -> unit effect { Exn } do
        raise Oops([=[oops]=])
        return ()
    endfn

    let main = fn () -> unit effect { Console } do
        try
            risky()
        catch e ->
            match e do
                case Oops(msg) -> print(val: msg)
                case RuntimeError(msg) -> print(val: msg)
                case InvalidIndex(i) ->
                    let m = i64_to_string(val: i)
                    print(val: m)
            endmatch
        endtry
        return ()
    endfn
    "#;
    if let Err(e) = check(src) {
        panic!("Typecheck failed: {}", e);
    }
}

#[test]
fn test_raise_requires_exn() {
    let src = r#"
    let fail = fn () -> unit do
        raise [=[oops]=] // Should fail: no Exn effect allowed
        return ()
    endfn
    "#;
    assert!(check(src).is_err());
}

#[test]
fn test_main_cannot_declare_exn_effect() {
    let src = r#"
    let main = fn () -> unit effect { Console, Exn } do
        return ()
    endfn
    "#;
    assert!(check(src).is_err());
}

#[test]
fn test_main_can_return_non_unit() {
    let src = r#"
    let main = fn () -> i64 do
        return 0
    endfn
    "#;
    assert!(check(src).is_ok());
}

#[test]
fn test_main_effect_net_only_is_rejected() {
    let src = r#"
    let main = fn () -> unit effect { Net } do
        return ()
    endfn
    "#;
    let err = check(src).expect_err("main with { Net } must fail");
    assert!(err.contains("main function effects must be {}, or { Console }"));
}

#[test]
fn test_main_require_net_is_rejected() {
    let src = r#"
    let main = fn () -> unit require { Net } effect { Console } do
        return ()
    endfn
    "#;
    assert!(check(src).is_err(), "main with require {{ Net }} should be rejected");
}

#[test]
fn test_effect_mismatch() {
    // g is declared pure but calls f (Console). Should fail.
    let src = r#"
    type Console = {}

    let f = fn () -> unit effect { Console } do
        return ()
    endfn

    let g = fn () -> unit effect {} do // Pure
        f()
    endfn

    let main = fn () -> unit do
        return ()
    endfn
    "#;
    assert!(check(src).is_err(), "Should fail because g calls impure f");
}

#[test]
fn test_effect_polymorphism() {
    // apply is polymorphic in effect E.
    // Calling it with pure function -> result is pure.
    // Calling it with impure function -> result is impure.
    let src = r#"
    type Console = {}

    let apply = fn <E>(f: () -> unit effect E) -> unit effect E do
        f()
    endfn

    let pure_fn = fn () -> unit effect {} do
        return ()
    endfn

    let impure_fn = fn () -> unit effect { Console } do
        return ()
    endfn

    let test_pure = fn () -> unit effect {} do
        apply(f: pure_fn)
    endfn

    let test_impure = fn () -> unit effect { Console } do
        apply(f: impure_fn)
    endfn

    let main = fn () -> unit do
        return ()
    endfn
    "#;
    assert!(check(src).is_ok());
}

#[test]
fn test_effect_polymorphism_mismatch() {
    // Calling apply with impure function in pure context.
    let src = r#"
    type Console = {}

    let apply = fn <E>(f: () -> unit effect E) -> unit effect E do
        f()
    endfn

    let impure_fn = fn () -> unit effect { Console } do
        return ()
    endfn

    let test_fail = fn () -> unit effect {} do // Declared Pure
        apply(f: impure_fn)     // Call is Impure (Console)
    endfn

    let main = fn () -> unit do
        return ()
    endfn
    "#;
    assert!(
        check(src).is_err(),
        "Should fail because apply instantiates E=Console, so call becomes Console"
    );
}
