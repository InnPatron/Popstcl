pub struct Program {
    data: Vec<Entry>,
}

pub enum Entry {
    Line(Line),
    Proc(Proc),
}

#[derive(Clone)]
pub enum Word {
    Single(String),
    String(String),
    Number(String),
    Compound(Vec<Word>),
}

pub struct Line {
    first: String,
    next: Vec<Word>,
}

pub struct Proc {
    name: String,
    cmds: Vec<Word>,
}
