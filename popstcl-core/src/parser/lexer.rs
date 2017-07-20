use line_info::*;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line_info: LineInfo,
}

impl Token {
    fn new(kind: TokenKind, line_info: LineInfo) -> Token {
        Token {kind: kind, line_info: line_info }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LBracket, //[
    RBracket, //]
    LBrace, //{
    RBrace, //}
    Dollar, //$
    Quote, //"
    Semicolon, //;
    At, //@
    FullStop, //.
    Pound, //#
    Backslash, // /
    Whitespace(char),
    Something(String),
}

macro_rules! push_token {
    ($maybe_something: ident, $result: ident, $tail: expr) => {{
        if $maybe_something.0.is_empty() == false {
            $result.push(Token::new(TokenKind::Something($maybe_something.0.clone()),
                                    range!($maybe_something.1, $maybe_something.2)));
            $maybe_something.0.clear();
        }
        $result.push($tail);
    }};
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut maybe_something: (String, usize, usize) = (String::new(), 0, 0);

    for (i, char) in input.chars().enumerate() {
        match char {
            '[' => push_token!(maybe_something, result, Token::new(TokenKind::LBracket, location!(i))),
            ']' => push_token!(maybe_something, result, Token::new(TokenKind::RBracket, location!(i))),
            '{' => push_token!(maybe_something, result, Token::new(TokenKind::LBrace, location!(i))),
            '}' => push_token!(maybe_something, result, Token::new(TokenKind::RBrace,location!(i))),
            '\"' => push_token!(maybe_something, result, Token::new(TokenKind::Quote, location!(i))),
            ';' => push_token!(maybe_something, result, Token::new(TokenKind::Semicolon, location!(i))),
            '@' => push_token!(maybe_something, result, Token::new(TokenKind::At, location!(i))),
            '$' => push_token!(maybe_something, result, Token::new(TokenKind::Dollar, location!(i))),
            '.' => push_token!(maybe_something, result, Token::new(TokenKind::FullStop,location!(i))),
            '#' => push_token!(maybe_something, result, Token::new(TokenKind::Pound, location!(i))),
            '/' => push_token!(maybe_something, result, Token::new(TokenKind::Backslash, location!(i))),
            c @ _ => {
                if c.is_whitespace() {
                    push_token!(maybe_something, result, Token::new(TokenKind::Whitespace(c), location!(i)));
                } else {
                    if maybe_something.0.is_empty() {
                        maybe_something.1 = i;
                    } else {
                        maybe_something.2 = i;
                    }
                    maybe_something.0.push(c);
                }
            }
        }
    }
    if maybe_something.0.len() > 0 {
        result.push(Token::new(TokenKind::Something(maybe_something.0.clone()), range!(maybe_something.1, maybe_something.2)));
    }
    result
}

impl TokenKind {
    pub fn to_string(&self) -> String {
        use self::TokenKind::*;
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
            &Pound => "#".to_owned(),
            &Backslash => "/".to_owned(),
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
        use super::TokenKind::*;
        let result = tokenize("test\t[bb!!12 13213 \" fdas\"!] ;{123\n #halp};");
        assert_eq!(result.into_iter().map(|token| token.kind).collect::<Vec<_>>(),
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
                        Pound,
                        Something("halp".to_string()),
                        RBrace,
                        Semicolon]);
    }

    #[test]
    fn tokenize_varsub() {
        use super::TokenKind::*;
        let result = tokenize("$abc");
        assert_eq!(result.into_iter().map(|token| token.kind).collect::<Vec<_>>(), vec![Dollar, Something("abc".to_string())]);
    }

    #[test]
    fn tokenize_single_letter_variants() {
        use super::TokenKind::*;
        macro_rules! compare {
            ($lhs: expr, $rhs: expr) => { 
                assert_eq!($lhs.into_iter().map(|token| token.kind).collect::<Vec<_>>(), $rhs);
            }
        }
        use super::TokenKind::*;
        compare!(tokenize("a"), vec![Something("a".to_string())]);
        compare!(tokenize("#"), vec![Pound]);
        compare!(tokenize(" "), vec![Whitespace(' ')]);
        compare!(tokenize("\n "), vec![Whitespace('\n'), Whitespace(' ')]);
        compare!(tokenize("a;"), vec![Something("a".to_string()), Semicolon]);
        compare!(tokenize("$a"), vec![Dollar, Something("a".to_string())]);
        compare!(tokenize("@a;"), vec![At, Something("a".to_string()), Semicolon]);
        compare!(tokenize("a@;"), vec![Something("a".to_string()), At, Semicolon]);
        compare!(tokenize("@;a"), vec![At, Semicolon, Something("a".to_string())]);
    }
}
