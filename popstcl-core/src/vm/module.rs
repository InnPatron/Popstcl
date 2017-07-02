use super::internal::*;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Module: Object {}

/// Interface to an Env for modules loaded by popstcl commands
#[derive(Clone, Debug, PartialEq)]
pub struct StdModule(RefCell<Env>);

impl StdModule {
	pub fn new(env: Env) -> StdModule {
		StdModule(RefCell::new(env))
	}
}

impl From<InternalModule> for StdModule {
    fn from(m: InternalModule) -> StdModule {
        StdModule(m.0)
    }
}

impl Module for StdModule {}

impl ToString for StdModule {
    fn to_string(&self) -> String {
        unimplemented!();
    }
}

impl Object for StdModule {

	fn insert(&self, name: &str, value: Value) -> Result<(), ObjectErr> {
		let env = &mut self.0.borrow_mut();
        if let Some(entry) = env.get(name) {
            //has_permission!(entry, Permissions::ForeignModWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<Value, ObjectErr> {
        let env = self.0.borrow();
		Ok(env.get(name).ok_or(ObjectErr::UnknownField(name.to_string()))?)
	}
}

/// Interface for the original modules created by a popstcl vm instance
#[derive(Clone, Debug, PartialEq)]
pub struct InternalModule(RefCell<Env>);

impl InternalModule {
	pub fn new(env: Env) -> InternalModule {
		InternalModule(RefCell::new(env))
	}
}

impl From<StdModule> for InternalModule {
    fn from(module: StdModule) -> InternalModule {
        InternalModule(module.0)
    }
}

impl Module for InternalModule {}

impl Object for InternalModule {

	fn insert(&self, name: &str, value: Value) -> Result<(), ObjectErr> {
		let env = &mut self.0.borrow_mut();
        if let Some(entry) = env.get(name) {
            //has_permission!(entry, Permissions::InternalWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<Value, ObjectErr> {
		let env = self.0.borrow();
        Ok(env.get(name).ok_or(ObjectErr::UnknownField(name.to_string()))?)
	}
}

/// Interface to an Env in a subprocess context.
/// All permission options are ignored b/c LocalModule is a temporary thing only.
/// Anything is free to inspect or insert into it...
#[derive(Clone, Debug, PartialEq)]
pub struct LocalModule(RefCell<Env>);

impl LocalModule {
	pub fn new(env: Env) -> LocalModule {
		LocalModule(RefCell::new(env))
	}
}

impl Module for LocalModule {}

impl Object for LocalModule {

	fn insert(&self, name: &str, value: Value) -> Result<(), ObjectErr> {
		self.0.borrow_mut().insert(name, value);
        Ok(())
	}

	fn get(&self, name: &str) -> Result<Value, ObjectErr> {
	    Ok(self.0.borrow().get(name)
                .ok_or(ObjectErr::UnknownField(name.to_string()))?
          )
	}
}
