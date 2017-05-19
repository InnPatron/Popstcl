#![macro_escape]

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! get_module {
        ($namespace: expr, $stack: expr) => {{
            match $namespace {
                Namespace::Local => {
                    $stack.get_local_env_mut()
                    .ok_or(ExecErr::LocalOpInNonlocalContext)?
                },

                Namespace::Module => {
                    $stack.get_module_env_mut()
                }

                Namespace::Args => return Err(ExecErr::InvalidNamespace {
                    expect: "Module or local".to_owned(),
                    found: Namespace::Args
                }),
            }
        }}
    }

    #[macro_export]
    macro_rules! mod_args {
        ($args: expr, $modulo: expr) => {
            if ($args.len() % $modulo != 0) || ($args.len() == 0) {
                return Err(ArityErr::Modulo {
                               modulo: $modulo,
                               found: $args.len(),
                           }
                           .into());
            }
        }
    }

    #[macro_export]
    macro_rules! min_args {
        ($args: expr, $minimum: expr) => {
            if $args.len() < $minimum {
                return Err(ArityErr::Min {
                           min: $minimum,
                           found: $args.len(),
                       }
                       .into());
            }
        }
    }

    #[macro_export]
    macro_rules! exact_args {
        ($args: expr, $exact: expr) => {
            if $args.len() != $exact {
                return Err(ArityErr::Exact {
                           exact: $exact,
                           found: $args.len(),
                       }
                       .into());
            }
        }
    }

    #[macro_export]
    macro_rules! max_args {
        ($args: expr, $maximum: expr) => {
            if $args.len() > $maximum {
                return Err(ArityErr::Max {
                           max: $maximum,
                           found: $args.len(),
                       }
                       .into());
            }
        }
    }

    #[macro_export]
    macro_rules! cir_extract {
        ($cir: expr => Single) => {
            cir_extract!($cir => Single, "Single")
        };

        ($cir: expr => Single, $expect: expr) => {
            {
                $cir.try_get_single().ok_or(ExecErr::InvalidArg {
                    expect: $expect.to_string(),
                    found: $cir.clone()
                })
            }
        };

        ($cir: expr => Untouched) => {
            cir_extract!($cir => Untouched, "Untouched")
        };

        ($cir: expr => Untouched, $expect: expr) => {
            {
                $cir.try_get_untouched().ok_or(ExecErr::InvalidArg {
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

        ($cir: expr => Value) => {
            cir_extract!($cir => Value, "Value")
        };

        ($cir: expr => Value, $expect: expr) => {
            {
                $cir.try_to_value().ok_or(ExecErr::InvalidArg {
                       expect: $expect.to_string(),
                       found: $cir.clone(),
                  })
            }
        };
    }
}

mod var_cmds;
mod if_cmd;
mod basic_math;
mod procedure;
mod module;
mod list;
mod comp;
mod while_cmd;
mod flow_control;

pub use self::var_cmds::{Let, Set};
pub use self::if_cmd::If;
pub use self::basic_math::{Add, Sub, Mul, Div};
pub use self::procedure::Proc;
pub use self::flow_control::{Return, Continue, Break};
pub use self::module::{Load, MakeModule};
pub use self::list::{List, ListLength, ListIndex, Remove, Append, Pop};
pub use self::comp::{Eq, InEq, GreaterThan, GreaterThanEq, LessThan, LessThanEq};
pub use self::while_cmd::While;
