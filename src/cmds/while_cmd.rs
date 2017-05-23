use vm::internal::*;

#[derive(Clone, Debug)]
pub struct While;

impl Cmd for While {

    //TODO: add way to inspect inside While command as it executes??
    //Have commands return a closure or an iterator which the Vm calls?
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 2);

        let conditional_stmt = cir_extract!(args[0] => String, "While Condition")?;
        let while_body = cir_extract!(args[1] => String, "While Body")?;

        let conditional_stmt = parse_statement_seq(conditional_stmt)?;
        let while_body = parse_statement_seq(while_body)?;

        'popstcl_loop: loop {
            //execute conditional
            //TODO: how should Continue and Break be handled inside the conditional

            //Execute condition
            //TODO: How should the conditional value be captured?
            //Should Return Signals be propogated?
            let mut conditional = None;
            for entry in conditional_stmt.iter() {
                match eval_some_cmd(stack, &entry.all())? {
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
            let condition = if let Value::Bool(b) = conditional {
                b
            } else {
                panic!("conditional statement did not return a boolean. (Make this an error)");
            };

            if condition == false {
                break;
            }

            //Execute While body
            for entry in while_body.iter() {
                match eval_some_cmd(stack, &entry.all())? {
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
