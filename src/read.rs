use std::sync::Arc;
use std::result;
use std::string::ToString;
use im;
use unicode_segmentation::UnicodeSegmentation;

use ::sx::Sx;

pub fn read(source: &str) -> Result {
    let mut opt_sx = None;
    let mut sxs = Vec::new();

    let mut read_stack = Vec::new();
    let mut nquotes = 0;

    let mut read_errors = Vec::new();

    let tokens = tokenize(source);
    for token in tokens.iter() {
        match token.kind {
            TokenKind::Nil => {
                opt_sx = Some(sx_nil!());
            },

            TokenKind::Boolean => {
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

            TokenKind::Integer => {
                match token.data.parse::<i64>() {
                    Ok(i) => {
                        opt_sx = Some(sx_integer!(i));
                    },

                    Err(_) => {
                        read_errors.push(Error::IntegerLimit(token.data.to_string()));
                    }
                }
            },

            TokenKind::Symbol => {
                opt_sx = Some(sx_symbol!(token.data));
            },

            TokenKind::String => {
                opt_sx = Some(sx_string!(token.data));
            },

            TokenKind::ListStart | TokenKind::VectorStart => {
                read_stack.push((sxs, token.kind, nquotes));

                let sub_sxs = Vec::new();
                sxs = sub_sxs;
                nquotes = 0;
            },

            TokenKind::ListEnd | TokenKind::VectorEnd => {
                match read_stack.pop () {
                    Some((mut top_sxs, top_delim, top_nquotes)) => {
                        match (top_delim, token.kind) {
                            (TokenKind::ListStart, TokenKind::ListEnd) => (),
                            (TokenKind::VectorStart, TokenKind::VectorEnd) => (),
                            _ => {
                                read_errors.push(Error::InvalidCloseDelimiter(top_delim, token.data.to_string()));
                                continue;
                            }
                        }

                        let mut sx = match token.kind {
                            TokenKind::ListEnd => sx_list_from_vec!(sxs),
                            TokenKind::VectorEnd => sx_vector_from_vec!(sxs),
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
                        read_errors.push(Error::TrailingDelimiter(token.data.to_string()));
                    }
                }
            },

            TokenKind::Quote => {
                nquotes += 1;
            },

            TokenKind::StringPartial => {
                read_errors.push(Error::PartialString(token.data.to_string()));
            },

            TokenKind::Invalid => {
                read_errors.push(Error::InvalidToken(token.data.to_string()));
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
        read_errors.push(Error::UnmatchedDelimiter(*top_delim));
    }

    if !read_errors.is_empty() {
        return Err(read_errors);
    }

    return Ok(sxs);
}

pub type Result = result::Result<Vec<Sx>, Vec<Error>>;

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    InvalidToken(String),
    IntegerLimit(String),
    PartialString(String),
    InvalidCloseDelimiter(TokenKind, String),
    TrailingDelimiter(String),
    UnmatchedDelimiter(TokenKind)
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
                return format!("non-terminated string: \"{}", s)
            }

            Error::InvalidCloseDelimiter(kind, s) => {
                match kind {
                    TokenKind::ListStart => {
                        return format!("invalid list close delimiter: '{}'", s)
                    },

                    TokenKind::VectorStart => {
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
                    TokenKind::ListStart => {
                        return format!("non-terminated list")
                    },

                    TokenKind::VectorStart => {
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TokenKind {
    Empty,

    Nil,
    Boolean,
    Integer,
    Symbol,
    StringPartial,
    String,
    ListStart,
    ListEnd,
    VectorStart,
    VectorEnd,
    Quote,

    Invalid
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub size: usize,
    pub data: &'a str
}

impl <'a> Token<'a> {
    fn new(kind: TokenKind, data: &'a str) -> Token<'a> {
        return Token {
            kind,
            size: 0,
            data
        }
    }

    fn update(&mut self, c: char) {
        self.size += c.len_utf8();
    }

    fn finalize(&mut self) {
        self.data = &self.data[.. self.size];
    }
}

struct TokenReader<'a> {
    window: &'a str
}

impl <'a> TokenReader<'a> {
    fn new(source: &'a str) -> TokenReader<'a> {
        return TokenReader {
            window: source
        }
    }

    fn next(&mut self) -> Token<'a> {
        let mut token = Token::new(TokenKind::Empty, self.window);

        let mut read_size = 0;
        for (_, (offset, s)) in self.window.grapheme_indices(true).enumerate() {
            let c = s.chars().next().unwrap();

            // TODO: fix this hack;
            read_size += c.len_utf8();

            match token.kind {
                TokenKind::Empty => {
                    match c {
                        // Empty -> Empty
                        _ if c.is_whitespace() => {
                            token.update(c);
                        }

                        // Empty -> Integer
                        _ if c.is_ascii_digit() => {
                            token = Token::new(TokenKind::Integer, &self.window[offset ..]);
                            token.update(c);
                        }

                        // Empty -> Symbol
                        _ if is_symbol_start(c) => {
                            token = Token::new(TokenKind::Symbol, &self.window[offset ..]);
                            token.update(c);
                        },

                        // Empty -> String
                        '"' => {
                            token = Token::new(TokenKind::StringPartial, &self.window[offset ..]);
                        }

                        // Empty -> Done (ListStart)
                        '(' => {
                            token = Token::new(TokenKind::ListStart, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Done (ListEnd)
                        ')' => {
                            token = Token::new(TokenKind::ListEnd, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Done (VectorStart)
                        '[' => {
                            token = Token::new(TokenKind::VectorStart, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Done (VectorEnd)
                        ']' => {
                            token = Token::new(TokenKind::VectorEnd, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Done (Quote)
                        '\'' => {
                            token = Token::new(TokenKind::Quote, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Invalid
                        _ => {
                            token = Token::new(TokenKind::Invalid, &self.window[offset ..]);
                            token.update(c);
                        }
                    }
                },

                TokenKind::Integer => {
                    match c {
                        // Integer -> Done
                        _ if c.is_whitespace() => {
                            break;
                        }

                        // Integer -> Done
                        '(' | ')' | '"' | '\'' | '[' | ']' => {
                            read_size -= c.len_utf8();
                            break;
                        }

                        // Integer -> Integer
                        _ if c.is_ascii_digit() => {
                            token.update(c);
                        }

                        // Integer -> Invalid
                        _ => {
                            token.kind = TokenKind::Invalid;
                            token.update(c);
                        }
                    }
                },

                TokenKind::Symbol => {
                    match c {
                        // Symbol -> Done
                        _ if c.is_whitespace() => {
                            break;
                        },

                        // Symbol -> Done
                        '(' | ')' | '"' | '\'' | '[' | ']' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Symbol -> Nil
                        'l' if token.size == 2 && token.data.starts_with("ni") => {
                            token.kind = TokenKind::Nil;
                            token.update(c);
                        },

                        // Symbol -> Boolean
                        'e' if token.size == 3 && token.data.starts_with("tru") => {
                            token.kind = TokenKind::Boolean;
                            token.update(c);
                        },

                        // Symbol -> Boolean
                        'e' if token.size == 4 && token.data.starts_with("fals") => {
                            token.kind = TokenKind::Boolean;
                            token.update(c);
                        },

                        // Symbol -> Integer
                        _ if token.size == 1 && token.data.starts_with("-") => {
                            token.kind = TokenKind::Integer;
                            token.update(c);
                        }

                        // Symbol -> Symbol
                        _ if is_symbol(c) => {
                            token.update(c);
                        },

                        // Symbol -> Invalid
                        _ => {
                            token.kind = TokenKind::Invalid;
                            token.update(c);
                        }
                    }
                },

                TokenKind::StringPartial => {
                    match c {
                        // StringPartial -> Done (String)
                        '"' => {
                            token.kind = TokenKind::String;
                            break;
                        }

                        // String -> String
                        _ if token.size == 0 => {
                            token.data = &self.window[offset ..];
                            token.update(c);
                        }

                        // String -> String
                        _ => {
                            token.update(c);
                        }
                    }
                },

                TokenKind::Nil => {
                    match c {
                        // Nil -> Done
                        _ if c.is_whitespace() => {
                            break;
                        },

                        // Nil -> Done
                        '(' | ')' | '"' | '\'' | '[' | ']' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Nil -> Symbol
                        _ if is_symbol(c) => {
                            token.kind = TokenKind::Symbol;
                            token.update(c);
                        },

                        // Nil -> Invalid
                        _ => {
                            token.kind = TokenKind::Invalid;
                            token.update(c);
                        }
                    }
                },

                TokenKind::Boolean => {
                    match c {
                        // Boolean -> Done
                        _ if c.is_whitespace() => {
                            break
                        },

                        // Boolean -> Done
                        '(' | ')' | '"' | '\'' | '[' | ']' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Boolean -> Symbol
                        _ if is_symbol(c) => {
                            token.kind = TokenKind::Symbol;
                            token.update(c);
                        },

                        // Boolean -> Invalid
                        _ => {
                            token.kind = TokenKind::Invalid;
                            token.update(c);
                        }
                    }
                },

                TokenKind::Invalid => {
                    match c {
                        // Invalid -> Done
                        _ if c.is_whitespace() => {
                            break;
                        }

                        // Invalid -> Done
                        '(' | ')' | '"' | '\'' | '[' | ']' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Invalid -> Invalid
                        _ => {
                            token.update(c);
                        }
                    }
                }

                _ => {
                    assert!(false);
                }
            }
        }

        self.window = &self.window[read_size ..];
        token.finalize();

        return token;
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut reader = TokenReader::new(source);

    loop {
        let token = reader.next();
        match token.kind {
            TokenKind::Empty => {
                return tokens;
            }

            _ => {
                tokens.push(token);
            }
        }
    }
}

fn is_symbol_start(c: char) -> bool {
    return c.is_ascii_lowercase() || "*-></+!?=".contains(c);
}

fn is_symbol(c: char) -> bool {
    return is_symbol_start(c) || c.is_numeric();
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

    fn test_errors(source: &str, exp_errs: Vec<Error>) {
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
            Error::InvalidToken("bar,,,".to_string())
        ];

        test_errors("foo bar,,, baz", exp_errs);
    }

    #[test]
    fn test_int_overflow() {
        let exp_errs = vec![
            Error::IntegerLimit("100200300400500600700800".to_string()),
            Error::IntegerLimit("-100200300400500600700800".to_string())
        ];

        test_errors("100200300400500600700800 -100200300400500600700800", exp_errs);
    }

    #[test]
    fn test_partial_string() {
        let exp_errs = vec![
            Error::PartialString("  ".to_string())
        ];

        test_errors("\"  ", exp_errs);
    }

    #[test]
    fn test_invalid_close_delimiter() {
        let exp_errs = vec![
            Error::InvalidCloseDelimiter(TokenKind::ListStart, "]".to_string()),
            Error::InvalidCloseDelimiter(TokenKind::VectorStart, ")".to_string())
        ];

        test_errors("(foo bar baz] [foo bar baz)", exp_errs);
    }

    #[test]
    fn test_unmatched_delimiter() {
        let exp_errs = vec![
            Error::UnmatchedDelimiter(TokenKind::ListStart),
            Error::UnmatchedDelimiter(TokenKind::VectorStart)
        ];

        test_errors("(foo bar baz [foo bar baz", exp_errs);
    }

    #[test]
    fn test_trailing_delimiter_list() {
        let exp_errs = vec![
            Error::TrailingDelimiter(")".to_string())
        ];

        test_errors("(foo bar baz))", exp_errs);
    }
}