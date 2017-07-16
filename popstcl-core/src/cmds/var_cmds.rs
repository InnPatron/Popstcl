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
    fn execute<'a, 'b, 'c>(&'a self, stack: &'b mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        mod_args!(&args, 2);
        let module = get_module!(self.0, stack);

        for (maybe_name, value) in args.iter().tuples() {
            let name = cir_extract!(maybe_name => String)?.inner();
    
            let value = value.value.clone();

            module.insert(&name,
						  value)
                  .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                                     dinsertion!(maybe_name.dinfo.line_info.clone(),
                                                                 maybe_name.dinfo)
                                                     )
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

        let maybe_name = &args[0];
        let name = cir_extract!(maybe_name => String)?.inner();
    
        let value = args[1].value.clone();

        module.insert(&name, 
                      value.clone())
              .map_err(|oerr| ExecErr::ObjectErr(oerr, 
                                                 dinsertion!(maybe_name.dinfo.line_info.clone(),
                                                             maybe_name.dinfo)
                                                )
                      )?;
        Ok(ExecSignal::NextInstruction(Some(value.into())))
    }
}
