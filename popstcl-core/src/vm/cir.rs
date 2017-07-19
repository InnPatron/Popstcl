use vm::internal::{Value, RcValue, StdModule, DebugInfo, DebugKind};
use ast::*;

use std::fmt;
use std::cell::Ref;
use std::rc::Weak;

/// Command Intermediate Representation
#[derive(Clone, Debug)]
pub struct CIR {
    pub value: RcValue,
    pub dinfo: DebugInfo,
}

impl CIR {
    
    pub fn new(value: RcValue, dinfo: DebugInfo) -> CIR {
        CIR { value: value, dinfo: dinfo }
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
            $cir.value.borrow().try_into_number().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => mut Number) => {
        cir_extract!($cir => mut Number, "Number")
    };

    ($cir: expr => mut Number, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_number().map_err(|_|ExecErr::InvalidArg {
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
            $cir.value.borrow().try_into_string().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => mut String) => {
        cir_extract!($cir => mut String, "String")
    };

    ($cir: expr => mut String, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_string().map_err(|_|ExecErr::InvalidArg {
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
            $cir.value.borrow().try_into_bool().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };
    
    ($cir: expr => mut Bool) => {
        cir_extract!($cir => mut Bool, "Bool")
    };

    ($cir: expr => mut Bool, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_bool().map_err(|_|ExecErr::InvalidArg {
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
            $cir.value.borrow().try_into_cmd().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => mut Cmd) => {
        cir_extract!($cir => mut Cmd, "Cmd")
    };

    ($cir: expr => mut Cmd, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_cmd().map_err(|_|ExecErr::InvalidArg {
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
            $cir.value.borrow().try_into_list().map_err(|_|ExecErr::InvalidArg {
                                        expect: $expect.to_string(),
                                        found: $cir.clone(),
            })
        }
    };

    ($cir: expr => mut List) => {
        cir_extract!($cir => mut List, "List")
    };

    ($cir: expr => mut List, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_list().map_err(|_|ExecErr::InvalidArg {
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
            $cir.value.borrow().try_into_object().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => mut Object) => {
        cir_extract!($cir => mut Object, "Object")
    };

    ($cir: expr => mut Object, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_object().map_err(|_|ExecErr::InvalidArg {
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
            $cir.value.borrow().try_into_module().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };

    ($cir: expr => mut Module) => {
        cir_extract!($cir => mut Module, "Module")
    };

    ($cir: expr => mut Module, $expect: expr) => {
        {
            $cir.value.borrow_mut().try_into_module().map_err(|_|ExecErr::InvalidArg {
                           expect: $expect.to_string(),
                           found: $cir.clone(),
            })
        }
    };
}
