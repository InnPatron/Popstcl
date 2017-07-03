use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Clone;

impl Cmd for Clone {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(args, 1);
        Ok(ExecSignal::NextInstruction(Some(args[0].value.inner_clone().into())))
    }
}
