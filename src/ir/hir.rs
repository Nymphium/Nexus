//! High-level IR — name-resolved AST with modules flattened.
//! All identifiers are fully qualified. Handlers collected.
#![allow(dead_code)]

use crate::lang::ast::{BinaryOp, Literal, Sigil, Type};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HirProgram {
    pub types: Vec<HirTypeDef>,
    pub enums: Vec<HirEnumDef>,
    pub exceptions: Vec<HirExceptionDef>,
    pub ports: Vec<HirPort>,
    pub functions: Vec<HirFunction>,
    pub externals: Vec<HirExternal>,
    /// binding_name → handler binding (port name + method implementations)
    pub handler_bindings: HashMap<String, HirHandlerBinding>,
    pub global_lets: Vec<HirGlobalLet>,
}

#[derive(Debug, Clone)]
pub struct HirHandlerBinding {
    pub port_name: String,
    pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone)]
pub struct HirTypeDef {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone)]
pub struct HirEnumDef {
    pub name: String,
    pub is_opaque: bool,
    pub type_params: Vec<String>,
    pub variants: Vec<HirVariantDef>,
}

#[derive(Debug, Clone)]
pub struct HirVariantDef {
    pub name: String,
    pub fields: Vec<(Option<String>, Type)>,
}

#[derive(Debug, Clone)]
pub struct HirExceptionDef {
    pub name: String,
    pub fields: Vec<(Option<String>, Type)>,
}

#[derive(Debug, Clone)]
pub struct HirPort {
    pub name: String,
    pub functions: Vec<HirPortMethod>,
}

#[derive(Debug, Clone)]
pub struct HirPortMethod {
    pub name: String,
    pub params: Vec<HirParam>,
    pub ret_type: Type,
    pub requires: Type,
    pub effects: Type,
}

#[derive(Debug, Clone)]
pub struct HirExternal {
    pub name: String,
    pub wasm_module: String,
    pub wasm_name: String,
    pub params: Vec<HirParam>,
    pub ret_type: Type,
    pub effects: Type,
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: String, // fully qualified: "module::fn_name"
    pub params: Vec<HirParam>,
    pub ret_type: Type,
    pub requires: Type, // row of required ports
    pub effects: Type,  // row of effects
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub struct HirParam {
    pub name: String,
    pub label: String,
    pub sigil: Sigil,
    pub typ: Type,
}

#[derive(Debug, Clone)]
pub struct HirGlobalLet {
    pub name: String,
    pub typ: Option<Type>,
    pub value: HirExpr,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Let {
        name: String,
        sigil: Sigil,
        typ: Option<Type>,
        value: HirExpr,
    },
    Expr(HirExpr),
    Return(HirExpr),
    Assign {
        target: HirExpr,
        value: HirExpr,
    },
    Conc(Vec<HirFunction>),
    Try {
        body: Vec<HirStmt>,
        catch_param: String,
        catch_body: Vec<HirStmt>,
    },
    Inject {
        handlers: Vec<String>,
        body: Vec<HirStmt>,
    },
}

#[derive(Debug, Clone)]
pub enum HirExpr {
    Literal(Literal),
    Variable(String, Sigil),
    BinaryOp(Box<HirExpr>, BinaryOp, Box<HirExpr>),
    Borrow(String, Sigil),
    Call {
        func: String,
        args: Vec<(String, HirExpr)>,
    },
    PortCall {
        port: String,
        method: String,
        args: Vec<(String, HirExpr)>,
    },
    Constructor {
        enum_name: String,
        variant: String,
        args: Vec<HirExpr>,
    },
    Record(Vec<(String, HirExpr)>),
    Array(Vec<HirExpr>),
    Index(Box<HirExpr>, Box<HirExpr>),
    FieldAccess(Box<HirExpr>, String),
    If {
        cond: Box<HirExpr>,
        then_branch: Vec<HirStmt>,
        else_branch: Option<Vec<HirStmt>>,
    },
    Match {
        target: Box<HirExpr>,
        cases: Vec<HirMatchCase>,
    },
    Lambda {
        type_params: Vec<String>,
        params: Vec<HirParam>,
        ret_type: Type,
        requires: Type,
        effects: Type,
        body: Vec<HirStmt>,
    },
    Raise(Box<HirExpr>),
    External(String, Vec<String>, Type),
    Handler {
        coeffect_name: String,
        requires: Type,
        functions: Vec<HirFunction>,
    },
}

#[derive(Debug, Clone)]
pub struct HirMatchCase {
    pub pattern: HirPattern,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub enum HirPattern {
    Literal(Literal),
    Variable(String, Sigil),
    Constructor {
        enum_name: String,
        variant: String,
        fields: Vec<(Option<String>, HirPattern)>,
    },
    Record(Vec<(String, HirPattern)>, bool),
    Wildcard,
}
