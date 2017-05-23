extern crate itertools;

#[macro_use]
pub mod line_info;
#[macro_use]
pub mod ast;
pub mod parser;
pub mod vm;
pub mod cmds;

mod namespace;
