#![allow(unused_imports)]

extern crate itertools;
extern crate ccrc;

#[macro_use]
mod line_info;
#[macro_use]
mod ast;
mod parser;
#[macro_use]
mod vm;
mod std_cmds;

mod namespace;

pub use vm::user::*;
pub use ast::Program;
pub use parser::parse_program;
pub use std_cmds::*;
pub use line_info::LineInfo;

pub mod internal {
    pub use super::ast::{Program, Statement};
    pub use super::vm::internal::*;
    pub use super::namespace::Namespace;
    pub use super::parser::parse_program;
    pub use super::std_cmds::*;
    pub use super::line_info::LineInfo;
}

#[allow(unused_must_use)]
pub fn basic_vm() -> Vm {
    Vm::new_with_main_module(EnvBuilder::std_env().consume())
}
