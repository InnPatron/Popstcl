use super::internal::*;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Module: Object {}

/// Interface to an Env for modules loaded by popstcl commands
#[derive(Clone, Debug, PartialEq)]
pub struct StdModule(Rc<RefCell<Env>>);

impl StdModule {
	pub fn new(env: Env) -> StdModule {
		StdModule(Rc::new(RefCell::new(env)))
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

	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr> {
		let env = &mut self.0.borrow_mut();
        if let Some(entry) = env.get(name) {
            has_permission!(entry, Permissions::ForeignModWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value, permissions);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<Value, ExecErr> {
        let env = self.0.borrow();
		let entry = env.get(name).ok_or(ExecErr::UnknownBinding(name.to_string()))?;
        has_permission!(entry, Permissions::ForeignModRead);
        Ok(entry.value().clone())
	}
}

/// Interface for the original modules created by a popstcl vm instance
#[derive(Clone, Debug, PartialEq)]
pub struct InternalModule(Rc<RefCell<Env>>);

impl InternalModule {
	pub fn new(env: Env) -> InternalModule {
		InternalModule(Rc::new(RefCell::new(env)))
	}
}

impl From<StdModule> for InternalModule {
    fn from(module: StdModule) -> InternalModule {
        InternalModule(module.0)
    }
}

impl Module for InternalModule {}

impl Object for InternalModule {

	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr> {
		let env = &mut self.0.borrow_mut();
        if let Some(entry) = env.get(name) {
            has_permission!(entry, Permissions::InternalWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value, permissions);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<Value, ExecErr> {
		let env = self.0.borrow();
        let entry = env.get(name).ok_or(ExecErr::UnknownBinding(name.to_string()))?;
        has_permission!(entry, Permissions::InternalRead);
        Ok(entry.value().clone())
	}
}

/// Interface to an Env in a subprocess context.
/// All permission options are ignored b/c LocalModule is a temporary thing only.
/// Anything is free to inspect or insert into it...
#[derive(Clone, Debug, PartialEq)]
pub struct LocalModule(Env);

impl LocalModule {
	pub fn new(env: Env) -> LocalModule {
		LocalModule(env)
	}
}

impl Module for LocalModule {}

impl Object for LocalModule {

	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr> {
		self.0.insert(name, value, permissions);
        Ok(())
	}

	fn get(&self, name: &str) -> Result<Value, ExecErr> {
	    Ok(self.0.get(name)
                .map(|entry| entry.value())
                .ok_or(ExecErr::UnknownBinding(name.to_string()))?
                .clone()
          )
	}
}
