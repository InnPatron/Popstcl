use ast::*;
use namespace::Namespace;
use super::internal::{Value, Stack, ExecErr, ExecSignal, CIR, Cmd, Object};

enum ReduceResult {
    Return(Option<Value>),
    Continue,
}

//methods for command running
pub fn eval_some_cmd(stack: &mut Stack, cmd: &[Word]) -> Result<ExecSignal, ExecErr> {
    assert!(cmd.len() > 0);
    let mut reduced_cmd = Vec::new();
    match reduce(stack, cmd, &mut reduced_cmd)? {
        ReduceResult::Return(value) => return Ok(ExecSignal::Return(value)),
        ReduceResult::Continue => (),
    }

    run(stack, reduced_cmd)
}

fn run(stack: &mut Stack, mut cmd: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
    assert!(cmd.len() > 0);
    let cmd_obj: Box<Cmd> = match cmd.remove(0).value {
        Value::String(ref cmd_name) => {
            let env = stack.get_local_env().unwrap_or(stack.get_module_env());
            if let Value::Cmd(ref boxed) = env.get_clone(cmd_name)? {
                boxed.clone()
            } else {
                return Err(ExecErr::NotCmd(cmd_name.to_string()));
            }
        },

        Value::Cmd(ref cmd) => {
            cmd.clone()
        },

        _ => return Err(ExecErr::NotCmd(cmd[0].to_string())),
    };

    let args = cmd;
    cmd_obj.execute(stack, args)
}

fn reduce(stack: &mut Stack, args: &[Word], reduction: &mut Vec<CIR>) -> Result<ReduceResult, ExecErr> {
    for word in args.iter() {
        match word.kind {
            WordKind::StrSub(ref string) => {
                let input = str_sub(stack, string)?;
                reduction.push(input);
            }

            WordKind::CmdSub(ref cmd) => {
                let mut reduced_cmd_sub = Vec::new();

                match reduce(stack, &cmd.all(), &mut reduced_cmd_sub)? {
                    signal @ ReduceResult::Return(_) => return Ok(signal),
                    ReduceResult::Continue => (),
                }
                
                match run(stack, reduced_cmd_sub)? {
                    ExecSignal::Return(val) => {
                        return Ok(ReduceResult::Return(val));
                        //vec![CIR::from(val.clone())]    
                        //TODO: is this ^ an error?
                        //A command is telling the executor to alter program flow and return during
                        //an argument reduction
                    },

                    ExecSignal::NextInstruction(Some(val)) => {
                        reduction.push(CIR::from(val.clone()));
                    }

                    _ => return Err(ExecErr::NoRet(word.clone())),
                }   
            }

            WordKind::VarSub(ref path, ref namespace) => reduction.push(var_sub(stack, path, namespace)?),
            _ => reduction.push(CIR::try_from(&word.kind).unwrap()),
        }
    }

    Ok(ReduceResult::Continue)
}

fn var_sub(stack: &Stack, path: &Path, namespace: &Namespace) -> Result<CIR, ExecErr> {
    assert!(path.0.len() > 0);
    let mut path_iter = path.0.iter();
    
    let first_name = path_iter.next().unwrap().to_string();

    if let &Namespace::Args = namespace {
        stack.get_args().ok_or(ExecErr::NoArguments)?
                    .get::<str>(&first_name)
                    .ok_or(ExecErr::MissingArg(first_name))
                    .map(|cir| cir.clone())
    } else {
        let module = match namespace {
            &Namespace::Local => stack.get_local_env().ok_or(ExecErr::LocalOpInNonlocalContext)?,
            &Namespace::Module => stack.get_module_env(),
            &Namespace::Args => panic!("Tring to get env of Namespace::Args"),
        };
        let first_obj = module.get(&first_name)?;
        walk_obj(&first_obj, path, &mut path_iter)
    }
}

fn walk_obj<'a, 'b, I>(obj: &Value, path: &Path, iter: &'b mut I) -> Result<CIR, ExecErr>
    where I: Iterator<Item = &'a Atom>
{
    let segment = iter.next();
    match segment {
        Some(atom) => {
            match obj {
                &Value::Object(ref object) => {
                    let value = object.get(&atom.to_string())?;
                    walk_obj(&value, path, iter)
                }

                _ => Err(ExecErr::NonobjectFieldAccess(atom.to_string(), path.clone())),
            }
        }
        None => Ok(From::from(obj.clone())),
    }
}

fn str_sub(stack: &Stack, sub: &StrSub) -> Result<CIR, ExecErr> {
    let mut result = String::new();
    for data in sub.0.iter() {
        match data {
            &StrData::String(ref s) => result.push_str(s),
            &StrData::VarSub(ref name, ref namespace) => {
                let module = match namespace {
                    &Namespace::Local => stack.get_local_env().ok_or(ExecErr::LocalOpInNonlocalContext)?,
                    &Namespace::Module => stack.get_module_env(),
                    &Namespace::Args => unimplemented!(),
                };
                let value = module.get_clone(name)?;
        
                match value {
                    Value::Number(num) => result.push_str(&num.to_string()),
                    Value::String(ref s) => result.push_str(s),
                    Value::Bool(ref b) => result.push_str(&b.to_string()),
                    Value::Cmd(_) => result.push_str(name),
                    Value::List(_) => unimplemented!(),
                    Value::Object(ref obj) => result.push_str(&obj.to_string()),
                    Value::Module(_) => unimplemented!(),
                }
            }
            &StrData::CmdSub => unimplemented!(),
        }
    }
    Ok(From::from(Value::String(result)))
}

