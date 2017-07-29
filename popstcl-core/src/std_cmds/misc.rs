use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Clone;

impl Cmd for Clone {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        Ok(ExecSignal::NextInstruction(Some(args[0].value.deep_clone())))
    }
}

#[derive(Clone, Debug)]
pub struct Std;

impl Cmd for Std {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 0);
        Ok(ExecSignal::NextInstruction(Some(
                    Value::Module(
                        StdModule::new(
                            EnvBuilder::std_env().consume()
                            )
                        ).into()
                    )
                )
            )
    }
}
