use vm::internal::*;

/// Create a list object and return it
/// Args: <Value>+
#[derive(Clone, Debug)]
pub struct List;

impl Cmd for List {
    
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        min_args!(&args, 1);
        let mut values = Vec::new();
        
        //Convert ALL CIR args into Values
        //Fail if CIR is NOT a value
        for argument in args.iter() {
            values.push(cir_extract!(argument => Value)?);
        }

        Ok(ExecSignal::NextInstruction(Some(Value::List(values))))
    }
}

/// Return length of list
/// Args: <Value::List>
#[derive(Clone, Debug)]
pub struct ListLength;

impl Cmd for ListLength {
    
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 1);
        let list = cir_extract!(args[0] => List)?;
        Ok(ExecSignal::NextInstruction(Some(Value::Number(list.len() as f64))))
    }
}

/// Get a list index value and return it.
/// Floors the index.
/// Args: <List> <Value::Number>
#[derive(Clone, Debug)]
pub struct ListIndex;

impl Cmd for ListIndex {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 2);

        let list = cir_extract!(args[0] => List)?;

        let float_index = cir_extract!(args[1] => Number)?;

        let usize_index = float_index.floor() as i64 as usize;

        Ok(ExecSignal::NextInstruction(Some(list.get(usize_index)
                              .ok_or(ExecErr::InvalidIndex(usize_index))?
                              .clone())
           ))
    }
}

/// Removes a value at an index.
/// Floors the index.
/// Because references do not exist (yet?), Append returns the list. TODO: references?
/// TODO: Return object with fields for list and removed item?
/// Args: <List> <Value::Number>
#[derive(Clone, Debug)]
pub struct Remove;

impl Cmd for Remove {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 2);

        let mut list = cir_extract!(args[0] => List)?.clone();

        let float_index = cir_extract!(args[1] => Number)?;

        let usize_index = float_index.floor() as i64 as usize;
        
        if usize_index >= list.len() {
            return Err(ExecErr::InvalidIndex(usize_index));
        } else {
            list.remove(usize_index);
        }
        Ok(ExecSignal::NextInstruction(Some(Value::List(list))))
    }
}

/// Append a value to the end of a list.
/// Because references do not exist (yet?), Append returns the list. TODO: references?
/// Args: <List> <Value>+
#[derive(Clone, Debug)]
pub struct Append;

impl Cmd for Append {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        min_args!(&args, 2);

        let mut list = cir_extract!(args[0] => List)?.clone();
        for index in 1..args.len() {
            let value = cir_extract!(args[index] => Value)?;
            list.push(value);
        }
        Ok(ExecSignal::NextInstruction(Some(Value::List(list))))
    }
}

/// Removes the value at the end of a list.
/// Because references do not exist (yet?), Append returns the list. TODO: references?
/// TODO: Return object with fields for list and removed item?
/// Args: <List>
#[derive(Clone, Debug)]
pub struct Pop;

impl Cmd for Pop {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(&args, 1);

        let mut list = cir_extract!(args[0] => List)?.clone();
        list.pop();
        Ok(ExecSignal::NextInstruction(Some(Value::List(list))))
    }
}
