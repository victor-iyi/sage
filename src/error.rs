//! `sage::error` contains the defacto error handler for building
//! Knowledge Graph database with `sage`. Handles errors from `io::Error`s
//! to data validation.
//!
use std::io;

/// Generic Error handler enum.
#[derive(Debug)]
pub enum SageError {
    /// The error was caused by a failure to read or write
    /// bytes to I/O stream.
    Io(io::Error),

    ///The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    Json(serde_json::Error),

    /// The error caused during data parsing from one data type to another.
    ParseError,
}

/// Conversion from `serde_json` to `SageError`.
impl From<serde_json::Error> for SageError {
    fn from(err: serde_json::Error) -> SageError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => SageError::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => SageError::Json(err),
        }
    }
}
