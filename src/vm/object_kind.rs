use vm::internal::*;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct StdObject(Env);

impl StdObject {
	pub fn with_env(env: Env) -> StdObject {
	    StdObject(env)
	}

    pub fn empty() -> StdObject {
        StdObject(Env::new())
    }
}

impl Object for StdObject {
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
