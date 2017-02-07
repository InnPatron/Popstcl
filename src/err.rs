use common_obj::*;

pub enum ExecErr {
    InvalidArgs(Vec<BindType>),
}
