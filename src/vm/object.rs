use vm::internal::*;

pub trait Object {
	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ExecErr>;

	fn get(&self, name: &str) -> Result<Value, ExecErr>;
}
