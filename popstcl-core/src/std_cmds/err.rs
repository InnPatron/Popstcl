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
