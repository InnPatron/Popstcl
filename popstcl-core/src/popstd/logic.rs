use vm::internal::*;

#[derive(Clone, Debug)]
pub struct And;

impl Cmd for And {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(args, 2);
      
        let mut args = args.into_iter();
        let first = args.next().unwrap();
        let mut result: bool = **cir_extract!(first => Bool)?;

        for argument in args {
            let argument: bool = **cir_extract!(argument => Bool)?;
            result = result && argument;
        }

        Ok(ExecSignal::NextInstruction(Some(result.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Or;

impl Cmd for Or {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(args, 2);
      
        let mut args = args.into_iter();
        let first = args.next().unwrap();
        let mut result: bool = **cir_extract!(first => Bool)?;

        for argument in args {
            let argument: bool = **cir_extract!(argument => Bool)?;
            result = result || argument;
        }

        Ok(ExecSignal::NextInstruction(Some(result.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Not;

impl Cmd for Not {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
      
        let result: bool = !**cir_extract!(args[0] => Bool)?;

        Ok(ExecSignal::NextInstruction(Some(result.into())))
    }
}
