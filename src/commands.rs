use std::fmt;

#[derive(Debug,PartialEq)]
pub enum Commands {
    NOP,
    ADD,
    JNE
}

#[derive(Debug, Clone)]
pub enum Errors {
    FunctionNotThere,
    InvalidToken(String)
}


#[derive(Debug, Clone)]
pub struct TokenError{
    pub error_type: Errors,
    pub line: (usize,usize)
}


impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            Errors::FunctionNotThere => write!(f, "no function found to call in {:?}", self.line),
            Errors::InvalidToken(token) => write!(f, "token {} not found {:?}", token ,self.line)
        }
    }
}

impl std::error::Error for TokenError {}