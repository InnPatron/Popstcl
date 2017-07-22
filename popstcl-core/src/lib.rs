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
    use super::ast::{Program, Statement};
    use super::vm::internal::*;
    use super::namespace::Namespace;
    use super::parser::parse_program;
    use super::std_cmds::*;
    use super::line_info::LineInfo;
}

use vm::Vm;
use vm::internal::EnvBuilder;

#[allow(unused_must_use)]
pub fn basic_vm() -> Vm {
    Vm::new_with_main_module(EnvBuilder::std_env().consume())
}
