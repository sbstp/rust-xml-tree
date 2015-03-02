use std::error;

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
