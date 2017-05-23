use vm::internal::*;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use parser::parse_program;

/// load <name> <filename>
#[derive(Clone, Debug)]
pub struct Load;

impl Cmd for Load {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 2);

        let binding = cir_extract!(args[0] => String, "Module Name (Single)")?;

        let file_name = cir_extract!(args[1] => String, "File Name (Single)")?;

        let mut file_path = env::current_dir().expect("Require workign directory");
        file_path.set_file_name(&file_name);
        file_path.set_extension("popstcl");

        let mut file = File::open(file_path.as_path()).expect(&format!("File {} failed to open", file_path.display()));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("Failed to read {}", file_path.display()));
        
        //parse loaded module
        let program = parse_program(&contents).unwrap();
        //build new Env and run loaded module
        let mut temp_module = InternalModule::new(EnvBuilder::basic_env().consume());
        {
            let mut other_stack = Stack::new_module(&mut temp_module);
            for stmt in program.code.iter() {
                eval_some_cmd(&mut other_stack, &stmt.all())?;
            }
        }

        //Set new binding
        stack.get_module_env_mut().insert(binding, Value::Module(temp_module.into_foreign()), observable_internal!())?;
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct MakeModule;

impl Cmd for MakeModule {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 2);

        let binding = cir_extract!(args[0] => String, "Module Name (Single)")?;

        let module_code = cir_extract!(args[1] => String, "Module Code (String)")?;

        //parse loaded module
        let program = parse_program(&module_code).unwrap();
        let mut temp_module = InternalModule::new(EnvBuilder::basic_env().consume());
        let mut other_env = EnvBuilder::basic_env().consume(); 
        {
            let mut other_stack = Stack::new_module(&mut temp_module);
            for stmt in program.code.iter() {
                eval_some_cmd(&mut other_stack, &stmt.all())?;
            }
        }

        //Set new binding
        stack.get_module_env_mut().insert(binding, Value::Module(temp_module.into_foreign()), observable_internal!())?;
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
/// move_mode $mod <untouched>
struct MoveMod;

impl Cmd for MoveMod {
	fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
		exact_args!(&args, 2);

		let mut pmod = cir_extract!(args[0] => Module)?;
		let program = cir_extract!(args[1] => String)?;
        let mut mod_scope = InternalModule::new(EnvBuilder::basic_env().consume());
		let mut temp_stack = Stack::new_module(&mut mod_scope);
		let program_seq = parse_statement_seq(program.trim())?;
		for stmt in program_seq.iter() {
			let signal = eval_some_cmd(&mut temp_stack, &stmt.all())?;
			if let ExecSignal::NextInstruction(_) = signal {
				continue;
			} else {
				return Ok(signal);
			}
		}

		Ok(ExecSignal::NextInstruction(None))
	}
}
