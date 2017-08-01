use super::internal::{Value, Env, Namespace};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EnvBuilder {
    values: HashMap<String, Value>,
}

impl EnvBuilder {
    pub fn new() -> EnvBuilder {
        EnvBuilder {
            values: HashMap::new()
        }
    }

    pub fn insert_value(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn build(&self) -> Env {
        let mut env = Env::new();
        for (name, entry) in self.values.iter() {
            env.insert(name, entry.clone().into());
        }
        env
    }

    pub fn consume(self) -> Env {
        let mut env = Env::new();
        for (name, entry) in self.values.into_iter() {
            env.insert(&name, entry.into());
        }
        env
    }
}

#[cfg(test)]
mod tests {
    use super::EnvBuilder;
    use vm::internal::{Value, IntoValue};

    #[test]
    fn builder() {
        let mut builder = EnvBuilder::new();
        builder.insert_value("test", (5.0).into_value());
        let env = builder.build();
        let value = env.get("test").expect("Missing binding \'test\'");
        let borrow = value.borrow();
        assert_eq!((5.0).into_value(), *borrow);
    }
}
