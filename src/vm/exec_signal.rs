use super::internal::Value;

pub enum ExecSignal {
    NextInstruction(Option<Value>),
    Continue,
    Return(Option<Value>),
    Break,
}
