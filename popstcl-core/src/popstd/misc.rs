use vm::internal::*;

/// args -> value
///
/// Performs a deep clone on the value
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct Clone;

impl Cmd for Clone {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        Ok(ExecSignal::NextInstruction(Some(args[0].value.deep_clone())))
    }
}

/// args -> NONE
///
/// Returns a new module with ONLY std commands
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct Std;

impl Cmd for Std {
    #[allow(unused_variables)]
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

/// args -> string
///
/// Checks a given namespace to check if a binding with the provided string exists. Does NOT
/// support field paths.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct Exists(pub Namespace);

impl Cmd for Exists {
    #[allow(unused_variables)]
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
