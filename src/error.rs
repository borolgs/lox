// TODO: https://craftinginterpreters.com/scanning.html#error-handling

use crate::{interpreter::IntrError, parser::ParserError};

pub enum LoxError {
    ParseError(ParserError),
    RuntimeError(IntrError),
}

impl From<ParserError> for LoxError {
    fn from(error: ParserError) -> Self {
        LoxError::ParseError(error)
    }
}

impl From<IntrError> for LoxError {
    fn from(error: IntrError) -> Self {
        LoxError::RuntimeError(error)
    }
}
