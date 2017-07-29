use ast::*;
use namespace::Namespace;
use line_info::LineInfo;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugInfo {
    pub kind: DebugKind,
    pub segment_span: LineInfo,
    pub common: CommonInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugKind {
    VarSub(Namespace, Path),
    StrSub,     //TODO: add detailed information for var subs WITHIN string
    CmdSub(Vec<DebugInfo>),
    Literal,
    VarInsertion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommonInfo {
    pub root_stmt_span: LineInfo,
    pub cmd_span: LineInfo,
    pub original_string: Rc<String>,
}

pub trait InfoGenerator {
    fn info(&self) -> CommonInfo;
}

#[macro_use]
macro_rules! dliteral {
    ($segment_span: expr, $common: expr) => {
        DebugInfo {
            kind: DebugKind::Literal,
            segment_span: $segment_span,
            common: $common
        }
    }
}

#[macro_use]
macro_rules! dinsertion {
    ($line_info: expr, $base_dinfo: expr) => {
        DebugInfo {
            kind: DebugKind::VarInsertion,
            line_info: $line_info,
            cmd_info: $base_dinfo.cmd_info.clone(),
            root_line: $base_dinfo.root_line.clone(),
            original_string: $base_dinfo.original_string.clone(),
        }
    }
}

#[macro_use]
macro_rules! dvar_sub {
    ($namespace: expr, $path: expr, $segment_span: expr, $common: expr) => {
        DebugInfo {
            kind: DebugKind::VarSub($namespace, $path),
            segment_span: $segment_span,
            common: $common
        }
    }
}

#[macro_use]
macro_rules! dcmd_sub {
    ($debug_info: expr, $segment_span: expr, $common: expr) => {
        DebugInfo {
            kind: DebugKind::CmdSub($debug_info),
            segment_span: $segment_span,
            common: $common,
        }
    }
}

#[macro_use]
macro_rules! dstr_sub {
    ($segment_span: expr, $common: expr) => {
        DebugInfo {
            kind: DebugKind::StrSub,
            segment_span: $segment_span,
            common: $common,
        }
    }
}
