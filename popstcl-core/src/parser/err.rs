use super::lexer::Token;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseErr {
    UnexpectedChar(char, String),
    CharNotFound(char),
    NoVarName,
    ExpectedAtom,
    UnexpectedToken(Token),
    NotBool(String),
    NoMoreTokens,
    MissingSemicolon,
    IncompleteComment,
    EmptyCmdSub
}
