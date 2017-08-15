use vm::internal::*;
use ast::*;
use parser::{parse_program, parse_arg_list };
use std::collections::HashMap;

/// args -> string string? string
///
/// (name, args?, program) = args
///
/// Create a procedure and set a binding to it at the given namespace. With only 2 arguments in
/// Popstcl code, the generated procedure takes NO arguments.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct Proc(pub Namespace);

impl Cmd for Proc {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        max_args!(&args, 3);

        let module = match self.0 {
            Namespace::Local => {
                stack.get_local_module()
                .ok_or(CmdErr::NoLocalModule)?
            },

            Namespace::Module => {
                stack.get_module()
            }

            Namespace::Args => unimplemented!(),
        };

        let maybe_name = &args[0];
        let name = cir_extract!(maybe_name => String, "Name of procedure")?;
        let proc_args = {
            if args.len() == 2 {
                Vec::new()
            } else {
                let proc_args = cir_extract!(args[1] => String, "Arguments of procedure")?;
                let proc_args = parse_arg_list(&proc_args)?
                                              .ok_or(CmdErr::MissingArg("Argument body".to_string()))?;
                let mut string_args = Vec::new();
                for arg in proc_args.all() {
                    if let WordKind::Atom(atom) = arg.kind {
                        string_args.push(atom);
                    } else {
                        return Err(CmdErr::Generic(format!("Arg list must consist only of atoms {:?}", arg.clone())));
                    }
                }
                string_args
            }
        };
    
        let proc_body = if args.len() == 2 {
            parse_program(&cir_extract!(args[1] => String, "Body of procedure")?)?
        } else {
            parse_program(&cir_extract!(args[2] => String, "Body of procedure")?)?
        };
        let new_cmd = ProcCmdObject::new(name.to_string(), proc_args, proc_body);

        module.insert(&name, Value::Cmd(Box::new(new_cmd)).into())
              .map_err(|oerr| CmdErr::ObjectErr(oerr))?;
        Ok(ExecSignal::NextInstruction(None))
    }
}

/// Not an actual callable command. This is an object to represent procedures created by the Proc
/// command.
///
/// **MAY MUTATE**
#[derive(Clone, Debug)]
pub struct ProcCmdObject {
    name: String,
    args: Vec<Atom>,
    body: Program,
}

impl ProcCmdObject {
    fn new(name: String, args: Vec<Atom>, body: Program) -> ProcCmdObject {
        ProcCmdObject {
            name: name,
            args: args,
            body: body,
        }
    }
}

impl Cmd for ProcCmdObject {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, self.args.len());
        
        let mut arg_map = HashMap::new();

        for (name, cir) in self.args.iter().zip(args.iter()) {
            arg_map.insert(name.to_string(), cir.clone());
        }
        let mut stack = Stack::local_with_args(stack, Env::empty(), arg_map);
        
        for stmt in self.body.iter() {
            match eval_stmt(&mut stack, &stmt)? {
                ExecSignal::Return(value) => return Ok(ExecSignal::NextInstruction(value)),
                signal @ ExecSignal::Continue => unimplemented!(), //Err: continue on procedure
                signal @ ExecSignal::Break => unimplemented!(), //Err: break on procedure
                ExecSignal::NextInstruction(_) => continue,
            }
        }

        Ok(ExecSignal::NextInstruction(None))
    }
}
