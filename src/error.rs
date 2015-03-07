use std::error;
use std::fmt;

use xml::common::Error as ParserError;

/// An error that occurs trying to build a `Document`.
pub enum BuildError {
    /// There was no root element. (empty source document).
    UndefinedRoot,
    /// A parser error, see `xml::common::error`.
    ParserError(ParserError),
}

impl error::FromError<ParserError> for BuildError {

    fn from_error(err: ParserError) -> BuildError {
        BuildError::ParserError(err)
    }

}

impl fmt::Display for BuildError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::UndefinedRoot => write!(f, "Undefined root element."),
            BuildError::ParserError(ref err) => err.fmt(f),
        }
    }

}
