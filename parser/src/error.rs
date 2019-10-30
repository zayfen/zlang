use lalrpop_util::ParseError as LalrpopError;

use crate::location::Location;
use crate::token::Token;

use std::error::Error;
use std::fmt;

/// Represents an error during lexical scanning
#[derive(Debug, PartialEq)]
pub struct LexicalError {
  pub error: LexicalErrorType,
  pub location: Location
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
  StringError,
  UnicodeEror,
  DefaultArgumentError,
  PositionalArgumentError,
  DuplicateKeywordArgumentError,
  UnrecognizedToken { tok: char },
  OtherError(String)
}

impl fmt::Display for LexicalErrorType {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LexicalErrorType::StringError => write!(f, "Got unexpected string"),
      LexicalErrorType::UnicodeEror => write!(f, "Got unexpected unicode"),
      LexicalErrorType::DefaultArgumentError => write!(f, "non-default argument follows default argument"),
      LexicalErrorType::PositionalArgumentError => write!(f, "positional argument follows keyword argument"),
      LexicalErrorType::DuplicateKeywordArgumentError => write!(f, "keyword arguemnt repeated"),
      LexicalErrorType::UnrecognizedToken { token } => write!(f, "Got unexpected token {}", token),
      LexicalErrorType::OtherError(msg) => write!(f, "{}", msg),
    }
  }
}


impl From<LexicalError> for LalrpopError<Location, Tok, LexicalError> {
  fn from (err: LexicalError) -> Self {
    lalrpop_util::ParseError::User { error: err }
  }
}


