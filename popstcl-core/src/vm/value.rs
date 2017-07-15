use std::fmt;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, Add, Sub, Mul, Div };
use std::borrow::Borrow;
use std::rc::Rc;
use super::internal::{StdObject, Cmd, Env, StdModule};
use ccrc::{Collectable, Ccrc, Tracer};

#[macro_export]
macro_rules! into_value {
    ($val: expr) => ($val.into_value())
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
    ($($x:expr,)*) => (Value::List(List::new(vec![$($x.into_value().into()),*])))
}

#[macro_export]
macro_rules! p_object {
    ( ) => (Object::new());
    ($([$name: expr, $value: expr],)*) => {{
        let mut obj = StdObject::empty();
        $(
            obj.insert($name, $value.into_value().into());
        )*
        Value::Object(obj)
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub struct RcValue(Ccrc<Value>);

impl RcValue {   
    pub fn new(value: Value) -> RcValue {
        RcValue(Ccrc::new(value))
    }

    pub fn inner_clone(&self) -> Value {
        let borrow: &Value = self.0.borrow();
        borrow.clone()
    }
}

impl Collectable for RcValue {
    fn trace(&self, tracer: &Tracer) {
        Collectable::trace(&self.0, tracer)
    }
}

impl fmt::Display for RcValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for RcValue {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Borrow<Value> for RcValue {
    fn borrow(&self) -> &Value {
        &*self.0
    }
}

impl From<Value> for RcValue {
    fn from(val: Value) -> RcValue {
        RcValue(Ccrc::new(val))
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(Number),
    String(PString),
    Bool(Bool),
    Cmd(Box<Cmd>),
    List(List),
    Object(StdObject),
    Module(StdModule),
}

impl Value {
    pub fn try_get_number(&self) -> Option<f64> {
        if let &Value::Number(ref n) = self {
            Some(n.inner())
        } else {
            None
        }
    }

    pub fn try_get_string(&self) -> Option<&PString> {
        if let &Value::String(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn try_get_bool(&self) -> Option<bool> {
        if let &Value::Bool(ref b) = self {
            Some(*b.inner())
        } else {
            None
        }
    }

    pub fn try_get_cmd(&self) -> Option<&Box<Cmd>> {
        if let &Value::Cmd(ref c) = self {
            Some(c)
        } else {
            None
        }
    }

    pub fn try_get_list(&self) -> Option<&List> {
        if let &Value::List(ref l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn try_get_object(&self) -> Option<&StdObject> {
        if let &Value::Object(ref obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    pub fn try_get_mod(&self) -> Option<&StdModule> {
        if let &Value::Module(ref module) = self {
            Some(module)
        } else {
            None
        }
    } 
}

impl Collectable for Value {
    fn trace(&self, tracer: &Tracer) {
        use super::object::Object;

        use self::Value::*;
        match *self {
            Number(_) | String(_) | Bool(_) | Cmd(_) => (),
            Object(ref obj) => Collectable::trace(obj, tracer),
            Module(ref obj) => Collectable::trace(obj, tracer),
            List(ref l) => {
                for item in l.list.borrow().iter() {
                    Collectable::trace(item, tracer);
                }
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use self::Value::*;
        match (self, other) {
            (&Number(ref lhs), &Number(ref rhs)) => lhs == rhs,
            (&String(ref lhs), &String(ref rhs)) => lhs == rhs,
            (&Bool(ref lhs), &Bool(ref rhs)) => lhs == rhs,
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
            &Value::Number(ref num) => write!(f, "Number: {}", num),
            &Value::String(ref s) => write!(f, "String: {}", s),
            &Value::Bool(ref b) => write!(f, "Bool: {}", b),
            &Value::Cmd(_) => write!(f, "CMD: "),
            &Value::List(ref vec) => {
                                        let vec = vec.inner().iter().fold(String::new(), 
                                                                  |mut collect, value|  {
                                                                        collect.push_str(&value.to_string());
                                                                        collect 
                                                                    }
                                                                  );
                                        write!(f, "List: {}", vec)
                                     },
            &Value::Object(_) => write!(f, "OBJ"),      //TODO: better display
            &Value::Module(_) => write!(f, "MODULE"),   //TODO: better display
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Number {
    num: RefCell<f64>,
}

impl Number {
    pub fn new(val: f64) -> Number {
        Number {
            num: RefCell::new(val)
        }
    }

    pub fn set(&self, val: f64) {
        *self.num.borrow_mut() = val;
    }

    pub fn inner(&self) -> f64 {
        self.num.borrow_mut().clone()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.num.borrow())
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        Number::new(*self.num.borrow() + *self.num.borrow())
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        Number::new(*self.num.borrow() - *self.num.borrow())
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        Number::new(*self.num.borrow() / *self.num.borrow())
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        Number::new(*self.num.borrow() * *self.num.borrow())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct PString {
    str: RefCell<String>
}

impl fmt::Display for PString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.str.borrow())
    }
}

impl PString {
    pub fn new(str: String) -> PString {
        PString {
            str: RefCell::new(str)
        }
    }

    pub fn set(&self, str: String) {
        *self.str.borrow_mut() = str;
    }

    pub fn inner(&self) -> Ref<String> {
        self.str.borrow()
    }

    pub fn inner_mut(&self) -> RefMut<String> {
        self.str.borrow_mut()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Bool {
    boolean: RefCell<bool>
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.boolean.borrow())
    }
}

impl Bool {
    pub fn new(b: bool) -> Bool {
        Bool {
            boolean: RefCell::new(b)
        }
    }

    pub fn set(&self, b: bool) {
        *self.boolean.borrow_mut() = b;
    }

    pub fn inner(&self) -> Ref<bool> {
        self.boolean.borrow()
    }

    pub fn inner_mut(&self) -> RefMut<bool> {
        self.boolean.borrow_mut()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    list: RefCell<Vec<RcValue>>
}

impl List {
    pub fn new(l: Vec<RcValue>) -> List {
        List {
            list: RefCell::new(l)
        }
    }

    pub fn set(&self, l: Vec<RcValue>) {
        *self.list.borrow_mut() = l;
    }

    pub fn inner(&self) -> Ref<Vec<RcValue>> {
        self.list.borrow()
    }

    pub fn inner_mut(&self) -> RefMut<Vec<RcValue>> {
        self.list.borrow_mut()
    }

    pub fn len(&self) -> usize {
        self.list.borrow().len()
    }

    pub fn pop(&self) -> Option<RcValue> {
        self.list.borrow_mut().pop()
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Number(Number::new(self))
    }
}

impl IntoValue for Number {
    fn into_value(self) -> Value {
        Value::Number(self)
    }
}

impl IntoValue for PString {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(PString::new(self))
    }
}

impl IntoValue for Bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(Bool::new(self))
    }
}

impl IntoValue for Box<Cmd> {
    fn into_value(self) -> Value {
        Value::Cmd(self)
    }
}

impl IntoValue for List {
    fn into_value(self) -> Value {
        Value::List(self)
    }
}

impl IntoValue for Vec<Value> {
    fn into_value(self) -> Value {
        Value::List(List::new(self.into_iter().map(|val| val.into()).collect::<Vec<RcValue>>()))
    }
}

impl IntoValue for Vec<RcValue> {
    fn into_value(self) -> Value {
        Value::List(List::new(self))
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
    fn value_macro_list() {
        assert_eq!(p_list!(123., true, true,), 
                   (vec![(123.).into_value(), true.into_value(), true.into_value()]).into_value());
    }

    #[test]
    fn value_macro_object() {
        assert_eq!(p_object!(["test", 123.], ["test2", false],),
                   {
                        let mut object = StdObject::empty();
                        object.insert("test", (123.).into_value().into());
                        object.insert("test2", false.into_value().into());
                        Value::Object(object)
                   });
    }
}
