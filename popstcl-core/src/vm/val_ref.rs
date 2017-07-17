use vm::value::*;
use vm::cmd::Cmd;
use vm::internal::{StdObject, StdModule};
use std::ops::{Add, Sub, Mul, Div, Deref, DerefMut};
use std::cell::{Ref, RefMut};

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

impl<'a> Deref for ValueRef<'a> {
    type Target = Value;
    fn deref(&self) -> &Value {
        &*self.borrow
    }
}

pub struct NumberRef<'a> {
    borrow: Ref<'a, Value>
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

impl<'a> Add for NumberRef<'a> {
    type Output = f64;

    fn add(self, other: NumberRef) -> Self::Output {
        &*self + &*other
    }
}

impl<'a> Sub for NumberRef<'a> {
    type Output = f64;

    fn sub(self, other: NumberRef) -> Self::Output {
        &*self - &*other
    }
}

impl<'a> Div for NumberRef<'a> {
    type Output = f64;

    fn div(self, other: NumberRef) -> Self::Output {
        &*self / &*other
    }
}

impl<'a> Mul for NumberRef<'a> {
    type Output = f64;

    fn mul(self, other: NumberRef) -> Self::Output {
        &*self * &*other
    }
}

pub struct StringRef<'a> {
    borrow: Ref<'a, Value>
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

pub struct BoolRef<'a> {
    borrow: Ref<'a, Value>
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

pub struct CmdRef<'a> {
    borrow: Ref<'a, Value>
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

pub struct ListRef<'a> {
    borrow: Ref<'a, Value>
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

pub struct ObjectRef<'a> {
    borrow: Ref<'a, Value>
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

pub struct ModuleRef<'a> {
    borrow: Ref<'a, Value>
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

pub struct NumberRefMut<'a> {
    borrow: RefMut<'a, Value>
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

impl<'a> Add for NumberRefMut<'a> {
    type Output = f64;

    fn add(self, other: NumberRefMut) -> Self::Output {
        &*self + &*other
    }
}

impl<'a> Sub for NumberRefMut<'a> {
    type Output = f64;

    fn sub(self, other: NumberRefMut) -> Self::Output {
        &*self - &*other
    }
}

impl<'a> Div for NumberRefMut<'a> {
    type Output = f64;

    fn div(self, other: NumberRefMut) -> Self::Output {
        &*self / &*other
    }
}

impl<'a> Mul for NumberRefMut<'a> {
    type Output = f64;

    fn mul(self, other: NumberRefMut) -> Self::Output {
        &*self * &*other
    }
}

pub struct StringRefMut<'a> {
    borrow: RefMut<'a, Value>
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

pub struct BoolRefMut<'a> {
    borrow: RefMut<'a, Value>
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

pub struct CmdRefMut<'a> {
    borrow: RefMut<'a, Value>
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

pub struct ListRefMut<'a> {
    borrow: RefMut<'a, Value>
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

pub struct ObjectRefMut<'a> {
    borrow: RefMut<'a, Value>
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

pub struct ModuleRefMut<'a> {
    borrow: RefMut<'a, Value>
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
