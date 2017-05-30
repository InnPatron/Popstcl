#![allow(unused_imports)]

extern crate itertools;

#[macro_use]
pub mod line_info;
#[macro_use]
pub mod ast;
pub mod parser;
#[macro_use]
pub mod vm;
pub mod cmds;

mod namespace;

pub mod user {
    use super::ast::Program;
    use super::parser::parse_program;
    use super::parser::err::ParseErr;
    use super::vm::user;
    use super::cmds::*;
    use super::line_info::LineInfo;
}

pub mod internal {
    use super::ast::{Program, Statement};
    use super::vm::internal;
    use super::namespace::Namespace;
    use super::parser::parse_program;
    use super::parser::err::ParseErr;
    use super::cmds::*;
    use super::line_info::LineInfo;
}
