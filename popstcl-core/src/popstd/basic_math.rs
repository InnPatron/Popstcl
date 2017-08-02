#![allow(unused_variables)]
use vm::internal::*;

#[derive(Clone, Debug)]
pub struct Add;

impl Cmd for Add {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(&args, 1);
        let mut accu = 0.;

        for input in args.iter() {
            accu += *cir_extract!(input => Number)?;
        }

        Ok(ExecSignal::NextInstruction(Some(accu.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Sub;

impl Cmd for Sub {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(&args, 2);
        let mut accu = 0.;

        let mut iter = args.iter(); 
        {
            let input = iter.next().unwrap();
            accu = *cir_extract!(input => Number)?;
        }

        for input in iter {
            accu -= *cir_extract!(input => Number)?;
        }
        
        Ok(ExecSignal::NextInstruction(Some(accu.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Mul;

impl Cmd for Mul {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(&args, 2);
        let mut accu = 0.;

        let mut iter = args.iter(); 
        {
            let input = iter.next().unwrap();
            accu = *cir_extract!(input => Number)?;
        }

        for input in iter {
            accu *= *cir_extract!(input => Number)?;
        }
        
        Ok(ExecSignal::NextInstruction(Some(accu.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Div;

impl Cmd for Div {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        min_args!(&args, 2);
        let mut accu = 0.;

        let mut iter = args.iter(); 
        {
            let input = iter.next().unwrap();
            accu = *cir_extract!(input => Number)?;

        }

        for input in iter {
            let divisor = *cir_extract!(input => Number)?;
            if divisor == 0.0 {
                accu = 0.0;
            } else {
                accu /= divisor;
            }
        }
        
        Ok(ExecSignal::NextInstruction(Some(accu.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Inc;

impl Cmd for Inc {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        let number = cir_extract!(args[0] => Number)?;
        let new_number = *number + 1.0;

        Ok(ExecSignal::NextInstruction(Some(new_number.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Dec;

impl Cmd for Dec {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        let number = cir_extract!(args[0] => Number)?;
        let new_number = *number - 1.0;

        Ok(ExecSignal::NextInstruction(Some(new_number.into())))
    }
}

#[derive(Clone, Debug)]
pub struct Negate;

impl Cmd for Negate {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        let number = cir_extract!(args[0] => Number)?;
        let new_number = *number * -1.0;

        Ok(ExecSignal::NextInstruction(Some(new_number.into())))
    }
}
