use line_info::*;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    ($buffer: ident, $result: ident, $tail: expr) => {{
        if $buffer.is_empty() == false {
            {
                let (first_i, last_i) = get_start_and_end(&$buffer);
                let something = $buffer.iter().fold(String::new(), |mut result, &(_, char)| { result.push(char); result });
                $result.push(Token::new(TokenKind::Something(something),
                                    range!(first_i, last_i)));
            }
            $buffer.clear();
        }
        $result.push($tail);
    }};
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut buffer: Vec<(usize, char)> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        match char {
            '[' => push_token!(buffer, result, Token::new(TokenKind::LBracket, location!(i))),
            ']' => push_token!(buffer, result, Token::new(TokenKind::RBracket, location!(i))),
            '{' => push_token!(buffer, result, Token::new(TokenKind::LBrace, location!(i))),
            '}' => push_token!(buffer, result, Token::new(TokenKind::RBrace,location!(i))),
            '\"' => push_token!(buffer, result, Token::new(TokenKind::Quote, location!(i))),
            ';' => push_token!(buffer, result, Token::new(TokenKind::Semicolon, location!(i))),
            '@' => push_token!(buffer, result, Token::new(TokenKind::At, location!(i))),
            '$' => push_token!(buffer, result, Token::new(TokenKind::Dollar, location!(i))),
            '.' => push_token!(buffer, result, Token::new(TokenKind::FullStop,location!(i))),
            '#' => push_token!(buffer, result, Token::new(TokenKind::Pound, location!(i))),
            '/' => push_token!(buffer, result, Token::new(TokenKind::Backslash, location!(i))),
            c @ _ => {
                if c.is_whitespace() {
                    push_token!(buffer, result, Token::new(TokenKind::Whitespace(c), location!(i)));
                } else {
                    buffer.push((i, c));
                }
            }
        }
    }
    if buffer.len() > 0 {
        let (first_i, last_i) = get_start_and_end(&buffer);
        let something = buffer.iter().fold(String::new(), |mut result, &(_, char)| { result.push(char); result });
        result.push(Token::new(TokenKind::Something(something), 
                               range!(first_i, last_i)));
    }
    result
}

fn get_start_and_end(buffer: &[(usize, char)]) -> (usize, usize) {
    assert!(buffer.len() > 0);
    let first_i = buffer[0].0;
    let last_i = buffer.last().unwrap().0 + 1;     //Add one b/c the indices are meant to serve as a inclusive start, exclusive end range (Unless inclusive ranges are a thing now?)
    (first_i, last_i)
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

    #[test]
    fn line_info() {
        macro_rules! info_test { 
            ($lhs: expr, $rhs: expr) => {
                let info = tokenize($rhs).into_iter().map(|token| token.line_info).collect::<Vec<_>>();
                assert_eq!($lhs, info);
            }
        }

        info_test!(vec![location!(0)], "a");
        info_test!(vec![location!(0), 
                   range!(1, 4)
        ],
        ";abc");
        info_test!(vec![
                   range!(0, 10),
                   location!(10),
        ],
        "0123456789@");
        info_test!(vec![
                   range!(0,3),
                   location!(3),
                   location!(4),
                   range!(5, 8)
        ],
        "012# abc");

    }
}
