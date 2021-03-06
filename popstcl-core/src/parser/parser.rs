use std::rc::Rc;
use std::iter::Peekable;

use ast::*;
use super::err::ParseErr;
use super::lexer::{tokenize, Token, TokenKind};
use namespace::Namespace;
use line_info::LineInfo;

const TRUE_STR: &'static str = "true";
const FALSE_STR: &'static str = "false";

pub struct Parser {
    original_string: Rc<String>
}

impl Parser {

    pub fn new(input: &str) -> Parser {
        Parser { original_string: Rc::new(input.to_string()) }
    }

    pub fn parse_program(&self, input: &str) -> Result<Program, ParseErr> {
        self.parse_statement_seq(input).map(|vec| Program { code: vec, program_string: self.original_string.clone() })
    }

    pub fn parse_statement_seq(&self, input: &str) -> Result<Vec<Statement>, ParseErr> {
        self.parse_tokenized_seq(&tokenize(input))
    }

    pub fn parse_arg_list(&self, input: &str) -> Result<Option<Statement>, ParseErr> {
        self.parse_word_seq(&mut tokenize(input).iter().peekable())
    }

    fn parse_tokenized_seq(&self, seq: &[Token]) -> Result<Vec<Statement>, ParseErr> {
        let mut seq_stream = seq.iter().peekable();
        let mut stmt_seq = Vec::new();
        let mut current_stmt = Vec::new();

        //Go through token sequence
        //If Token t is before a TokenKind::Semicolon, feed to parse_word and push to current_stmt
        //Find TokenKind::Semicolon => push current_stmt to stmt_seq
        while let Some(next) = seq_stream.next() {
            if let TokenKind::Semicolon = next.kind {
                if current_stmt.len() != 0 {
                    stmt_seq.push(Statement::new(current_stmt.clone(),
                                                 self.original_string.clone(),
                                                )
                                 );
                    current_stmt.clear();
                }
            } else {
                if let Some(word) = self.parse_word(next, &mut seq_stream)? {
                    current_stmt.push(word);
                }
            }
        }

        if current_stmt.len() != 0 {
            Err(ParseErr::MissingSemicolon)
        } else {
            Ok(stmt_seq)
        }
    }

    /// word_seq -> [word]*;
    fn parse_word_seq<'a, 'b, I>(&self, seq: &'b mut Peekable<I>) -> Result<Option<Statement>, ParseErr>
        where I: Iterator<Item = &'a Token> + 'a
    {
        let mut seq_stream = seq;
        let mut result = Vec::new();

        while let Some(ref next) = seq_stream.next() {
            if let Some(word) = self.parse_word(next, &mut seq_stream)? {
                result.push(word);
            }
        }
        if result.len() == 0 {
            for t in seq_stream {
                if let TokenKind::Whitespace(_) = t.kind {
                    continue;
                } else {
                    panic!("result.len() > 0 if parse_word return no error and seq is not all TokenKind::Whitespace");
                }
            }
            Ok(None)
        } else {
            Ok(Some(Statement::new(result,
                                   self.original_string.clone(),
                                   )
                    )
               )
        }
    }

    /// word -> quoted | cmd | untouched | var_sub | parse_number | parse_bool_then_atom
    /// Order matters. Atom MUST be last
    fn parse_word<'a, 'b, I>(&self, maybe_word: &Token,
                             stream: &'b mut Peekable<I>)
                             -> Result<Option<Word>, ParseErr>
        where I: Iterator<Item = &'a Token> + 'a
    {
        use super::lexer::TokenKind::*;
        match maybe_word.kind {
            Quote => self.parse_quoted(stream).map(|w| Some(w)),        //quoted -> ".*"
            LBracket => self.parse_cmd(stream).map(|w| Some(w)),        //cmd -> \[word_seq\]
            LBrace => self.parse_untouched(stream).map(|w| Some(w)),    //untouched -> {.*}
            Dollar => self.parse_varsub(stream, Namespace::Module).map(|w| Some(w)),       //var_sub -> $path
            At => self.parse_varsub(stream, Namespace::Args).map(|w| Some(w)),
            Pound => self.parse_varsub(stream, Namespace::Local).map(|w| Some(w)),
            Backslash => self.eat_comment(stream).map(|_| None),
            Something(ref s) => {
                if let Some(first_char) = s.chars().nth(0) {
                    if first_char == '-' || first_char.is_numeric() {
                        self.parse_number(maybe_word, stream).map(|w| Some(w)) //parse_number -> [-]?[0-9]*[\.]?[0-9]*
                    } else {
                        self.parse_bool_then_atom(maybe_word).map(|w| Some(w)) //parse_bool_then_atom -> bool | atom
                    }
                } else {
                    panic!("TokenKind::Something(s) was empty");
                }
            }
            Whitespace(_) => Ok(None),
            _ => panic!("Found token {}", maybe_word.kind.to_string()),
        }
    }

    ///var_sub -> $path
    fn parse_varsub<'a, 'b, I>(&self, stream: &'b mut Peekable<I>, vsub_t: Namespace) -> Result<Word, ParseErr>
        where I: Iterator<Item = &'a Token> + 'a
    {
        let path = self.parse_path(stream)?;
        if let WordKind::Path(path_data) = path.kind {
            Ok(word!(WordKind::VarSub(path_data, vsub_t), path.line_info.clone()))
        } else {
            panic!("parse_path should only return WordKind::Path, not {}", path.kind);
        }
    }

    ///path -> atom path_tail
    ///path_tail -> E | .path
    fn parse_path<'a, 'b, I>(&self, stream: &'b mut Peekable<I>) -> Result<Word, ParseErr>
        where I: Iterator<Item = &'a Token> + 'a
    {
        let mut result = Vec::new();
        let mut line_info = Vec::new();
        {
            if let Some(something @ &Token { kind: TokenKind::Something(_), line_info: _}) = stream.next() {
                let first_atom = self.parse_atom(something)?;
                if let WordKind::Atom(atom) = first_atom.kind {
                    result.push(PathSegment::new(atom, first_atom.line_info.clone()));
                    line_info.push(first_atom.line_info.clone());
                } else {
                    panic!("parse_atom should return WordKind::Atom, not {}", first_atom.kind);
                }
            } else {
                return Err(ParseErr::NoMoreTokens);
            }
        }

        loop {
            if let Some(token) = stream.peek() {
                if token.kind != TokenKind::FullStop {
                    break;
                }
            } else {
                break;
            }
            stream.next(); //consume TokenKind::FullStop, look for next segment (Token::Something -> Atom) now
            if let Some(segment @ &Token { kind: TokenKind::Something(_), line_info: _}) = stream.next() {
                let next_seg = self.parse_atom(segment)?;
                if let WordKind::Atom(next) = next_seg.kind {
                    result.push(PathSegment::new(next, next_seg.line_info.clone()));
                    line_info.push(next_seg.line_info.clone());
                } else {
                    panic!("parse_atom should return WordKind::Atom, not {}", next_seg.kind);
                }
            } else {
                return Err(ParseErr::ExpectedAtom);
            }
        }

        let line_info = LineInfo::collapse(&line_info);
        Ok(word!(WordKind::Path(Path(result)), line_info))
    }

    /// quoted -> ".*"
    fn parse_quoted<'a, 'b, I>(&self, stream: &'b mut I) -> Result<Word, ParseErr>
        where I: Iterator<Item = &'a Token>
    {
        range_get(None, TokenKind::Quote, stream).and_then(|(tok_vec, end_info)| self.str_sub(&tok_vec, end_info))
    }

    /// untouched -> {.*}
    fn parse_untouched<'a, 'b, I>(&self, stream: &'b mut I) -> Result<Word, ParseErr>
        where I: Iterator<Item = &'a Token>
    {
        range_get(Some(TokenKind::LBrace), TokenKind::RBrace, stream).map(|(tok_vec, end_info)| {
                                    let kind = WordKind::Untouched(tok_vec.iter().fold(String::new(), 
                                        |mut result, token| {
                                            result.push_str(&token.kind.to_string());
                                            result
                                        }));
                                    let info = if tok_vec.len() == 0 {
                                        end_info
                                    } else {
                                        LineInfo::collapse(&tok_vec.iter()
                                                                         .map(|tok| tok.line_info.clone())
                                                                         .collect::<Vec<_>>())
                                    };
                                    word!(kind, info)
                                })

    }

    /// cmd -> \[word_seq]
    fn parse_cmd<'a, 'b, I>(&self, stream: &'b mut I) -> Result<Word, ParseErr>
        where I: Iterator<Item = &'a Token>
    {
        let stmt = range_get(Some(TokenKind::LBracket), TokenKind::RBracket, stream)
                    .and_then(|(cmd_body, _)| self.parse_word_seq(&mut cmd_body.iter().peekable()))?;
        {
            let stmt = stmt.ok_or(ParseErr::EmptyCmdSub)?;
                    
            let info = LineInfo::collapse(&stmt.words
                                               .iter()
                                               .map(|word| word.line_info.clone())
                                               .collect::<Vec<_>>());
            Ok(word!(WordKind::CmdSub(Box::new(stmt)), info.clone()))
        }
                    
    }

    /// Searches through the tokesn of a quoted and looks for var_sub
    fn str_sub(&self, base: &[Token], end_info: LineInfo) -> Result<Word, ParseErr> {
        let mut result = Vec::new();
        let mut line_info = Vec::new();
        let mut current = String::new();
        let mut iter = base.iter().peekable();
        while let Some(t) = iter.next() {
            match t.kind {
                TokenKind::Dollar => {
                    if current.is_empty() == false {
                        result.push(StrData::String(current.clone()));
                        current.clear();
                    }

                    let namespace = Namespace::Module;
                    if let WordKind::Path(path) = self.parse_path(&mut iter)?.kind {
                        result.push(StrData::VarSub(path, namespace));
                    } else {
                        panic!("parse_path should only return a Path");
                    }
                }

                TokenKind::Pound => {
                    if current.is_empty() == false {
                        result.push(StrData::String(current.clone()));
                        current.clear();
                    }
                    let namespace = Namespace::Local;
                    if let WordKind::Path(path) = self.parse_path(&mut iter)?.kind {
                        result.push(StrData::VarSub(path, namespace));
                    } else {
                        panic!("parse_path should only return a Path");
                    }
                }

                TokenKind::At => {
                    if current.is_empty() == false {
                        result.push(StrData::String(current.clone()));
                        current.clear();
                    }
                    let namespace = Namespace::Args;
                    if let WordKind::Path(path) = self.parse_path(&mut iter)?.kind {
                        result.push(StrData::VarSub(path, namespace));
                    } else {
                        panic!("parse_path should only return a Path");
                    }
                }

                _ => { 
                    current.push_str(&t.kind.to_string()); 
                    line_info.push(t.line_info.clone());
                },
            }
        }
        if current.is_empty() == false {
            result.push(StrData::String(current.clone()));
            current.clear();
        }

        let info = if line_info.len() == 0 {
            end_info
        } else {
            LineInfo::collapse(&line_info)
        };

        Ok(word!(WordKind::str_sub_from(result), info))
    }

    /// parse_bool_then_atom -> bool | atom
    fn parse_bool_then_atom(&self, maybe: &Token) -> Result<Word, ParseErr> {
        self.parse_bool(maybe).or(self.parse_atom(maybe))
    }

    /// bool -> true | false
    fn parse_bool(&self, maybe: &Token) -> Result<Word, ParseErr> {
        if let TokenKind::Something(ref bool_str) = maybe.kind {
            if bool_str == TRUE_STR {
                Ok(word!(WordKind::Bool(true), maybe.line_info.clone()))
            } else if bool_str == FALSE_STR {
                Ok(word!(WordKind::Bool(false), maybe.line_info.clone()))
            } else {
                Err(ParseErr::NotBool(bool_str.to_string()))
            }
        } else {
            panic!("Should have been TokenKind::Something");
        }
    }

    /// atom -> [^0-9].*
    /// TODO: This function CAN produce Atom("true") or Atom("false"). Is this desireable?
    fn parse_atom(&self, maybe: &Token) -> Result<Word, ParseErr> {

        if let TokenKind::Something(ref something) = maybe.kind {
            let mut iter = something.chars();
            if iter.next().unwrap().is_numeric() == true {
                panic!("parse_atom should not have been called if first char was a number");
            }
            Ok(word!(WordKind::Atom(From::from(something.to_owned())), maybe.line_info.clone()))
        } else {
            panic!("Should have been TokenKind::Something");
        }
    }

    ///parse_number -> [-]?[0-9]*[\.]?[0-9]*
    fn parse_number<'a, 'b, 'c, I>(&self, 
                                   maybe: &'c Token,
                                   stream: &'b mut Peekable<I>)
                                   -> Result<Word, ParseErr>
        where I: Iterator<Item = &'a Token>
    {
        let num_string = if let TokenKind::Something(ref string) = maybe.kind {
            string
        } else {
            panic!("Input should have been TokenKind::Something");
        };
        {
            let mut iter = num_string.chars();

            let first = iter.next().unwrap();
            if first.clone().is_numeric() == false && first != '-' {
                return Err(ParseErr::UnexpectedChar(first, num_string.to_string()));
            }

            for c in iter {
                if c.clone().clone().is_numeric() == false {
                    return Err(ParseErr::UnexpectedChar(c, num_string.to_string()));
                }
            }
        }
        if let Some(next) = stream.peek() {
            if next.kind != TokenKind::FullStop {
                return Ok(word!(WordKind::Number(num_string.parse::<f64>().unwrap()),
                                                 maybe.line_info.clone()));
            }
        }

        stream.next(); //found FullStop, proceeding to parse rest
        if let Some(next) = stream.peek() {
            if let TokenKind::Something(_) = next.kind {
                ()
            } else {
                return Ok(word!(WordKind::Number(num_string.parse::<f64>().unwrap()), 
                                                maybe.line_info.clone()));
            }
        } else {
            return Ok(word!(WordKind::Number(num_string.parse::<f64>().unwrap()), 
                                                maybe.line_info.clone()));
        }

        let next = stream.next();
        if let Some(&Token { kind: TokenKind::Something(ref s), line_info: _ }) = next {
            for char in s.chars() {
                if char.is_numeric() == false {
                    return Err(ParseErr::UnexpectedChar(char, num_string.to_string()));
                }
            }
            let num_string = format!("{}.{}", num_string, s).to_string();
            return Ok(word!(WordKind::Number(num_string.parse::<f64>().unwrap()), 
                                                maybe.line_info.clone()));    
        } else {
            panic!("Previous if let should have caught NO TokenKind::Something. {} {:?}",
                   num_string,
                   next);
        }
    }

    fn eat_comment<'a, 'b, I>(&self, stream: &'b mut Peekable<I>) -> Result<(), ParseErr>
        where I: Iterator<Item = &'a Token> 
    {
        if let Some(ref t) = stream.next() {
            if t.kind == TokenKind::Backslash {
                loop {
                    if let Some(ref t) = stream.next() {
                        if t.kind == TokenKind::Whitespace('\n') {
                            break; //comment ends at newline
                        }
                    } else {
                        break; //end of input; comment goes to end
                    }
                }
                Ok(())
            } else {
                Err(ParseErr::IncompleteComment)
                //Found single backslash
            }
        } else {
            Err(ParseErr::IncompleteComment)
            //Found trailing backslash
        }
    }
}

/// Collects tokens until it reaches the end token,
    /// If a repeat token is defined, then it has a counter that increments when a repeat token is
    /// found and decrements when a end token is found. This is for embedding and brace/bracket
    /// matching.
    /// TODO: is the cloning necessary? Try taking another inner slice
    fn range_get<'a, 'b, I>(repeat_token: Option<TokenKind>,
                            end_token: TokenKind,
                            stream: &'b mut I)
                            -> Result<(Vec<Token>, LineInfo), ParseErr>
        where I: Iterator<Item = &'a Token>
    {
        use super::lexer::TokenKind::*;
        let end_char = match end_token {
            Quote => '\"',
            RBracket => ']',
            RBrace => '}',
            _ => panic!("Does not represent an end to a range of tokens"),
        };

        let mut result = Vec::new();
        let mut found_end = false;
        let mut end_info = location!(0);
        let mut repeats: usize = 0;
        
        {
            for token in stream {
                if let Some(ref repeat) = repeat_token {
                    if token.kind == *repeat {
                        repeats += 1;
                    }
                }
                if token.kind == end_token {
                    if repeats == 0 {
                        end_info = token.line_info.clone();
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
            Ok((result, end_info))
        } else {
            Err(ParseErr::CharNotFound(end_char))
        }
    }

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use namespace::Namespace;

    macro_rules! quick_stmt {
        ($kind: expr) => {
            {
                Statement::new(vec![word!($kind, location!(0))], Rc::new(String::new()))
            }
        };

        ($($kind: expr,)+) => {
            {
                Statement::new(vec![$(word!($kind, location!(0))),+], Rc::new(String::new()))
            }
        };
    }

    #[test]
    fn test_parse_localvarsub() {
        //WordKind::VarSub test
        let word = "#_a23;";
        let parser = Parser { original_string: Rc::new("#_a23;".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(
            quick_stmt!(
                WordKind::VarSub(
                    Path(
                        vec![
                             PathSegment::new("_a23".to_string().into(), dummy!())
                            ]
                        ), 
                    Namespace::Local
                )
            ),
            result[0]);
    }

    #[test]
    fn test_parse_modulevarsub() {
        //WordKind::VarSub test
        let word = "$_a23;";
        let parser = Parser { original_string: Rc::new("$_a23;".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(quick_stmt!(WordKind::VarSub(Path(vec![
                                                     PathSegment::new("_a23".to_string().into(), dummy!())
                                                    ]), Namespace::Module)),
                   result[0]);
    }

    #[test]
    fn test_parse_number() {
        //--------------
        //WordKind::Number test
        
        let word = "-123.5;";
        let parser = Parser { original_string: Rc::new("-123.5".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(quick_stmt!(WordKind::Number(-123.5f64)), result[0]);
    }

    #[test]
    fn test_parse_neg_number() {
        let word = "-1337.5;";
        let parser = Parser { original_string: Rc::new("-1337.5".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(quick_stmt!(WordKind::Number(-1337.5_f64)), result[0]);
    }

    #[test]
    fn test_parse_bool() {
        //--------------
        //WordKind::Bool test
        let word = "false;";
        let parser = Parser { original_string: Rc::new("false;".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(quick_stmt!(WordKind::Bool(false)), result[0]);
    }

    #[test]
    fn test_parse_arg_list() {
        let parser = Parser { original_string: Rc::new("number1 number2".to_string()) };
        let proc_args = parser.parse_arg_list("number1 number2").unwrap().unwrap();
        let mut args = Vec::new();
        for arg in proc_args.all() {
            if let WordKind::Atom(atom) = arg.kind {
                args.push(atom);
            } else {
                panic!("Did not expect {}", arg.kind);
            }
        }
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn test_parse_untouched() {
        //--------------
        //WordKind::Untouched test
        let word = "{untoucheda $123};";
        let parser = Parser { original_string: Rc::new("{untoucheda $123};".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();
        assert_eq!(quick_stmt!(WordKind::Untouched("untoucheda $123".to_string())),
                   result[0]);
    }

    #[test]
    fn test_parse_string() {
        let word = "\" #var 123 2#var2$var3\";";
        let parser = Parser { original_string: Rc::new("\" #var 123 2#var2$var3\";".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(word)).unwrap();

        assert_eq!(quick_stmt!(WordKind::StrSub(StrSub(vec![
                                        StrData::String(" ".to_string()),
                                        StrData::VarSub(Path(vec![PathSegment { segment: Atom("var".to_string()), line_info: dummy!()}]), Namespace::Local),
                                        StrData::String(" 123 2".to_string()),
                                        StrData::VarSub(Path(vec![PathSegment { segment: Atom("var2".to_string()), line_info: dummy!()}]), Namespace::Local),
                                        StrData::VarSub(Path(vec![PathSegment { segment: Atom("var3".to_string()), line_info: dummy!()}]), Namespace::Module),
                                        ]))),
                    result[0]);
    }

    #[test]
    fn embed_cmd() {
        let cmd = "add [add 2 [add 1 3]];";
        let parser = Parser { original_string: Rc::new("add [add 2 [add 1 3]];".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(cmd)).unwrap();


        let cmd_1 = WordKind::CmdSub(Box::new(quick_stmt!(
                                            WordKind::Atom(From::from("add".to_string())),
                                            WordKind::Number(1f64),
                                            WordKind::Number(3f64),
        )));
        let cmd_2 = WordKind::CmdSub(Box::new(quick_stmt!(
                                            WordKind::Atom(From::from("add".to_string())),
                                            WordKind::Number(2f64),
                                            cmd_1,
        )));
        assert_eq!(quick_stmt!(WordKind::Atom(From::from("add".to_string())), cmd_2,),
                    result[0]);
    }

    #[test]
    fn test_parse_tokenized_seq() {
        let seq = "add 1 [add 3 1 test{123} \"#var _123\"];";
        let tokenized = tokenize(seq);

        let parser = Parser { original_string: Rc::new("add 1 [add 3 1 test{123} \"#var _123\"];".to_string()) };
        let result = parser.parse_tokenized_seq(&tokenize(seq)).unwrap();
        assert_eq!(quick_stmt!(
                       WordKind::Atom(From::from("add".to_string())),
                       WordKind::Number(1f64),
                       WordKind::CmdSub(Box::new(quick_stmt!(
                                                    WordKind::Atom(From::from("add".to_string())),
                                                    WordKind::Number(3f64),
                                                    WordKind::Number(1f64),
                                                    WordKind::Atom(From::from("test".to_string())),
                                                    WordKind::Untouched("123".to_string()),
                                                    WordKind::StrSub(StrSub(
                                                        vec![
                                                        StrData::VarSub(Path(vec![PathSegment { segment: Atom("var".to_string()), line_info: dummy!()}]), Namespace::Local),
                                                        StrData::String(" _123".to_string()),
                                                        ]
                                                    )),
                                                 )
                        )),
                    ),
                    result[0])
    }
}
