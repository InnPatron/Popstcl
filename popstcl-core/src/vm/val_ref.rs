use vm::value::*;
use vm::cmd::Cmd;
use vm::internal::{StdObject, StdModule};
use std::ops::{Add, Sub, Mul, Div, Deref, DerefMut};
use std::borrow::Borrow;
use std::cell::{Ref, RefMut};
use std::fmt;

#[derive(Debug)]
pub struct ValueRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> ValueRef<'a> {

    pub fn new(borrow: Ref<'a, Value>) -> ValueRef<'a> {
        ValueRef {
            borrow: borrow
        }
    }

    pub fn try_into_number(self) -> Result<NumberRef<'a>, Self> {
        if let &Value::Number(_) = &*self.borrow {
            Ok(NumberRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_string(self) -> Result<StringRef<'a>, Self> {
        if let &Value::String(_) = &*self.borrow {
            Ok(StringRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_bool(self) -> Result<BoolRef<'a>, Self> {
        if let &Value::Bool(_) = &*self.borrow {
            Ok(BoolRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_cmd(self) -> Result<CmdRef<'a>, Self> {
        if let &Value::Cmd(_) = &*self.borrow {
            Ok(CmdRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_list(self) -> Result<ListRef<'a>, Self> {
        if let &Value::List(_) = &*self.borrow {
            Ok(ListRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_object(self) -> Result<ObjectRef<'a>, Self> {
        if let &Value::Object(_) = &*self.borrow {
            Ok(ObjectRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_module(self) -> Result<ModuleRef<'a>, Self> {
        if let &Value::Module(_) = &*self.borrow {
            Ok(ModuleRef { borrow: self.borrow })
        } else {
            Err(self)
        }
    }
}

impl<'a> fmt::Display for ValueRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ValueRef<'a> {
    type Target = Value;
    fn deref(&self) -> &Value {
        &*self.borrow
    }
}

impl<'a> PartialEq for ValueRef<'a> {
    fn eq(&self, other: &ValueRef) -> bool {
        *self.borrow == *other.borrow
    }
}

impl<'a> PartialEq<RcValue> for ValueRef<'a> {
    fn eq(&self, other: &RcValue) -> bool {
        *self.borrow == *other.borrow()
    }
}

#[derive(Debug)]
pub struct NumberRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for NumberRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Borrow<f64> for NumberRef<'a> {
    fn borrow(&self) -> &f64 {
        &**self
    }
}

impl<'a> Deref for NumberRef<'a> {
    type Target = f64;
    fn deref(&self) -> &f64 {
        if let &Value::Number(ref v) = &*self.borrow {
            &*v
        } else {
            panic!("{} is not Value::Number", &*self.borrow);
        }
    }
}

impl<'a, T> PartialEq<T> for NumberRef<'a> where T: Borrow<f64> {
    fn eq(&self, other: &T) -> bool {
        &**self == other.borrow()
    }
}

impl<'a, T> Add<T> for NumberRef<'a> where T: Borrow<f64> {
    type Output = f64;

    fn add(self, other: T) -> Self::Output {
        *self + *other.borrow()
    }
}

impl<'a, T> Sub<T> for NumberRef<'a> where T: Borrow<f64>{
    type Output = f64;

    fn sub(self, other: T) -> Self::Output {
        *self - *other.borrow()
    }
}

impl<'a, T> Div<T> for NumberRef<'a> where T: Borrow<f64>{
    type Output = f64;

    fn div(self, other: T) -> Self::Output {
        *self / *other.borrow()
    }
}

impl<'a, T> Mul<T> for NumberRef<'a> where T: Borrow<f64>{
    type Output = f64;

    fn mul(self, other: T) -> Self::Output {
        *self * *other.borrow()
    }
}

#[derive(Debug)]
pub struct StringRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for StringRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Borrow<str> for StringRef<'a> {
    fn borrow(&self) -> &str {
        &**self
    }
}

impl<'a> Deref for StringRef<'a> {
    type Target = PString;
    fn deref(&self) -> &PString {
        if let &Value::String(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::PString", &*self.borrow);
        }
    }
}

impl<'a, T> PartialEq<T> for StringRef<'a> where T: Borrow<str> {
    fn eq(&self, other: &T) -> bool {
        let lhs: &str = self.borrow();
        lhs == other.borrow()
    }
}

#[derive(Debug)]
pub struct BoolRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for BoolRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Borrow<bool> for BoolRef<'a> {
    fn borrow(&self) -> &bool {
        &**self
    }
}

impl<'a> Deref for BoolRef<'a> {
    type Target = Bool;
    fn deref(&self) -> &Bool {
        if let &Value::Bool(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Bool", &*self.borrow);
        }
    }
}

impl<'a, T> PartialEq<T> for BoolRef<'a> where T: Borrow<bool> {
    fn eq(&self, other: &T) -> bool {
        **self == *other.borrow()
    }
}

#[derive(Debug)]
pub struct CmdRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for CmdRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for CmdRef<'a> {
    type Target = Box<Cmd>;
    fn deref(&self) -> &Box<Cmd> {
        if let &Value::Cmd(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Cmd", &*self.borrow);
        }
    }
}

#[derive(Debug)]
pub struct ListRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for ListRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ListRef<'a> {
    type Target = List;
    fn deref(&self) -> &List {
        if let &Value::List(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::List", &*self.borrow);
        }
    }
}

impl<'a> PartialEq for ListRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<'a> PartialEq<List> for ListRef<'a> {
    fn eq(&self, other: &List) -> bool {
        *self == *other
    }
}

#[derive(Debug)]
pub struct ObjectRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for ObjectRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ObjectRef<'a> {
    type Target = StdObject;
    fn deref(&self) -> &StdObject {
        if let &Value::Object(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Object", &*self.borrow);
        }
    }
}

impl<'a> PartialEq for ObjectRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<'a> PartialEq<StdObject> for ObjectRef<'a> {
    fn eq(&self, other: &StdObject) -> bool {
        *self == *other
    }
}

#[derive(Debug)]
pub struct ModuleRef<'a> {
    borrow: Ref<'a, Value>
}

impl<'a> fmt::Display for ModuleRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ModuleRef<'a> {
    type Target = StdModule;
    fn deref(&self) -> &StdModule {
        if let &Value::Module(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Module", &*self.borrow);
        }
    }
}

impl<'a> PartialEq for ModuleRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<'a> PartialEq<StdModule> for ModuleRef<'a> {
    fn eq(&self, other: &StdModule) -> bool {
        *self == *other
    }
}

#[derive(Debug)]
pub struct ValueRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> ValueRefMut<'a> {

    pub fn new(borrow: RefMut<'a, Value>) -> ValueRefMut<'a> {
        ValueRefMut {
            borrow: borrow
        }
    }

    pub fn try_into_number(self) -> Result<NumberRefMut<'a>, Self> {
        if let &Value::Number(_) = &*self.borrow {
            Ok(NumberRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_string(self) -> Result<StringRefMut<'a>, Self> {
        if let &Value::String(_) = &*self.borrow {
            Ok(StringRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_bool(self) -> Result<BoolRefMut<'a>, Self> {
        if let &Value::Bool(_) = &*self.borrow {
            Ok(BoolRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_cmd(self) -> Result<CmdRefMut<'a>, Self> {
        if let &Value::Cmd(_) = &*self.borrow {
            Ok(CmdRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_list(self) -> Result<ListRefMut<'a>, Self> {
        if let &Value::List(_) = &*self.borrow {
            Ok(ListRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_object(self) -> Result<ObjectRefMut<'a>, Self> {
        if let &Value::Object(_) = &*self.borrow {
            Ok(ObjectRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }

    pub fn try_into_module(self) -> Result<ModuleRefMut<'a>, Self> {
        if let &Value::Module(_) = &*self.borrow {
            Ok(ModuleRefMut { borrow: self.borrow })
        } else {
            Err(self)
        }
    }
}

impl<'a> fmt::Display for ValueRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ValueRefMut<'a> {
    type Target = Value;
    fn deref(&self) -> &Value {
        &*self.borrow
    }
}

impl<'a> DerefMut for ValueRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.borrow
    }
}

impl<'a> PartialEq for ValueRefMut<'a> {
    fn eq(&self, other: &ValueRefMut) -> bool {
        *self.borrow == *other.borrow
    }
}

impl<'a> PartialEq<RcValue> for ValueRefMut<'a> {
    fn eq(&self, other: &RcValue) -> bool {
        *self.borrow == *other.borrow()
    }
}

#[derive(Debug)]
pub struct NumberRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for NumberRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Borrow<f64> for NumberRefMut<'a> {
    fn borrow(&self) -> &f64 {
        &**self
    }
}

impl<'a> Deref for NumberRefMut<'a> {
    type Target = f64;
    fn deref(&self) -> &f64 {
        if let &Value::Number(ref v) = &*self.borrow {
            &*v
        } else {
            panic!("{} is not Value::Number", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for NumberRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::Number(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::Number");
        }
    }
}

impl<'a, T> PartialEq<T> for NumberRefMut<'a> where T: Borrow<f64> {
    fn eq(&self, other: &T) -> bool {
        &**self == other.borrow()
    }
}

impl<'a, T> Add<T> for NumberRefMut<'a> where T: Borrow<f64> {
    type Output = f64;

    fn add(self, other: T) -> Self::Output {
        *self + *other.borrow()
    }
}

impl<'a, T> Sub<T> for NumberRefMut<'a> where T: Borrow<f64>{
    type Output = f64;

    fn sub(self, other: T) -> Self::Output {
        *self - *other.borrow()
    }
}

impl<'a, T> Div<T> for NumberRefMut<'a> where T: Borrow<f64>{
    type Output = f64;

    fn div(self, other: T) -> Self::Output {
        *self / *other.borrow()
    }
}

impl<'a, T> Mul<T> for NumberRefMut<'a> where T: Borrow<f64>{
    type Output = f64;

    fn mul(self, other: T) -> Self::Output {
        *self * *other.borrow()
    }
}

#[derive(Debug)]
pub struct StringRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for StringRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Borrow<str> for StringRefMut<'a> {
    fn borrow(&self) -> &str {
        &**self
    }
}

impl<'a> Deref for StringRefMut<'a> {
    type Target = PString;
    fn deref(&self) -> &PString {
        if let &Value::String(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::PString", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for StringRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::String(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::String");
        }
    }
}

impl<'a, T> PartialEq<T> for StringRefMut<'a> where T: Borrow<str> {
    fn eq(&self, other: &T) -> bool {
        let lhs: &str = self.borrow();
        lhs == other.borrow()
    }
}

#[derive(Debug)]
pub struct BoolRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for BoolRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Borrow<bool> for BoolRefMut<'a> {
    fn borrow(&self) -> &bool {
        &**self
    }
}

impl<'a> Deref for BoolRefMut<'a> {
    type Target = Bool;
    fn deref(&self) -> &Bool {
        if let &Value::Bool(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Bool", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for BoolRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::Bool(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::Bool");
        }
    }
}

impl<'a, T> PartialEq<T> for BoolRefMut<'a> where T: Borrow<bool> {
    fn eq(&self, other: &T) -> bool {
        **self == *other.borrow()
    }
}

#[derive(Debug)]
pub struct CmdRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for CmdRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for CmdRefMut<'a> {
    type Target = Box<Cmd>;
    fn deref(&self) -> &Box<Cmd> {
        if let &Value::Cmd(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Cmd", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for CmdRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::Cmd(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::Cmd");
        }
    }
}

#[derive(Debug)]
pub struct ListRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for ListRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ListRefMut<'a> {
    type Target = List;
    fn deref(&self) -> &List {
        if let &Value::List(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::List", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for ListRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::List(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::List");
        }
    }
}

impl<'a> PartialEq for ListRefMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<'a> PartialEq<List> for ListRefMut<'a> {
    fn eq(&self, other: &List) -> bool {
        *self == *other
    }
}

#[derive(Debug)]
pub struct ObjectRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for ObjectRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ObjectRefMut<'a> {
    type Target = StdObject;
    fn deref(&self) -> &StdObject {
        if let &Value::Object(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Object", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for ObjectRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::Object(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::Object");
        }
    }
}

impl<'a> PartialEq for ObjectRefMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<'a> PartialEq<StdObject> for ObjectRefMut<'a> {
    fn eq(&self, other: &StdObject) -> bool {
        *self == *other
    }
}

#[derive(Debug)]
pub struct ModuleRefMut<'a> {
    borrow: RefMut<'a, Value>
}

impl<'a> fmt::Display for ModuleRefMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.borrow.fmt(fmt)
    }
}

impl<'a> Deref for ModuleRefMut<'a> {
    type Target = StdModule;
    fn deref(&self) -> &StdModule {
        if let &Value::Module(ref v) = &*self.borrow {
            v
        } else {
            panic!("{} is not Value::Module", &*self.borrow);
        }
    }
}

impl<'a> DerefMut for ModuleRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let &mut Value::Module(ref mut v) = &mut *self.borrow {
            &mut *v
        } else {
            panic!("Not Value::Module");
        }
    }
}

impl<'a> PartialEq for ModuleRefMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<'a> PartialEq<StdModule> for ModuleRefMut<'a> {
    fn eq(&self, other: &StdModule) -> bool {
        *self == *other
    }
}
