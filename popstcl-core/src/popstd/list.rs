#![allow(unused_variables)]
use vm::internal::*;

/// args -> value*
///
/// Takes the list of arguments, increments the ref count, and inserts them into a list. Return
/// that list.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct List;

impl Cmd for List {
    
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        let mut list = Vec::new();
        
        //Convert ALL CIR args into Values
        //Fail if CIR is NOT a value
        for argument in args.iter() {
            list.push(argument.value.clone());
        }

        Ok(ExecSignal::NextInstruction(Some(list.into())))
    }
}

/// args -> list
///
/// Takes a list and returns its count
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct ListLength;

impl Cmd for ListLength {
    
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 1);
        let list = cir_extract!(args[0] => List)?;
        Ok(ExecSignal::NextInstruction(Some((list.len() as f64).into())))
    }
}

/// args -> list number
///
/// Floors the number and uses it to index into the list. If the index is valid, it increments the
/// ref count and return the value. Otherwise, return a CmdErr::InvalidIndex.
///
/// **NON-MUTATING**
#[derive(Clone, Debug)]
pub struct ListIndex;

impl Cmd for ListIndex {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let list = cir_extract!(args[0] => List)?;

        let float_index = cir_extract!(args[1] => Number)?;

        let usize_index = float_index.floor() as i64 as usize;

        Ok(ExecSignal::NextInstruction(Some(list.inner().get(usize_index)
                              .ok_or(CmdErr::InvalidIndex(usize_index))?
                              .clone().into())
           ))
    }
}

/// args -> list number
///
/// Floors the number and uses it to index into the list. If the index is valid, remove the value
/// at the index and returns it. Otherwise, return a CmdErr::InvalidIndex.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct Remove;

impl Cmd for Remove {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let mut list = cir_extract!(args[0] => mut List)?;

        let float_index = cir_extract!(args[1] => Number)?;

        let usize_index = float_index.floor() as i64 as usize;
        
        if usize_index >= list.len() {
            return Err(CmdErr::InvalidIndex(usize_index));
        } else {
            list.remove(usize_index);
        }
        Ok(ExecSignal::NextInstruction(Some(args[0].value.clone())))
    }
}

/// args -> list value+
///
/// Takes a list and appends values to it.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct Append;

impl Cmd for Append {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(&args, 2);

        let mut list = cir_extract!(args[0] => mut List)?;
        for index in 1..args.len() {
            let value = args[index].value.clone();
            list.push(value);
        }
        Ok(ExecSignal::NextInstruction(Some(args[0].value.clone())))
    }
}

/// args -> list
///
/// Removes the last item from a list. Does **NOT** return the popped value.
///
/// **MUTATING**
#[derive(Clone, Debug)]
pub struct Pop;

impl Cmd for Pop {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 1);

        let mut list = cir_extract!(args[0] => mut List)?;
        list.pop();
        Ok(ExecSignal::NextInstruction(Some(args[0].value.clone())))
    }
}
