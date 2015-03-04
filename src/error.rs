use std::error;
use std::fmt;

use xml::common::Error as ParserError;

pub enum BuildError {
    UndefinedRoot,
    Unreachable,
    UnexpectedEvent,
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
            BuildError::UndefinedRoot => write!(f, "Undefined root element"),
            BuildError::Unreachable => write!(f, "Logic error in algorithm"),
            BuildError::UnexpectedEvent => write!(f, "Unexpected event"),
            BuildError::ParserError(ref err) => err.fmt(f),
        }
    }

}
