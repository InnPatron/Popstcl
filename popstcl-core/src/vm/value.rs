use std::fmt;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut, Add, Sub, Mul, Div };
use std::borrow::Borrow;
use std::rc::Rc;
use super::internal::{StdObject, Cmd, Env, StdModule};
use super::val_ref::*;
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
pub struct RcValue(Ccrc<RefCell<Value>>);

impl RcValue {   
    pub fn new(value: Value) -> RcValue {
        RcValue(Ccrc::new(RefCell::new(value)))
    }

    pub fn borrow(&self) -> ValueRef {
        ValueRef::new((*self.0).borrow())
    }

    pub fn borrow_mut(&self) -> ValueRefMut {
        ValueRefMut::new((*self.0).borrow_mut())
    }

    pub fn inner_clone(&self) -> Value {
        let borrow = (*self.0).borrow();
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
        (*self.0).borrow().fmt(f)
    }
}

impl From<Value> for RcValue {
    fn from(val: Value) -> RcValue {
        RcValue(Ccrc::new(RefCell::new(val)))
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

    pub fn is_cmd(&self) -> bool {
        match self {
            &Value::Cmd(_) => true,
            _ => false,
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
                for item in l.list.iter() {
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
    num: f64,
}

impl Number {
    pub fn new(val: f64) -> Number {
        Number {
            num: val 
        }
    }

    pub fn set(&mut self, val: f64) {
        self.num = val;
    }

    pub fn inner(&self) -> f64 {
        self.num.clone()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl Deref for Number {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.num
    }
}

impl DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.num
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        Number::new(self.num + self.num)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        Number::new(self.num - self.num)
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        Number::new(self.num / self.num)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        Number::new(self.num * self.num)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct PString {
    str: String
}

impl fmt::Display for PString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl PString {
    pub fn new(str: String) -> PString {
        PString {
            str: str
        }
    }

    pub fn set(&mut self, str: String) {
        self.str = str;
    }

    pub fn inner(&self) -> &String {
        &self.str
    }

    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.str
    }
}

impl Deref for PString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.str
    }
}

impl DerefMut for PString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.str
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Bool {
    boolean: bool 
}

impl Bool {
    pub fn new(b: bool) -> Bool {
        Bool {
            boolean: b
        }
    }

    pub fn set(&mut self, b: bool) {
        self.boolean = b;
    }

    pub fn inner(&self) ->  bool {
        self.boolean
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.boolean) 
    }
}

impl Deref for Bool {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.boolean
    }
}

impl DerefMut for Bool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boolean
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    list: Vec<RcValue>
}

impl List {
    pub fn new(l: Vec<RcValue>) -> List {
        List {
            list: l
        }
    }

    pub fn set(&mut self, l: Vec<RcValue>) {
        self.list = l;
    }

    pub fn inner(&self) -> &Vec<RcValue> {
        &self.list
    }

    pub fn inner_mut(&mut self) -> &mut Vec<RcValue> {
        &mut self.list
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn pop(&mut self) -> Option<RcValue> {
        self.list.pop()
    }
}

impl Deref for List {
    type Target = Vec<RcValue>;
    fn deref(&self) -> &Vec<RcValue> {
        &self.list
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
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

pub trait DeepClone: Clone {
    fn deep_clone(&self) -> Self;
}

impl DeepClone for RcValue {
    fn deep_clone(&self) -> Self {
        let value: Value = self.borrow().deep_clone();
        RcValue::new(value)
    }
}

impl DeepClone for Value {
    fn deep_clone(&self) -> Self {
        use self::Value::*;
        match *self {
            Number(ref n) => Value::Number(n.clone()),
            String(ref s) => Value::String(s.clone()),
            Bool(ref b) => Value::Bool(b.clone()),
            Cmd(ref c) => Value::Cmd(c.clone()),
            List(ref l) => Value::List(l.deep_clone()),
            Object(ref o) => Value::Object(o.deep_clone()),
            Module(ref m) => Value::Module(m.deep_clone()),
        }
    }
}

impl DeepClone for List {
    fn deep_clone(&self) -> Self {
        let mut l = Vec::new();
        for item in self.list.iter() {
            l.push(item.deep_clone());
        }
        List { list: l }
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
