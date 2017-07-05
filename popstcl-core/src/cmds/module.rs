use vm::internal::*;
use parser::parse_program;

#[derive(Clone, Debug)]
pub struct MakeModule;

impl MakeModule {
    pub fn make_module(stack: &mut Stack, parent_module: StdModule, binding: &str, module_code: &str, binding_info: &DebugInfo) -> Result<ExecSignal, ExecErr> {
        //parse loaded module
        let program = parse_program(&module_code).unwrap();
        let mut temp_module = InternalModule::from(parent_module);
        {
            let mut other_stack = Stack::new_module(&mut temp_module);
            eval_program(&mut other_stack, &program)?;
        }

        //Set new binding
        stack.get_module_env_mut()
             .insert(binding, Value::Module(temp_module.into()))
             .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                                dinsertion!(binding_info.line_info.clone(),
                                                            binding_info)
                                                )
                      )?;
        Ok(ExecSignal::NextInstruction(None))
    }
}

impl Cmd for MakeModule {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        max_args!(&args, 3);

        let parent_module;
        let binding;
        let binding_info;
        let module_code;

        if args.len() == 2 {
            exact_args!(&args, 2);
            parent_module = StdModule::new(EnvBuilder::basic_env().consume());
            binding = cir_extract!(args[0] => String, "Module Name (Single)")?.inner();
            binding_info = &args[0].dinfo;
            module_code = cir_extract!(args[1] => String, "Module Code (String)")?.inner();
        } else if args.len() == 3 {
            exact_args!(&args, 3);
            parent_module = cir_extract!(args[0] => Module, "Module for execution")?;
            binding = cir_extract!(args[1] => String, "Module Name (Single)")?.inner();
            binding_info = &args[1].dinfo;
            module_code = cir_extract!(args[2] => String, "Module Code (String)")?.inner();
        } else {
            return Err(ArityErr::Min { min: 2, found: args.len() }.into());
        }

        MakeModule::make_module(stack, parent_module, &binding, &module_code, binding_info)
    }
}

#[derive(Clone, Debug)]
/// move_mode $mod <untouched>
struct MoveMod;

impl Cmd for MoveMod {
	fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
		exact_args!(&args, 2);

		let mut pmod = cir_extract!(args[0] => Module)?;
		let program = cir_extract!(args[1] => String)?.inner();
        let mut mod_scope = InternalModule::from(pmod);
		let mut temp_stack = Stack::new_module(&mut mod_scope);
		let program_seq = parse_program(program.trim())?;
		for stmt in program_seq.iter() {
			let signal = eval_stmt(&mut temp_stack, &stmt)?;
			if let ExecSignal::NextInstruction(_) = signal {
				continue;
			} else {
				return Ok(signal);
			}
		}

		Ok(ExecSignal::NextInstruction(None))
	}
}
