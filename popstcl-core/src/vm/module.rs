use super::internal::*;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use ccrc::{Collectable, Tracer};
use std::fmt;

pub trait Module: Object {}

/// Interface to an Env for modules loaded by popstcl commands
#[derive(Clone, Debug, PartialEq)]
pub struct StdModule(Env);

impl StdModule {
	pub fn new(env: Env) -> StdModule {
		StdModule(env)
	}
}

impl Module for StdModule {}

impl Object for StdModule {

	fn insert(&mut self, name: &str, value: RcValue) -> Result<(), ObjectErr> {
        self.0.insert(name, value);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<RcValue, ObjectErr> {
		Ok(self.0.get(name).ok_or(ObjectErr::UnknownField(name.to_string()))?.clone())
	}

    fn remove(&mut self, name: &str) -> Option<RcValue> {
        self.0.remove(name)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn env(&self) -> &Env {
        &self.0
    }

    fn env_mut(&mut self) -> &mut Env {
        &mut self.0
    }
}

impl Collectable for StdModule {
    fn trace(&self, tracer: &Tracer) {
        Collectable::trace(&self.0, tracer);
    }
}

impl DeepClone for StdModule {
    fn deep_clone(&self) -> Self {
        StdModule::new(self.0.deep_clone())
    }
}

impl fmt::Display for StdModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Module[{}]", ToString::to_string(&self.0))
    }
}

impl IntoValue for StdModule {
    fn into_value(self) -> Value {
        Value::Module(self)
    }
}
