use vm::internal::*;

pub trait Object {
	fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) -> Result<(), ObjectErr>;

	fn get(&self, name: &str) -> Result<Value, ObjectErr>;
}
