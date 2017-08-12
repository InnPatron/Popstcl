pub mod parser;
pub mod err;
mod lexer;

use self::parser::Parser;
use ast::{Program, Statement};
use self::err::ParseErr;

pub fn parse_program(input: &str) -> Result<Program, ParseErr> {
    let parser = Parser::new(input);
    parser.parse_program(input)
}

pub fn parse_arg_list(input: &str) -> Result<Option<Statement>, ParseErr> {
    let parser = Parser::new(input);
    parser.parse_arg_list(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_quote_braces() {
        parse_program("{};").unwrap();
        parse_program("\"\";").unwrap();
    }

    #[test]
    fn empty_brackets() {
        match parse_program("[];") {
            Ok(_) => panic!("Empty cmd sub should fail"),
            Err(e) => assert_eq!(e, ParseErr::EmptyCmdSub),
        }
    }
}
