use vm::internal::*;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct MakeObject;

impl Cmd for MakeObject {
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

#[derive(Clone, Debug)]
pub struct FSet;

impl Cmd for FSet {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 3);
        let mut obj = cir_extract!(args[0] => mut Object)?;
        let name = cir_extract!(args[1] => String)?;
        obj.insert(&name, args[2].value.clone());

        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct FMut;

impl Cmd for FMut {
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

#[derive(Clone, Debug)]
pub struct RmField;

impl Cmd for RmField {
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
