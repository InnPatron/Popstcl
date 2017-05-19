use ast::*;
use super::err::ParseErr;
use super::lexer::{tokenize, Token};
use std::iter::Peekable;
use namespace::Namespace;

const TRUE_STR: &'static str = "true";
const FALSE_STR: &'static str = "false";

pub fn parse_program(input: &str) -> Result<Program, ParseErr> {
    parse_statement_seq(input).map(|vec| Program { code: vec })
}

pub fn parse_statement_seq(input: &str) -> Result<Vec<Statement>, ParseErr> {
    parse_tokenized_seq(&tokenize(input))
}

pub fn parse_arg_list(input: &str) -> Result<Option<Statement>, ParseErr> {
    parse_word_seq(&mut tokenize(input).iter().peekable())
}

fn parse_tokenized_seq(seq: &[Token]) -> Result<Vec<Statement>, ParseErr> {
    let mut seq_stream = seq.iter().peekable();
    let mut stmt_seq = Vec::new();
    let mut current_stmt = Vec::new();

    //Go through token sequence
    //If Token t is before a Token::Semicolon, feed to parse_word and push to current_stmt
    //Find Token::Semicolon => push current_stmt to stmt_seq
    while let Some(next) = seq_stream.next() {
        if let &Token::Semicolon = next {
            if current_stmt.len() != 0 {
                stmt_seq.push(Statement::new(current_stmt.clone()));
                current_stmt.clear();
            }
        } else {
            match parse_word(next, &mut seq_stream) {
                Ok(some_w) => {
                    if let Some(w) = some_w {
                        current_stmt.push(w);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    Ok(stmt_seq)
}

/// word_seq -> [word]*;
pub fn parse_word_seq<'a, 'b, I>(seq: &'b mut Peekable<I>) -> Result<Option<Statement>, ParseErr>
    where I: Iterator<Item = &'a Token> + 'a
{
    let mut seq_stream = seq;
    let mut result = Vec::new();

    while let Some(ref next) = seq_stream.next() {
        match parse_word(next, &mut seq_stream) {
            Ok(some_w) => {
                if let Some(w) = some_w {
                    result.push(w)
                }
            }
            Err(e) => return Err(e),
        }
    }
    if result.len() == 0 {
        for t in seq_stream {
            if let &Token::Whitespace(_) = t {
                continue;
            } else {
                panic!("result.len() > 0 if parse_word return no error and seq is not all Token::Whitespace");
            }
        }
        Ok(None)
    } else {
        Ok(Some(Statement::new(result)))
    }
}

/// word -> quoted | cmd | untouched | var_sub | parse_number | parse_bool_then_atom
/// Order matters. Atom MUST be last
fn parse_word<'a, 'b, I>(maybe_word: &Token,
                         stream: &'b mut Peekable<I>)
                         -> Result<Option<Word>, ParseErr>
    where I: Iterator<Item = &'a Token> + 'a
{
    use super::lexer::Token::*;
    match maybe_word {
        &Quote => parse_quoted(stream).map(|w| Some(w)),        //quoted -> ".*"
        &LBracket => parse_cmd(stream).map(|w| Some(w)),        //cmd -> \[word_seq\]
        &LBrace => parse_untouched(stream).map(|w| Some(w)),    //untouched -> {.*}
        &Dollar => parse_varsub(stream, Namespace::Local).map(|w| Some(w)),       //var_sub -> $path
        &At => parse_varsub(stream, Namespace::Module).map(|w| Some(w)),
        &Caret => parse_varsub(stream, Namespace::Args).map(|w| Some(w)),
        &Something(ref s) => {
            if let Some(first_char) = s.chars().nth(0) {
                if first_char == '-' || first_char.is_numeric() {
                    parse_number(maybe_word, stream).map(|w| Some(w)) //parse_number -> [-]?[0-9]*[\.]?[0-9]*
                } else {
                    parse_bool_then_atom(maybe_word).map(|w| Some(w)) //parse_bool_then_atom -> bool | atom
                }
            } else {
                panic!("Token::Something(s) was empty");
            }
        }
        &Whitespace(_) => Ok(None),
        _ => panic!("Found token {}", maybe_word.to_string()),
    }
}

///var_sub -> $path
fn parse_varsub<'a, 'b, I>(stream: &'b mut Peekable<I>, vsub_t: Namespace) -> Result<Word, ParseErr>
    where I: Iterator<Item = &'a Token> + 'a
{
    let path = parse_path(stream)?;
    if let Word::Path(path) = path {
        Ok(Word::VarSub(path, vsub_t))
    } else {
        panic!("parse_path should only return Word::Path, not {}", path);
    }
}

///path -> atom path_tail
///path_tail -> E | .path
fn parse_path<'a, 'b, I>(stream: &'b mut Peekable<I>) -> Result<Word, ParseErr>
    where I: Iterator<Item = &'a Token> + 'a
{
    let mut result = Vec::new();
    {
        if let Some(something @ &Token::Something(_)) = stream.next() {
            let first_atom = parse_atom(something)?;
            if let Word::Atom(first_atom) = first_atom {
                result.push(first_atom);
            } else {
                panic!("parse_atom should return Word::Atom, not {}", first_atom);
            }
        } else {
            return Err(ParseErr::NoMoreTokens);
        }
    }

    loop {
        if let Some(token) = stream.peek() {
            if token != &&Token::FullStop {
                break;
            }
        } else {
            break;
        }
        stream.next(); //consume Token::FullStop, look for next segment (Token::Something -> Atom) now
        if let Some(segment @ &Token::Something(_)) = stream.next() {
            let next_seg = parse_atom(segment)?;
            if let Word::Atom(atom) = next_seg {
                result.push(atom);
            } else {
                panic!("parse_atom should return Word::Atom, not {}", next_seg);
            }
        } else {
            return Err(ParseErr::ExpectedAtom);
        }
    }

    Ok(Word::Path(Path(result)))
}

/// quoted -> ".*"
fn parse_quoted<'a, 'b, I>(stream: &'b mut I) -> Result<Word, ParseErr>
    where I: Iterator<Item = &'a Token>
{
    range_get(None, Token::Quote, stream).and_then(|tok_vec| str_sub(&tok_vec))
}

/// untouched -> {.*}
fn parse_untouched<'a, 'b, I>(stream: &'b mut I) -> Result<Word, ParseErr>
    where I: Iterator<Item = &'a Token>
{
    range_get(Some(Token::LBrace), Token::RBrace, stream).map(|tok_vec| {
                                Word::Untouched(tok_vec.iter().fold(String::new(), 
                                    |mut result, token| {
                                        result.push_str(&token.to_string());
                                        result
                                    }))
                            })

}

/// cmd -> \[word_seq]
fn parse_cmd<'a, 'b, I>(stream: &'b mut I) -> Result<Word, ParseErr>
    where I: Iterator<Item = &'a Token>
{
    range_get(Some(Token::LBracket), Token::RBracket, stream)
                .and_then(|cmd_body| parse_word_seq(&mut cmd_body.iter().peekable()))
                .map(|entry| Word::CmdSub(Box::new(entry.expect("Should only be None if input stream was all whitespace"))))
}

/// Searches through the tokesn of a quoted and looks for var_sub
fn str_sub(base: &[Token]) -> Result<Word, ParseErr> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut iter = base.iter();

    while let Some(t) = iter.next() {
        match t {
            &Token::Dollar => {
                if current.is_empty() == false {
                    result.push(StrData::String(current.clone()));
                    current.clear();
                }
                if let Some(atom @ &Token::Something(_)) = iter.next() {
                    match parse_atom(atom) {
                        Ok(Word::Atom(ref atom)) => result.push(StrData::VarSub(atom.0.clone(), Namespace::Local)),
                        Ok(_) => panic!("parse_atom should only return Word::Atom"),
                        Err(e) => return Err(e),
                }
                } else {
                    return Err(ParseErr::NoVarName);
                }
            }

            &Token::At => {
                if current.is_empty() == false {
                    result.push(StrData::String(current.clone()));
                    current.clear();
                }
                if let Some(atom @ &Token::Something(_)) = iter.next() {
                    match parse_atom(atom) {
                        Ok(Word::Atom(ref atom)) => result.push(StrData::VarSub(atom.0.clone(), Namespace::Module)),
                        Ok(_) => panic!("parse_atom should only return Word::Atom"),
                        Err(e) => return Err(e),
                }
                } else {
                    return Err(ParseErr::NoVarName);
                }
            }

            _ => current.push_str(&t.to_string()),
        }
    }
    if current.is_empty() == false {
        result.push(StrData::String(current.clone()));
        current.clear();
    }

    Ok(Word::str_sub_from(result))
}

/// Collects tokens until it reaches the end token,
/// If a repeat token is defined, then it has a counter that increments when a repeat token is
/// found and decrements when a end token is found. This is for embedding and brace/bracket
/// matching.
/// TODO: is the cloning necessary? Try taking another inner slice
fn range_get<'a, 'b, I>(repeat_token: Option<Token>,
                        end_token: Token,
                        stream: &'b mut I)
                        -> Result<Vec<Token>, ParseErr>
    where I: Iterator<Item = &'a Token>
{
    use super::lexer::Token::*;
    let end_char = match end_token {
        Quote => '\"',
        RBracket => ']',
        RBrace => '}',
        _ => panic!("Does not represent an end to a range of tokens"),
    };

    let mut result = Vec::new();
    let mut found_end = false;
    let mut repeats: usize = 0;
    {
        for token in stream {
            if let Some(ref repeat) = repeat_token {
                if token == repeat {
                    repeats += 1;
                }
            }
            if token == &end_token {
                if repeats == 0 {
                    found_end = true;
                    break;
                } else {
                    repeats -= 1;
                }
            }
            result.push(token.clone())
        }
    }

    if found_end {
        Ok(result)
    } else {
        Err(ParseErr::CharNotFound(end_char))
    }
}

/// parse_bool_then_atom -> bool | atom
fn parse_bool_then_atom(maybe: &Token) -> Result<Word, ParseErr> {
    parse_bool(maybe).or(parse_atom(maybe))
}

/// bool -> true | false
fn parse_bool(maybe: &Token) -> Result<Word, ParseErr> {
    if let &Token::Something(ref maybe) = maybe {
        if maybe == TRUE_STR {
            Ok(Word::Bool(true))
        } else if maybe == FALSE_STR {
            Ok(Word::Bool(false))
        } else {
            Err(ParseErr::NotBool(maybe.to_string()))
        }
    } else {
        panic!("Should have been Token::Something");
    }
}

/// atom -> [^0-9].*
/// TODO: This function CAN produce Atom("true") or Atom("false"). Is this desireable?
fn parse_atom(maybe: &Token) -> Result<Word, ParseErr> {

    if let &Token::Something(ref maybe) = maybe {
        let mut iter = maybe.chars();
        if iter.next().unwrap().is_numeric() == true {
            panic!("parse_atom should not have been called if first char was a number");
        }
        Ok(Word::Atom(From::from(maybe.to_string())))
    } else {
        panic!("Should have been Token::Something");
    }
}

///parse_number -> [-]?[0-9]*[\.]?[0-9]*
fn parse_number<'a, 'b, 'c, I>(maybe: &'c Token,
                               stream: &'b mut Peekable<I>)
                               -> Result<Word, ParseErr>
    where I: Iterator<Item = &'a Token>
{
    let maybe = if let &Token::Something(ref string) = maybe {
        string
    } else {
        panic!("Input should have been Token::Something");
    };
    {
        let mut iter = maybe.chars();
        let mut found_dot = false;

        let first = iter.next().unwrap();
        if first.clone().is_numeric() == false && first != '-' {
            return Err(ParseErr::UnexpectedChar(first, maybe.to_string()));
        }

        for c in iter {
            if c.clone().clone().is_numeric() == false {
                return Err(ParseErr::UnexpectedChar(c, maybe.to_string()));
            }
        }
    }
    if let Some(next) = stream.peek() {
        if *next != &Token::FullStop {
            return Ok(Word::Number(maybe.parse::<f64>().unwrap()));
        }
    }

    stream.next(); //found FullStop, proceeding to parse rest
    if let Some(next) = stream.peek() {
        if let &&Token::Something(_) = next {
            ()
        } else {
            return Ok(Word::Number(maybe.parse::<f64>().unwrap()));
        }
    } else {
        return Ok(Word::Number(maybe.parse::<f64>().unwrap()));
    }

    let next = stream.next();
    if let Some(&Token::Something(ref s)) = next {
        for char in s.chars() {
            if char.is_numeric() == false {
                return Err(ParseErr::UnexpectedChar(char, maybe.to_string()));
            }
        }
        let num_string = format!("{}.{}", maybe, s).to_string();
        return Ok(Word::Number(num_string.parse::<f64>().unwrap()));
    } else {
        panic!("Previous if let should have caught NO Token::Something. {} {:?}",
               maybe,
               next);
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use namespace::Namespace;

    #[test]
    fn test_parse_localvarsub() {
        //Word::VarSub test
        let word = "$_a23;";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(Statement::new(vec![Word::VarSub(From::from("_a23".to_string()), Namespace::Local)]),
                   result[0]);
    }

    #[test]
    fn test_parse_modulevarsub() {
        //Word::VarSub test
        let word = "@_a23;";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(Statement::new(vec![Word::VarSub(From::from("_a23".to_string()), Namespace::Module)]),
                   result[0]);
    }

    #[test]
    fn test_parse_number() {
        //--------------
        //Word::Number test
        let word = "-123.5;";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(Statement::new(vec![Word::Number(-123.5f64)]), result[0]);
    }

    #[test]
    fn test_parse_neg_number() {
        let word = "-1337.5;";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(Statement::new(vec![Word::Number(-1337.5_f64)]), result[0]);
    }

    #[test]
    fn test_parse_bool() {
        //--------------
        //Word::Bool test
        let word = "false;";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(Statement::new(vec![Word::Bool(false)]), result[0]);
    }

    #[test]
    fn test_parse_arg_list() {
        let proc_args = parse_arg_list("number1 number2").unwrap().unwrap();
        let mut args = Vec::new();
        for arg in proc_args.all() {
            if let Word::Atom(atom) = arg {
                args.push(atom);
            } else {
                panic!("Did not expect {}", arg);
            }
        }
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn test_parse_untouched() {
        //--------------
        //Word::Untouched test
        let word = "{untoucheda $123};";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(Statement::new(vec![Word::Untouched("untoucheda $123".to_string())]),
                   result[0]);
    }

    #[test]
    fn test_parse_string() {
        let word = "\" $var 123 2$var2@var3\";";
        let result = parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(result[0],
                   Statement::new(vec![Word::StrSub(StrSub(vec![
                                        StrData::String(" ".to_string()),
                                        StrData::VarSub(From::from("var".to_string()), Namespace::Local),
                                        StrData::String(" 123 2".to_string()),
                                        StrData::VarSub(From::from("var2".to_string()), Namespace::Local),
                                        StrData::VarSub(From::from("var3".to_string()), Namespace::Module),
                                        ]))]));
    }

    #[test]
    fn embed_cmd() {
        let cmd = "add [add 2 [add 1 3]];";
        let result = parse_tokenized_seq(&tokenize(cmd)).unwrap();

        let cmd_1 = Word::CmdSub(Box::new(Statement::new(vec![
                                            Word::Atom(From::from("add".to_string())),
                                            Word::Number(1f64),
                                            Word::Number(3f64),
        ])));
        let cmd_2 = Word::CmdSub(Box::new(Statement::new(vec![
                                            Word::Atom(From::from("add".to_string())),
                                            Word::Number(2f64),
                                            cmd_1,
        ])));
        assert_eq!(result[0],
                   Statement::new(vec![Word::Atom(From::from("add".to_string())), cmd_2]));
    }

    #[test]
    fn test_parse_tokenized_seq() {
        let seq = "add 1 [add 3 1 test{123} \"$var _123\"];";
        let tokenized = tokenize(seq);

        let result = parse_tokenized_seq(&tokenized).unwrap();
        assert_eq!(result[0],
                    Statement::new(vec![
                               Word::Atom(From::from("add".to_string())),
                               Word::Number(1f64),
                               Word::CmdSub(Box::new(Statement::new(vec![
                                                                Word::Atom(From::from("add".to_string())),
                                                                Word::Number(3f64),
                                                                Word::Number(1f64),
                                                                Word::Atom(From::from("test".to_string())),
                                                                Word::Untouched("123".to_string()),
                                                                Word::StrSub(StrSub(
                                                                    vec![
                                                                    StrData::VarSub(From::from("var".to_string()), Namespace::Local),
                                                                    StrData::String(" _123".to_string()),
                                                                    ]
                                                                )),
                               ])))
                    ]))
    }
}
