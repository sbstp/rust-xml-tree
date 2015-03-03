use std::error;
use std::fmt;

use xml::common::Error as ParserError;

pub enum BuildError {
    BuildError,
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
            BuildError::BuildError => write!(f, "Unexpected end"),
            BuildError::ParserError(ref err) => err.fmt(f),
        }
    }

}
