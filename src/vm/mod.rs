pub mod user {
    pub use super::Vm;
    pub use super::basic_vm;
    pub use super::cmd::Cmd;
    pub use super::exec_signal::ExecSignal;
    pub use super::env::Env;
    pub use super::value::{Value, IntoValue};
    pub use super::err::ExecErr;
    pub use super::object::Object;
    pub use super::object_kind::StdObject;
    pub use super::env_builder::EnvBuilder;
}

pub mod internal {
    pub use super::cmd::Cmd;
    pub use super::exec_signal::ExecSignal;
    pub use super::err::{ExecErr, ArityErr};
    pub use super::env::Env;
    pub use super::value::{Value, IntoValue};
    pub use super::object_kind::{ ObjectKind, StdObject };
    pub use super::stack::Stack;
    pub use super::cir::CIR;
    pub use super::env_builder::EnvBuilder;
    pub use super::env_entry::EnvEntry;
    pub use super::executor::{eval_program, eval_stmt};
    pub use super::permissions::{EntryPermissions, Permissions};

    pub use namespace::Namespace;
    pub use parser::err::ParseErr;

    pub use super::object::Object;
	pub use super::module::{StdModule, InternalModule, LocalModule, Module};
}

#[macro_use]
mod permissions;
#[macro_use]
mod value;
mod err;
mod cmd;
mod executor;
mod stack;
mod object_kind;
mod env_entry;
#[macro_use]
mod cir;
mod env;
mod env_builder;
mod exec_signal;
#[macro_use]
mod object;
mod module;

use ast::Program;
use self::err::*;
use self::value::Value;
use self::env::Env;
use self::executor::eval_program;
use self::stack::Stack;
use self::exec_signal::ExecSignal;
use self::env_builder::EnvBuilder;
use self::module::InternalModule;
use self::object::Object;
use parser::parse_program;

#[allow(unused_must_use)]
pub fn basic_vm() -> Vm {
    Vm::new_with_main_module(EnvBuilder::basic_env().consume())
}

pub struct Vm {
    main_module: InternalModule,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { main_module: InternalModule::new(Env::new()) }
    }

    pub fn new_with_main_module(env: Env) -> Vm {
        Vm { main_module: InternalModule::new(env) }
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<(), ExecErr> {
        eval_program(&mut Stack::new_module(&mut self.main_module), program)
    }

    pub fn eval_string(&mut self, program: &str) -> Result<(), ExecErr> {
        let program = parse_program(program)?;
        eval_program(&mut Stack::new_module(&mut self.main_module), &program)
    }

    pub fn inspect_value(&self, name: &str) -> Result<Value, ExecErr> {
        self.main_module.get(name)
    }
}
