use std::error;
use std::fmt;

pub enum ExecResult<T> {
    Success(T),
    Empty,
    Exception(ExecError),
}

impl<T> ExecResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            ExecResult::Success(t) => t,
            ExecResult::Empty => panic!("unwrap on Empty"),
            ExecResult::Exception(e) => panic!("unwrap on exception: {}", e),
        }
    }
}

#[derive(Debug)]
pub struct ExecError {
    kind: ErrorKind,
    error: Box<error::Error + Send + Sync>,
}

impl ExecError {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
        where E: Into<Box<error::Error + Send + Sync>>
    {
        ExecError {
            kind: kind,
            error: error.into(),
        }
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Overflow => write!(f, "Checked operation overflowed."),
        }
    }
}

impl error::Error for ExecError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Overflow => "checked operation overflowed",
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    Overflow,
}
