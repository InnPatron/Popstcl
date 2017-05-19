#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LBracket, //[
    RBracket, //]
    LBrace, //{
    RBrace, //}
    Dollar, //$
    Quote, //"
    Semicolon, //;
    At, //@
    FullStop, //.
    Caret, //^
    Whitespace(char),
    Something(String),
}

macro_rules! push_current {


    ($string: ident, $result: ident, $tail: expr) => {{
        if $string.is_empty() == false {
            $result.push(Token::Something($string.clone()));
            $string.clear();
        }
        $result.push($tail);
    }};
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut current = String::new();
    for char in input.chars() {
        match char {
            '[' => push_current!(current, result, Token::LBracket),
            ']' => push_current!(current, result, Token::RBracket),
            '{' => push_current!(current, result, Token::LBrace),
            '}' => push_current!(current, result, Token::RBrace), 
            '\"' => push_current!(current, result, Token::Quote), 
            ';' => push_current!(current, result, Token::Semicolon),
            '@' => push_current!(current, result, Token::At),
            '$' => push_current!(current, result, Token::Dollar),
            '.' => push_current!(current, result, Token::FullStop),
            '^' => push_current!(current, result, Token::Caret),
            c @ _ => {
                if c.is_whitespace() {
                    push_current!(current, result, Token::Whitespace(c));
                } else {
                    current.push(c);
                }
            }
        }
    }
    if current.len() > 0 {
        result.push(Token::Something(current.clone()));
    }
    result
}

impl Token {
    pub fn to_string(&self) -> String {
        use self::Token::*;
        match self {
            &LBracket => "[".to_owned(),
            &RBracket => "]".to_owned(),
            &LBrace => "{".to_owned(),
            &RBrace => "}".to_owned(),
            &Dollar => "$".to_owned(),
            &Quote => "\"".to_owned(),
            &Semicolon => ";".to_owned(),
            &At => "@".to_owned(),
            &FullStop => ".".to_owned(),
            &Caret => "^".to_owned(),
            &Whitespace(char) => char.to_string(),
            &Something(ref s) => s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[test]
    fn tokenize_single_statement_test() {
        use super::Token::*;
        let result = tokenize("test\t[bb!!12 13213 \" fdas\"!] ;{123\n #halp};");
        assert_eq!(result,
                   vec![Something("test".to_string()),
                        Whitespace('\t'),
                        LBracket,
                        Something("bb!!12".to_string()),
                        Whitespace(' '),
                        Something("13213".to_string()),
                        Whitespace(' '),
                        Quote,
                        Whitespace(' '),
                        Something("fdas".to_string()),
                        Quote,
                        Something("!".to_string()),
                        RBracket,
                        Whitespace(' '),
                        Semicolon,
                        LBrace,
                        Something("123".to_string()),
                        Whitespace('\n'),
                        Whitespace(' '),
                        Something("#halp".to_string()),
                        RBrace,
                        Semicolon]);
    }

    #[test]
    fn tokenize_varsub() {
        use super::Token::*;
        let result = tokenize("$abc");
        assert_eq!(result, vec![Dollar, Something("abc".to_string())]);
    }
}
