use std::rc::Rc;

use ::sexp::Sexp;
use ::token::{tokenize, Kind};

#[derive(Eq, PartialEq, Debug)]
pub enum ReadError {
    InvalidToken(String),
    IntegerLimit(String),
    PartialString(String),
    TrailingDelimiter(String),
    UnmatchedDelimiter
}

pub fn sexps(source: &str) -> Result<Sexp, Vec<ReadError>> {
    let mut sexps = Vec::new();
    let mut stack = Vec::new();
    let mut read_errors = Vec::new();

    let tokens = tokenize(source);
    for token in tokens.iter() {
        match token.kind {
            Kind::Nil => {
                sexps.push(Sexp::Nil);
            },

            Kind::Integer => {
                match token.data.parse::<i64>() {
                    Ok(i) => {
                        sexps.push(Sexp::Int(i));
                    },

                    Err(_) => {
                        read_errors.push(ReadError::IntegerLimit(token.data.to_string()));
                    }
                }
            },

            Kind::Symbol => {
                sexps.push(Sexp::Symbol(Rc::new(token.data.to_string())));
            },

            Kind::String => {
                sexps.push(Sexp::String(Rc::new(token.data.to_string())));
            },

            Kind::ListStart => {
                stack.push(sexps);
                sexps = Vec::new();
            },

            Kind::ListEnd => {
                match stack.pop() {
                    Some(mut x) => {
                        x.push(Sexp::List(Rc::new(sexps)));
                        sexps = x;
                    }

                    None => {
                        read_errors.push(ReadError::TrailingDelimiter(token.data.to_string()));
                    }
                }
            },

            Kind::StringPartial => {
                read_errors.push(ReadError::PartialString(token.data.to_string()));
            },

            Kind::Invalid => {
                read_errors.push(ReadError::InvalidToken(token.data.to_string()));
            },

            _ => {
                assert!(false);
            }
        }
    }

    if !stack.is_empty() {
        read_errors.push(ReadError::UnmatchedDelimiter);
    }

    if !read_errors.is_empty() {
        return Err(read_errors);
    }

    return Ok(Sexp::List(Rc::new(sexps)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sexps(source: &str, exp_sexps: Sexp) {
        let act_sexps = sexps(source);
        assert_eq!(Ok(exp_sexps), act_sexps);
    }

    fn test_errors(source: &str, exp_errs: Vec<ReadError>) {
        let act_errs = sexps(source);
        assert_eq!(Err(exp_errs), act_errs);
    }

    #[test]
    fn test_empty() {
        test_sexps("", Sexp::List(Rc::new(Vec::new())));
    }

    #[test]
    fn test_nil() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::Nil
        ]));

        test_sexps("nil", exp_sexps);
    }

    #[test]
    fn test_int() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::Int(0),
            Sexp::Int(1),
            Sexp::Int(12345678)
        ]));

        test_sexps("0 1 12345678", exp_sexps);
    }

    #[test]
    fn test_negative_int() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::Int(-0),
            Sexp::Int(-1),
            Sexp::Int(-12345678)
        ]));

        test_sexps("-0 -1 -12345678", exp_sexps);
    }

    #[test]
    fn test_symbol() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::Symbol(Rc::new("foo".to_string()))
        ]));

        test_sexps("foo", exp_sexps);
    }

    #[test]
    fn test_string() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::String(Rc::new("北京市".to_string()))
        ]));

        test_sexps("\"北京市\"", exp_sexps);
    }

    #[test]
    fn test_list_empty() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::List(Rc::new(vec![]))
        ]));

        test_sexps("()", exp_sexps);
    }

    #[test]
    fn test_list_nonempty() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::List(Rc::new(vec![
                Sexp::Nil,
                Sexp::Symbol(Rc::new("foo".to_string())),
                Sexp::String(Rc::new("北京市".to_string()))
            ]))
        ]));

        test_sexps("(nil foo \"北京市\")", exp_sexps);
    }

    #[test]
    fn test_list_nested_front() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::List(Rc::new(vec![
                Sexp::List(Rc::new(vec![
                    Sexp::List(Rc::new(vec![
                        Sexp::Nil,
                    ])),
                    Sexp::Symbol(Rc::new("foo".to_string())),
                ])),
                Sexp::String(Rc::new("北京市".to_string()))
            ]))
        ]));

        test_sexps("(((nil) foo) \"北京市\")", exp_sexps);
    }

    #[test]
    fn test_list_nested_back() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::List(Rc::new(vec![
                Sexp::Nil,
                Sexp::List(Rc::new(vec![
                    Sexp::Symbol(Rc::new("foo".to_string())),
                    Sexp::List(Rc::new(vec![
                        Sexp::String(Rc::new("北京市".to_string())),
                    ]))
                ]))
            ]))
        ]));

        test_sexps("(nil (foo (\"北京市\")))", exp_sexps);
    }

    #[test]
    fn test_list_nested_middle() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::List(Rc::new(vec![
                Sexp::Nil,
                Sexp::List(Rc::new(vec![
                    Sexp::Symbol(Rc::new("foo".to_string())),
                ])),
                Sexp::String(Rc::new("北京市".to_string()))
            ]))
        ]));

        test_sexps("(nil (foo) \"北京市\")", exp_sexps);
    }

    #[test]
    fn test_multi() {
        let exp_sexps = Sexp::List(Rc::new(vec![
            Sexp::Nil,
            Sexp::List(Rc::new(vec![])),
            Sexp::List(Rc::new(vec![
                Sexp::Symbol(Rc::new("bar".to_string())),
                Sexp::Nil
            ])),
            Sexp::Symbol(Rc::new("foo".to_string())),
            Sexp::String(Rc::new("北京市".to_string()))
        ]));

        test_sexps("nil () (bar nil) foo \"北京市\"", exp_sexps);
    }

    #[test]
    fn test_invalid_tokens() {
        let exp_errs = vec![
            ReadError::InvalidToken("bar,,,".to_string())
        ];

        test_errors("foo bar,,, baz", exp_errs);
    }

    #[test]
    fn test_int_overflow() {
        let exp_errs = vec![
                ReadError::IntegerLimit("100200300400500600700800".to_string()),
                ReadError::IntegerLimit("-100200300400500600700800".to_string())
        ];

        test_errors("100200300400500600700800 -100200300400500600700800", exp_errs);
    }

    #[test]
    fn test_partial_string() {
        let exp_errs = vec![
                ReadError::PartialString("  ".to_string())
        ];

        test_errors("\"  ", exp_errs);
    }

    #[test]
    fn test_unmatched_delimiter_list() {
        let exp_errs = vec![
                ReadError::UnmatchedDelimiter
        ];

        test_errors("(foo bar baz", exp_errs);
    }

    #[test]
    fn test_trailing_delimiter_list() {
        let exp_errs = vec![
            ReadError::TrailingDelimiter(")".to_string())
        ];

        test_errors("(foo bar baz))", exp_errs);
    }
}
