use vm::internal::{Value, StdModule, DebugInfo, DebugKind};
use ast::*;

use std::fmt;

/// Command Intermediate Representation
#[derive(Clone, Debug)]
pub struct CIR {
    pub value: Value,
    pub dinfo: DebugInfo,
}

impl CIR {
    
    pub fn new(value: Value, dinfo: DebugInfo) -> CIR {
        CIR { value: value, dinfo: dinfo }
    }

    pub fn try_get_number(&self) -> Option<f64> {
        if let Value::Number(n) = self.value {
            Some(n)
        } else {
            None
        }
    }

    pub fn try_get_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self.value {
            Some(b)
        } else {
            None
        }
    }

	pub fn try_get_mod(&self) -> Option<StdModule> {
		if let Value::Module(ref module) = self.value {
			Some(module.clone())
		} else {
			None
		}
	} 

    pub fn try_get_list(&self) -> Option<&Vec<Value>> {
        if let Value::List(ref vec) = self.value {
            Some(vec)
        } else {
            None
        }
    }

    pub fn try_get_string(&self) -> Option<&str> {
        if let Value::String(ref s) = self.value {
            Some(s)
        } else {
            None
        }
    }

    pub fn clone_value(&self) -> Value {
        self.value.clone()
    }
}

impl PartialEq for CIR {
    fn eq(&self, other: &CIR) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for CIR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[macro_export]
macro_rules! cir_extract {

    ($cir: expr => Bool) => {
        cir_extract!($cir => Number, "Bool")
    };

    ($cir: expr => Bool, $expect: expr) => {
        {
            $cir.try_get_bool().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => Number) => {
        cir_extract!($cir => Number, "Number")
    };

    ($cir: expr => Number, $expect: expr) => {
        {
            $cir.try_get_number().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => String) => {
        cir_extract!($cir => String, "String")
    };

    ($cir: expr => String, $expect: expr) => {
        {
            $cir.try_get_string().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => List) => {
        cir_extract!($cir => List, "List")
    };

    ($cir: expr => List, $expect: expr) => {
        {
            $cir.try_get_list().ok_or(ExecErr::InvalidArg {
                                        expect: $expect.to_string(),
                                        found: $cir.clone(),
            })
        }
    };

    ($cir: expr => Module) => {
        cir_extract!($cir => Module, "Module")
    };

    ($cir: expr => Module, $expect: expr) => {
        {
            $cir.try_get_mod().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };
}
