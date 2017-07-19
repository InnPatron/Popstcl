#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! get_module {
        ($namespace: expr, $stack: expr) => {{
            match $namespace {
                Namespace::Local => {
                    $stack.get_local_module()
                    .ok_or(ExecErr::NoLocalModule)?
                },

                Namespace::Module => {
                    $stack.get_module()
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
            if $args.len() % $modulo != 0 {
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
mod print;
mod misc;
mod object;

pub use self::var_cmds::{Let, Set, Mutate};
pub use self::if_cmd::If;
pub use self::basic_math::{Add, Sub, Mul, Div};
pub use self::procedure::Proc;
pub use self::flow_control::{Return, Continue, Break};
pub use self::module::{MakeModule};
pub use self::list::{List, ListLength, ListIndex, Remove, Append, Pop};
pub use self::comp::{Eq, InEq, GreaterThan, GreaterThanEq, LessThan, LessThanEq};
pub use self::while_cmd::While;
pub use self::print::Print;
pub use self::object::{MakeObject, FSet, FMut, RmField};
pub use self::misc::{Std, Clone};
