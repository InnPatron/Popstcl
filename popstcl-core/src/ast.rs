use std::fmt;
use std::slice::Iter;
use std::ops::Deref;
use std::rc::Rc;
use namespace::Namespace;
use line_info::LineInfo;

#[macro_use]
macro_rules! word {
    ($kind: expr, $line_info: expr) => { Word { kind: $kind, line_info: $line_info } }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub code: Vec<Statement>,
    pub program_string: Rc<String>
}

impl Program {
    pub fn iter<'a>(&'a self) -> ProgramIter<'a> {
        ProgramIter { code: self.code.iter() }
    }
}

pub struct ProgramIter<'a> {
    code: Iter<'a, Statement>
}

impl<'a> Iterator for ProgramIter<'a> {
    type Item = &'a Statement;
    fn next(&mut self) -> Option<&'a Statement> {
        self.code.next()
    }
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub words: Vec<Word>,
    pub original_string: Rc<String>,
    pub line_info: LineInfo,
}

impl PartialEq for Statement {
    fn eq(&self, other: &Statement) -> bool {
        if self.words.len() != other.words.len() {
            return false;
        }

        for (ref l, ref r) in self.words.iter().zip(other.words.iter()) {
            if l.kind != r.kind {
                return false;
            }
        }

        true
    }
}

impl Statement {
    pub fn new(words: Vec<Word>, original_string: Rc<String>) -> Statement {
        assert!(words.len() > 0);
        let line_info = LineInfo::collapse(&words.iter()
                                                 .map(|word: &Word| word.line_info.clone())
                                                 .collect::<Vec<_>>());
        Statement { 
            words: words, 
            original_string: original_string,
            line_info: line_info
        }
    }

    pub fn first(&self) -> Word {
        self.words[0].clone()
    }

    pub fn args(&self) -> Vec<Word> {
        let mut ret = self.words.clone();
        ret.remove(0);
        ret
    }

    pub fn to_string(&self) -> String {
        self.words
            .iter()
            .fold(String::new(), |mut result, word| {
                result.push_str(&word.kind.to_string());
                result
            })
    }

    pub fn all(&self) -> Vec<Word> {
        self.words.clone()
    }

    pub fn get_args(&self) -> Vec<&Word> {
        let mut iter = self.words.iter();
        iter.next();
        iter.collect::<Vec<_>>()
    }

    pub fn get_all(&self) -> Vec<&Word> {
        self.words.iter().collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
pub struct Word {
    pub kind: WordKind,
    pub line_info: LineInfo,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WordKind {
    Atom(Atom),
    StrSub(StrSub),
    Number(f64),
    Bool(bool),
    VarSub(Path, Namespace),
    CmdSub(Box<Statement>),
    Untouched(String),
    Path(Path),
}

impl WordKind {
    pub fn str_sub_from(vec: Vec<StrData>) -> WordKind {
        WordKind::StrSub(StrSub(vec))
    }

    pub fn to_string(&self) -> String {
        use self::WordKind::*;
        match self {
            &Atom(ref s) => s.0.clone(),
            &StrSub(ref sub) => {
                unimplemented!();
            }
            &Number(n) => n.to_string(),
            &Bool(b) => b.to_string(),
            &VarSub(ref path, ref namespace) => path.to_string(),
            &Untouched(ref str) => str.clone(),
            &CmdSub(ref entry) => {
                let mut result = "[".to_string();
                result.push_str(&entry.to_string());
                result.push(']');
                result
            }
            &Path(ref path) => unimplemented!(),
        }
    }
}

impl fmt::Display for WordKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &WordKind::Atom(ref s) => write!(f, "Single: {}", s),
            &WordKind::StrSub(ref s) => write!(f, "String: {}", s),
            &WordKind::VarSub(ref path, ref namespace) => write!(f, "Path: {}", &path.to_string()),
            &WordKind::Number(num) => write!(f, "Number: {}", num),
            &WordKind::Bool(b) => write!(f, "Bool: {}", b),
            &WordKind::CmdSub(ref e) => write!(f, "CmdSub: {}", e.first().kind),
            &WordKind::Untouched(ref s) => write!(f, "Untouched: {}", s),
            &WordKind::Path(ref vec) => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Atom(pub String);

impl Deref for Atom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<String> for Atom {
    fn from(val: String) -> Atom {
        Atom(val)
    }
}

impl Atom {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path(pub Vec<PathSegment>);

impl fmt::Display for self::Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl Path {
    pub fn to_string(&self) -> String {
        assert!(self.0.len() > 0);
        let mut iter = self.0.iter();
        let first = iter.next().unwrap();
        let mut result = first.segment.to_string();
        for segment in iter {
            result.push_str(&*segment.segment);
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct PathSegment {
    pub segment: Atom,
    pub line_info: LineInfo
}

impl PathSegment {
    pub fn new(segment: Atom, line_info: LineInfo) -> PathSegment {
        PathSegment {
            segment: segment,
            line_info: line_info
        }
    }
}

impl PartialEq for PathSegment {
    fn eq(&self, other: &PathSegment) -> bool {
        self.segment == other.segment
    }
}

impl Deref for PathSegment {
    type Target = Atom;
    fn deref(&self) -> &Self::Target {
        &self.segment
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StrSub(pub Vec<StrData>);

impl fmt::Display for StrSub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               self.0
                   .iter()
                   .fold(String::new(), |mut result, data| {
            result.push_str(&data.to_string());
            result
        }))
    }
}

#[derive(Clone, Debug)]
pub enum StrData {
    String(String),
    VarSub(String, Namespace, LineInfo),
    CmdSub,
}

impl PartialEq for StrData {
    fn eq(&self, other: &StrData) -> bool {
        use self::StrData::*;
        match (self, other) {
            (&String(ref lhs), &String(ref rhs)) => lhs == rhs,
            (&VarSub(ref lhstr, ref lhnamespace, _), &VarSub(ref rhstr, ref rhnamespace, _)) => lhstr == rhstr && lhnamespace == rhnamespace,
            (&CmdSub, &CmdSub) => unimplemented!(),
            _ => false,
        }
    }
}

impl ToString for StrData {
    fn to_string(&self) -> String {
        match self {
            &StrData::String(ref s) => s.clone(),
            &StrData::VarSub(ref s, ref namespace, _) => {
                let mut r = String::new();
                match namespace {
                    &Namespace::Local => r.push('$'),
                    &Namespace::Module => r.push('@'),
                    &Namespace::Args => r.push('^'),
                }
                r.push_str(s);
                r
            }

            &StrData::CmdSub => unimplemented!(),
        }
    }
}
