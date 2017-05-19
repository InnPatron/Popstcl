use vm::internal::*;

pub trait Object {
	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr>;

	fn get(&self, name: &str) -> Result<&Value, ExecErr>;

	fn get_clone(&self, name: &str) -> Result<Value, ExecErr> {
        self.get(name).map(|val_ref| val_ref.clone())
    }
}

pub trait ObjectEnv: Object {
	fn get_env(&self) -> &Env;
	fn get_env_mut(&mut self) -> &mut Env;
	fn clone_env(&self) -> Env;
}
