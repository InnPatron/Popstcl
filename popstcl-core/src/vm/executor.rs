use ast::*;
use namespace::Namespace;
use super::internal::{RcValue, Value, Stack, ExecErr, VarSubErr, ExecSignal, CIR, Cmd, Object, DebugInfo, DebugKind, IntoValue};
use line_info::LineInfo;

use std::rc::Rc;

pub fn eval_program<'a>(stack: &mut Stack, program: &Program) -> Result<(), ExecErr> {
    for stmt in program.iter() {
        eval_stmt(stack, stmt)?;
    }
    Ok(())
}

pub fn eval_stmt<'a, 'b, 'c:'a>(stack: &'a mut Stack<'c>, stmt: &'b Statement) -> Result<ExecSignal, ExecErr> {
    let cmd = stmt.all();
    let reduced_cmd = {
        let mut reducer = Reducer::new(stmt, stack, stmt);
        match reducer.reduce()? {
            ReduceResult::Return(value) => return Ok(ExecSignal::Return(value)),
            ReduceResult::Continue(vec) => vec,
        }
    };

    let mut executor = Executor::new(stmt, stack, reduced_cmd);
    executor.run()
}

enum ReduceResult {
    Return(Option<RcValue>),
    Continue(Vec<CIR>)
}

struct Executor<'a, 'b, 'c:'b> {
    root_stmt: &'a Statement,
    stack: &'b mut Stack<'c>,
    cmd: Vec<CIR>,
    cmd_span: LineInfo, 
}

impl<'a, 'b, 'c:'b> Executor<'a, 'b, 'c> {
    fn new(root_stmt: &'a Statement, stack: &'b mut Stack<'c>, cmd: Vec<CIR>) -> Executor<'a, 'b, 'c> {
        Executor {
            root_stmt: root_stmt,
            stack: stack,
            cmd_span: {
                assert!(cmd.len() > 0);
                LineInfo::collapse(&cmd.iter().map(|cir| cir.dinfo.line_info.clone())
                                              .collect::<Vec<_>>())
            },
            cmd: {  
                cmd
            },

        }
    }
 
    fn run(mut self) -> Result<ExecSignal, ExecErr> {
        assert!(self.cmd.len() > 0);
        let cmd_obj: Box<Cmd> = match *self.cmd.remove(0).value {
            Value::String(ref cmd_name) => {
                let cmd = {
                    let mut rcmd = Err(ExecErr::NotCmd(cmd_name.to_string()));
                    if let Some(module) = self.stack.get_local_module() {
                        match module.get(&cmd_name.inner()) {
                            Ok(rc)  => {
                                if let &Value::Cmd(_) = &*rc {
                                    rcmd = Ok(rc);
                                }
                            },
                            _ => (),
                        }
                    }

                    if rcmd.is_err() {      //rcmd.is_err() == true => no local command w/ cmd_name
                        match self.stack.get_module().get(&cmd_name.inner()) {
                            Ok(rc)  => {
                                if let &Value::Cmd(_) = &*rc {
                                    rcmd = Ok(rc);
                                }                            
                            },
                            _ => (),
                        }
                    }
                    rcmd
                }?;

                if let Value::Cmd(ref boxed) = *cmd {
                    boxed.clone()
                } else {
                    return Err(ExecErr::NotCmd(cmd_name.to_string()));
                }
            },

            Value::Cmd(ref cmd) => {
                cmd.clone()
            },

            _ => return Err(ExecErr::NotCmd(self.cmd[0].to_string())),
        };

        let args = self.cmd;
        cmd_obj.execute(self.stack, args)
    }
}

struct Reducer<'a, 'b, 'c, 'd:'b> {
    root_stmt: &'a Statement,
    stack: &'b mut Stack<'d>,
    to_reduce: &'c Statement,
    reduction_span: LineInfo,
}

impl<'a, 'b, 'c, 'd:'c> Reducer<'a, 'b, 'c, 'd> {

    fn new(root_stmt: &'a Statement, stack: &'b mut Stack<'d>, to_reduce: &'c Statement) -> Reducer<'a, 'b, 'c, 'd> {
        Reducer {
            root_stmt: root_stmt,
            stack: stack,
            reduction_span: LineInfo::collapse(&to_reduce.get_all().iter()
                                                        .map(|word| word.line_info.clone())
                                                        .collect::<Vec<_>>()
                                              ),
            to_reduce: to_reduce,
        }
    }

    fn reduce(mut self) -> Result<ReduceResult, ExecErr> {
        let mut reduction = Vec::new();

        for word in self.to_reduce.words.iter() {
            match word.kind {
                WordKind::StrSub(ref string) => {
                    let input = str_sub(self.stack, string, &word.line_info, self.root_stmt, self.to_reduce)?;
                    reduction.push(input);
                }

                WordKind::CmdSub(ref cmd) => {
                    let debug_info;

                    let reduced_cmd = {
                        let mut reducer = Reducer::new(self.root_stmt, self.stack, cmd);

                        match reducer.reduce()? {
                            signal @ ReduceResult::Return(_) => return Ok(signal),
                            ReduceResult::Continue(vec) => vec,
                        }
                    };

                    debug_info = reduced_cmd.iter().map(|cir| cir.dinfo.clone()).collect::<Vec<DebugInfo>>();
                    let executor = Executor::new(self.root_stmt, self.stack, reduced_cmd);
                    match executor.run()? {
                        ExecSignal::Return(val) => {
                            return Ok(ReduceResult::Return(val));
                            //vec![CIR::from(val.clone())]    
                            //TODO: is this ^ an error?
                            //A command is telling the executor to alter program flow and return during
                            //an argument reduction
                        },

                        ExecSignal::NextInstruction(Some(val)) => {
                            reduction.push(CIR::new(val.clone().into(), dcmd_sub!(debug_info,
                                                                           word.line_info.clone(),
                                                                           self.to_reduce,
                                                                           self.root_stmt
                                                                           )
                                                    )
                                           );
                        }

                        _ => return Err(ExecErr::NoRet(word.clone())),
                    }   
                }

                WordKind::VarSub(ref path, ref namespace) => {
                    let mut var_subber = VarSubber::new(self.root_stmt,
                                                        self.stack,
                                                        self.to_reduce,
                                                        path,
                                                        namespace.clone());
                    let sub_result = var_subber.var_sub()?;
                    reduction.push(sub_result);
                },
                _ => reduction.push(try_from_word(&word, self.to_reduce, &self.root_stmt)
                                    .expect(&format!("If {:?} is not handled by the match, it NEEDS to be directly convertable to CIR", word.kind))),
            }
        }

        Ok(ReduceResult::Continue(reduction))
    }
}

struct VarSubber <'a, 'b, 'c, 'd:'b, 'e> {
    root_stmt: &'a Statement,
    stack: &'b mut Stack<'d>,
    cur_stmt: &'e Statement,
    var_sub: &'c Path,
    namespace: Namespace,
    sub_span: LineInfo,
}

impl<'a, 'b, 'c, 'd:'b, 'e> VarSubber<'a, 'b, 'c ,'d, 'e> {
    fn new(root_stmt: &'a Statement, stack: &'b mut Stack<'d>, cur_stmt: &'e Statement, to_reduce: &'c Path, namespace: Namespace) -> VarSubber<'a, 'b, 'c, 'd, 'e> {
        VarSubber {
            root_stmt: root_stmt,
            stack: stack,
            cur_stmt: cur_stmt,
            var_sub: to_reduce,
            namespace: namespace,
            sub_span: LineInfo::collapse(&to_reduce.0.iter()
                                                          .map(|segment| segment.line_info.clone())
                                                          .collect::<Vec<_>>()
                                              ),
        }
    }

    fn var_sub(&mut self) -> Result<CIR, ExecErr> {
        assert!(self.var_sub.0.len() > 0);
        let mut path_iter = self.var_sub.0.iter();
        
        let first_name = path_iter.next().unwrap();

        let first_obj: RcValue = if let Namespace::Args = self.namespace {
            let value: Option<RcValue> = self.stack.get_args()
                      .ok_or(ExecErr::from(VarSubErr::NoArgs(dvar_sub!(
                                      self.namespace.clone(), 
                                      self.var_sub.clone(), 
                                      first_name.line_info.clone(),
                                      self.cur_stmt,
                                      self.root_stmt
                                      )
                                  )
                              )
                          )?
                      .get(&***first_name)
                      .map(|cir| cir.value.clone());
            let value: Result<RcValue, ExecErr> = value.ok_or(VarSubErr::UnknownBinding(first_name.to_string(), 
                                                      Namespace::Args, 
                                                      dvar_sub!(self.namespace.clone(),
                                                                self.var_sub.clone(),
                                                                first_name.line_info.clone(),
                                                                self.cur_stmt,
                                                                self.root_stmt)
                                                      ).into()
                             );
            value?
        } else {
            let module = match self.namespace {
                Namespace::Local => { 
                    self.stack.get_local_module()
                              .ok_or(VarSubErr::NoLocalModule(dvar_sub!(
                                        self.namespace.clone(),
                                        self.var_sub.clone(),
                                        first_name.line_info.clone(),
                                        self.cur_stmt,
                                        self.root_stmt)
                                                             )
                                    )?
                },

                Namespace::Module => self.stack.get_module(),
                Namespace::Args => panic!("Tring to get env of Namespace::Args"),
            };
            module.get(&***first_name)
                  .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                             dvar_sub!(self.namespace.clone(), 
                                                       self.var_sub.clone(),
                                                       first_name.line_info.clone(),
                                                       self.cur_stmt,
                                                       self.root_stmt
                                                      )
                                                 )
                          )?
        };
        self.walk_obj(first_obj, &mut path_iter)
    }

    fn walk_obj<'f, 'g, I>(&mut self, obj: RcValue, iter: &'g mut I) -> Result<CIR, ExecErr>
    where I: Iterator<Item = &'f PathSegment>
{
    let segment = iter.next();
    match segment {
        Some(segment) => {
            match &*obj {
                &Value::Object(ref object) => {
                    let value = object.get(&segment.segment.to_string())
                                      .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                                                         dvar_sub!(
                                                                             self.namespace.clone(), 
                                                                             self.var_sub.clone(),
                                                                             segment.line_info.clone(),
                                                                             self.cur_stmt,
                                                                             self.root_stmt
                                                                             )
                                                                         )
                                               )?;
                    self.walk_obj(value, iter)
                }

                _ => Err(VarSubErr::NonobjectFieldAccess(segment.segment.to_string(),
                                                      dvar_sub!(self.namespace.clone(),
                                                                self.var_sub.clone(),
                                                                segment.line_info.clone(), 
                                                                self.cur_stmt,
                                                                self.root_stmt
                                                               )
                                                        )
                            .into()),
            }
        }
        None => { 
            let line_info = self.var_sub.0.last().unwrap().line_info.clone();
            Ok(CIR::new(obj, 
                        dvar_sub!(self.namespace.clone(), 
                                  self.var_sub.clone(),
                                  line_info,
                                  self.cur_stmt,
                                  self.root_stmt
                                 )
                       )
              )
        }
    }
}
}

fn str_sub(stack: &Stack, sub: &StrSub, line_info: &LineInfo, root_stmt: &Statement, cur_stmt: &Statement) -> Result<CIR, ExecErr> {
    let mut result = String::new();
    for data in sub.0.iter() {
        match data {
            &StrData::String(ref s) => result.push_str(s),
            &StrData::VarSub(ref name, ref namespace, _) => {
                let module = match namespace {
                    &Namespace::Local => stack.get_local_module().ok_or(VarSubErr::NoLocalModule(unimplemented!()))?,
                    &Namespace::Module => stack.get_module(),
                    &Namespace::Args => unimplemented!(),
                };
                let value = module.get(name)
                                  .map_err(|oerr| ExecErr::ObjectErr(oerr, unimplemented!()))?;
                match *value {
                    Value::Number(ref num) => result.push_str(&num.to_string()),
                    Value::String(ref s) => result.push_str(&s.inner()),
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
    Ok(CIR::new(result.into_value().into(), dstr_sub!(line_info.clone(), 
                                               cur_stmt,
                                               root_stmt
                                               )
                )
       )
}


pub fn try_from_word(word: &Word, cur_stmt: &Statement, root_stmt: &Statement) -> Option<CIR> {
    match &word.kind {
        &WordKind::Atom(ref s) => Some(CIR::new(s.to_string().into_value().into(), 
                                                dliteral!(word.line_info.clone(), 
                                                          cur_stmt, 
                                                          root_stmt)
                                                )
                                       ),
        &WordKind::Number(n) => Some(CIR::new(n.into_value().into(),
                                              dliteral!(word.line_info.clone(), 
                                                        cur_stmt,
                                                        root_stmt
                                                        )
                                              )
                                     ),
        &WordKind::Bool(b) => Some(CIR::new(b.into_value().into(),
                                            dliteral!(word.line_info.clone(), 
                                                      cur_stmt,
                                                      root_stmt
                                                      )
                                            )
                                   ),
        &WordKind::Untouched(ref s) => Some(CIR::new(s.to_string().into_value().into(), 
                                                     dliteral!(word.line_info.clone(), 
                                                               cur_stmt,
                                                               root_stmt
                                                               )
                                                     )
                                            ),
        _ => None,
    }
}
