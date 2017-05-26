use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Print;

impl Cmd for Print {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 1);
        println!("{}", args[0]);

        Ok(ExecSignal::NextInstruction(None))
    }
}
