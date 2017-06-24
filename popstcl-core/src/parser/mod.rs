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
