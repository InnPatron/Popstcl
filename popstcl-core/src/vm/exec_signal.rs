use super::internal::RcValue;

pub enum ExecSignal {
    NextInstruction(Option<RcValue>),
    Continue,
    Return(Option<RcValue>),
    Break,
}
