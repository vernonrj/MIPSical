use std::error;
use std::fmt;

pub type ExecResult<T> = Result<T, ExecError>;

#[derive(Debug)]
pub struct ExecError {
    kind: ErrorKind,
    error: Box<error::Error+Send+Sync>
}

impl ExecError {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self where
        E: Into<Box<error::Error + Send + Sync>>
    {
        ExecError {
            kind: kind,
            error: error.into()
        }
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Exception => write!(f, "exception occurred"),
            ErrorKind::Other => write!(f, "dunno")
        }
    }
}

impl error::Error for ExecError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Exception => "exception occurred",
            ErrorKind::Other => "dunno"
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    Exception,
    Other
}


