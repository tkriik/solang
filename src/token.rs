use unicode_segmentation::UnicodeSegmentation;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
    Nil,
    Symbol,
    String,
    ListStart,
    ListEnd,
    Invalid
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: Kind,
    pub size: usize,
    pub data: &'a str
}

impl <'a> Token<'a> {
    fn init(&mut self, kind: Kind, source: &'a str, offset: usize, c: char) {
        self.kind = kind;
        self.size = c.len_utf8();
        self.data = &source[offset ..];
    }

    fn update(&mut self, c: char) {
        self.size += c.len_utf8();
    }

    fn finalize(&mut self) {
        self.data = &self.data[.. self.size];
    }
}

enum State {
    Seek,
    Start(Kind),
    At(Kind)
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = State::Seek;

    let mut token = Token {
        kind: Kind::Invalid,
        data: &source,
        size: 0
    };

    for (_, (offset, s)) in source.grapheme_indices(true).enumerate() {
        let c = s.chars().next().unwrap();
        match state {
            State::Seek => {
                match c {
                    _ if c.is_whitespace() => (),

                    _ if is_symbol_start(c) => {
                        token.init(Kind::Symbol, source, offset, c);
                        state = State::At(Kind::Symbol);
                    },

                    '"' => {
                        state = State::Start(Kind::String)
                    },

                    '(' | ')' => {
                        let kind = if c == '(' {
                            Kind::ListStart
                        } else {
                            Kind::ListEnd
                        };

                        token.init(kind, source, offset, c);
                        token.finalize();
                        tokens.push(token);
                        state = State::Seek;
                    }

                    _ => {
                        token.init(Kind::Invalid, source, offset, c);
                        state = State::At(Kind::Invalid);
                    }
                }
            }

            State::At(Kind::Nil) => {
                match c {
                    _ if c.is_whitespace() => {
                        token.finalize();
                        tokens.push(token);
                        state = State::Seek;
                    }

                    _ if is_symbol(c) => {
                        token.kind = Kind::Symbol;
                        token.update(c);
                        state = State::At(Kind::Symbol);
                    }

                    '"' => {
                        token.finalize();
                        tokens.push(token);
                        state = State::Start(Kind::String);
                    }

                    '(' | ')' => {
                        token.finalize();
                        tokens.push(token);

                        let kind = if c == '(' {
                            Kind::ListStart
                        } else {
                            Kind::ListEnd
                        };

                        token.init(kind, source, offset, c);
                        token.finalize();
                        tokens.push(token);
                        state = State::Seek;
                    }

                    _ => {
                        token.kind = Kind::Invalid;
                        token.update(c);
                        state = State::At(Kind::Invalid);
                    }
                }
            }

            State::At(Kind::Symbol) => {
                match c {
                    'l' if token.size == 2 && token.data.starts_with("ni") => {
                        token.kind = Kind::Nil;
                        token.update(c);
                        state = State::At(Kind::Nil);
                    }

                    _ if c.is_whitespace() => {
                        token.finalize();
                        tokens.push(token);
                        state = State::Seek;
                    }

                    '"' => {
                        token.finalize();
                        tokens.push(token);
                        state = State::Start(Kind::String);
                    }

                    '(' | ')' => {
                        token.finalize();
                        tokens.push(token);

                        let kind = if c == '(' {
                            Kind::ListStart
                        } else {
                            Kind::ListEnd
                        };

                        token.init(kind, source, offset, c);
                        token.finalize();
                        tokens.push(token);
                        state = State::Seek;
                    }

                    _ if !is_symbol(c) => {
                        token.kind = Kind::Invalid;
                        token.update(c);
                        state = State::At(Kind::Invalid);
                    }

                    _ => {
                        token.update(c);
                    }
                }
            }

            State::Start(Kind::String) => {
                if c == '"' {
                    token.kind = Kind::String;
                    token.finalize();
                    tokens.push(token);
                    state = State::Seek;
                    continue;
                }

                token.init(Kind::String, source, offset, c);
                state = State::At(Kind::String);
                continue;
            }

            State::At(Kind::String) => {
                if c == '"' {
                    token.finalize();
                    tokens.push(token);
                    state = State::Seek;
                    continue;
                }

                token.update(c);
                continue;
            }

            State::At(Kind::Invalid) => {
                if c.is_whitespace() {
                    token.finalize();
                    tokens.push(token);
                    state = State::Seek;
                    continue;
                }

                token.update(c);
                continue;
            }

            _ => {
                assert!(false);
            }
        }
    }

    match state {
        State::At(Kind::String) => {
            token.kind = Kind::Invalid;
            token.finalize();
            tokens.push(token);
        }

        State::At(_) => {
            token.finalize();
            tokens.push(token);
        }

        _ => ()
    }

    return tokens;
}

fn is_symbol_start(c: char) -> bool {
    return c.is_ascii_lowercase() || "*-></+!".contains(c);
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
            Token { kind: Kind::Invalid, size: 3, data: "123"   }
        ];

        test_tokenize("foo,, äöå 123", &exp_tokens);
        test_tokenize("\n foo,,\täöå\n\t123\t", &exp_tokens);
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
            Token { kind: Kind::ListEnd,   size: 1, data: ")",     }
        ];

        test_tokenize("\"abc\"(\"北京市\")", &exp_tokens);
    }

    #[test]
    fn test_string_invalid() {
        let exp_tokens = vec![
            Token { kind: Kind::Invalid, size: 6, data: "xyz..." }
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
    fn test_multi() {
        let exp_tokens = vec![
            Token { kind: Kind::ListStart, size: 1, data: "("      },
            Token { kind: Kind::Symbol,    size: 3, data: "foo"    },
            Token { kind: Kind::Invalid,   size: 2, data: "a,"     },
            Token { kind: Kind::Nil,       size: 3, data: "nil"    },
            Token { kind: Kind::String,    size: 3, data: "abc"    },
            Token { kind: Kind::Symbol,    size: 3, data: "bar"    },
            Token { kind: Kind::String,    size: 9, data: "北京市"  },
            Token { kind: Kind::ListEnd,   size: 1, data: ")"      }
        ];

        test_tokenize("(foo a, nil\"abc\"bar\"北京市\")", &exp_tokens);
        test_tokenize("(\n\t foo\na, \tnil \"abc\" \nbar\"北京市\" \t\t)\r\n", &exp_tokens);
    }
}
