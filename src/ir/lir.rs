//! Low-level IR — ANF form, WASM-ready.
//! Adapted from the original `src/compiler/anf.rs` with additions for evidence-passing.

use crate::lang::ast::{BinaryOp, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct LirProgram {
    pub functions: Vec<LirFunction>,
    pub externals: Vec<LirExternal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LirExternal {
    pub name: String,
    pub wasm_module: String,
    pub wasm_name: String,
    pub params: Vec<LirParam>,
    pub ret_type: Type,
    pub effects: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LirFunction {
    pub name: String,
    pub params: Vec<LirParam>,
    /// Evidence params (i32 base indices into funcref table)
    pub evidence_params: Vec<LirParam>,
    pub ret_type: Type,
    pub requires: Type,
    pub effects: Type,
    pub body: Vec<LirStmt>,
    pub ret: LirAtom,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LirParam {
    pub label: String,
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LirStmt {
    Let {
        name: String,
        typ: Type,
        expr: LirExpr,
    },
    If {
        cond: LirAtom,
        then_body: Vec<LirStmt>,
        else_body: Vec<LirStmt>,
    },
    IfReturn {
        cond: LirAtom,
        then_body: Vec<LirStmt>,
        then_ret: LirAtom,
        else_body: Vec<LirStmt>,
        else_ret: Option<LirAtom>,
        ret_type: Type,
    },
    TryCatch {
        body: Vec<LirStmt>,
        body_ret: Option<LirAtom>,
        catch_param: String,
        catch_param_typ: Type,
        catch_body: Vec<LirStmt>,
        catch_ret: Option<LirAtom>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LirExpr {
    Atom(LirAtom),
    Binary {
        op: BinaryOp,
        lhs: LirAtom,
        rhs: LirAtom,
        typ: Type,
    },
    Call {
        func: String,
        args: Vec<(String, LirAtom)>,
        typ: Type,
    },
    Constructor {
        name: String,
        args: Vec<LirAtom>,
        typ: Type,
    },
    Record {
        fields: Vec<(String, LirAtom)>,
        typ: Type,
    },
    ObjectTag {
        value: LirAtom,
        typ: Type,
    },
    ObjectField {
        value: LirAtom,
        index: usize,
        typ: Type,
    },
    Raise {
        value: LirAtom,
        typ: Type,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LirAtom {
    Var { name: String, typ: Type },
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Unit,
}

impl LirAtom {
    pub fn typ(&self) -> Type {
        match self {
            LirAtom::Var { typ, .. } => typ.clone(),
            LirAtom::Int(_) => Type::I64,
            LirAtom::Float(_) => Type::F64,
            LirAtom::Bool(_) => Type::Bool,
            LirAtom::String(_) => Type::String,
            LirAtom::Unit => Type::Unit,
        }
    }
}
