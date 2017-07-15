use vm::internal::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub trait Object {
	fn insert(&self, name: &str, value: RcValue) -> Result<(), ObjectErr>;

	fn get(&self, name: &str) -> Result<RcValue, ObjectErr>;

    fn remove(&self, name: &str) -> Option<RcValue>;
}

#[derive(Clone, Debug)]
pub struct StdObject(RefCell<Env>);

impl StdObject {
	pub fn with_env(env: Env) -> StdObject {
	    StdObject(RefCell::new(env))
	}

    pub fn empty() -> StdObject {
        StdObject(RefCell::new(Env::new()))
    }
}

impl Object for StdObject {
	fn insert(&self, name: &str, value: RcValue) -> Result<(), ObjectErr> {
		let env = &mut self.0.borrow_mut();       
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

impl IntoValue for StdObject {
    fn into_value(self) -> Value {
        Value::Object(self)
    }
}

impl ToString for StdObject {
    fn to_string(&self) -> String {
        unimplemented!();
    }
}

impl PartialEq for StdObject {
    fn eq(&self, other: &StdObject) -> bool {
        self.0 == other.0
    }
}

impl Eq for StdObject {}

#[derive(Clone, Debug)]
pub struct ObjectKind<V>
    where V: ToString + Clone + Debug + Display + PartialEq
{
    pub fields: HashMap<String, V>,
}

impl<V> ObjectKind<V>
    where V: ToString + Clone + Debug + Display + PartialEq
{
    pub fn new() -> ObjectKind<V> {
        ObjectKind { fields: HashMap::new() }
    }

    pub fn from_map(map: HashMap<String, V>) -> ObjectKind<V> {
        ObjectKind { fields: map }
    }

    pub fn get(&self, name: &str) -> Option<V> {
        self.fields.get(name).map(|ref_v| ref_v.clone())
    }
}

impl<V> ToString for ObjectKind<V>
    where V: ToString + Clone + Debug + Display + PartialEq
{
    fn to_string(&self) -> String {
        unimplemented!();
    }
}

impl<V> PartialEq for ObjectKind<V>
    where V: ToString + Clone + Debug + Display + PartialEq
{
    fn eq(&self, other: &ObjectKind<V>) -> bool {
        if self.fields.len() != other.fields.len() {
            return false;
        }

        for (key, value) in self.fields.iter() {
            if let Some(ref rhs) = other.fields.get(key) {
                if &value != rhs {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
