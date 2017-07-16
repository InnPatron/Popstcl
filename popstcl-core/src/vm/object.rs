use vm::internal::*;

pub trait Object {
	fn insert(&self, name: &str, value: RcValue) -> Result<(), ObjectErr>;

	fn get(&self, name: &str) -> Result<RcValue, ObjectErr>;
}
