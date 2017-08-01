#![allow(unused_variables)]
use vm::internal::*;

#[derive(Clone, Debug)]
pub struct RefEq;

impl Cmd for RefEq {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 2);

        let lhs = &args[0].value;
        let rhs = &args[1].value;

        Ok(ExecSignal::NextInstruction(Some(RcValue::ptr_eq(lhs, rhs).into())))
    }
}

#[derive(Clone, Debug)]
pub struct RefInEq;

impl Cmd for RefInEq {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 2);

        let lhs = &args[0].value;
        let rhs = &args[1].value;

        Ok(ExecSignal::NextInstruction(Some((!RcValue::ptr_eq(lhs, rhs)).into())))
    }
}

#[derive(Clone, Debug)]
pub struct Eq;

impl Cmd for Eq {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let lhs = &args[0];
        let rhs = &args[1];

        Ok(ExecSignal::NextInstruction(Some((lhs == rhs).into())))
    }
}

#[derive(Clone, Debug)]
pub struct InEq;

impl Cmd for InEq {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let lhs = &args[0];
        let rhs = &args[1];

        Ok(ExecSignal::NextInstruction(Some((lhs != rhs).into())))
    }
}

#[derive(Clone, Debug)]
pub struct GreaterThan;

impl Cmd for GreaterThan {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let lhs = *cir_extract!(args[0] => Number)?;
        let rhs = *cir_extract!(args[1] => Number)?;

        Ok(ExecSignal::NextInstruction(Some((lhs > rhs).into())))
    }
}

#[derive(Clone, Debug)]
pub struct GreaterThanEq;

impl Cmd for GreaterThanEq {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let lhs = *cir_extract!(args[0] => Number)?;
        let rhs = *cir_extract!(args[1] => Number)?;

        Ok(ExecSignal::NextInstruction(Some((lhs >= rhs).into())))
    }
}

#[derive(Clone, Debug)]
pub struct LessThan;

impl Cmd for LessThan {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let lhs = *cir_extract!(args[0] => Number)?;
        let rhs = *cir_extract!(args[1] => Number)?;

        Ok(ExecSignal::NextInstruction(Some((lhs < rhs).into())))
    }
}

#[derive(Clone, Debug)]
pub struct LessThanEq;

impl Cmd for LessThanEq {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(&args, 2);

        let lhs = *cir_extract!(args[0] => Number)?;
        let rhs = *cir_extract!(args[1] => Number)?;

        Ok(ExecSignal::NextInstruction(Some((lhs <= rhs).into())))
    }
}
