use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Clone;

impl Cmd for Clone {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        Ok(ExecSignal::NextInstruction(Some(args[0].value.deep_clone())))
    }
}

#[derive(Clone, Debug)]
pub struct Std;

impl Cmd for Std {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 0);
        Ok(ExecSignal::NextInstruction(Some(
                    Value::Module(
                        StdModule::new(
                            super::std_env().consume()
                            )
                        ).into()
                    )
                )
            )
    }
}

#[derive(Clone, Debug)]
pub struct Exists(pub Namespace);

impl Cmd for Exists {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        
        let name = cir_extract!(args[0] => String)?;
        let module = match self.0 {
            Namespace::Local => {
                stack.get_local_module()
                    .ok_or(CmdErr::NoLocalModule)?
            },

            Namespace::Module => stack.get_module(),

            Namespace::Args => {
                 let map = stack.get_args()
                    .ok_or(ExecErr::Generic("No argument module".to_string()))?;
                 let result = map.get(&**name).is_some();
                 return Ok(ExecSignal::NextInstruction(Some(result.into())));       
            },
        };

        let result = module.get(&**name).is_ok();

        Ok(ExecSignal::NextInstruction(Some(result.into())))
    }
}
