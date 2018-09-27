use unicode_segmentation::UnicodeSegmentation;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Empty,

    Nil,
    Boolean,
    Integer,
    Symbol,

    StringPartial,
    String,

    ListStart,
    ListEnd,

    Quote,

    Invalid
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: Kind,
    pub size: usize,
    pub data: &'a str
}

impl <'a> Token<'a> {
    fn new(kind: Kind, data: &'a str) -> Token<'a> {
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
        let mut token = Token::new(Kind::Empty, self.window);

        let mut read_size = 0;
        for (_, (offset, s)) in self.window.grapheme_indices(true).enumerate() {
            let c = s.chars().next().unwrap();

            // TODO: fix this hack;
            read_size += c.len_utf8();

            match token.kind {
                Kind::Empty => {
                    match c {
                        // Empty -> Empty
                        _ if c.is_whitespace() => {
                            token.update(c);
                        }

                        // Empty -> Integer
                        _ if c.is_ascii_digit() => {
                            token = Token::new(Kind::Integer, &self.window[offset ..]);
                            token.update(c);
                        }

                        // Empty -> Symbol
                        _ if is_symbol_start(c) => {
                            token = Token::new(Kind::Symbol, &self.window[offset ..]);
                            token.update(c);
                        },

                        // Empty -> String
                        '"' => {
                            token = Token::new(Kind::StringPartial, &self.window[offset ..]);
                        }

                        // Empty -> Done (ListStart)
                        '(' => {
                            token = Token::new(Kind::ListStart, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Done (ListEnd)
                        ')' => {
                            token = Token::new(Kind::ListEnd, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Done (Quote)
                        '\'' => {
                            token = Token::new(Kind::Quote, &self.window[offset ..]);
                            token.update(c);
                            break;
                        }

                        // Empty -> Invalid
                        _ => {
                            token = Token::new(Kind::Invalid, &self.window[offset ..]);
                            token.update(c);
                        }
                    }
                },

                Kind::Integer => {
                    match c {
                        // Integer -> Done
                        _ if c.is_whitespace() => {
                            break;
                        }

                        // Integer -> Done
                        '(' | ')' | '"' | '\'' => {
                            read_size -= c.len_utf8();
                            break;
                        }

                        // Integer -> Integer
                        _ if c.is_ascii_digit() => {
                            token.update(c);
                        }

                        // Integer -> Invalid
                        _ => {
                            token.kind = Kind::Invalid;
                            token.update(c);
                        }
                    }
                },

                Kind::Symbol => {
                    match c {
                        // Symbol -> Done
                        _ if c.is_whitespace() => {
                            break;
                        },

                        // Symbol -> Done
                        '(' | ')' | '"' | '\'' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Symbol -> Nil
                        'l' if token.size == 2 && token.data.starts_with("ni") => {
                            token.kind = Kind::Nil;
                            token.update(c);
                        },

                        // Symbol -> Boolean
                        'e' if token.size == 3 && token.data.starts_with("tru") => {
                            token.kind = Kind::Boolean;
                            token.update(c);
                        },

                        // Symbol -> Boolean
                        'e' if token.size == 4 && token.data.starts_with("fals") => {
                            token.kind = Kind::Boolean;
                            token.update(c);
                        },

                        // Symbol -> Integer
                        _ if token.size == 1 && token.data.starts_with("-") => {
                            token.kind = Kind::Integer;
                            token.update(c);
                        }

                        // Symbol -> Symbol
                        _ if is_symbol(c) => {
                            token.update(c);
                        },

                        // Symbol -> Invalid
                        _ => {
                            token.kind = Kind::Invalid;
                            token.update(c);
                        }
                    }
                },

                Kind::StringPartial => {
                    match c {
                        // StringPartial -> Done (String)
                        '"' => {
                            token.kind = Kind::String;
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

                Kind::Nil => {
                    match c {
                        // Nil -> Done
                        _ if c.is_whitespace() => {
                            break;
                        },

                        // Nil -> Done
                        '(' | ')' | '"' | '\'' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Nil -> Symbol
                        _ if is_symbol(c) => {
                            token.kind = Kind::Symbol;
                            token.update(c);
                        },

                        // Nil -> Invalid
                        _ => {
                            token.kind = Kind::Invalid;
                            token.update(c);
                        }
                    }
                },

                Kind::Boolean => {
                    match c {
                        // Boolean -> Done
                        _ if c.is_whitespace() => {
                            break
                        },

                        // Boolean -> Done
                        '(' | ')' | '"' | '\'' => {
                            read_size -= c.len_utf8();
                            break;
                        },

                        // Boolean -> Symbol
                        _ if is_symbol(c) => {
                            token.kind = Kind::Symbol;
                            token.update(c);
                        },

                        // Boolean -> Invalid
                        _ => {
                            token.kind = Kind::Invalid;
                            token.update(c);
                        }
                    }
                },

                Kind::Invalid => {
                    match c {
                        // Invalid -> Done
                        _ if c.is_whitespace() => {
                            break;
                        }

                        // Invalid -> Invalid
                        _ => {
                            token.update(c);
                        }
                    }
                },

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
            Kind::Empty => {
                return tokens;
            }

            _ => {
                tokens.push(token);
            }
        }
    }
}

fn is_symbol_start(c: char) -> bool {
    return c.is_ascii_lowercase() || "*-></+!?".contains(c);
}

fn is_symbol(c: char) -> bool {
    return is_symbol_start(c) || c.is_numeric();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tokenize(source: &str, exp_tokens: &Vec<Token>) {
        let act_tokens = tokenize(source);
        assert_eq!(exp_tokens, &act_tokens);
    }

    #[test]
    fn test_empty() {
        let exp_tokens = vec![];
        test_tokenize("", &exp_tokens);
        test_tokenize("    ", &exp_tokens);
        test_tokenize("\n\r\t\n", &exp_tokens);
    }

    #[test]
    fn test_nil() {
        let exp_tokens = vec![
            Token { kind: Kind::Nil, size: 3, data: "nil" }
        ];

        test_tokenize("nil", &exp_tokens);
        test_tokenize("\n\t nil\n ", &exp_tokens);
    }

    #[test]
    fn test_nil_to_list() {
        let exp_tokens = vec![
            Token { kind: Kind::Nil,       size: 3, data: "nil" },
            Token { kind: Kind::ListStart, size: 1, data: "("   },
            Token { kind: Kind::Nil,       size: 3, data: "nil" },
            Token { kind: Kind::ListEnd,   size: 1, data: ")"   }
        ];

        test_tokenize("nil(nil)", &exp_tokens);
    }

    #[test]
    fn test_nil_invalid() {
        let exp_tokens = vec![
            Token { kind: Kind::Invalid, size: 4, data: "nil," }
        ];

        test_tokenize("nil,", &exp_tokens);
        test_tokenize("\t\nnil, ", &exp_tokens);
    }

    #[test]
    fn test_boolean() {
        let exp_tokens = vec![
            Token { kind: Kind::Boolean, size: 4, data: "true"  },
            Token { kind: Kind::Boolean, size: 5, data: "false" }
        ];

        test_tokenize("true false", &exp_tokens);
        test_tokenize("\n true\r\t false\n", &exp_tokens);
    }

    #[test]
    fn test_integer() {
        let exp_tokens = vec![
            Token { kind: Kind::Integer, size: 1, data: "0"         },
            Token { kind: Kind::Integer, size: 2, data: "-1"        },
            Token { kind: Kind::Integer, size: 8, data: "12345678"  },
            Token { kind: Kind::Integer, size: 9, data: "-12345678" }
        ];

        test_tokenize("0 -1 12345678 -12345678", &exp_tokens);
        test_tokenize("\n0 \t-1  12345678\n-12345678 ", &exp_tokens);
    }

    #[test]
    fn test_integer_to_list() {
        let exp_tokens = vec![
            Token { kind: Kind::Integer,   size: 2, data: "-1"       },
            Token { kind: Kind::ListEnd,   size: 1, data: ")"        },
            Token { kind: Kind::Integer,   size: 8, data: "12345678" },
            Token { kind: Kind::ListStart, size: 1, data: "("        }
        ];

        test_tokenize("-1)12345678(", &exp_tokens);
    }

    #[test]
    fn test_integer_invalid() {
        let exp_tokens = vec![
            Token { kind: Kind::Invalid,   size: 5, data: "-123," },
            Token { kind: Kind::Invalid,   size: 2, data: "0$"    }
        ];

        test_tokenize("-123, 0$", &exp_tokens);
    }

    #[test]
    fn test_symbol() {
        let exp_tokens = vec![
            Token { kind: Kind::Symbol, size: 3, data: "foo"  },
            Token { kind: Kind::Symbol, size: 1, data: "a"    },
            Token { kind: Kind::Symbol, size: 4, data: "nill" }
        ];

        test_tokenize("foo a nill", &exp_tokens);
        test_tokenize("\n\t foo\na   nill \n\t\t\r\n", &exp_tokens);
    }

    #[test]
    fn test_symbol_to_list() {
        let exp_tokens = vec![
            Token { kind: Kind::Symbol,    size: 3, data: "foo" },
            Token { kind: Kind::ListEnd,   size: 1, data: ")"   },
            Token { kind: Kind::Symbol,    size: 3, data: "bar" },
            Token { kind: Kind::ListStart, size: 1, data: "("   }
        ];

        test_tokenize("foo)bar(", &exp_tokens);
    }

    #[test]
    fn test_symbol_invalid() {
        let exp_tokens = vec![
            Token { kind: Kind::Invalid, size: 5, data: "foo,," },
            Token { kind: Kind::Invalid, size: 6, data: "äöå"   },
            Token { kind: Kind::Invalid, size: 4, data: "_123"  }
        ];

        test_tokenize("foo,, äöå _123", &exp_tokens);
        test_tokenize("\n foo,,\täöå\n\t_123\t", &exp_tokens);
    }

    #[test]
    fn test_string() {
        let exp_tokens = vec![
            Token { kind: Kind::String, size: 0, data: "",      },
            Token { kind: Kind::String, size: 3, data: "abc",   },
            Token { kind: Kind::String, size: 4, data: "a\\nc", },
            Token { kind: Kind::String, size: 9, data: "北京市" }
        ];

        test_tokenize("\"\" \"abc\" \"a\\nc\" \"北京市\"", &exp_tokens);
        test_tokenize("\n \"\" \t \"abc\" \"a\\nc\" \"北京市\" \n", &exp_tokens);
    }

    #[test]
    fn test_string_to_list() {
        let exp_tokens = vec![
            Token { kind: Kind::String,    size: 3, data: "abc",   },
            Token { kind: Kind::ListStart, size: 1, data: "(",     },
            Token { kind: Kind::String,    size: 9, data: "北京市" },
            Token { kind: Kind::ListEnd,   size: 1, data: ")"      }
        ];

        test_tokenize("\"abc\"(\"北京市\")", &exp_tokens);
    }

    #[test]
    fn test_string_partial() {
        let exp_tokens = vec![
            Token { kind: Kind::StringPartial, size: 6, data: "xyz..." }
        ];

        test_tokenize("  \"xyz...", &exp_tokens);
    }

    #[test]
    fn test_list() {
        let exp_tokens = vec![
            Token { kind: Kind::ListStart, size: 1, data: "(" },
            Token { kind: Kind::ListStart, size: 1, data: "(" },
            Token { kind: Kind::ListEnd,   size: 1, data: ")" },
            Token { kind: Kind::ListEnd,   size: 1, data: ")" }
        ];

        test_tokenize("(())", &exp_tokens);
        test_tokenize("\n(\t (\n  )\n\r\t)\n ", &exp_tokens);
    }

    #[test]
    fn test_quote() {
        let exp_tokens = vec![
            Token { kind: Kind::Quote,  size: 1, data: "'" },
            Token { kind: Kind::Symbol, size: 3, data: "foo" },
            Token { kind: Kind::Quote,  size: 1, data: "'" },
            Token { kind: Kind::Quote,  size: 1, data: "'" },
            Token { kind: Kind::Symbol, size: 3, data: "bar" },
        ];

        test_tokenize("'foo '' bar", &exp_tokens);
    }

    #[test]
    fn test_multi() {
        let exp_tokens = vec![
            Token { kind: Kind::ListStart, size: 1, data: "("      },
            Token { kind: Kind::Symbol,    size: 3, data: "foo"    },
            Token { kind: Kind::Boolean,   size: 4, data: "true"   },
            Token { kind: Kind::Invalid,   size: 2, data: "a,"     },
            Token { kind: Kind::Nil,       size: 3, data: "nil"    },
            Token { kind: Kind::String,    size: 3, data: "abc"    },
            Token { kind: Kind::Symbol,    size: 3, data: "bar"    },
            Token { kind: Kind::Boolean,   size: 5, data: "false"  },
            Token { kind: Kind::String,    size: 9, data: "北京市" },
            Token { kind: Kind::ListEnd,   size: 1, data: ")"      }
        ];

        test_tokenize("(foo true a, nil\"abc\"bar false\"北京市\")", &exp_tokens);
        test_tokenize("(\n\t foo\ttrue\na, \tnil \"abc\" \nbar false\t\"北京市\" \t\t)\r\n", &exp_tokens);
    }
}
