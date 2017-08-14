use vm::internal::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cell::Ref;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use ccrc::{Collectable, Tracer};

pub trait Object {
	fn insert(&mut self, name: &str, value: RcValue) -> Result<(), ObjectErr> {
        self.env_mut().insert(name, value);
        Ok(())
    }

	fn get(&self, name: &str) -> Result<RcValue, ObjectErr> {
        Ok(self.env().get(name).ok_or(ObjectErr::UnknownField(name.to_string()))?.clone())
    }

    fn remove(&mut self, name: &str) -> Option<RcValue> {
        self.env_mut().remove(name)
    }

    fn len(&self) -> usize {
        self.env().len()
    }

    fn env(&self) -> &Env;

    fn env_mut(&mut self) -> &mut Env;
}

#[derive(Clone, Debug, PartialEq)]
pub struct StdObject(Env);

impl StdObject {
	pub fn with_env(env: Env) -> StdObject {
	    StdObject(env)
	}

    pub fn empty() -> StdObject {
        StdObject(Env::empty())
    }
}

impl Object for StdObject {
   fn env(&self) -> &Env {
        &self.0
    }

    fn env_mut(&mut self) -> &mut Env {
        &mut self.0
    }
}

impl Collectable for StdObject {
    fn trace(&self, tracer: &Tracer) {
        Collectable::trace(&self.0, tracer);
    }
}

impl IntoValue for StdObject {
    fn into_value(self) -> Value {
        Value::Object(self)
    }
}

impl DeepClone for StdObject {
    fn deep_clone(&self) -> Self {
        StdObject::with_env(self.0.deep_clone())
    }
}

impl Eq for StdObject {}

impl Display for StdObject {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Object[{}]", ToString::to_string(&self.0))
    }
}

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
