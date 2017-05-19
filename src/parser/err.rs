use super::lexer::Token;
#[derive(Debug)]
pub enum ParseErr {
    UnexpectedChar(char, String),
    CharNotFound(char),
    NoVarName,
    ExpectedAtom,
    UnexpectedToken(Token),
    NotBool(String),
    NoMoreTokens,
}
