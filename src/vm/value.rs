use std::fmt;
use super::internal::{StdObject, Cmd, Env, StdModule};

#[macro_export]
macro_rules! into_value {
    ($val: expr) => ($val.into_value())
}

#[macro_export]
macro_rules! p_number {
    ($num: expr) => {
        {
            Value::Number($num)
        }
    }
}

#[macro_export]
macro_rules! p_string {
    ($string: expr) => {
        {
            Value::String($string)
        }
    }
}

#[macro_export]
macro_rules! p_bool {
    ($bool: expr) => {
        {
            Value::Bool($bool)
        }
    }
}

#[macro_export]
macro_rules! p_cmd {
    ($cmd: expr) => {
        {
            Value::Cmd($cmd)
        }
    }
}

#[macro_export]
macro_rules! p_list {
    ($($x:expr,)*) => (Value::List(vec![$($x.into_value()),*]))
}

#[macro_export]
macro_rules! p_object {
    ( ) => (Object::new());
    ($([$name: expr, $value: expr, $permissions: expr],)*) => {{
        let mut obj = StdObject::empty();
        $(
            obj.insert($name, $value.into_value(), $permissions);
        )*
        Value::Object(obj)
    }};
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Cmd(Box<Cmd>),
    List(Vec<Value>),
    Object(StdObject),
    Module(StdModule),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use self::Value::*;
        match (self, other) {
            (&Number(lhs), &Number(rhs)) => lhs == rhs,
            (&String(ref lhs), &String(ref rhs)) => lhs == rhs,
            (&Bool(lhs), &Bool(rhs)) => lhs == rhs,
            (&Cmd(ref lhs), &Cmd(ref rhs)) => unimplemented!(),     //TODO: add fn to Cmd which allows differentiation of Cmd implementations
            (&List(ref lhs), &List(ref rhs)) => lhs == rhs,
            (&Object(ref lhs), &Object(ref rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Number(num) => write!(f, "Number: {}", num),
            &Value::String(ref s) => write!(f, "String: {}", s),
            &Value::Bool(b) => write!(f, "Bool: {}", b),
            &Value::Cmd(_) => write!(f, "CMD: "),
            &Value::List(ref vec) => {
                                        let vec = vec.iter().fold(String::new(), 
                                                                  |mut collect, value|  {
                                                                        collect.push_str(&value.to_string());
                                                                        collect 
                                                                    }
                                                                  );
                                        write!(f, "List: {}", vec)
                                     },
            &Value::Object(_) => write!(f, "OBJ"),
            &Value::Module(_) => unimplemented!(),
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Number(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for Box<Cmd> {
    fn into_value(self) -> Value {
        Value::Cmd(self)
    }
}

impl IntoValue for Vec<Value> {
    fn into_value(self) -> Value {
        Value::List(self)
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm::internal::*;

    #[test]
    fn value_macro_bool() {
        assert_eq!(p_bool!(true), Value::Bool(true));
    }

    #[test]
    fn value_macro_number() {
        assert_eq!(p_number!(1337.7), Value::Number(1337.7));
    }

    #[test]
    fn value_macro_string() {
        assert_eq!(p_string!("YOLO".to_string()), 
                             Value::String("YOLO".to_string()));
    }

    #[test]
    fn value_macro_list() {
        assert_eq!(p_list!(123., true, true,), 
                   Value::List(vec![Value::Number(123.), Value::Bool(true), Value::Bool(true)]));
    }

    #[test]
    fn value_macro_object() {
        assert_eq!(p_object!(["test", 123., all_read_write!()], ["test2", false, all_read_write!()],),
                   {
                        let mut object = StdObject::empty();
                        object.insert("test", Value::Number(123.), all_read_write!());
                        object.insert("test2", Value::Bool(false), all_read_write!());
                        Value::Object(object)
                   });
    }
}
