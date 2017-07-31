use std::convert::From;
use super::internal::{CIR, ParseErr, DebugInfo};
use ast::*;
use namespace::Namespace;
use line_info::LineInfo;

#[derive(Clone, Debug, PartialEq)]
pub enum ExecErr {
    CmdErr(CmdErr, DebugInfo),
    NotCmd(String, DebugInfo),
    VarSubErr(VarSubErr, DebugInfo),

    NoRet(String, DebugInfo),
    
    ObjectErr(ObjectErr, DebugInfo),

    BadBreak(DebugInfo),
    BadContinue(DebugInfo),

    Generic(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CmdErr {
    InvalidArg { expect: String, found: CIR },
    InvalidNamespace { expect: String, found: Namespace },
    MissingArg(String),
    UnexpectedArg(CIR),
    Arity(ArityErr),

    ParseErr(ParseErr),

    ObjectErr(ObjectErr),

    InvalidIndex(usize),
    NoLocalModule,
    ExecErr(Box<ExecErr>),
    Generic(String)
}

impl From<ExecErr> for CmdErr {
    fn from(e: ExecErr) -> CmdErr {
        CmdErr::ExecErr(Box::new(e))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArityErr {
    Modulo { modulo: usize, found: usize },

    Exact { exact: usize, found: usize },

    Min { min: usize, found: usize },

    Max { max: usize, found: usize },
}

impl From<ArityErr> for CmdErr {
    fn from(a: ArityErr) -> CmdErr {
        CmdErr::Arity(a)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarSubErr {
    UnknownBinding(String, Namespace),
    NonobjectFieldAccess(String),
    NoArgs,
    NoLocalModule,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectErr {
    UnknownField(String),
//    InsufficientPermissions(Permissions),
}

impl From<ParseErr> for CmdErr {
    fn from(e: ParseErr) -> CmdErr {
        CmdErr::ParseErr(e)
    }
}
