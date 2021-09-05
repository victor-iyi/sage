//! `sage::error` contains the defacto error handler for building
//! Knowledge Graph database with `sage`. Handles errors from `io::Error`s
//! to data validation.
//!
//! `sage::error` provides extensive error handler capabilities for
//!  the `sage` Knowledge Graph engine. These functionalities ranges
//! from loading files, associating namespaces, linked-data, relational
//! oncology, bad syntax and many more.
//!
//! Since `sage` relies mostly on serde_json for parsing JSON files, support
//! for converting `Error`s into `serde_json::Error` is also provided with
//! additional functionalities.

// #![allow(dead_code)]

use std::error;
use std::fmt::{self, Debug, Display};
use std::io;

/// This type represents all possible errors that can occur when working
/// with the `sage` Knowledge Graph.
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible.
    /// A larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}

// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?}, line: {}, column: {})",
            self.err.code.to_string(),
            self.err.line,
            self.err.column
        )
    }
}



impl Error {
    /// One-based line number at which the error was detected.
    ///
    /// Characters in the first line of the input (before the first newline
    /// character) are in line 1.
    pub fn line(&self) -> usize {
        self.err.line
    }

    /// One-based column number at which the error was detected.
    ///
    /// The first character in the input and any characters immediately
    /// following a newline character are in column 1.
    ///
    /// Note that errors may occur in column 0, for example if a read from an IO
    /// stream fails immediately following a previously read newline character.
    pub fn column(&self) -> usize {
        self.err.column
    }

    /// Categorizes the cause of this error.
    ///
    /// - `Category::Io` - failure to read or write bytes on an IO stream
    /// - `Category::Syntax` - input that is not syntactically valid JSON
    /// - `Category::Data` - input data that is semantically incorrect
    /// - `Category::Eof` - unexpected end of the input data
    pub fn classify(&self) -> Category {
        match self.err.code {
            ErrorCode::Message(_) => Category::Data,

            ErrorCode::Io(_) | ErrorCode::Json(_) => Category::Io,

            ErrorCode::EofWhileParsingList
            | ErrorCode::EofWhileParsingObject
            | ErrorCode::EofWhileParsingString
            | ErrorCode::EofWhileParsingValue => Category::Eof,

            ErrorCode::ParseError
            | ErrorCode::IllegalNamespace
            | ErrorCode::UnknownNode
            | ErrorCode::ExpectedColon
            | ErrorCode::ExpectedListCommaOrEnd
            | ErrorCode::ExpectedObjectCommaOrEnd
            | ErrorCode::ExpectedObjectOrArray
            | ErrorCode::ExpectedSomeIdent
            | ErrorCode::ExpectedSomeValue
            | ErrorCode::ExpectedSomeString
            | ErrorCode::InvalidEscape
            | ErrorCode::InvalidNumber
            | ErrorCode::NumberOutOfRange
            | ErrorCode::InvalidUnicodeCodePoint
            | ErrorCode::ControlCharacterWhileParsingString
            | ErrorCode::KeyMustBeAString
            | ErrorCode::LoneLeadingSurrogateInHexEscape
            | ErrorCode::TrailingComma
            | ErrorCode::TrailingCharacters
            | ErrorCode::UnexpectedEndOfHexEscape
            | ErrorCode::RecursionLimitExceeded
            | ErrorCode::RegexParser => Category::Syntax,
        }
    }

    /// Returns true if this error was caused by a failure to read or write
    /// bytes on an IO stream.
    pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }

    /// Returns true if this error was caused by input that was not
    /// syntactically valid JSON.
    pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }

    /// Returns true if this error was caused by input data that was
    /// semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    pub fn is_data(&self) -> bool {
        self.classify() == Category::Data
    }

    /// Returns true if this error was caused by prematurely reaching the end of
    /// the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }
}

impl Error {
    // Not public API. Should be pub(crate).
    #[doc(hidden)]
    #[cold]
    pub fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }

    // Not public API. Should be pub(crate).
    //
    // Update `eager_json` crate when this function changes.
    #[doc(hidden)]
    #[cold]
    pub fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                line: 0,
                column: 0,
            }),
        }
    }

    // Not public API. Should be pub(crate).
    #[doc(hidden)]
    #[cold]
    pub fn fix_position<F>(self, f: F) -> Self
    where
        F: FnOnce(ErrorCode) -> Error,
    {
        if self.err.line == 0 {
            f(self.err.code)
        } else {
            self
        }
    }
}

/// Categorizes the cause of a `sage::Error`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    /// The error was caused by a failure to read or write bytes on an IO
    /// stream.
    Io,

    /// The error was caused by input that was not syntactically valid JSON.
    Syntax,

    /// The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    Data,

    /// The error was caused by prematurely reaching the end of the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    Eof,
}

impl From<Error> for io::Error {
    /// Convert a `sage::Error` into an `io::Error`.
    ///
    /// JSON syntax and data errors are turned into `InvalidData` IO errors.
    /// EOF errors are turned into `UnexpectedEof` IO errors.
    ///
    /// ```edition2018
    /// use std::io;
    ///
    /// enum MyError {
    ///     Io(io::Error),
    ///     Json(sage::Error),
    /// }
    ///
    /// impl From<sage::Error> for MyError {
    ///     fn from(err: sage::Error) -> MyError {
    ///         use sage::error::Category;
    ///         match err.classify() {
    ///             Category::Io => {
    ///                 MyError::Io(err.into())
    ///             }
    ///             Category::Syntax | Category::Data | Category::Eof => {
    ///                 MyError::Json(err)
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    fn from(j: Error) -> Self {
        if let ErrorCode::Io(err) = j.err.code {
            err
        } else {
            match j.classify() {
                Category::Io => unreachable!(),
                Category::Syntax | Category::Data => io::Error::new(io::ErrorKind::InvalidData, j),
                Category::Eof => io::Error::new(io::ErrorKind::UnexpectedEof, j),
            }
        }
    }
}

struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}

// Not public API. Should be pub(crate).
#[doc(hidden)]
pub enum ErrorCode {
    /// The error was caused by a failure to read or write
    /// bytes to I/O stream.
    Io(io::Error),

    ///The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect
    /// when the type being deserialized into holds a String.
    Json(serde_json::Error),

    /// The error caused during data parsing from one data type to another.
    ParseError,

    /// The error caused by illegal or invalid namespace.
    IllegalNamespace,

    /// The error caused by using unknown node or node that doesn't
    /// follow certain criteria.
    UnknownNode,

    /// Catchall for syntax error messages
    Message(Box<str>),

    /// EOF while parsing a list.
    EofWhileParsingList,

    /// EOF while parsing an object.
    EofWhileParsingObject,

    /// EOF while parsing a string.
    EofWhileParsingString,

    /// EOF while parsing a JSON value.
    EofWhileParsingValue,

    /// Expected this character to be a `':'`.
    ExpectedColon,

    /// Expected this character to be either a `','` or a `']'`.
    ExpectedListCommaOrEnd,

    /// Expected this character to be either a `','` or a `'}'`.
    ExpectedObjectCommaOrEnd,

    /// Expected this character to be either a `'{'` or a `'['`.
    ExpectedObjectOrArray,

    /// Expected to parse either a `true`, `false`, or a `null`.
    ExpectedSomeIdent,

    /// Expected this character to start a JSON value.
    ExpectedSomeValue,

    /// Expected this character to start a JSON string.
    ExpectedSomeString,

    /// Invalid hex escape code.
    InvalidEscape,

    /// Invalid number.
    InvalidNumber,

    /// Number is bigger than the maximum value of its type.
    NumberOutOfRange,

    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,

    /// Control character found while parsing a string.
    ControlCharacterWhileParsingString,

    /// Object key is not a string.
    KeyMustBeAString,

    /// Lone leading surrogate in hex escape.
    LoneLeadingSurrogateInHexEscape,

    /// JSON has a comma after the last value in an array or map.
    TrailingComma,

    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,

    /// Unexpected end of hex escape.
    UnexpectedEndOfHexEscape,

    /// Encountered nesting of JSON maps and arrays more than 128 layers deep.
    RecursionLimitExceeded,

    /// Could not parse regular expression pattern or pattern wasn't a match.
    RegexParser,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => Display::fmt(err, f),
            ErrorCode::Json(ref err) => Display::fmt(err, f),
            ErrorCode::ParseError => f.write_str("Error while parsing an object"),
            ErrorCode::IllegalNamespace => f.write_str("Use of unregistered namespace"),
            ErrorCode::UnknownNode => f.write_str("Encountered unrecognized node"),

            ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => f.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => f.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => f.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedObjectOrArray => f.write_str("expected `{` or `[`"),
            ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            ErrorCode::ExpectedSomeString => f.write_str("expected string"),
            ErrorCode::InvalidEscape => f.write_str("invalid escape"),
            ErrorCode::InvalidNumber => f.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => f.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => f.write_str("invalid unicode code point"),
            ErrorCode::ControlCharacterWhileParsingString => {
                f.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            }
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                f.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => f.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => f.write_str("unexpected end of hex escape"),
            ErrorCode::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
            ErrorCode::RegexParser => {
                f.write_str("regular expression wasn't a match or malformed.")
            }
        }
    }
}

impl error::Error for Error {
    // /// Deprecated! Use Display impl or to_string()
    // fn description(&self) -> &str {
    //     match self.err.code {
    //         ErrorCode::Io(ref err) => error::Error::description(err),
    //         _ => {
    //             // If you want a better message, use Display::fmt or to_string().
    //             "Sage error"
    //         }
    //     }
    // }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self.err.code {
            ErrorCode::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            Display::fmt(&self.code, f)
        } else {
            write!(
                f,
                "{} at line {} column {}",
                self.code, self.line, self.column
            )
        }
    }
}
