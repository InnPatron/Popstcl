use std::convert::From;
use super::internal::{CIR, ParseErr, DebugInfo};
use ast::*;
use namespace::Namespace;
use line_info::LineInfo;

#[derive(Clone, Debug, PartialEq)]
pub enum ExecErr {
    UnknownBinding(String),
    NotCmd(String),

    Arity(ArityErr),
    VarSubErr(VarSubErr),

    InvalidArg { expect: String, found: CIR },
    InvalidNamespace { expect: String, found: Namespace },

    MissingArg(String),
    UnexpectedArg(CIR),

    InvalidIndex(usize),
    VarSubErrOnCmd(String),

    NoRet(Word),
    
    CmdReturned(Word),
    ParseError(ParseErr),
    ObjectErr(ObjectErr, DebugInfo),
    NoLocalModule,

    BadBreak,
    BadContinue,

    Generic(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArityErr {
    Modulo { modulo: usize, found: usize },

    Exact { exact: usize, found: usize },

    Min { min: usize, found: usize },

    Max { max: usize, found: usize },
}

impl From<ArityErr> for ExecErr {
    fn from(a: ArityErr) -> ExecErr {
        ExecErr::Arity(a)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarSubErr {
    UnknownBinding(String, Namespace, DebugInfo),
    NonobjectFieldAccess(String, DebugInfo),
    NoArgs(DebugInfo),
    NoLocalModule(DebugInfo),
}

impl From<VarSubErr> for ExecErr {
    fn from(e: VarSubErr) -> ExecErr {
        ExecErr::VarSubErr(e)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectErr {
    UnknownField(String),
//    InsufficientPermissions(Permissions),
}

impl From<ParseErr> for ExecErr {
    fn from(a: ParseErr) -> ExecErr {
        ExecErr::ParseError(a)
    }
}
