use vm::internal::*;

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
