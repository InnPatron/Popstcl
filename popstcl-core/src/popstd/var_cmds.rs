#![allow(unused_variables)]
use vm::internal::*;
use itertools::Itertools;

/// args -> [string value]+
///
/// **REFERENCE ASSIGNMENT**
///
/// Override variables whose names are the given strings with the provided ref-counted
/// pointers in the internal Namespace.
///
/// **MUTATING**
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

/// args -> string value
///
/// **REFERENCE ASSIGNMENT**
///
/// Override a variable whose name is the given string with the provided ref-counted
/// pointer in the internal Namespace.
///
/// Returns a ref-counted pointer to the value.
///
/// **MUTATING**
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

/// args -> string value
///
/// **VALUE ASSIGNMENT**
///
/// Retrieve a mutable reference to a variable in the internal Namespace with the given string.
/// Dereference the variable and set it to the derefenced value.
///
/// If that value does not exist in the internal Namespace, insert it.
///
/// **MUTATING**
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
