#![allow(unused_variables)]
use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Return;

impl Cmd for Return {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        max_args!(&args, 1);

        if args.len() == 0 {
            Ok(ExecSignal::Return(None))
        } else {
            Ok(ExecSignal::Return(Some(args[0].value.clone())))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Continue;

impl Cmd for Continue {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 0);
        Ok(ExecSignal::Continue)
    }
}

#[derive(Clone, Debug)]
//TODO: allow break return values?
pub struct Break;

impl Cmd for Break {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 0);
        Ok(ExecSignal::Break)
    }
}
