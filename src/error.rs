//! This module contains `al`'s Error type: an `enum` that contains all errors
//! that can be produced by the `al` API.

use std::fmt;

/// This is the error type used in `al`'s `Result`s.
///
/// For performance reasons, `al` makes an effort to always return `Error`s
/// instead of using `panic!()`.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Too many Expressions/Values/Instructions were stored in the Slab.
    ///
    /// A Slab is pre-allocated at the beginning of the process, and it is
    /// not re-sized.  You can use `Slab::with_capacity()` to increase the
    /// number of items that can be stored.
    SlabOverflow,

    /// Returned by `EvalNamespace::create_cached()`.
    ///
    /// An entry with the same name already exists in the Namespace.  If you
    /// intend to overwrite existing entries, use EvalNamespace::set_cached()`
    /// instead.
    AlreadyExists,

    /// Reached an unexpected End Of Input during parsing.
    EOF,

    /// Reached an unexpected End Of Input during parsing.
    ///
    /// The `String` field contains information about what was being parsed
    /// when the EOF was reached.
    EofWhileParsing(String),

    /// UTF8 decoding error.
    ///
    /// The `String` field contains information about what was being parsed
    /// when the UTF8 error occurred.
    Utf8ErrorWhileParsing(String),

    /// The expression string input was too long.
    ///
    /// This is a safety check that prevents malicious inputs that would
    /// be expensive to parse.
    TooLong,

    /// The expression was too recursive.
    ///
    /// This is a safety check that prevents malicious inputs that would
    /// be expensive to parse.
    TooDeep,

    /// An expression was parsed, but there is still input data remaining.
    ///
    /// The `String` field contains the un-parsed input data.
    UnparsedTokensRemaining(String),

    /// A value was expected, but invalid input data was found.
    InvalidValue,

    /// An error occurred during the parsing of a f64 number.
    ///
    /// The `String` field contains the data that caused the error.
    ParseF64(String),

    /// The expected input data was not found.
    ///
    /// The `String` field tells you what was expected.
    Expected(String),

    /// A function was called with the wrong arguments.
    ///
    /// The `String` field contains information about the expected arguments.
    WrongArgs(String),

    /// The expression tried to use an undefined variable/function.
    ///
    /// You can define variables/functions with a Namespace.
    Undefined(String),

    /// This error should never occur because it is only produced by code paths
    /// that should never execute.  This is more performant than using the
    /// `unreachable!()` macro.
    Unreachable,
}

impl std::error::Error for Error {
    // The defaults are fine for now.
}

impl fmt::Display for Error {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)  // Re-use Debug for now...
    }
}

