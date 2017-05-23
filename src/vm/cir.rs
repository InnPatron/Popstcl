use vm::internal::{Value, Cmd, ObjectKind, Env, Module};
use ast::*;

use std::collections::HashMap;
use std::fmt;

/// Command Intermediate Representation
#[derive(Clone, Debug)]
pub enum CIR {
    Single(String),
    Untouched(String),
    Value(Value),
}

impl CIR {
    pub fn try_from(w: &WordKind) -> Option<CIR> {
        match w {
            &WordKind::Atom(ref s) => Some(CIR::Single(s.0.clone())),
            &WordKind::Number(n) => Some(CIR::Value(Value::Number(n))),
            &WordKind::Bool(b) => Some(CIR::Value(Value::Bool(b))),
            &WordKind::Untouched(ref s) => Some(CIR::Untouched(s.clone())),
            _ => None,
        }
    }

    pub fn try_to_value(&self) -> Option<Value> {
        match self {
            &CIR::Single(_) => None,
            &CIR::Untouched(_) => None,
            &CIR::Value(ref v) => Some(v.clone()),
        }
    }

    pub fn try_get_single(&self) -> Option<&str> {
        if let &CIR::Single(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn try_get_number(&self) -> Option<f64> {
        if let &CIR::Value(Value::Number(n)) = self{
            Some(n)
        } else {
            None
        }
    }

    pub fn try_get_untouched(&self) -> Option<&str> {
        if let &CIR::Untouched(ref s) = self{
            Some(s)
        } else {
            None
        }
    }

	pub fn try_get_mod(&self) -> Option<&Module> {
		if let &CIR::Value(Value::Module(ref module)) = self {
			Some(module)
		} else {
			None
		}
	} 

    pub fn try_get_list(&self) -> Option<&Vec<Value>> {
        if let &CIR::Value(Value::List(ref vec)) = self {
            Some(vec)
        } else {
            None
        }
    }

    pub fn try_get_string(&self) -> Option<&str> {
        if let &CIR::Value(Value::String(ref s)) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &CIR::Single(ref s) => s.to_owned(),
            &CIR::Untouched(ref s) => format!("{{{}}}", s).to_owned(),
            &CIR::Value(ref v) => v.to_string(),
        }
    }

    pub fn is_single(&self) -> bool {
        if let &CIR::Single(_) = self {
            true
        } else {
            false
        }
    }
    
    /// Return true if matching enum Value.
    /// TODO: Should Value be its own invariant to make conversion / checking / adding more values
    /// easier?
    pub fn is_value(&self) -> bool {
        match self {
            &CIR::Value(_) => true,
            _ => false,
        }
    }
}

impl From<Value> for CIR {
    fn from(val: Value) -> CIR {
        CIR::Value(val)
    }
}

impl PartialEq for CIR {
    fn eq(&self, other: &CIR) -> bool {
        use self::CIR::*;
        match (self, other) {
            (&Single(ref lhs), &Single(ref rhs)) => lhs == rhs,
            (&Untouched(ref lhs), &Untouched(ref rhs)) => lhs == rhs,
            (&Value(ref lhs), &Value(ref rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl fmt::Display for CIR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CIR::Single(ref s) => write!(f, "Atom: {}", s),
            &CIR::Untouched(ref s) => write!(f, "{{{}}}", s),
            &CIR::Value(ref v) => write!(f, "{}", v),
        }
    }
}
