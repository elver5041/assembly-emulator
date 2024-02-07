use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Commands {
    NOP,        //no op
    ADD(usize), //add number
    SUB(usize), //subtract number
    JMP(usize), //jump to function
    JNZ(usize), //jump to function if acc!=0 
    SAV,        //save acc in bak
    SWP,        //swap acc and bak
    FUN,        //function
    LDN(usize), //load number
    LDA(usize), //load address
    
}

#[derive(Debug, Clone)]
pub enum Errors {
    FunctionNotThere,
    InvalidContinuation(Commands,String),
    InvalidToken(String),
    FunctionAlreadyInUse(String),
    FunctionDoesntExist(String)
}


#[derive(Debug, Clone)]
pub struct TokenError{
    pub error_type: Errors,
    pub line: (usize,usize)
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Commands::ADD(a) => write!(f,"ADD {}",a),
            Commands::JNZ(a) => write!(f,"JNZ {}",a),
            Commands::JMP(a) => write!(f,"JMP {}",a),
            Commands::SUB(a) => write!(f,"SUB {}",a),
            Commands::LDA(a) => write!(f,"LDA {}",a),
            Commands::LDN(a) => write!(f,"LDN {}",a),
            Commands::SAV | Commands::SWP | Commands::FUN |
            Commands::NOP=> todo!()
        }
    }
}


impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            Errors::FunctionNotThere => write!(f, "no function found to call in {:?}", self.line),
            Errors::InvalidContinuation(command,token) => write!(f, "found {} after <{}>, this is the wrong type {:?}", token, command,self.line),
            Errors::InvalidToken(token) => write!(f, "token <{}> not found {:?}", token ,self.line),
            Errors::FunctionAlreadyInUse(fun) => write!(f, "function <{}> already exists {:?}", fun ,self.line),
            Errors::FunctionDoesntExist(fun) => write!(f, "function <{}> doesn't exist or is uninitialized (use in line {})", fun ,self.line.0),
        }
    }
}

impl std::error::Error for TokenError {}