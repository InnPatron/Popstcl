#![allow(unused_variables)]
#[macro_use]
use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Add;

impl Cmd for Add {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        min_args!(&args, 1);
        let mut accu = 0.;

        for input in args.iter() {
            accu += cir_extract!(input => Number)?;
        }

        Ok(ExecSignal::NextInstruction(Some(Value::Number(accu))))
    }
}

#[derive(Clone, Debug)]
pub struct Sub;

impl Cmd for Sub {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        min_args!(&args, 2);
        let mut accu = 0.;

        let mut iter = args.iter(); 
        {
            let input = iter.next().unwrap();
            accu = cir_extract!(input => Number)?;
        }

        for input in iter {
            accu -= cir_extract!(input => Number)?;
        }
        
        Ok(ExecSignal::NextInstruction(Some(Value::Number(accu))))
    }
}

#[derive(Clone, Debug)]
pub struct Mul;

impl Cmd for Mul {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        min_args!(&args, 2);
        let mut accu = 0.;

        let mut iter = args.iter(); 
        {
            let input = iter.next().unwrap();
            accu = cir_extract!(input => Number)?;
        }

        for input in iter {
            accu *= cir_extract!(input => Number)?;
        }
        
        Ok(ExecSignal::NextInstruction(Some(Value::Number(accu))))
    }
}

#[derive(Clone, Debug)]
pub struct Div;

impl Cmd for Div {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        min_args!(&args, 2);
        let mut accu = 0.;

        let mut iter = args.iter(); 
        {
            let input = iter.next().unwrap();
            accu = cir_extract!(input => Number)?;

        }

        for input in iter {
            let divisor = cir_extract!(input => Number)?;
            if divisor == 0.0 {
                accu = 0.0;
            } else {
                accu /= divisor;
            }
        }
        
        Ok(ExecSignal::NextInstruction(Some(Value::Number(accu))))
    }
}
