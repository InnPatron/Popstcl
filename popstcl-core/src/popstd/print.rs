use vm::internal::*;

/// args -> string
///
/// Print a string to stdout.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct Print;

impl Cmd for Print {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 1);
        println!("{}", args[0]);

        Ok(ExecSignal::NextInstruction(None))
    }
}

/// args -> string
///
/// Print a string to stderr.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct EPrint;

impl Cmd for EPrint {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 1);
        eprintln!("{}", args[0]);

        Ok(ExecSignal::NextInstruction(None))
    }
}
