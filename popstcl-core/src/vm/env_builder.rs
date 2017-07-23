use super::internal::{Value, Env, Namespace};
use std_cmds::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EnvBuilder {
    values: HashMap<String, Value>,
}

impl EnvBuilder {
    pub fn std_env() -> EnvBuilder {
        let mut builder = EnvBuilder { values: HashMap::new() };

        builder.insert_value("llet", Value::Cmd(Box::new(Let(Namespace::Local))));
        builder.insert_value("lset", Value::Cmd(Box::new(Set(Namespace::Local))));
        builder.insert_value("lproc", Value::Cmd(Box::new(Proc(Namespace::Local))));
        builder.insert_value("lmut", Value::Cmd(Box::new(Mutate(Namespace::Local))));

        builder.insert_value("let", Value::Cmd(Box::new(Let(Namespace::Module))));
        builder.insert_value("set", Value::Cmd(Box::new(Set(Namespace::Module))));
        builder.insert_value("proc", Value::Cmd(Box::new(Proc(Namespace::Module))));
        builder.insert_value("mut", Value::Cmd(Box::new(Mutate(Namespace::Module))));

        builder.insert_value("mlet", Value::Cmd(Box::new(Let(Namespace::Module))));
        builder.insert_value("mset", Value::Cmd(Box::new(Set(Namespace::Module))));
        builder.insert_value("mproc", Value::Cmd(Box::new(Proc(Namespace::Module))));
        builder.insert_value("mmut", Value::Cmd(Box::new(Mutate(Namespace::Module))));

        builder.insert_value("add", Value::Cmd(Box::new(Add)));
        builder.insert_value("sub", Value::Cmd(Box::new(Sub)));
        builder.insert_value("mul", Value::Cmd(Box::new(Mul)));
        builder.insert_value("div", Value::Cmd(Box::new(Div)));
       
        builder.insert_value("if", Value::Cmd(Box::new(If)));
        builder.insert_value("while", Value::Cmd(Box::new(While)));

        builder.insert_value("return", Value::Cmd(Box::new(Return)));
        builder.insert_value("continue", Value::Cmd(Box::new(Continue)));
        builder.insert_value("break", Value::Cmd(Box::new(Break)));

        builder.insert_value("make", Value::Cmd(Box::new(MakeModule)));

        builder.insert_value("==", Value::Cmd(Box::new(Eq)));
        builder.insert_value("!=", Value::Cmd(Box::new(InEq)));
        builder.insert_value(">", Value::Cmd(Box::new(GreaterThan)));
        builder.insert_value("<", Value::Cmd(Box::new(LessThan)));
        builder.insert_value("<=", Value::Cmd(Box::new(LessThanEq)));
        builder.insert_value(">=", Value::Cmd(Box::new(GreaterThanEq)));

        builder.insert_value("list", Value::Cmd(Box::new(List)));
        builder.insert_value("list_len", Value::Cmd(Box::new(ListLength)));
        builder.insert_value("list_get", Value::Cmd(Box::new(ListIndex)));
        builder.insert_value("list_remove", Value::Cmd(Box::new(Remove)));
        builder.insert_value("list_pop", Value::Cmd(Box::new(Pop)));
        builder.insert_value("list_append", Value::Cmd(Box::new(Append)));

        builder.insert_value("print", Value::Cmd(Box::new(Print)));
        builder.insert_value("eprint", Value::Cmd(Box::new(EPrint)));

        builder.insert_value("clone", Value::Cmd(Box::new(Clone)));

        builder.insert_value("object", Value::Cmd(Box::new(MakeObject)));
        builder.insert_value("fset", Value::Cmd(Box::new(FSet)));
        builder.insert_value("fmut", Value::Cmd(Box::new(FMut)));
        builder.insert_value("rmf", Value::Cmd(Box::new(RmField)));

        builder.insert_value("assert", Value::Cmd(Box::new(Assert)));
        builder.insert_value("std", Value::Cmd(Box::new(Std)));
        
        builder
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
        let mut builder = EnvBuilder::std_env();
        builder.insert_value("test", (5.0).into_value());
        let env = builder.build();
        let value = env.get("test").expect("Missing binding \'test\'");
        let borrow = value.borrow();
        assert_eq!((5.0).into_value(), *borrow);
    }
}
