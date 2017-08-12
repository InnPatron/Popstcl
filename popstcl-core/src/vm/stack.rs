use super::internal::{Module, Env, CIR, StdModule };
use std::collections::HashMap;

pub struct Stack<'a> {
    module: &'a mut StdModule,
    local_module: Option<StdModule>,
    args: Option<HashMap<String, CIR>>,
}

impl<'a> Stack<'a> {
    pub fn new_module(module: &'a mut StdModule) -> Stack<'a> {
        Stack {
            module: module,
            local_module: None,
            args: None,
        }
    }

    pub fn new_local<'b>(previous: &'a mut Stack<'b>, local: Env) -> Stack<'a> {
        Stack {
            module: previous.module,
            local_module: Some(StdModule::new(local)),
            args: None,
        }
    }

    pub fn local_with_args<'b>(previous: &'a mut Stack<'b>, local: Env, args: HashMap<String, CIR>) -> Stack<'a> {
        Stack {
            module: previous.module,
            local_module: Some(StdModule::new(local)),
            args: {
                if args.is_empty() {
                    None
                } else {
                    Some(args)
                }
            },
        }
    }

    pub fn get_local_module<'b>(&'b mut self) -> Option<&mut StdModule> {
        if let Some(ref mut module) = self.local_module {
			Some(module)
		} else {
			None
		}
    }

    pub fn get_module<'b>(&'b mut self) -> &mut StdModule {
        self.module
    }

    pub fn get_args<'b>(&'b self) -> Option<&HashMap<String, CIR>> {
        self.args.as_ref()
    }
}
