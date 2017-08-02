pub mod user {
    pub use super::{Vm, VmErr};
    pub use super::cmd::Cmd;
    pub use super::value::{RcValue, Value, IntoValue, DeepClone};
    pub use super::val_ref::*;
    pub use super::err::{ExecErr};
    pub use super::object::{Object, ObjectKind};
    pub use super::env_builder::EnvBuilder;
    pub use super::debug_info::{DebugInfo, DebugKind, CommonInfo};
}

pub mod internal {
    pub use super::{Vm, VmErr};
    pub use super::cmd::Cmd;
    pub use super::exec_signal::ExecSignal;
    pub use super::err::{ExecErr, CmdErr, ArityErr, VarSubErr, ObjectErr};
    pub use super::env::Env;
    pub use super::value::{Value, IntoValue, RcValue, DeepClone};
    pub use super::val_ref::*;
    pub use super::object::{ Object, ObjectKind, StdObject };
    pub use super::stack::Stack;
    pub use super::cir::CIR;
    pub use super::env_builder::EnvBuilder;
    pub use super::executor::{eval_program, eval_stmt};
    pub use super::debug_info::{DebugInfo, DebugKind, CommonInfo, InfoGenerator};

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
use parser::err::*;
use self::err::*;
use self::value::{Value, RcValue};
use self::env::Env;
use self::executor::eval_program;
use self::stack::Stack;
use self::exec_signal::ExecSignal;
use self::env_builder::EnvBuilder;
use self::module::StdModule;
use self::object::Object;
use parser::parse_program;

#[derive(Debug, Clone, PartialEq)]
pub enum VmErr {
    ParseErr(ParseErr),
    ExecErr(ExecErr),
}

impl From<ParseErr> for VmErr {
    fn from(e: ParseErr) -> Self {
        VmErr::ParseErr(e)
    }
}

impl From<ExecErr> for VmErr {
    fn from(e: ExecErr) -> Self {
        VmErr::ExecErr(e)
    }
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

    pub fn eval_program(&mut self, program: &Program) -> Result<Option<RcValue>, VmErr> {
        eval_program(&mut Stack::new_module(&mut self.main_module), program)
            .map_err(|e| e.into())
    }

    pub fn eval_str(&mut self, program: &str) -> Result<Option<RcValue>, VmErr> {
        let program = parse_program(program)?;
        eval_program(&mut Stack::new_module(&mut self.main_module), &program)
            .map_err(|e| e.into())
    }

    pub fn get(&self, name: &str) -> Result<RcValue, ObjectErr> {
        self.main_module.get(name)
    }

    pub fn insert(&mut self, name: &str, value: RcValue) {
        self.main_module.insert(name, value);
    }

    pub fn get_value(&self, name: &str) -> Result<Value, ObjectErr> {
        self.main_module.get(name).map(|rcv| rcv.inner_clone())
    }

    pub fn insert_value(&mut self, name: &str, value: Value) {
        self.main_module.insert(name, RcValue::new(value));
    }
}
