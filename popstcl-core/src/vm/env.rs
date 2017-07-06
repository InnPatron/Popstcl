use super::internal::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    bindings: HashMap<String, RcValue>,
}

impl Env {
    pub fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    pub fn insert(&mut self, name: &str, value: Value) {
        self.bindings.insert(name.to_string(), RcValue::new(value));
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.bindings.get(name).map(|rc_value| rc_value.inner_clone())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn object_walk() {
        use parser::parse_program;
        use vm::internal::*;
        use cmds::*;
        let mut env = Env::new();
        env.insert("gset", 
                   Value::Cmd(Box::new(Set(Namespace::Module))),
                   );
        let obj_1 = {
            let mut obj = StdObject::empty();
            obj.insert("foo", (1337_f64).into_value());
            obj.insert("bar", (-1_f64).into_value());
            Value::Object(obj)
        };
        env.insert("obj", obj_1);

        let program = parse_program("
gset a @obj.foo;
gset b @obj.bar;")
                .unwrap();
        let mut temp_mod = StdModule::new(env);
        eval_program(&mut Stack::new_module(&mut temp_mod), &program).unwrap();
        

        let inspecting = vec![("a", (1337_f64).into_value()), ("b", (-1_f64).into_value())];

        for pair in inspecting.iter() {
            match temp_mod.get(pair.0) {
                Ok(val) => {
                    assert_eq!(pair.1, val);
                }

                Err(e @ _) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn nested_object_walk() {
        use vm::internal::*;
        use parser::parse_program;
        use cmds::*;

        let mut env = Env::new();
        env.insert("gset", 
                   Value::Cmd(Box::new(Set(Namespace::Module))),
                   );
        let obj_1 = {
            let mut obj = StdObject::empty();
            obj.insert("foo", (1337_f64).into_value());
            obj.insert("bar", (-1_f64).into_value());
            obj
        };
        let obj_2 = {
            let mut obj = StdObject::empty();
            obj.insert("nested", Value::Object(obj_1));
            Value::Object(obj)
        };
        env.insert("obj", obj_2);

        let program = parse_program("
gset a @obj.nested.foo;
gset b @obj.nested.bar;")
                .unwrap();

        let mut temp_mod = StdModule::new(env);
        eval_program(&mut Stack::new_module(&mut temp_mod), &program).unwrap();
        

        let inspecting = vec![("a", (1337_f64).into_value()), ("b", (-1_f64).into_value())];
        for pair in inspecting.iter() {
            match temp_mod.get(pair.0) {
                Ok(val) => {
                    assert_eq!(pair.1, val);
                }

                Err(err @ _) => panic!("{:?}", err),
            }
        }
    }
}
