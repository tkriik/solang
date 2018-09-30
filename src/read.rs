use std::string::ToString;
use std::sync::Arc;

use im;

use ::sx::Sx;
use ::token::{tokenize, Kind};

#[derive(Eq, PartialEq, Debug)]
pub enum ReadError {
    InvalidToken(String),
    IntegerLimit(String),
    PartialString(String),
    InvalidCloseDelimiter(Kind, String),
    TrailingDelimiter(String),
    UnmatchedDelimiter(Kind)
}

pub fn read(source: &str) -> Result<Vec<Sx>, Vec<ReadError>> {
    let mut opt_sx = None;
    let mut sxs = Vec::new();

    let mut read_stack = Vec::new();
    let mut nquotes = 0;

    let mut read_errors = Vec::new();

    let tokens = tokenize(source);
    for token in tokens.iter() {
        match token.kind {
            Kind::Nil => {
                opt_sx = Some(sx_nil!());
            },

            Kind::Boolean => {
                match token.data {
                    "true" => {
                        opt_sx = Some(sx_boolean!(true));
                    },

                    "false" => {
                        opt_sx = Some(sx_boolean!(false));
                    },

                    _ => {
                        assert!(false);
                    }
                }
            }

            Kind::Integer => {
                match token.data.parse::<i64>() {
                    Ok(i) => {
                        opt_sx = Some(sx_integer!(i));
                    },

                    Err(_) => {
                        read_errors.push(ReadError::IntegerLimit(token.data.to_string()));
                    }
                }
            },

            Kind::Symbol => {
                opt_sx = Some(sx_symbol!(token.data));
            },

            Kind::String => {
                opt_sx = Some(sx_string!(token.data));
            },

            Kind::ListStart | Kind::VectorStart => {
                read_stack.push((sxs, token.kind, nquotes));

                let sub_sxs = Vec::new();
                sxs = sub_sxs;
                nquotes = 0;
            },

            Kind::ListEnd | Kind::VectorEnd => {
                match read_stack.pop () {
                    Some((mut top_sxs, top_delim, top_nquotes)) => {
                        match (top_delim, token.kind) {
                            (Kind::ListStart, Kind::ListEnd) => (),
                            (Kind::VectorStart, Kind::VectorEnd) => (),
                            _ => {
                                read_errors.push(ReadError::InvalidCloseDelimiter(top_delim, token.data.to_string()));
                                continue;
                            }
                        }

                        let mut sx = match token.kind {
                            Kind::ListEnd => sx_list_from_vec!(sxs),
                            Kind::VectorEnd => sx_vector_from_vec!(sxs),
                            _ => {
                                assert!(false);
                                sx_nil!()
                            }
                        };

                        nquotes = top_nquotes;
                        for _ in 0 .. nquotes {
                            sx = sx_quote!(sx);
                            nquotes -= 1;
                        }

                        top_sxs.push(sx.clone());
                        sxs = top_sxs
                    },

                    None => {
                        read_errors.push(ReadError::TrailingDelimiter(token.data.to_string()));
                    }
                }
            },

            Kind::Quote => {
                nquotes += 1;
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
                for _ in 0 .. nquotes {
                    sx = sx_quote!(sx);
                    nquotes -= 1;
                }

                sxs.push(sx);
                opt_sx = None;
            },

            None => ()
        }

    }

    for (_, top_delim, _) in read_stack.iter() {
        read_errors.push(ReadError::UnmatchedDelimiter(*top_delim));
    }

    if !read_errors.is_empty() {
        return Err(read_errors);
    }

    return Ok(sxs);
}

impl ToString for ReadError {
    fn to_string(&self) -> String {
        match self {
            ReadError::InvalidToken(s) => {
                return format!("invalid token: {}", s)
            },

            ReadError::IntegerLimit(s) => {
                return format!("integer limit: {}", s)
            },

            ReadError::PartialString(s) => {
                return format!("non-terminated string: \"{}", s)
            }

            ReadError::InvalidCloseDelimiter(kind, s) => {
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

            ReadError::TrailingDelimiter(s) => {
                return format!("trailing delimiter: '{}'", s)
            },

            ReadError::UnmatchedDelimiter(kind) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sxs(source: &str, exp_sxs: Vec<Sx>) {
        let act_sxs = read(source);
        assert!(act_sxs.is_ok());
        assert_eq!(sx_list_from_vec!(exp_sxs).to_string(),
                   sx_list_from_vec!(act_sxs.unwrap()).to_string());
    }

    fn test_errors(source: &str, exp_errs: Vec<ReadError>) {
        let act_errs = read(source);
        assert_eq!(Err(exp_errs), act_errs);
    }

    #[test]
    fn test_empty() {
        test_sxs("", vec![]);
    }

    #[test]
    fn test_nil() {
        let exp_sxs = vec![
            sx_nil!()
        ];

        test_sxs("nil", exp_sxs);
    }

    #[test]
    fn test_boolean() {
        let exp_sxs = vec![
            sx_boolean!(true),
            sx_boolean!(false)
        ];

        test_sxs("true false", exp_sxs);
    }

    #[test]
    fn test_int() {
        let exp_sxs = vec![
            sx_integer!(0),
            sx_integer!(1),
            sx_integer!(12345678)
        ];

        test_sxs("0 1 12345678", exp_sxs);
    }

    #[test]
    fn test_negative_int() {
        let exp_sxs = vec![
            sx_integer!(-0),
            sx_integer!(-1),
            sx_integer!(-12345678)
        ];

        test_sxs("-0 -1 -12345678", exp_sxs);
    }

    #[test]
    fn test_symbol() {
        let exp_sxs = vec![
            sx_symbol!("foo")
        ];

        test_sxs("foo", exp_sxs);
    }

    #[test]
    fn test_string() {
        let exp_sxs = vec![
            sx_string!("北京市")
        ];

        test_sxs("\"北京市\"", exp_sxs);
    }

    #[test]
    fn test_list_empty() {
        let exp_sxs = vec![
            sx_list![]
        ];

        test_sxs("()", exp_sxs);
    }

    #[test]
    fn test_list_singleton() {
        let exp_sxs = vec![
            sx_list![
                sx_symbol!("foo")
            ]
        ];

        test_sxs("(foo)", exp_sxs);
    }

    #[test]
    fn test_list_pair() {
        let exp_sxs = vec![
            sx_list![
                sx_symbol!("foo"),
                sx_string!("Åbo")
            ]
        ];

        test_sxs("(foo \"Åbo\")", exp_sxs);
    }

    #[test]
    fn test_list_nonempty() {
        let exp_sxs = vec![
            sx_list![
                sx_nil!(),
                sx_symbol!("foo"),
                sx_string!("北京市")
            ]
        ];

        test_sxs("(nil foo \"北京市\")", exp_sxs);
    }

    #[test]
    fn test_list_nested_front() {
        let exp_sxs = vec![
            sx_list![
                sx_list![
                    sx_list![
                        sx_nil!()
                    ],
                    sx_symbol!("foo")
                ],
                sx_string!("北京市")
            ]
        ];

        test_sxs("(((nil) foo) \"北京市\")", exp_sxs);
    }

    #[test]
    fn test_list_nested_back() {
        let exp_sxs = vec![
            sx_list![
                sx_nil!(),
                sx_list![
                    sx_symbol!("foo"),
                    sx_list![
                        sx_string!("北京市")
                    ]
                ]
            ]
        ];

        test_sxs("(nil (foo (\"北京市\")))", exp_sxs);
    }

    #[test]
    fn test_list_nested_middle() {
        let exp_sxs = vec![
            sx_list![
                sx_nil!(),
                sx_list![
                    sx_symbol!("foo")
                ],
                sx_string!("北京市")
            ]
        ];

        test_sxs("(nil (foo) \"北京市\")", exp_sxs);
    }

    #[test]
    fn test_vector_empty() {
        let exp_sxs = vec![
            sx_vector![]
        ];

        test_sxs("[]", exp_sxs);
    }

    #[test]
    fn test_vector_singleton() {
        let exp_sxs = vec![
            sx_vector![sx_integer!(3)]
        ];

        test_sxs("[3]", exp_sxs);
    }

    #[test]
    fn test_vector_nonempty() {
        let exp_sxs = vec![
            sx_vector![
                sx_nil!(),
                sx_symbol!("foo"),
                sx_string!("北京市"),
                sx_symbol!("bar")
            ]
        ];

        test_sxs("[nil foo \"北京市\" bar]", exp_sxs);
    }

    #[test]
    fn test_vector_nested() {
        let exp_sxs = vec![
            sx_vector![
                sx_vector![sx_nil!()],
                sx_vector![
                    sx_symbol!("foo"),
                    sx_vector![sx_string!("北京市")]
                ],
                sx_vector![sx_symbol!("bar")]
            ]
        ];

        test_sxs("[[nil] [foo [\"北京市\"]] [bar]]", exp_sxs);
    }

    #[test]
    fn test_multi_flat() {
        let exp_sxs = vec![
            sx_nil!(),
            sx_symbol!("foo"),
            sx_string!("北京市"),
            sx_symbol!("bar")
        ];

        test_sxs("nil foo \"北京市\" bar", exp_sxs);
    }

    #[test]
    fn test_multi_nested() {
        let exp_sxs = vec![
            sx_nil!(),
            sx_list![],
            sx_list![
                sx_symbol!("bar"),
                sx_nil!()
            ],
            sx_symbol!("foo"),
            sx_string!("北京市")
        ];

        test_sxs("nil () (bar nil) foo \"北京市\"", exp_sxs);
    }

    #[test]
    fn test_quoted_sym_1() {
        let exp_sxs = vec![
            sx_quote!(sx_symbol!("foo"))
        ];

        test_sxs("'foo", exp_sxs);
    }

    #[test]
    fn test_quoted_sym_2() {
        let exp_sxs = vec![
            sx_quote!(sx_quote!(sx_symbol!("foo")))
        ];

        test_sxs("''foo", exp_sxs);
    }

    #[test]
    fn test_quoted_list_1() {
        let exp_sxs = vec![
            sx_quote!(
                sx_list![
                    sx_integer!(1),
                    sx_nil!(),
                    sx_symbol!("foo")
                ]
            )
        ];

        test_sxs("'(1 nil foo)", exp_sxs);
    }

    #[test]
    fn test_quoted_list_3() {
        let exp_sxs = vec![
            sx_quote!(sx_quote!(sx_quote!(
                sx_list![
                    sx_integer!(1),
                    sx_nil!(),
                    sx_symbol!("foo")
                ]
            )))
        ];

        test_sxs("'''(1 nil foo)", exp_sxs);
    }

    #[test]
    fn test_quoted_list_nested() {
        let exp_sxs = vec![
            sx_quote!(
                sx_list![
                    sx_integer!(1),
                    sx_integer!(2),
                    sx_quote!(
                        sx_list![
                            sx_quote!(sx_symbol!("foo")),
                            sx_quote!(sx_symbol!("bar"))
                        ]
                    )
                ]
            )
        ];

        test_sxs("'(1 2 '('foo 'bar))", exp_sxs);
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
    fn test_invalid_close_delimiter() {
        let exp_errs = vec![
            ReadError::InvalidCloseDelimiter(Kind::ListStart, "]".to_string()),
            ReadError::InvalidCloseDelimiter(Kind::VectorStart, ")".to_string())
        ];

        test_errors("(foo bar baz] [foo bar baz)", exp_errs);
    }

    #[test]
    fn test_unmatched_delimiter() {
        let exp_errs = vec![
                ReadError::UnmatchedDelimiter(Kind::ListStart),
                ReadError::UnmatchedDelimiter(Kind::VectorStart)
        ];

        test_errors("(foo bar baz [foo bar baz", exp_errs);
    }

    #[test]
    fn test_trailing_delimiter_list() {
        let exp_errs = vec![
            ReadError::TrailingDelimiter(")".to_string())
        ];

        test_errors("(foo bar baz))", exp_errs);
    }
}
