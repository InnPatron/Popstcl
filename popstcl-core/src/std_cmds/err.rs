use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Assert;

impl Cmd for Assert {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(args, 1);
        let b = cir_extract!(args[0] => Bool)?;
        if **b {
            Ok(ExecSignal::NextInstruction(None))
        } else {
            Err(ExecErr::Generic("Assertion failed".to_string()))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Error;

impl Cmd for Error {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(args, 1);
        let s = cir_extract!(args[0] => String)?;
        Err(ExecErr::Generic((**s).clone()))
    }
}
