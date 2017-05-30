use super::internal::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    bindings: HashMap<String, EnvEntry>,
}

impl Env {
    pub fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    pub fn insert(&mut self, name: &str, value: Value, permissions: EntryPermissions) {
        self.bindings.insert(name.to_string(), EnvEntry::new(value, permissions));
    }

    pub fn insert_entry(&mut self, name: &str, entry: EnvEntry) {
        self.bindings.insert(name.to_string(), entry);
    }

    pub fn get(&self, name: &str) -> Option<&EnvEntry> {
        self.bindings.get(name)
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
                   observable_internal!(),
                   );
        let obj_1 = {
            let mut obj = StdObject::empty();
            obj.insert("foo", Value::Number(1337_f64), all_read_write!());
            obj.insert("bar", Value::Number(-1_f64), all_read_write!());
            Value::Object(obj)
        };
        env.bindings
            .insert("obj".to_string(), EnvEntry::observable_internal(obj_1));

        let program = parse_program("
gset a @obj.foo;
gset b @obj.bar;")
                .unwrap();
        let mut temp_mod = InternalModule::new(env);
        eval_program(&mut Stack::new_module(&mut temp_mod), &program).unwrap();
        

        let inspecting = vec![("a", Value::Number(1337_f64)), ("b", Value::Number(-1_f64))];

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
                   observable_internal!(),
                   );
        let obj_1 = {
            let mut obj = StdObject::empty();
            obj.insert("foo", Value::Number(1337_f64), all_read_write!());
            obj.insert("bar", Value::Number(-1_f64), all_read_write!());
            obj
        };
        let obj_2 = {
            let mut obj = StdObject::empty();
            obj.insert("nested", Value::Object(obj_1), all_read_write!());
            Value::Object(obj)
        };
        env.bindings
            .insert("obj".to_string(), EnvEntry::observable_internal(obj_2));

        let program = parse_program("
gset a @obj.nested.foo;
gset b @obj.nested.bar;")
                .unwrap();

        let mut temp_mod = InternalModule::new(env);
        eval_program(&mut Stack::new_module(&mut temp_mod), &program).unwrap();
        

        let inspecting = vec![("a", Value::Number(1337_f64)), ("b", Value::Number(-1_f64))];
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
