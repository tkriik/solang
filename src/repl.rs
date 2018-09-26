extern crate rustyline;

use std::error::Error;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use ::env::Env;
use ::eval::{eval, EvalError};
use ::read::{read, ReadError};
use ::sx::Sx;

pub fn enter() {
    let history_path = ".solang_history";

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_path);

    let mut env = Env::new();

    loop {
        let readline  = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match read(&line) {
                    Ok(Sx::List(sxs)) => {
                        for sx in sxs.iter() {
                            match eval(&mut env, sx) {
                                Ok(result) => {
                                    println!("{}", result.to_string());
                                },

                                Err(eval_error) => {
                                    print_eval_error(&eval_error);
                                }
                            }
                        }
                    },

                    Err(read_errors) => {
                        for read_error in read_errors {
                            print_read_error(&read_error);
                        }
                    },

                    _ => {
                        assert!(false);
                    }
                }

                rl.add_history_entry(line.as_ref());
            },

            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }

            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }

            Err(err) => {
                println!("Error while reading line: {:?}", err);
                break;
            }
        }
    }

    match rl.save_history(history_path) {
        Ok(_) => (),

        Err(err) => {
            println!("Failed to save shell history to {}: {}",
                     history_path, err.description());
        }
    }
}

fn print_read_error(read_error: &ReadError) {
    match read_error {
        ReadError::InvalidToken(s) => {
            println!("read error: invalid token: {}", s);
        },

        ReadError::IntegerLimit(s) => {
            println!("read error: integer limit: {}", s);
        },

        ReadError::PartialString(s) => {
            println!("read error: non-terminated string: {}", s);
        },

        ReadError::TrailingDelimiter(s) => {
            println!("read error: trailing delimiter: '{}'", s);
        },

        ReadError::UnmatchedDelimiter => {
            println!("read error: unmatched delimiter");
        }
    }
}

fn print_eval_error(eval_error: &EvalError ) {
    match eval_error {
        EvalError::Undefined(sx) => {
            println!("eval error: undefined symbol: {}", sx.to_string());
        },

        EvalError::Redefine(symbol) => {
            println!("eval error: cannot redefine symbol {}", symbol.to_string());
        },

        EvalError::DefineTooFewArgs => {
            println!("eval error: too few arguments to def");
        },

        EvalError::DefineTooManyArgs => {
            println!("eval error: too many arguments to def");
        },

        EvalError::DefineBadSymbol(sx) => {
            println!("eval error: first argument to def must be a symbol, got {}", sx.to_string());
        },

        EvalError::Unknown(sx) => {
            println!("eval error: don't know how to evaluate expression: {}", sx.to_string());
        }
    }
}