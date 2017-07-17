pub mod user {
    pub use super::Vm;
    pub use super::basic_vm;
    pub use super::cmd::Cmd;
    pub use super::exec_signal::ExecSignal;
    pub use super::env::Env;
    pub use super::value::{Value, IntoValue};
    pub use super::val_ref::*;
    pub use super::err::ExecErr;
    pub use super::object::{Object, ObjectKind};
    pub use super::env_builder::EnvBuilder;
    pub use super::debug_info::{DebugInfo, DebugKind};
}

pub mod internal {
    pub use super::cmd::Cmd;
    pub use super::exec_signal::ExecSignal;
    pub use super::err::{ExecErr, ArityErr, VarSubErr, ObjectErr};
    pub use super::env::Env;
    pub use super::value::{Value, IntoValue, RcValue};
    pub use super::val_ref::*;
    pub use super::object::{ Object, ObjectKind, StdObject };
    pub use super::stack::Stack;
    pub use super::cir::CIR;
    pub use super::env_builder::EnvBuilder;
    pub use super::executor::{eval_program, eval_stmt};
    pub use super::debug_info::{DebugInfo, DebugKind};

    pub use namespace::Namespace;
    pub use parser::err::ParseErr;

	pub use super::module::{StdModule, Module};
}

#[macro_use]
mod value;
mod err;
mod cmd;
#[macro_use]
mod debug_info;
mod executor;
mod stack;
#[macro_use]
mod cir;
mod env;
mod env_builder;
mod exec_signal;
#[macro_use]
mod object;
mod module;
mod val_ref;

use ast::Program;
use self::err::*;
use self::value::RcValue;
use self::env::Env;
use self::executor::eval_program;
use self::stack::Stack;
use self::exec_signal::ExecSignal;
use self::env_builder::EnvBuilder;
use self::module::StdModule;
use self::object::Object;
use parser::parse_program;

#[allow(unused_must_use)]
pub fn basic_vm() -> Vm {
    Vm::new_with_main_module(EnvBuilder::basic_env().consume())
}

pub struct Vm {
    main_module: StdModule,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { main_module: StdModule::new(Env::new()) }
    }

    pub fn new_with_main_module(env: Env) -> Vm {
        Vm { main_module: StdModule::new(env) }
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<(), ExecErr> {
        eval_program(&mut Stack::new_module(&mut self.main_module), program)
    }

    pub fn eval_string(&mut self, program: &str) -> Result<(), ExecErr> {
        let program = parse_program(program)?;
        eval_program(&mut Stack::new_module(&mut self.main_module), &program)
    }

    pub fn inspect_value(&self, name: &str) -> Result<RcValue, ObjectErr> {
        self.main_module.get(name)
    }
}
