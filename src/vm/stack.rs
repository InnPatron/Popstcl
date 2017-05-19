use super::internal::{Module, Env, CIR, LocalModule};
use std::collections::HashMap;

pub struct Stack<'a> {
    module_env: &'a mut Module,
    local_env: Option<LocalModule>,
    args: Option<HashMap<String, CIR>>,
}

impl<'a> Stack<'a> {
    pub fn new_module(global: &'a mut Module) -> Stack<'a> {
        Stack {
            module_env: global,
            local_env: None,
            args: None,
        }
    }

    pub fn new_local<'b>(previous: &'a mut Stack<'b>, local: Env) -> Stack<'a> {
        Stack {
            module_env: previous.module_env,
            local_env: Some(LocalModule::new(local)),
            args: None,
        }
    }

    pub fn local_with_args<'b>(previous: &'a mut Stack<'b>, local: Env, args: HashMap<String, CIR>) -> Stack<'a> {
        Stack {
            module_env: previous.module_env,
            local_env: Some(LocalModule::new(local)),
            args: {
                if args.is_empty() {
                    None
                } else {
                    Some(args)
                }
            },
        }
    }

    pub fn get_module_env_mut<'b>(&'b mut self) -> &'b mut Module {
        self.module_env
    }

    pub fn get_local_env_mut<'b>(&'b mut self) -> Option<&'b mut Module> {
    	if let Some(ref mut module) = self.local_env {
			Some(&mut *module)
		} else {
			None
		}
    }

    pub fn get_local_env<'b>(&'b self) -> Option<&Module> {
        if let Some(ref module) = self.local_env {
			Some(&*module)
		} else {
			None
		}
    }

    pub fn get_module_env<'b>(&'b self) -> &Module {
        self.module_env
    }

    pub fn get_args<'b>(&'b self) -> Option<&HashMap<String, CIR>> {
        self.args.as_ref()
    }
}
