use vm::internal::*;

pub trait Object {
	fn insert(&self, name: &str, value: Value) -> Result<(), ObjectErr>;

	fn get(&self, name: &str) -> Result<Value, ObjectErr>;
}
