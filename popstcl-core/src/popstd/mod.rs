#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! get_module {
        ($namespace: expr, $stack: expr) => {{
            match $namespace {
                Namespace::Local => {
                    $stack.get_local_module()
                    .ok_or(CmdErr::NoLocalModule)?
                },

                Namespace::Module => {
                    $stack.get_module()
                }

                Namespace::Args => return Err(CmdErr::InvalidNamespace {
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
mod err;
mod logic;
mod eval;

// Using
pub use self::var_cmds::{Let, Set, Mutate};
pub use self::if_cmd::If;
pub use self::basic_math::{Add, Sub, Mul, Div, Inc, Dec, Negate};
pub use self::procedure::Proc;
pub use self::flow_control::{Return, Continue, Break};
pub use self::module::{MakeModule, InMod};
pub use self::list::{List, ListLength, ListIndex, Remove, Append, Pop};
pub use self::comp::{RefEq, RefInEq, Eq, InEq, GreaterThan, GreaterThanEq, LessThan, LessThanEq};
pub use self::while_cmd::While;
pub use self::print::{Print, EPrint};
pub use self::object::{MakeObject, FSet, FMut, RmField};
pub use self::err::{Assert, AssertEq, Error};
pub use self::logic::{And, Or, Not};
pub use self::eval::{Eval, EvalInPlace};
pub use self::misc::{Std, Clone, Exists};

use vm::user::*;
use super::namespace::Namespace;

// StdBuilding
pub fn std_env() -> EnvBuilder {
        let mut builder = EnvBuilder::new();

        builder.insert_value("llet", Value::Cmd(Box::new(Let(Namespace::Local))));
        builder.insert_value("lset", Value::Cmd(Box::new(Set(Namespace::Local))));
        builder.insert_value("lproc", Value::Cmd(Box::new(Proc(Namespace::Local))));
        builder.insert_value("lmut", Value::Cmd(Box::new(Mutate(Namespace::Local))));

        builder.insert_value("let", Value::Cmd(Box::new(Let(Namespace::Module))));
        builder.insert_value("set", Value::Cmd(Box::new(Set(Namespace::Module))));
        builder.insert_value("proc", Value::Cmd(Box::new(Proc(Namespace::Module))));
        builder.insert_value("mut", Value::Cmd(Box::new(Mutate(Namespace::Module))));

        builder.insert_value("mlet", Value::Cmd(Box::new(Let(Namespace::Module))));
        builder.insert_value("mset", Value::Cmd(Box::new(Set(Namespace::Module))));
        builder.insert_value("mproc", Value::Cmd(Box::new(Proc(Namespace::Module))));
        builder.insert_value("mmut", Value::Cmd(Box::new(Mutate(Namespace::Module))));

        builder.insert_value("add", Value::Cmd(Box::new(Add)));
        builder.insert_value("sub", Value::Cmd(Box::new(Sub)));
        builder.insert_value("mul", Value::Cmd(Box::new(Mul)));
        builder.insert_value("div", Value::Cmd(Box::new(Div)));
        builder.insert_value("inc", Value::Cmd(Box::new(Inc)));
        builder.insert_value("dec", Value::Cmd(Box::new(Dec)));
        builder.insert_value("neg", Value::Cmd(Box::new(Negate)));
       
        builder.insert_value("if", Value::Cmd(Box::new(If)));
        builder.insert_value("while", Value::Cmd(Box::new(While)));

        builder.insert_value("eval", Value::Cmd(Box::new(Eval)));
        builder.insert_value("inplace", Value::Cmd(Box::new(EvalInPlace)));

        builder.insert_value("return", Value::Cmd(Box::new(Return)));
        builder.insert_value("continue", Value::Cmd(Box::new(Continue)));
        builder.insert_value("break", Value::Cmd(Box::new(Break)));

        builder.insert_value("make", Value::Cmd(Box::new(MakeModule)));
        builder.insert_value("inmod", Value::Cmd(Box::new(InMod)));

        builder.insert_value("===", Value::Cmd(Box::new(RefEq)));
        builder.insert_value("!===", Value::Cmd(Box::new(RefInEq)));
        builder.insert_value("==", Value::Cmd(Box::new(Eq)));
        builder.insert_value("!=", Value::Cmd(Box::new(InEq)));
        builder.insert_value(">", Value::Cmd(Box::new(GreaterThan)));
        builder.insert_value("<", Value::Cmd(Box::new(LessThan)));
        builder.insert_value("<=", Value::Cmd(Box::new(LessThanEq)));
        builder.insert_value(">=", Value::Cmd(Box::new(GreaterThanEq)));
        builder.insert_value("inv", Value::Cmd(Box::new(Not)));

        builder.insert_value("&&", Value::Cmd(Box::new(And)));
        builder.insert_value("||", Value::Cmd(Box::new(Or)));
        builder.insert_value("!!", Value::Cmd(Box::new(Not)));

        builder.insert_value("list", Value::Cmd(Box::new(List)));
        builder.insert_value("list_len", Value::Cmd(Box::new(ListLength)));
        builder.insert_value("list_get", Value::Cmd(Box::new(ListIndex)));
        builder.insert_value("list_remove", Value::Cmd(Box::new(Remove)));
        builder.insert_value("list_pop", Value::Cmd(Box::new(Pop)));
        builder.insert_value("list_append", Value::Cmd(Box::new(Append)));

        builder.insert_value("print", Value::Cmd(Box::new(Print)));
        builder.insert_value("eprint", Value::Cmd(Box::new(EPrint)));

        builder.insert_value("clone", Value::Cmd(Box::new(Clone)));

        builder.insert_value("object", Value::Cmd(Box::new(MakeObject)));
        builder.insert_value("fset", Value::Cmd(Box::new(FSet)));
        builder.insert_value("fmut", Value::Cmd(Box::new(FMut)));
        builder.insert_value("rmf", Value::Cmd(Box::new(RmField)));

        builder.insert_value("assert", Value::Cmd(Box::new(Assert)));
        builder.insert_value("assert_eq", Value::Cmd(Box::new(AssertEq)));
        builder.insert_value("err", Value::Cmd(Box::new(Error)));
        builder.insert_value("exists", Value::Cmd(Box::new(Exists(Namespace::Module))));
        builder.insert_value("mexists", Value::Cmd(Box::new(Exists(Namespace::Module))));
        builder.insert_value("lexists", Value::Cmd(Box::new(Exists(Namespace::Local))));
        builder.insert_value("aexists", Value::Cmd(Box::new(Exists(Namespace::Args))));

        builder.insert_value("std", Value::Cmd(Box::new(Std)));
        
        builder
    }
