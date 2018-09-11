use std::fmt;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Sexp {
    Nil,
    Int(i64),
    Symbol(Rc<String>),
    String(Rc<String>),
    List(Rc<Vec<Sexp>>),
    Error(Rc<SexpError>)
}

#[derive(Eq, PartialEq, Debug)]
pub enum SexpError {
    Undefined,
    ReadError(Vec<SexpReadError>)
}

#[derive(Eq, PartialEq, Debug)]
pub enum SexpReadError {
    InvalidToken(String),
    IntegerLimit(String),
    PartialString(String),
    TrailingDelimiter(String),
    UnmatchedDelimiter
}

impl Sexp {
    pub fn int_from_str(s: &str) -> Result<Sexp, SexpReadError> {
        match s.parse::<i64>() {
            Ok(i) => {
                return Ok(Sexp::Int(i));
            }

            Err(_) => {
                return Err(SexpReadError::IntegerLimit(s.to_string()));
            }
        }
    }

    pub fn symbol_from_str(s: &str) -> Result<Sexp, SexpReadError> {
        // TODO: validation
        return Ok(Sexp::Symbol(Rc::new(s.to_string())));
    }

    pub fn string_from_str(s: &str) -> Result<Sexp, SexpReadError> {
        return Ok(Sexp::String(Rc::new(s.to_string())));
    }
}
