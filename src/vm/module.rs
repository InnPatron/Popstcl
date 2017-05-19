#[macro_use]
use super::internal::*;

pub trait Module: Object {}

/// Interface to an Env for modules loaded by popstcl commands
#[derive(Clone, Debug)]
pub struct StdModule(Env);

impl StdModule {
	pub fn new(env: Env) -> StdModule {
		StdModule(env)
	}
}

impl Module for StdModule {}

impl ToString for StdModule {
    fn to_string(&self) -> String {
        unimplemented!();
    }
}

impl PartialEq for StdModule {
    fn eq(&self, other: &StdModule) -> bool {
        self.0 == other.0
    }
}

impl Eq for StdModule {}

impl Object for StdModule {

	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr> {
		let env = &mut self.0;
        if let Some(entry) = env.get(name) {
            has_permission!(entry, Permissions::ForeignModWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value, permissions);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<&Value, ExecErr> {
		let entry = self.0.get(name).ok_or(ExecErr::UnknownBinding(name.to_string()))?;
        has_permission!(entry, Permissions::ForeignModRead);
        Ok(entry.value())
	}
}

/// Interface for the original modules created by a popstcl vm instance
#[derive(Clone, Debug)]
pub struct InternalModule(Env);

impl InternalModule {
	pub fn new(env: Env) -> InternalModule {
		InternalModule(env)
	}

    pub fn into_foreign(self) -> StdModule {
        StdModule(self.0)
    }
}

impl Module for InternalModule {}

impl ObjectEnv for InternalModule {

	fn get_env(&self) -> &Env {
		&self.0
	}

	fn clone_env(&self) -> Env {
		self.0.clone()
	}

	fn get_env_mut(&mut self) -> &mut Env {
		&mut self.0
	}
}

impl Object for InternalModule {

	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr> {
		let env = &mut self.0;
        if let Some(entry) = env.get(name) {
            has_permission!(entry, Permissions::InternalWrite);
        }
        // else no present entry and can write anyways
        
        env.insert(name, value, permissions);
		Ok(())
	}

	fn get(&self, name: &str) -> Result<&Value, ExecErr> {
		let entry = self.0.get(name).ok_or(ExecErr::UnknownBinding(name.to_string()))?;
        has_permission!(entry, Permissions::InternalRead);
        Ok(entry.value())
	}
}

/// Interface to an Env in a subprocess context.
/// All permission options are ignored b/c LocalModule is a temporary thing only.
/// Anything is free to inspect or insert into it...
#[derive(Clone, Debug)]
pub struct LocalModule(Env);

impl LocalModule {
	pub fn new(env: Env) -> LocalModule {
		LocalModule(env)
	}
}

impl Module for LocalModule {}

impl ObjectEnv for LocalModule {

	fn get_env(&self) -> &Env {
		&self.0
	}

	fn clone_env(&self) -> Env {
		self.0.clone()
	}

	fn get_env_mut(&mut self) -> &mut Env {
		&mut self.0
	}
}

impl Object for LocalModule {

	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr> {
		self.0.insert(name, value, permissions);
        Ok(())
	}

	fn get(&self, name: &str) -> Result<&Value, ExecErr> {
	    Ok(self.0.get(name)
                .map(|entry| entry.value())
                .ok_or(ExecErr::UnknownBinding(name.to_string()))?
          )
	}
}
