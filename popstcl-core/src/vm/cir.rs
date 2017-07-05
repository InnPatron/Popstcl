use vm::internal::{Value, StdModule, DebugInfo, DebugKind};
use ast::*;

use std::fmt;
use std::cell::Ref;
use std::rc::Weak;

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

    ($cir: expr => Number) => {
        cir_extract!($cir => Number, "Number")
    };

    ($cir: expr => Number, $expect: expr) => {
        {
            $cir.value.try_get_number().ok_or(ExecErr::InvalidArg {
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
            $cir.value.try_get_string().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => Bool) => {
        cir_extract!($cir => Bool, "Bool")
    };

    ($cir: expr => Bool, $expect: expr) => {
        {
            $cir.value.try_get_bool().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => Cmd) => {
        cir_extract!($cir => Cmd, "Cmd")
    };

    ($cir: expr => Cmd, $expect: expr) => {
        {
            $cir.value.try_get_cmd().ok_or(ExecErr::InvalidArg {
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
            $cir.value.try_get_list().ok_or(ExecErr::InvalidArg {
                                        expect: $expect.to_string(),
                                        found: $cir.clone(),
            })
        }
    };

    ($cir: expr => Object) => {
        cir_extract!($cir => Object, "Object")
    };

    ($cir: expr => Object, $expect: expr) => {
        {
            $cir.value.try_get_object().ok_or(ExecErr::InvalidArg {
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
            $cir.value.try_get_mod().ok_or(ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => Ref) => {
        cir_extract!($cir => Ref, "Reference")
    };

    ($cir: expr => Ref, $expect: expr) => {
        {
            $cir.value.try_get_ref().ok_or(ExecErr::InvalidArg {
                                     expect: $expect.to_string(),
                                     found: $cir.clone()
            })
        }
    }
}
