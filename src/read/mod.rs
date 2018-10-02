mod parse;
mod token;

use ::std::result;

use ::read::parse::parse;
use ::read::token::Kind;
use ::sx::Sx;

pub fn read(source: &str) -> Result {
    return parse(source);
}

pub fn from_str(source: &str) -> Result {
    return parse(source);
}

pub type Result = result::Result<Vec<Sx>, Vec<Error>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidToken(String),
    IntegerLimit(String),
    PartialString(String),
    InvalidCloseDelimiter(Kind, String),
    TrailingDelimiter(String),
    UnmatchedDelimiter(Kind)
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::InvalidToken(s) => {
                return format!("invalid token: {}", s)
            },

            Error::IntegerLimit(s) => {
                return format!("integer limit: {}", s)
            },

            Error::PartialString(s) => {
                return format!("non-terminated string: {:?}", s)
            }

            Error::InvalidCloseDelimiter(kind, s) => {
                match kind {
                    Kind::ListStart => {
                        return format!("invalid list close delimiter: '{}'", s)
                    },

                    Kind::VectorStart => {
                        return format!("invalid vector close delimiter: '{}'", s)
                    },

                    _ => {
                        assert!(false);
                        return "".to_string();
                    }
                }
            },

            Error::TrailingDelimiter(s) => {
                return format!("trailing delimiter: '{}'", s)
            },

            Error::UnmatchedDelimiter(kind) => {
                match kind {
                    Kind::ListStart => {
                        return format!("non-terminated list")
                    },

                    Kind::VectorStart => {
                        return format!("non-terminated vector")
                    },

                    _ => {
                        assert!(false);
                        return "".to_string();
                    }
                }
            }
        }
    }
}
