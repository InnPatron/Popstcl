use super::internal::*;
use std::rc::Rc;
use std::cell::RefCell;
use ccrc::{Collectable, Tracer};
use std::fmt;

pub trait Module: Object {}

/// Interface to an Env for modules loaded by popstcl commands
#[derive(Clone, Debug, PartialEq)]
pub struct StdModule(RefCell<Env>);

impl StdModule {
	pub fn new(env: Env) -> StdModule {
		StdModule(RefCell::new(env))
	}
}

impl Module for StdModule {}

impl Object for StdModule {

	fn insert(&self, name: &str, value: RcValue) -> Result<(), ObjectErr> {
		let env = &mut self.0.borrow_mut();
        if let Some(entry) = env.get(name) {
            //has_permission!(entry, Permissions::ForeignModWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<RcValue, ObjectErr> {
        let env = self.0.borrow();
		Ok(env.get(name).ok_or(ObjectErr::UnknownField(name.to_string()))?.clone())
	}

    fn remove(&self, name: &str) -> Option<RcValue> {
        let env = &mut self.0.borrow_mut();
        env.remove(name)
    }
}

impl Collectable for StdModule {
    fn trace(&self, tracer: &Tracer) {
        Collectable::trace(&*self.0.borrow(), tracer);
    }
}

impl DeepClone for StdModule {
    fn deep_clone(&self) -> Self {
        StdModule::new(self.0.borrow().deep_clone())
    }
}

impl fmt::Display for StdModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cell::Ref;
        let r: Ref<Env> = self.0.borrow();
        write!(f, "Module[{}]", ToString::to_string(&*r))
    }
}

impl IntoValue for StdModule {
    fn into_value(self) -> Value {
        Value::Module(self)
    }
}
