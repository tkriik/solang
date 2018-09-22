use std::rc::Rc;

use ::sx::Sx;
use ::token::{tokenize, Kind};

#[derive(Eq, PartialEq, Debug)]
pub enum ReadError {
    InvalidToken(String),
    IntegerLimit(String),
    PartialString(String),
    TrailingDelimiter(String),
    UnmatchedDelimiter
}

pub fn read(source: &str) -> Result<Sx, Vec<ReadError>> {
    let mut opt_sx = None;
    let mut sxs = Vec::new();
    let mut stack = Vec::new();
    let mut read_errors = Vec::new();
    let mut num_quotes = 0;

    let tokens = tokenize(source);
    for token in tokens.iter() {
        match token.kind {
            Kind::Nil => {
                opt_sx = Some(Sx::Nil);
            },

            Kind::Integer => {
                match token.data.parse::<i64>() {
                    Ok(i) => {
                        opt_sx = Some(Sx::Integer(i));
                    },

                    Err(_) => {
                        read_errors.push(ReadError::IntegerLimit(token.data.to_string()));
                    }
                }
            },

            Kind::Symbol => {
                opt_sx = Some(Sx::Symbol(Rc::new(token.data.to_string())));
            },

            Kind::String => {
                opt_sx = Some(Sx::String(Rc::new(token.data.to_string())));
            },

            Kind::ListStart => {
                stack.push(sxs);
                let sub_sxs = Vec::new();
                sxs = sub_sxs;
            },

            Kind::ListEnd => {
                match stack.pop() {
                    Some(mut top_sxs) => {
                        top_sxs.push(Sx::List(Rc::new(sxs)));
                        sxs = top_sxs;
                    }

                    None => {
                        read_errors.push(ReadError::TrailingDelimiter(token.data.to_string()));
                    }
                }
            },

            Kind::Quote => {
                num_quotes += 1;
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

        match opt_sx {
            Some(mut sx) => {
                for _ in 0 .. num_quotes {
                    sx = Sx::Quote(Rc::new(sx));
                }

                sxs.push(sx.clone());
                opt_sx = None;
            },

            None => ()
        }

    }

    if !stack.is_empty() {
        read_errors.push(ReadError::UnmatchedDelimiter);
    }

    if !read_errors.is_empty() {
        return Err(read_errors);
    }

    return Ok(Sx::List(Rc::new(sxs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sxs(source: &str, exp_sxs: Sx) {
        let act_sxs = read(source);
        assert_eq!(Ok(exp_sxs), act_sxs);
    }

    fn test_errors(source: &str, exp_errs: Vec<ReadError>) {
        let act_errs = read(source);
        assert_eq!(Err(exp_errs), act_errs);
    }

    #[test]
    fn test_empty() {
        test_sxs("", Sx::List(Rc::new(Vec::new())));
    }

    #[test]
    fn test_nil() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Nil
        ]));

        test_sxs("nil", exp_sxs);
    }

    #[test]
    fn test_int() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Integer(0),
            Sx::Integer(1),
            Sx::Integer(12345678)
        ]));

        test_sxs("0 1 12345678", exp_sxs);
    }

    #[test]
    fn test_negative_int() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Integer(-0),
            Sx::Integer(-1),
            Sx::Integer(-12345678)
        ]));

        test_sxs("-0 -1 -12345678", exp_sxs);
    }

    #[test]
    fn test_symbol() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Symbol(Rc::new("foo".to_string()))
        ]));

        test_sxs("foo", exp_sxs);
    }

    #[test]
    fn test_string() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::String(Rc::new("北京市".to_string()))
        ]));

        test_sxs("\"北京市\"", exp_sxs);
    }

    #[test]
    fn test_list_empty() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::List(Rc::new(vec![]))
        ]));

        test_sxs("()", exp_sxs);
    }

    #[test]
    fn test_list_nonempty() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::List(Rc::new(vec![
                Sx::Nil,
                Sx::Symbol(Rc::new("foo".to_string())),
                Sx::String(Rc::new("北京市".to_string()))
            ]))
        ]));

        test_sxs("(nil foo \"北京市\")", exp_sxs);
    }

    #[test]
    fn test_list_nested_front() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::List(Rc::new(vec![
                Sx::List(Rc::new(vec![
                    Sx::List(Rc::new(vec![
                        Sx::Nil,
                    ])),
                    Sx::Symbol(Rc::new("foo".to_string())),
                ])),
                Sx::String(Rc::new("北京市".to_string()))
            ]))
        ]));

        test_sxs("(((nil) foo) \"北京市\")", exp_sxs);
    }

    #[test]
    fn test_list_nested_back() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::List(Rc::new(vec![
                Sx::Nil,
                Sx::List(Rc::new(vec![
                    Sx::Symbol(Rc::new("foo".to_string())),
                    Sx::List(Rc::new(vec![
                        Sx::String(Rc::new("北京市".to_string())),
                    ]))
                ]))
            ]))
        ]));

        test_sxs("(nil (foo (\"北京市\")))", exp_sxs);
    }

    #[test]
    fn test_list_nested_middle() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::List(Rc::new(vec![
                Sx::Nil,
                Sx::List(Rc::new(vec![
                    Sx::Symbol(Rc::new("foo".to_string())),
                ])),
                Sx::String(Rc::new("北京市".to_string()))
            ]))
        ]));

        test_sxs("(nil (foo) \"北京市\")", exp_sxs);
    }

    #[test]
    fn test_multi() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Nil,
            Sx::List(Rc::new(vec![])),
            Sx::List(Rc::new(vec![
                Sx::Symbol(Rc::new("bar".to_string())),
                Sx::Nil
            ])),
            Sx::Symbol(Rc::new("foo".to_string())),
            Sx::String(Rc::new("北京市".to_string()))
        ]));

        test_sxs("nil () (bar nil) foo \"北京市\"", exp_sxs);
    }

    #[test]
    fn test_quoted_sym_1() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Quote(Rc::new(Sx::Symbol(Rc::new("foo".to_string()))))
        ]));

        test_sxs("'foo", exp_sxs);
    }

    #[test]
    fn test_quoted_sym_2() {
        let exp_sxs = Sx::List(Rc::new(vec![
            Sx::Quote(Rc::new(Sx::Quote(Rc::new(Sx::Symbol(Rc::new("foo".to_string()))))))
        ]));

        test_sxs("''foo", exp_sxs);
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
