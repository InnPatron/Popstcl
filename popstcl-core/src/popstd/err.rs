use vm::internal::*;

/// args -> bool
///
/// If the given argument is true, continue program execution. Otherwise, return a CmdErr::Generic
/// and halt program execution.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct Assert;

impl Cmd for Assert {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        let b = cir_extract!(args[0] => Bool)?;
        if **b {
            Ok(ExecSignal::NextInstruction(None))
        } else {
            Err(CmdErr::Generic("Assertion failed".to_string()))
        }
    }
}

/// args -> value value 
///
/// Performs a value equality check. If true, continue program execution. Otherwise, return a CmdErr::Generic and halt program execution.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct AssertEq;

impl Cmd for AssertEq {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 2);

        if args[0].value == args[1].value {
            Ok(ExecSignal::NextInstruction(None))
        } else {
            Err(CmdErr::Generic(format!("Assertion failed. {} != {}", args[0].value, args[1].value)))
        }
    }
}

/// args -> string
///
/// Returns a CmdErr::Generic with the given string as a message. Halt program execution.
#[derive(Clone, Debug)]
pub struct Error;

impl Cmd for Error {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        let s = cir_extract!(args[0] => String)?;
        Err(CmdErr::Generic((**s).clone()))
    }
}
