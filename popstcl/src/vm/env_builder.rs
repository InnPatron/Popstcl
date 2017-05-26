use super::internal::{Value, Env, EnvEntry, Namespace, EntryPermissions};
use cmds::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EnvBuilder {
    values: HashMap<String, EnvEntry>,
}

impl EnvBuilder {
    pub fn basic_env() -> EnvBuilder {
        let mut builder = EnvBuilder { values: HashMap::new() };

        builder.insert_value("let", Value::Cmd(Box::new(Let(Namespace::Local))), all_readonly!());
        builder.insert_value("set", Value::Cmd(Box::new(Set(Namespace::Local))), all_readonly!());
        builder.insert_value("proc", Value::Cmd(Box::new(Proc(Namespace::Local))), all_readonly!());

        builder.insert_value("mlet", Value::Cmd(Box::new(Let(Namespace::Module))), all_readonly!());
        builder.insert_value("mset", Value::Cmd(Box::new(Set(Namespace::Module))), all_readonly!());
        builder.insert_value("mproc", Value::Cmd(Box::new(Proc(Namespace::Module))), all_readonly!());

        builder.insert_value("add", Value::Cmd(Box::new(Add)), all_readonly!());
        builder.insert_value("sub", Value::Cmd(Box::new(Sub)), all_readonly!());
        builder.insert_value("mul", Value::Cmd(Box::new(Mul)), all_readonly!());
        builder.insert_value("div", Value::Cmd(Box::new(Div)), all_readonly!());
       
        builder.insert_value("if", Value::Cmd(Box::new(If)), all_readonly!());
        builder.insert_value("while", Value::Cmd(Box::new(While)), all_readonly!());

        builder.insert_value("return", Value::Cmd(Box::new(Return)), all_readonly!());
        builder.insert_value("continue", Value::Cmd(Box::new(Continue)), all_readonly!());
        builder.insert_value("break", Value::Cmd(Box::new(Break)), all_readonly!());

        builder.insert_value("make", Value::Cmd(Box::new(MakeModule)), all_readonly!());

        builder.insert_value("==", Value::Cmd(Box::new(Eq)), all_readonly!());
        builder.insert_value("!=", Value::Cmd(Box::new(InEq)), all_readonly!());
        builder.insert_value(">", Value::Cmd(Box::new(GreaterThan)), all_readonly!());
        builder.insert_value("<", Value::Cmd(Box::new(LessThan)), all_readonly!());
        builder.insert_value("<=", Value::Cmd(Box::new(LessThanEq)), all_readonly!());
        builder.insert_value(">=", Value::Cmd(Box::new(GreaterThanEq)), all_readonly!());

        builder.insert_value("list", Value::Cmd(Box::new(List)), all_readonly!());
        builder.insert_value("list_len", Value::Cmd(Box::new(ListLength)), all_readonly!());
        builder.insert_value("list_get", Value::Cmd(Box::new(ListIndex)), all_readonly!());
        builder.insert_value("list_remove", Value::Cmd(Box::new(Remove)), all_readonly!());
        builder.insert_value("list_pop", Value::Cmd(Box::new(Pop)), all_readonly!());
        builder.insert_value("list_append", Value::Cmd(Box::new(Append)), all_readonly!());
        
        builder
    }

    pub fn insert_value(&mut self, name: &str, value: Value, permissions: EntryPermissions) {
        self.values.insert(name.to_string(), EnvEntry::new(value, permissions));
    }

    pub fn build(&self) -> Env {
        let mut env = Env::new();
        for (name, entry) in self.values.iter() {
            env.insert_entry(name, entry.clone());
        }
        env
    }

    pub fn consume(self) -> Env {
        let mut env = Env::new();
        for (name, entry) in self.values.into_iter() {
            env.insert_entry(&name, entry);
        }
        env
    }
}

#[cfg(test)]
mod tests {
    use super::EnvBuilder;
    use vm::internal::{Value, EntryPermissions};

    #[test]
    fn builder() {
        let mut builder = EnvBuilder::basic_env();
        builder.insert_value("test", Value::Number(5.0), EntryPermissions::new().internal_read().external_read());
        let env = builder.build();
        assert_eq!(Value::Number(5.0), env.get("test").expect("Missing binding \'test\'").clone_value());
    }
}
