use ast::*;
use namespace::Namespace;
use line_info::LineInfo;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugInfo {
    pub kind: DebugKind,
    pub root_line: LineInfo,
    pub cmd_info: LineInfo,
    pub line_info: LineInfo,
    pub original_string: Rc<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugKind {
    VarSub(Namespace, Path),
    StrSub,     //TODO: add detailed information for var subs WITHIN string
    CmdSub(Vec<DebugInfo>),
    Literal,
    VarInsertion,
}

#[macro_use]
macro_rules! dliteral {
    ($line_info: expr, $cur_stmt: expr, $root_stmt: expr) => {
        DebugInfo {
            kind: DebugKind::Literal,
            line_info: $line_info,
            cmd_info: $cur_stmt.line_info.clone(),
            root_line: $root_stmt.line_info.clone(),
            original_string: $root_stmt.original_string.clone(),
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
    ($namespace: expr, $path: expr, $line_info: expr, $cur_stmt: expr, $root_stmt: expr) => {
        DebugInfo {
            kind: DebugKind::VarSub($namespace, $path),
            line_info: $line_info,
            cmd_info: $cur_stmt.line_info.clone(),
            root_line: $root_stmt.line_info.clone(),
            original_string: $root_stmt.original_string.clone(),
        }
    }
}

#[macro_use]
macro_rules! dcmd_sub {
    ($debug_info: expr, $line_info: expr, $cur_stmt: expr, $root_stmt: expr) => {
        DebugInfo {
            kind: DebugKind::CmdSub($debug_info),
            line_info: $line_info,
            root_line: $root_stmt.line_info.clone(),
            cmd_info: $cur_stmt.line_info.clone(),
            original_string: $root_stmt.original_string.clone(),
        }
    }
}

#[macro_use]
macro_rules! dstr_sub {
    ($line_info: expr, $cur_stmt: expr, $root_stmt: expr) => {
        DebugInfo {
            kind: DebugKind::StrSub,
            line_info: $line_info,
            root_line: $root_stmt.line_info.clone(),
            cmd_info: $cur_stmt.line_info.clone(),
            original_string: $root_stmt.original_string.clone(),
        }
    }
}
