#[macro_use]
use vm::internal::*;
use ast::*;
use parser::parse_arg_list;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Proc(pub Namespace);

impl Cmd for Proc {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 3);

        let module = match self.0 {
            Namespace::Local => {
                stack.get_local_env_mut()
                .ok_or(ExecErr::LocalOpInNonlocalContext)?
            },

            Namespace::Module => {
                stack.get_module_env_mut()
            }

            Namespace::Args => unimplemented!(),
        };

        let name = cir_extract!(args[0] => Single, "Name of procedure")?;
        let proc_args = {
            let proc_args = cir_extract!(args[1] => Untouched, "Arguments of procedure")?;
            let proc_args = parse_arg_list(proc_args)?
                                          .ok_or(ExecErr::MissingArg("Argument body".to_string()))?;
            let mut string_args = Vec::new();
            for arg in proc_args.all() {
                if let Word::Atom(atom) = arg {
                    string_args.push(atom);
                } else {
                    return Err(ExecErr::UnexpectedWord(arg.clone()));
                }
            }
            string_args
        };

        let proc_body = parse_statement_seq(cir_extract!(args[2] => Untouched, "Body of procedure")?)?;
        let new_cmd = ProcCmdObject::new(name.to_string(), proc_args, proc_body);

        module.insert(name, Value::Cmd(Box::new(new_cmd)), observable_internal!())?;
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct ProcCmdObject {
    name: String,
    args: Vec<Atom>,
    body: Vec<Statement>,
}

impl ProcCmdObject {
    fn new(name: String, args: Vec<Atom>, body: Vec<Statement>) -> ProcCmdObject {
        assert!(args.len() > 0);
        assert!(body.len() > 0);
        ProcCmdObject {
            name: name,
            args: args,
            body: body,
        }
    }
}

impl Cmd for ProcCmdObject {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, self.args.len());
        
        let mut builder = EnvBuilder::basic_env();
        let mut arg_map = HashMap::new();

        for (name, cir) in self.args.iter().zip(args.iter()) {
            arg_map.insert(name.to_string(), cir.clone());
        }
        let mut stack = Stack::local_with_args(stack, builder.consume(), arg_map);
        
        for statement in self.body.iter() {
            match eval_some_cmd(&mut stack, &statement.all())? {
                ExecSignal::Return(value) => return Ok(ExecSignal::NextInstruction(value)),
                signal @ ExecSignal::Continue => unimplemented!(), //Err: continue on procedure
                signal @ ExecSignal::Break => unimplemented!(), //Err: break on procedure
                ExecSignal::NextInstruction(_) => continue,
            }
        }

        Ok(ExecSignal::NextInstruction(None))
    }
}
