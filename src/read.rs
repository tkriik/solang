use ::sexp::{Sexp, Error, ReadError};
use ::token::{tokenize, Kind};

pub fn sexps(source: &str) -> Sexp {
    let mut sexps = Vec::new();
    let mut stack = Vec::new();
    let mut read_errors = Vec::new();

    let tokens = tokenize(source);
    for token in tokens.iter() {
        match token.kind {
            Kind::Nil => {
                sexps.push(Sexp::Nil);
            }

            Kind::Integer => {
                match token.data.parse::<i64>() {
                    Ok(i) => {
                        sexps.push(Sexp::Int(i));
                    }

                    Err(_) => {
                        read_errors.push(ReadError::IntegerLimit(token.data));
                    }
                }
            }

            Kind::Symbol => {
                sexps.push(Sexp::Symbol(token.data));
            }

            Kind::String => {
                sexps.push(Sexp::String(token.data));
            }

            Kind::ListStart => {
                stack.push(sexps);
                sexps = Vec::new();
            }

            Kind::ListEnd => {
                match stack.pop() {
                    Some(mut x) => {
                        x.push(Sexp::List(sexps));
                        sexps = x;
                    }

                    None => {
                        read_errors.push(ReadError::TrailingDelimiter(token.data))
                    }
                }
            }

            Kind::Invalid => {
                read_errors.push(ReadError::InvalidToken(token.data));
            }
        }
    }

    if !stack.is_empty() {
        read_errors.push(ReadError::UnmatchedDelimiter);
    }

    if !read_errors.is_empty() {
        return Sexp::Error(
            Error::ReadError(read_errors)
        );
    }

    return Sexp::List(sexps);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sexps(source: &str, exp_sexps: Sexp) {
        let act_sexps = sexps(source);
        assert_eq!(exp_sexps, act_sexps);
    }

    #[test]
    fn test_empty() {
        test_sexps("", Sexp::List(Vec::new()));
    }

    #[test]
    fn test_nil() {
        let exp_sexps = Sexp::List(vec![
            Sexp::Nil
        ]);

        test_sexps("nil", exp_sexps);
    }

    #[test]
    fn test_int() {
        let exp_sexps = Sexp::List(vec![
            Sexp::Int(0),
            Sexp::Int(1),
            Sexp::Int(12345678)
        ]);

        test_sexps("0 1 12345678", exp_sexps);
    }

    #[test]
    fn test_negative_int() {
        let exp_sexps = Sexp::List(vec![
            Sexp::Int(-0),
            Sexp::Int(-1),
            Sexp::Int(-12345678)
        ]);

        test_sexps("-0 -1 -12345678", exp_sexps);
    }

    #[test]
    fn test_symbol() {
        let exp_sexps = Sexp::List(vec![
            Sexp::Symbol("foo")
        ]);

        test_sexps("foo", exp_sexps);
    }

    #[test]
    fn test_string() {
        let exp_sexps = Sexp::List(vec![
            Sexp::String("北京市")
        ]);

        test_sexps("\"北京市\"", exp_sexps);
    }

    #[test]
    fn test_list_empty() {
        let exp_sexps = Sexp::List(vec![
            Sexp::List(vec![])
        ]);

        test_sexps("()", exp_sexps);
    }

    #[test]
    fn test_list_nonempty() {
        let exp_sexps = Sexp::List(vec![
            Sexp::List(vec![
                Sexp::Nil,
                Sexp::Symbol("foo"),
                Sexp::String("北京市")
            ])
        ]);

        test_sexps("(nil foo \"北京市\")", exp_sexps);
    }

    #[test]
    fn test_list_nested_front() {
        let exp_sexps = Sexp::List(vec![
            Sexp::List(vec![
                Sexp::List(vec![
                    Sexp::List(vec![
                        Sexp::Nil,
                    ]),
                    Sexp::Symbol("foo"),
                ]),
                Sexp::String("北京市")
            ])
        ]);

        test_sexps("(((nil) foo) \"北京市\")", exp_sexps);
    }

    #[test]
    fn test_list_nested_back() {
        let exp_sexps = Sexp::List(vec![
            Sexp::List(vec![
                Sexp::Nil,
                Sexp::List(vec![
                    Sexp::Symbol("foo"),
                    Sexp::List(vec![
                        Sexp::String("北京市")
                    ])
                ])
            ])
        ]);

        test_sexps("(nil (foo (\"北京市\")))", exp_sexps);
    }

    #[test]
    fn test_list_nested_middle() {
        let exp_sexps = Sexp::List(vec![
            Sexp::List(vec![
                Sexp::Nil,
                Sexp::List(vec![
                    Sexp::Symbol("foo"),
                ]),
                Sexp::String("北京市")
            ])
        ]);

        test_sexps("(nil (foo) \"北京市\")", exp_sexps);
    }

    #[test]
    fn test_multi() {
        let exp_sexps = Sexp::List(vec![
            Sexp::Nil,
            Sexp::List(vec![]),
            Sexp::List(vec![
                Sexp::Symbol("bar"),
                Sexp::Nil
            ]),
            Sexp::Symbol("foo"),
            Sexp::String("北京市")
        ]);

        test_sexps("nil () (bar nil) foo \"北京市\"", exp_sexps);
    }

    #[test]
    fn test_invalid_tokens() {
        let exp_sexps = Sexp::Error(
            Error::ReadError(vec![
                ReadError::InvalidToken("bar,,,")
            ])
        );

        test_sexps("foo bar,,, baz", exp_sexps);
    }

    #[test]
    fn test_int_overflow() {
        let exp_sexps = Sexp::Error(
            Error::ReadError(vec![
                ReadError::IntegerLimit("100200300400500600700800"),
                ReadError::IntegerLimit("-100200300400500600700800")
            ])
        );

        test_sexps("100200300400500600700800 -100200300400500600700800", exp_sexps);
    }

    #[test]
    fn test_unmatched_delimiter_list() {
        let exp_sexps = Sexp::Error(
            Error::ReadError(vec![
                ReadError::UnmatchedDelimiter
            ])
        );

        test_sexps("(foo bar baz", exp_sexps);
    }

    #[test]
    fn test_trailing_delimiter_list() {
        let exp_sexps = Sexp::Error(
            Error::ReadError(vec![
                ReadError::TrailingDelimiter(")")
            ])
        );

        test_sexps("(foo bar baz))", exp_sexps);
    }
}
