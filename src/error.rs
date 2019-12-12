use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    SlabOverflow,
    AlreadyExists,
    EOF,
    EofWhileParsing(String),
    Utf8ErrorWhileParsing(String),
    TooLong,
    TooDeep,
    UnparsedTokensRemaining(String),
    InvalidValue,
    ParseF64(String),
    Expected(String),
    WrongArgs(String),
    Undefined(String),
}

impl std::error::Error for Error {
    // The defaults are fine for now.
}

impl fmt::Display for Error {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)  // Re-use Debug for now...
    }
}

