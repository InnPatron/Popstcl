use ast::*;
use namespace::Namespace;
use super::internal::*;
use line_info::LineInfo;

pub fn eval_program<'a>(stack: &mut Stack, program: &Program) -> Result<Option<RcValue>, ExecErr> {
    for stmt in program.iter() {
        match eval_stmt(stack, stmt)? {
            ExecSignal::Return(ret) => return Ok(ret),
            ExecSignal::Continue => panic!("Bad continue in eval_program. Should return error"),
            ExecSignal::Break => panic!("Bad break in eval_program. Should return error"),
            ExecSignal::NextInstruction(_) => (),
        }
    }
    Ok(None)
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
                LineInfo::collapse(&cmd.iter().map(|cir| cir.dinfo.segment_span.clone())
                                              .collect::<Vec<_>>())
            },
            cmd: {  
                cmd
            },

        }
    }
 
    fn run(mut self) -> Result<ExecSignal, ExecErr> {
        assert!(self.cmd.len() > 0);
        let cir = self.cmd.remove(0);
        let value = cir.value;
        let borrow = value.borrow();
        let cmd_obj: Box<Cmd> = match &*borrow {
            &Value::String(ref cmd_name) => {
                let cmd: RcValue = {
                    let mut rcmd = Err(ExecErr::NotCmd(cmd_name.to_string(), cir.dinfo.clone()));
                    if let Some(module) = self.stack.get_local_module() {
                        match module.get(&cmd_name.inner()) {
                            Ok(rc)  => {
                                if rc.borrow().is_cmd() == true {
                                    rcmd = Ok(rc);
                                }
                            },
                            _ => (),
                        }
                    }

                    if rcmd.is_err() {      //rcmd.is_err() == true => no local command w/ cmd_name
                        match self.stack.get_module().get(&cmd_name.inner()) {
                            Ok(rc)  => {
                                if rc.borrow().is_cmd() == true {
                                    rcmd = Ok(rc);
                                }                            
                            },
                            _ => (),
                        }
                    }
                    rcmd
                }?;

                let borrow = cmd.borrow();
                if let &Value::Cmd(ref boxed) = &*borrow {
                    boxed.clone()
                } else {
                    return Err(ExecErr::NotCmd(cmd_name.to_string(), cir.dinfo.clone()));
                }
            },

            &Value::Cmd(ref cmd) => {
                cmd.clone()
            },

            val @ _ => return Err(ExecErr::NotCmd(val.to_string(), cir.dinfo.clone())),
        };

        let common = self.info();
        let args = self.cmd;
        let debug_info = args.iter().map(|cir| cir.dinfo.clone()).collect::<Vec<DebugInfo>>();
        let debug_info = dcmd_exec!(Box::new(cir.dinfo.clone()),
                debug_info, 
                cir.dinfo.segment_span.clone(),
                common);
        cmd_obj.execute(self.stack, args).map_err(|cmd_e| ExecErr::CmdErr(cmd_e, debug_info))
    }
}

impl<'a, 'b, 'c:'b> InfoGenerator for Executor<'a, 'b, 'c> {
    fn info(&self) -> CommonInfo {
        CommonInfo {
            root_stmt_span: self.root_stmt.line_info.clone(),
            cmd_span: self.cmd_span.clone(),
            original_string: self.root_stmt.original_string.clone(),
        }
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
        let common = self.info();

        for word in self.to_reduce.words.iter() {
            match word.kind {
                WordKind::StrSub(ref string) => {
                    let input = str_sub(self.stack, string, &word.line_info, common.clone())?;
                    reduction.push(input);
                }

                WordKind::CmdSub(ref cmd) => {
                    let cmd_dinfo;
                    let arg_dinfo;

                    let reduced_cmd = {
                        let mut reducer = Reducer::new(self.root_stmt, self.stack, cmd);

                        match reducer.reduce()? {
                            signal @ ReduceResult::Return(_) => return Ok(signal),
                            ReduceResult::Continue(vec) => vec,
                        }
                    };

                    {
                        let mut info = reduced_cmd.iter().map(|cir| cir.dinfo.clone()).collect::<Vec<DebugInfo>>();
                        cmd_dinfo = Box::new(info.remove(0));
                        arg_dinfo = info;
                    }

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
                            reduction.push(CIR::new(val.clone().into(), dcmd_sub!(cmd_dinfo,
                                                                                  arg_dinfo,
                                                                                  word.line_info.clone(),
                                                                                  common.clone()
                                                                           )
                                                    )
                                           );
                        },

                        ExecSignal::Continue => return Err(ExecErr::BadContinue(dcmd_sub!(cmd_dinfo,
                                                                                arg_dinfo,
                                                                                word.line_info.clone(),
                                                                                common.clone()
                                                                                ))),
                        ExecSignal::Break => return Err(ExecErr::BadBreak(dcmd_sub!(cmd_dinfo,
                                                                                arg_dinfo,
                                                                                word.line_info.clone(),
                                                                                common.clone()
                                                                                ))),
                        ExecSignal::NextInstruction(None) => return Err(ExecErr::NoRet(word.kind.to_string(),
                                                                            dcmd_sub!(cmd_dinfo,
                                                                                arg_dinfo,
                                                                                word.line_info.clone(),
                                                                                common.clone()
                                                                                )
                                                                        )),
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
                _ => reduction.push(try_from_word(&word, self.info())
                                    .expect(&format!("If {:?} is not handled by the match, it NEEDS to be directly convertable to CIR", word.kind))),
            }
        }

        Ok(ReduceResult::Continue(reduction))
    }
}

impl<'a, 'b, 'c, 'd:'c> InfoGenerator for Reducer<'a, 'b, 'c, 'd> {
    fn info(&self) -> CommonInfo {
        CommonInfo {
            root_stmt_span: self.root_stmt.line_info.clone(),
            cmd_span: self.reduction_span.clone(),
            original_string: self.root_stmt.original_string.clone(),
        }
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
        let common = self.info();

        let first_obj: RcValue = if let Namespace::Args = self.namespace {
            let value: Option<RcValue> = self.stack.get_args()
                      .ok_or(ExecErr::VarSubErr(VarSubErr::NoArgs,
                                                dvar_sub!(
                                                    self.namespace.clone(), 
                                                    self.var_sub.clone(), 
                                                    first_name.line_info.clone(),
                                                    common.clone()
                                                )
                                  )
                              )?
                      .get(&***first_name)
                      .map(|cir| cir.value.clone());
            let value: Result<RcValue, ExecErr> = value.ok_or(ExecErr::VarSubErr(VarSubErr::UnknownBinding(
                                                        first_name.to_string(), 
                                                        Namespace::Args),
                                                        dvar_sub!(self.namespace.clone(),
                                                                    self.var_sub.clone(),
                                                                    first_name.line_info.clone(),
                                                                    common.clone())
                                                      )
                             );
            value?
        } else {
            let namespace = self.namespace.clone();
            let var_sub = self.var_sub.clone();
            let module = match self.namespace {
                Namespace::Local => {
                    let local = self.stack.get_local_module();
                    local
                              .ok_or(ExecErr::VarSubErr(VarSubErr::NoLocalModule,
                                                        dvar_sub!(
                                                            namespace.clone(),
                                                            var_sub.clone(),
                                                            first_name.line_info.clone(),
                                                            common.clone())
                                    )
                                )?
                },

                Namespace::Module => self.stack.get_module(),
                Namespace::Args => panic!("Tring to get env of Namespace::Args"),
            };
            module.get(&***first_name)
                  .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                             dvar_sub!(namespace,
                                                       var_sub,
                                                       first_name.line_info.clone(),
                                                       common)
                                                 )
                          )?
        };
        self.walk_obj(first_obj, &mut path_iter)
    }

    fn walk_obj<'f, 'g, I>(&mut self, obj: RcValue, iter: &'g mut I) -> Result<CIR, ExecErr>
    where I: Iterator<Item = &'f PathSegment>
{
    let segment = iter.next();
    let common = self.info();
    match segment {
        Some(segment) => {
            match &*obj.borrow() {
                &Value::Object(ref object) => {
                    let value = object.get(&segment.segment.to_string())
                                      .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                                                         dvar_sub!(
                                                                             self.namespace.clone(), 
                                                                             self.var_sub.clone(),
                                                                             segment.line_info.clone(),
                                                                             common)
                                                                         )
                                               )?;
                    self.walk_obj(value, iter)
                },

                &Value::Module(ref module) => {
                    let value = module.get(&segment.segment.to_string())
                        .map_err(|oerr| ExecErr::ObjectErr(oerr,
                                                           dvar_sub!(
                                                                self.namespace.clone(),
                                                                self.var_sub.clone(),
                                                                segment.line_info.clone(),
                                                                common
                                                               )
                                                           )
                                 )?;
                    self.walk_obj(value, iter)
                        
                },

                _ => Err(ExecErr::VarSubErr(VarSubErr::NonobjectFieldAccess(
                                segment.segment.to_string()
                            ),
                            dvar_sub!(self.namespace.clone(),
                                    self.var_sub.clone(),
                                    segment.line_info.clone(), 
                                    common)
                                )
                    )?,
                            
            }
        }
        None => { 
            let line_info = self.var_sub.0.last().unwrap().line_info.clone();
            Ok(CIR::new(obj, 
                        dvar_sub!(self.namespace.clone(), 
                                  self.var_sub.clone(),
                                  line_info,
                                  common)
                       )
              )
        }
    }
}
}

impl<'a, 'b, 'c, 'd:'b, 'e> InfoGenerator for VarSubber<'a, 'b, 'c ,'d, 'e> {
    fn info(&self) -> CommonInfo {
        CommonInfo {
            root_stmt_span: self.root_stmt.line_info.clone(),
            cmd_span: self.cur_stmt.line_info.clone(),
            original_string: self.root_stmt.original_string.clone(),
        }
    }
}

fn str_sub(stack: &mut Stack, sub: &StrSub, line_info: &LineInfo, common: CommonInfo) -> Result<CIR, ExecErr> {
    let mut result = String::new();
    for data in sub.0.iter() {
        match data {
            &StrData::String(ref s) => result.push_str(s),
            &StrData::VarSub(ref name, ref namespace, _) => {
                let module = match namespace {
                    &Namespace::Local => stack.get_local_module()
                        .ok_or(ExecErr::VarSubErr(
                                VarSubErr::NoLocalModule,
                                unimplemented!()
                                )
                            )?,
                    &Namespace::Module => stack.get_module(),
                    &Namespace::Args => unimplemented!(),
                };
                let value = module.get(name)
                                  .map_err(|oerr| ExecErr::ObjectErr(oerr, unimplemented!()))?;
                let borrow = value.borrow();
                match *borrow {
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
    Ok(CIR::new(result.into(), dstr_sub!(line_info.clone(), 
                                         common)
                )
       )
}


pub fn try_from_word(word: &Word, common: CommonInfo) -> Option<CIR> {
    match &word.kind {
        &WordKind::Atom(ref s) => Some(CIR::new(s.to_string().into(), 
                                                dliteral!(word.line_info.clone(), 
                                                          common)
                                                )
                                       ),
        &WordKind::Number(n) => Some(CIR::new(n.into(),
                                              dliteral!(word.line_info.clone(), 
                                                        common)
                                              )
                                     ),
        &WordKind::Bool(b) => Some(CIR::new(b.into(),
                                            dliteral!(word.line_info.clone(), 
                                                      common)
                                            )
                                   ),
        &WordKind::Untouched(ref s) => Some(CIR::new(s.to_string().into(), 
                                                     dliteral!(word.line_info.clone(), 
                                                               common) 
                                                     )
                                            ),
        _ => None,
    }
}
