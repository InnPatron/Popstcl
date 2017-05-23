pub mod user {
    pub use super::basic_vm;
    pub use super::cmd::Cmd;
    pub use super::exec_signal::ExecSignal;
    pub use super::env::Env;
    pub use super::value::{Value, IntoValue};
    pub use super::err::ExecErr;
    pub use super::object_kind::ObjectKind;
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
    pub use super::executor::eval_some_cmd;
    pub use super::permissions::{EntryPermissions, Permissions};

    pub use parser::parse_statement_seq;
    pub use namespace::Namespace;
    pub use parser::err::ParseErr;

    pub use super::object::{Object, ObjectEnv};
	pub use super::module::{StdModule, InternalModule, LocalModule, Module};
}

#[macro_use]
mod permissions;
#[macro_use]
pub mod value;
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

use ast::Word;
use self::err::*;
use self::value::Value;
use self::env::Env;
use self::executor::eval_some_cmd;
use self::stack::Stack;
use self::exec_signal::ExecSignal;
use self::env_builder::EnvBuilder;
use self::module::InternalModule;
use self::object::Object;

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

    pub fn eval_some_cmd(&mut self, cmd: &[Word]) -> Result<ExecSignal, ExecErr> {

        eval_some_cmd(&mut Stack::new_module(&mut self.main_module), cmd)
    }

    pub fn inspect_value(&self, name: &str) -> Result<Value, ExecErr> {
        self.main_module.get_clone(name)
    }
}

#[cfg(test)]
impl Vm {
    pub fn get_main_module(&mut self) -> &mut Env {
        use self::internal::ObjectEnv;
        self.main_module.get_env_mut()
    }
}
