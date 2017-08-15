use vm::internal::*;
use parser::parse_program;

/// args -> string string
///
/// (condition, body) = args
///
/// Evaluates the condition string as a program within the current scope. The condition program
/// **MUST RETURN** a boolean to the While command. 
///
/// If the condition program evaluates to true, execute the given body within the current scope.
///
/// **MAY MUTATE**
#[derive(Clone, Debug)]
pub struct While;

impl Cmd for While {

    //TODO: add way to inspect inside While command as it executes??
    //Have commands return a closure or an iterator which the Vm calls?
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let conditional_program = cir_extract!(args[0] => String, "While Condition")?;
        let while_body = cir_extract!(args[1] => String, "While Body")?;

        let conditional_program = parse_program(&conditional_program)?;
        let while_body = parse_program(&while_body)?;

        'popstcl_loop: loop {
            //execute conditional
            //TODO: how should Continue and Break be handled inside the conditional

            //Execute condition
            //TODO: How should the conditional value be captured?
            //Should Return Signals be propogated?
            let mut conditional = None;
            for stmt in conditional_program.iter() {
                match eval_stmt(stack, &stmt)? {
                    ExecSignal::Return(conditional_return) => {
                        conditional = conditional_return;
                        //TODO: replace with 'break conditional_return;'
                        //See rust-lang issue 37339
                        break;
                    },
                    ExecSignal::NextInstruction(_) => (),
                    _ => panic!("Continued or Broke in conditional"),
                }
            }

            //Check condition
            let conditional = conditional.expect("Conditional did not return a value. (Make this an error)");
            let condition = if let Value::Bool(ref b) = *conditional.borrow() {
                b.clone()
            } else {
                panic!("conditional statement did not return a boolean. (Make this an error)");
            };

            if condition.inner() == false {
                break;
            }

            //Execute While body
            for stmt in while_body.iter() {
                match eval_stmt(stack, &stmt)? {
                    signal @ ExecSignal::Return(_) => return Ok(signal),    //propogate Return
                    ExecSignal::NextInstruction(_) => (),
                    ExecSignal::Continue => continue 'popstcl_loop,
                    ExecSignal::Break => break 'popstcl_loop,
                }
            }
        }

        //By default, While command returns nothing
        return Ok(ExecSignal::NextInstruction(None));
    }
}
