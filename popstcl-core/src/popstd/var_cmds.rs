#![allow(unused_variables)]
use vm::internal::*;
use itertools::Itertools;

/// popstcl VM command in Rust
///
/// # VM
/// Takes variable tuples of form (Word::String, Value)
/// Does NOT return any value
///
#[derive(Clone, Debug)]
pub struct Let(pub Namespace);

impl Cmd for Let {
    fn execute<'a, 'b, 'c>(&'a self, stack: &'b mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        mod_args!(&args, 2);
        let module = get_module!(self.0, stack);

        for (maybe_name, value) in args.iter().tuples() {
            let name = cir_extract!(maybe_name => String)?;
    
            let value = value.value.clone();

            module.insert(&name,
						  value)
                  .map_err(|oerr| CmdErr::ObjectErr(oerr))?;
        }
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct Set(pub Namespace);

impl Cmd for Set {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);
        let module = get_module!(self.0, stack);

        let maybe_name = &args[0];
        let name = cir_extract!(maybe_name => String)?;
    
        let value = args[1].value.clone();

        module.insert(&name, 
                      value.clone())
              .map_err(|oerr| CmdErr::ObjectErr(oerr)
                      )?;
        Ok(ExecSignal::NextInstruction(Some(value.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Mutate(pub Namespace);

impl Cmd for Mutate {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);
        let module = get_module!(self.0, stack);

        let maybe_name = &args[0];
        let name = cir_extract!(maybe_name => String)?;
    
        let value = args[1].value.inner_clone();

        match module.get(&name) {
            Ok(rcval) => {
                *rcval.borrow_mut() = value;
            }

            Err(_) => {
                module.insert(&name, 
                              value.into())
                .map_err(|oerr| CmdErr::ObjectErr(oerr))?;
            }
        }

        Ok(ExecSignal::NextInstruction(None))
    }
}
