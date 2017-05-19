#![allow(unused_variables)]
#![feature(macro_rules)]
use vm::internal::*;
use itertools::Itertools;

#[macro_use]
use cmds;
/// popstcl VM command in Rust
///
/// # VM
/// Takes variable tuples of form (Word::Single, Value)
/// Does NOT return any value
///
#[derive(Clone, Debug)]
pub struct Let(pub Namespace);

impl Cmd for Let {
    fn execute<'a, 'b, 'c>(&'a self, stack: &'b mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        mod_args!(&args, 2);
        let module = get_module!(self.0, stack);

        for (name, value) in args.iter().tuples() {
            let name = cir_extract!(name => Single)?;
    
            let value = cir_extract!(value => Value)?;

            module.insert(name, 
						  value, 
						  observable_internal!(),

			)?;
        }
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct Set(pub Namespace);

impl Cmd for Set {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 2);
        let module = get_module!(self.0, stack);

        let name = cir_extract!(args[0] => Single)?;
    
        let value = cir_extract!(args[1] => Value)?;

        module.insert(name, 
                      value.clone(), 
                      observable_internal!());
        Ok(ExecSignal::NextInstruction(Some(value)))
    }
}
