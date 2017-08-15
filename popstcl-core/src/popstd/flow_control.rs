#![allow(unused_variables)]
use vm::internal::*;

/// args -> value?
///
/// Returns a ExecSignal::Return. If a value has been passed, return that value with it.
///
/// **THERE IS NO GUARANTEE THAT THIS COMMAND WILL BEHAVE AS EXPECTED IF THE ENCLOSING CONTEXT OF
/// EXECUTION DOES NOT HANDLE ExecSignal::Return PROPERLY.**
///
/// If the interpreter received an ExecSignal::Return while reducing arguments of a command, the
/// signal will propogate up to callers and will continue to propogate until the original caller of
/// the Popstcl interpreter receives it.
///
/// **NON-MUTATING
#[derive(Clone, Debug)]
pub struct Return;

impl Cmd for Return {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        max_args!(&args, 1);

        if args.len() == 0 {
            Ok(ExecSignal::Return(None))
        } else {
            Ok(ExecSignal::Return(Some(args[0].value.clone())))
        }
    }
}

/// args -> NONE
///
/// Returns an ExecSignal::Continue.
///
/// **THERE IS NO GUARANTEE THAT THIS COMMAND WILL BEHAVE AS EXPECTED IF THE ENCLOSING CONTEXT OF
/// EXECUTION DOES NOT HANDLE ExecSignal::Return PROPERLY.**
/// 
/// If the interpreter receives an ExecSignal::Continue AT ANY POINT, an ExecErr::BadContinue will
/// be returned.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct Continue;

impl Cmd for Continue {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 0);
        Ok(ExecSignal::Continue)
    }
}

/// args -> NONE
///
/// Returns an ExecSignal::Break.
///
/// **THERE IS NO GUARANTEE THAT THIS COMMAND WILL BEHAVE AS EXPECTED IF THE ENCLOSING CONTEXT OF
/// EXECUTION DOES NOT HANDLE ExecSignal::Return PROPERLY.**
/// 
/// If the interpreter receives an ExecSignal::Break AT ANY POINT, an ExecErr::BadBreak will
/// be returned.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
//TODO: allow break return values?
pub struct Break;

impl Cmd for Break {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 0);
        Ok(ExecSignal::Break)
    }
}
