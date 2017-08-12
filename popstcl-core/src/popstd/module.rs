use vm::internal::*;
use parser::parse_program;

#[derive(Clone, Debug)]
pub struct MakeModule;

impl MakeModule {
    pub fn make_module(stack: &mut Stack, parent_module: StdModule, binding: &str, module_code: &str, binding_info: &DebugInfo) -> Result<ExecSignal, CmdErr> {
        //parse loaded module
        let program = parse_program(&module_code).unwrap();
        let mut temp_module = parent_module;
        {
            let mut other_stack = Stack::new_module(&mut temp_module);
            eval_program(&mut other_stack, &program)?;
        }

        //Set new binding
        stack.get_module()
             .insert(binding, Value::Module(temp_module).into())
             .map_err(|oerr| CmdErr::ObjectErr(oerr 
                                                //dinsertion!(binding_info.line_info.clone(),
                                                //            binding_info)
                                                )
                      )?;
        Ok(ExecSignal::NextInstruction(None))
    }
}

impl Cmd for MakeModule {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        max_args!(&args, 3);

        let parent_module;
        let binding;
        let binding_info;
        let module_code;

        if args.len() == 2 {
            exact_args!(&args, 2);
            parent_module = StdModule::new(super::std_env().consume());
            binding = cir_extract!(args[0] => String, "Module Name (Single)")?;
            binding_info = &args[0].dinfo;
            module_code = cir_extract!(args[1] => String, "Module Code (String)")?;
        } else if args.len() == 3 {
            exact_args!(&args, 3);
            parent_module = cir_extract!(args[0] => Module, "Module for execution")?.clone();
            binding = cir_extract!(args[1] => String, "Module Name (Single)")?;
            binding_info = &args[1].dinfo;
            module_code = cir_extract!(args[2] => String, "Module Code (String)")?;
        } else {
            return Err(ArityErr::Min { min: 2, found: args.len() }.into());
        }

        MakeModule::make_module(stack, parent_module, &binding, &module_code, binding_info)
    }
}

#[derive(Clone, Debug)]
pub struct InMod;

impl Cmd for InMod {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 2);
        let mut module = cir_extract!(args[0] => mut Module)?;
        let program = {
            let program = cir_extract!(args[1] => String)?;
            parse_program(&**program)?
        };

        let mut stack = Stack::new_module(&mut *module);
        eval_program(&mut stack, &program)
            .map(|val| ExecSignal::NextInstruction(val))
            .map_err(|err| CmdErr::ExecErr(Box::new(err)))
    }
}
