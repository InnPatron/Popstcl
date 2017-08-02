use std::fmt;

use super::internal::{ExecSignal, CmdErr, Stack, CIR};

pub trait Cmd: CmdClone + fmt::Debug {
    fn execute(&self, env: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr>;
}

pub trait CmdClone {
    fn clone_box(&self) -> Box<Cmd>;
}

pub trait CmdDebug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

//I think this means that CmdClone is implemented for any static types (i.e. available @ compile
//time) implementing Cmd and Clone
//Since Cmd: CmdClone, static type implementors MUST implement Clone as well
impl<T> CmdClone for T
    where T: 'static + Cmd + Clone
{
    fn clone_box(&self) -> Box<Cmd> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Cmd> {
    fn clone(&self) -> Box<Cmd> {
        self.clone_box()
    }
}

impl<T> CmdDebug for T
    where T: 'static + Cmd + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
