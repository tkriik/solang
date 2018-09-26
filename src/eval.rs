use ::std::sync::Arc;

use ::env::Env;
use ::sx::{Sx, SxSymbol, SxList};

#[derive(Debug)]
pub enum EvalError {
    Undefined(SxSymbol),
    Redefine(SxSymbol),

    TooFewArgs(SxSymbol),
    TooManyArgs(SxSymbol),

    DefineTooFewArgs,
    DefineTooManyArgs,
    DefineBadSymbol(Sx),

    QuoteTooFewArgs,
    QuoteTooManyArgs,

    Unknown(Sx)
}

pub fn eval(env: &mut Env, sx: &Sx) -> Result<Sx, EvalError> {
    match sx {
        Sx::Nil | Sx::Boolean(_) | Sx::Integer(_) | Sx::String(_) => {
            return Ok(sx.clone());
        },

        Sx::List(l) if l.is_empty() => {
            return Ok(sx.clone());
        },

        Sx::Quote(v) => {
            return Ok(v.as_ref().clone());
        },

        Sx::Symbol(symbol) => {
            match env.lookup(symbol) {
                Some(v) => {
                    return Ok(v.clone());
                },

                None => {
                    return Err(EvalError::Undefined(symbol.clone()));
                }
            }
        },

        Sx::List(l) => {
            match l.first() {
                Some(Sx::Symbol(name)) => {
                    match name.as_str() {
                        "def" => {
                            return do_def(env, l);
                        },

                        "if" => {
                            return do_if(env, l);
                        },

                        "quote" => {
                            return do_quote(l);
                        },

                        _ => {
                            return Err(EvalError::Unknown(sx.clone()))
                        }
                    }
                }

                _ => {
                    return Err(EvalError::Unknown(sx.clone()));
                }
            }
        }
    }
}


// TODO: refactor
fn do_def(env: &mut Env, list: &SxList) -> Result<Sx, EvalError> {
    let mut args = Vec::new();
    let mut first = true;
    for sub_sx in list.iter() {
        if first {
            first = false;
            continue;
        }

        args.push(sub_sx);
    }

    if args.len() < 2 {
        return Err(EvalError::DefineTooFewArgs);
    }

    if 2 < args.len() {
        return Err(EvalError::DefineTooManyArgs);
    }

    let symbol = args[0];
    match symbol {
        Sx::Symbol(name) => {
            match env.lookup(name) {
                None => {
                    let value = args[1];
                    match eval(env, value) {
                        Ok(result) => {
                            env.define(name, &result);
                            return Ok(symbol.clone());
                        },

                        error @ Err(_) => {
                            return error;
                        }
                    }
                },

                Some(_) => {
                    return Err(EvalError::Redefine(name.clone()));
                }
            }
        },

        _ => {
            return Err(EvalError::DefineBadSymbol(symbol.clone()))
        }
    }
}

fn do_if(env: &mut Env, list: &SxList) -> Result<Sx, EvalError> {
    let mut args = Vec::new();
    let mut first = true;
    for sub_sx in list.iter() {
        if first {
            first = false;
            continue;
        }

        args.push(sub_sx);
    }

    if args.len() < 3 {
        return Err(EvalError::TooFewArgs(Arc::new("if".to_string())));
    }

    if 3 < args.len() {
        return Err(EvalError::TooManyArgs(Arc::new("if".to_string())));
    }

    let cond = args[0];
    let true_path = args[1];
    let false_path = args[2];

    match eval(env, cond) {
        Ok(Sx::Nil) | Ok(Sx::Boolean(false)) => {
            return eval(env, false_path);
        },

        Ok(_) => {
            return eval(env, true_path);
        },

        error @ Err(_) => {
            return error;
        }
    }
}

fn do_quote(list: &SxList) -> Result<Sx, EvalError> {
    let mut args = Vec::new();
    let mut first = true;
    for sub_sx in list.iter() {
        if first {
            first = false;
            continue;
        }

        args.push(sub_sx);
    }

    if args.len() < 1 {
        return Err(EvalError::QuoteTooFewArgs);
    }

    if 1 < args.len() {
        return Err(EvalError::QuoteTooManyArgs);
    }

    return Ok(args[0].clone());
}
