use std::fmt;
use std::ops::Deref;
use namespace::Namespace;

pub struct Program {
    pub code: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Statement {
    words: Vec<Word>,
}

impl PartialEq for Statement {
    fn eq(&self, other: &Statement) -> bool {
        if self.words.len() != other.words.len() {
            return false;
        }

        for (ref l, ref r) in self.words.iter().zip(other.words.iter()) {
            if l != r {
                return false;
            }
        }

        true
    }
}

impl Statement {
    pub fn new(words: Vec<Word>) -> Statement {
        assert!(words.len() > 0);
        Statement { words: words }
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
                result.push_str(&word.to_string());
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

#[derive(Clone, Debug, PartialEq)]
pub enum Word {
    Atom(Atom),
    StrSub(StrSub),
    Number(f64),
    Bool(bool),
    VarSub(Path, Namespace),
    CmdSub(Box<Statement>),
    Untouched(String),
    Path(Path),
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
pub struct Path(pub Vec<Atom>);

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
        let mut result = first.to_string();
        for segment in iter {
            result.push_str(*&segment);
        }
        result
    }
}

impl From<String> for Path {
    fn from(string: String) -> Path {
        let mut path = Path(Vec::new());
        path.0.push(From::from(string));
        path
    }
}

impl Word {
    pub fn str_sub_from(vec: Vec<StrData>) -> Word {
        Word::StrSub(StrSub(vec))
    }

    pub fn to_string(&self) -> String {
        use self::Word::*;
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

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Word::Atom(ref s) => write!(f, "Single: {}", s),
            &Word::StrSub(ref s) => write!(f, "String: {}", s),
            &Word::VarSub(ref path, ref namespace) => write!(f, "Path: {}", &path.to_string()),
            &Word::Number(num) => write!(f, "Number: {}", num),
            &Word::Bool(b) => write!(f, "Bool: {}", b),
            &Word::CmdSub(ref e) => write!(f, "CmdSub: {}", e.first()),
            &Word::Untouched(ref s) => write!(f, "Untouched: {}", s),
            &Word::Path(ref vec) => unimplemented!(),
        }
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

#[derive(Clone, Debug, PartialEq)]
pub enum StrData {
    String(String),
    VarSub(String, Namespace),
    CmdSub,
}

impl StrData {
    pub fn to_string(&self) -> String {
        match self {
            &StrData::String(ref s) => s.clone(),
            &StrData::VarSub(ref s, ref namespace) => {
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

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use namespace::Namespace;
    #[test]
    fn entry_eq_test() {
        let word = "yoyo";
        assert_eq!(Statement::new(vec![Word::Atom(From::from(word.to_string()))]),
                   Statement::new(vec![Word::Atom(From::from(word.to_string()))]));

        assert_eq!(Statement::new(vec![Word::VarSub(From::from(word.to_string()), Namespace::Local)]),
                   Statement::new(vec![Word::VarSub(From::from(word.to_string()), Namespace::Local)]));

        assert_eq!(Statement::new(vec![Word::VarSub(From::from(word.to_string()), Namespace::Module)]),
                   Statement::new(vec![Word::VarSub(From::from(word.to_string()), Namespace::Module)]));
    }
}
