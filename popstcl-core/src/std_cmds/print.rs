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

#[derive(Clone, Debug)]
pub struct EPrint;

impl Cmd for EPrint {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 1);
        eprintln!("{}", args[0]);

        Ok(ExecSignal::NextInstruction(None))
    }
}
