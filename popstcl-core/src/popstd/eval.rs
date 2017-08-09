use vm::internal::*;
use parser;

#[derive(Debug, Clone)]
pub struct Eval;

impl Cmd for Eval {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
    
        let program = cir_extract!(args[0] => String)?;
        let program = parser::parse_program(&*program)?;
        eval_program(stack, &program)
            .map(|option| ExecSignal::NextInstruction(option))
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone)]
pub struct EvalInPlace;

impl Cmd for EvalInPlace {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
    
        let program = cir_extract!(args[0] => String)?;
        let program = parser::parse_program(&*program)?;
        
        for stmt in program.iter() {
            match eval_stmt(stack, &stmt)? {
                ExecSignal::NextInstruction(_) => (),
                s @ ExecSignal::Return(_) => return Ok(s),
                s @ ExecSignal::Break => return Ok(s),
                s @ ExecSignal::Continue => return Ok(s),
            }
        }

        Ok(ExecSignal::NextInstruction(None))
    }
}
