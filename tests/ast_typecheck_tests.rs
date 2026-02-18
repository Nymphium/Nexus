use nexus::ast::*;
use nexus::typecheck::TypeChecker;

fn spanned<T>(node: T) -> Spanned<T> {
    Spanned { node, span: 0..0 }
}

fn expr_lit_int(i: i64) -> Spanned<Expr> {
    spanned(Expr::Literal(Literal::Int(i)))
}
fn expr_lit_bool(b: bool) -> Spanned<Expr> {
    spanned(Expr::Literal(Literal::Bool(b)))
}
fn expr_var(n: &str) -> Spanned<Expr> {
    spanned(Expr::Variable(n.to_string(), Sigil::Immutable))
}
fn expr_call(func: &str, args: Vec<(&str, Spanned<Expr>)>) -> Spanned<Expr> {
    spanned(Expr::Call {
        func: func.to_string(),
        args: args.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        perform: false,
    })
}

#[test]
fn test_basic_poly() {
    let func_id = Function {
        name: "id".to_string(),
        is_public: false,
        type_params: vec!["T".to_string()],
        params: vec![Param {
            name: "x".to_string(),
            sigil: Sigil::Immutable,
            typ: Type::UserDefined("T".to_string(), vec![]),
        }],
        ret_type: Type::UserDefined("T".to_string(), vec![]),
        effects: Type::Row(vec![], None),
        body: vec![spanned(Stmt::Return(expr_var("x")))],
    };

    let program = Program {
        definitions: vec![spanned(TopLevel::Function(func_id))],
    };
    let mut checker = TypeChecker::new();
    assert!(checker.check_program(&program).is_ok());
}

#[test]
fn test_complex_typecheck() {
    let func_id = Function {
        name: "id".to_string(),
        is_public: false,
        type_params: vec!["T".to_string()],
        params: vec![Param {
            name: "x".to_string(),
            sigil: Sigil::Immutable,
            typ: Type::UserDefined("T".to_string(), vec![]),
        }],
        ret_type: Type::UserDefined("T".to_string(), vec![]),
        effects: Type::Row(vec![], None),
        body: vec![spanned(Stmt::Return(expr_var("x")))],
    };

    let func_main = Function {
        name: "main".to_string(),
        is_public: false,
        type_params: vec![],
        params: vec![],
        ret_type: Type::Unit,
        effects: Type::Row(vec![], None),
        body: vec![
            spanned(Stmt::Let {
                name: "f".to_string(),
                sigil: Sigil::Immutable,
                typ: None,
                value: expr_var("id"),
            }),
            spanned(Stmt::Let {
                name: "res1".to_string(),
                sigil: Sigil::Immutable,
                typ: None,
                value: expr_call("f", vec![("x", expr_lit_int(10))]),
            }),
            spanned(Stmt::Let {
                name: "res2".to_string(),
                sigil: Sigil::Immutable,
                typ: None,
                value: expr_call("f", vec![("x", expr_lit_bool(true))]),
            }),
            spanned(Stmt::Return(spanned(Expr::Literal(Literal::Unit)))),
        ],
    };

    let program = Program {
        definitions: vec![
            spanned(TopLevel::Function(func_id)),
            spanned(TopLevel::Function(func_main)),
        ],
    };
    let mut checker = TypeChecker::new();
    assert!(checker.check_program(&program).is_ok());
}

#[test]
fn test_mismatch_fail() {
    let func_main = Function {
        name: "main".to_string(),
        is_public: false,
        type_params: vec![],
        params: vec![],
        ret_type: Type::I64,
        effects: Type::Row(vec![], None),
        body: vec![spanned(Stmt::Return(expr_lit_bool(true)))],
    };
    let program = Program {
        definitions: vec![spanned(TopLevel::Function(func_main))],
    };
    let mut checker = TypeChecker::new();
    let res = checker.check_program(&program);
    assert!(res.is_err());
}
