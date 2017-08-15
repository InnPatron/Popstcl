use vm::internal::*;
use itertools::Itertools;

/// args -> [string value]*
///
/// (field, value)* = args
///
/// Creates an object. Inserts a variable with the given names and given value. The value is
/// obtained by incrementing the ref count (NO DEEP CLONES PERFORMED).
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct MakeObject;

impl Cmd for MakeObject {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        mod_args!(args, 2);
        let mut obj = StdObject::empty();
        for (maybe_name, value) in args.iter().tuples() {
            let name = cir_extract!(maybe_name => String)?;
            let value = value.value.clone();
            obj.insert(&name, value);
        }

        Ok(ExecSignal::NextInstruction(Some(obj.into())))
    }
}

/// args -> object string value
///
/// **REFERENCE ASSIGNMENT**
///
/// Given an object, override a field whose name is the given string with the provided ref-counted
/// pointer.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct FSet;

impl Cmd for FSet {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 3);
        let mut obj = cir_extract!(args[0] => mut Object)?;
        let name = cir_extract!(args[1] => String)?;
        obj.insert(&name, args[2].value.clone());

        Ok(ExecSignal::NextInstruction(None))
    }
}

/// args -> object string value
///
/// **VALUE ASSIGNMENT**
///
/// Given an object, retrieve a mutable reference to a field with the string. Dereference the field
/// and set it to the derefenced value.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct FMut;

impl Cmd for FMut {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 3);
        let mut obj = cir_extract!(args[0] => mut Object)?;
        let name = cir_extract!(args[1] => String)?;
        let value = args[2].value.inner_clone();
        
        let field = match obj.get(&name) {
            Ok(value) => value,
            Err(_) => {
                let v: RcValue = (0.0).into();
                obj.insert(&name, v.clone());
                v
            },
        };

        *field.borrow_mut() = value;

        Ok(ExecSignal::NextInstruction(None))
    }
}

/// args -> object string
///
/// Given an object, remove the binding.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct RmField;

impl Cmd for RmField {
    #[allow(unused_variables)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 2);
        let mut obj = cir_extract!(args[0] => mut Object)?;
        let name = cir_extract!(args[1] => String)?;
        match obj.remove(&name) {
            Some(val) => Ok(ExecSignal::NextInstruction(Some(val))),
            None => Ok(ExecSignal::NextInstruction(None)),
        }
    }
}
