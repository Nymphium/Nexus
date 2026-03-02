//! Mid-level IR — effects eliminated via evidence-passing.
//! No inject/handler/port.method() — replaced with evidence params and indirect calls.
#![allow(dead_code)]

use crate::lang::ast::{BinaryOp, Literal, Sigil, Type};

#[derive(Debug, Clone)]
pub struct MirProgram {
    pub functions: Vec<MirFunction>,
    pub externals: Vec<MirExternal>,
    pub evidence_table: EvidenceTable,
}

#[derive(Debug, Clone)]
pub struct EvidenceTable {
    pub entries: Vec<EvidenceEntry>,
}

#[derive(Debug, Clone)]
pub struct EvidenceEntry {
    pub port_name: String,
    pub method_name: String,
    pub func_name: String,   // implementation function name
    pub handler_name: String, // handler binding this came from
}

impl EvidenceTable {
    pub fn new() -> Self {
        EvidenceTable {
            entries: Vec::new(),
        }
    }

    /// Get the base index for a port's methods in the funcref table.
    pub fn base_index_for_port(&self, port_name: &str) -> Option<usize> {
        self.entries
            .iter()
            .position(|e| e.port_name == port_name)
    }

    /// Get the offset of a specific method within a port's entries.
    pub fn method_offset(&self, port_name: &str, method_name: &str) -> Option<usize> {
        let base = self.base_index_for_port(port_name)?;
        self.entries[base..]
            .iter()
            .take_while(|e| e.port_name == port_name)
            .position(|e| e.method_name == method_name)
    }
}

#[derive(Debug, Clone)]
pub struct MirExternal {
    pub name: String,
    pub wasm_module: String,
    pub wasm_name: String,
    pub params: Vec<MirParam>,
    pub ret_type: Type,
    pub effects: Type,
}

#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<MirParam>,
    /// Evidence params: one i32 per required port (base index into funcref table)
    pub evidence_params: Vec<MirEvidenceParam>,
    pub ret_type: Type,
    pub body: Vec<MirStmt>,
}

#[derive(Debug, Clone)]
pub struct MirParam {
    pub label: String,
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone)]
pub struct MirEvidenceParam {
    pub port_name: String,
    pub param_name: String, // e.g. "__ev_Logger"
}

#[derive(Debug, Clone)]
pub enum MirStmt {
    Let {
        name: String,
        typ: Type,
        expr: MirExpr,
    },
    Expr(MirExpr),
    Return(MirExpr),
    Assign {
        target: MirExpr,
        value: MirExpr,
    },
    If {
        cond: Box<MirExpr>,
        then_body: Vec<MirStmt>,
        else_body: Option<Vec<MirStmt>>,
    },
    Conc(Vec<MirFunction>),
    Try {
        body: Vec<MirStmt>,
        catch_param: String,
        catch_body: Vec<MirStmt>,
    },
}

#[derive(Debug, Clone)]
pub enum MirExpr {
    Literal(Literal),
    Variable(String),
    BinaryOp(Box<MirExpr>, BinaryOp, Box<MirExpr>),
    Call {
        func: String,
        args: Vec<(String, MirExpr)>,
        /// Evidence arguments to thread through
        evidence_args: Vec<MirExpr>,
        ret_type: Type,
    },
    /// Port method call via evidence: call_indirect(evidence_base + method_offset, args)
    CallIndirect {
        evidence_base: Box<MirExpr>,
        method_offset: usize,
        args: Vec<(String, MirExpr)>,
        ret_type: Type,
    },
    Constructor {
        name: String,
        args: Vec<MirExpr>,
    },
    Record(Vec<(String, MirExpr)>),
    Array(Vec<MirExpr>),
    Index(Box<MirExpr>, Box<MirExpr>),
    FieldAccess(Box<MirExpr>, String),
    ObjectTag(Box<MirExpr>),
    ObjectField {
        value: Box<MirExpr>,
        index: usize,
    },
    If {
        cond: Box<MirExpr>,
        then_body: Vec<MirStmt>,
        else_body: Option<Vec<MirStmt>>,
    },
    Match {
        target: Box<MirExpr>,
        cases: Vec<MirMatchCase>,
    },
    Lambda {
        params: Vec<MirParam>,
        evidence_params: Vec<MirEvidenceParam>,
        ret_type: Type,
        body: Vec<MirStmt>,
    },
    Borrow(String),
    Raise(Box<MirExpr>),
}

#[derive(Debug, Clone)]
pub struct MirMatchCase {
    pub pattern: MirPattern,
    pub body: Vec<MirStmt>,
}

#[derive(Debug, Clone)]
pub enum MirPattern {
    Literal(Literal),
    Variable(String, Sigil),
    Constructor {
        name: String,
        fields: Vec<(Option<String>, MirPattern)>,
    },
    Record(Vec<(String, MirPattern)>, bool),
    Wildcard,
}
