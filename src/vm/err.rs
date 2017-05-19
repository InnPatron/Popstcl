use std::convert::From;
use super::internal::{CIR, ParseErr, Permissions};
use ast::*;
use namespace::Namespace;

#[derive(Debug)]
pub enum ExecErr {
    LocalOpInNonlocalContext,
    UnknownBinding(String),
    NotCmd(String),

    Arity(ArityErr),
    NoArguments,

    InvalidArg { expect: String, found: CIR },
    InvalidNamespace { expect: String, found: Namespace },

    MissingArg(String),
    UnexpectedArg(CIR),
    UnexpectedWord(Word),

    InvalidIndex(usize),
    VarSubOnCmd(String),

    NoRet(Word),

    CmdReturned(Word),

    NoPermission(Permissions),
    ParseError(ParseErr),
    UnknownField(String, Path),
    NonobjectFieldAccess(String, Path),
}

#[derive(Debug)]
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

impl From<ParseErr> for ExecErr {
    fn from(a: ParseErr) -> ExecErr {
        ExecErr::ParseError(a)
    }
}
