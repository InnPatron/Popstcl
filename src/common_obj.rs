use std::collections::HashMap;
use ast::*;
use err::*;

pub enum BindType {
    Int,
    Float,
    Long,
    Double,
    String,
    Void,
}

pub enum Binding {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
    Command(Box<Command>),
}

pub struct Context {
    bindings: HashMap<String, Binding>,
}

impl Context {
    fn add_binding(&mut self, label: &str, binding: Binding) -> bool {
        match self.bindings.insert(label.to_string(), binding) {
            Some(_) => true,
            None => false,
        }
    }
}

pub trait Command {
    fn get_type(&self) -> BindType;
    fn execute(&self, context: &mut Context, args: &Vec<Word>) -> Result<(), ExecErr>;
}
