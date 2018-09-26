use ::env::Env;
use ::sx::{Sx, SxSymbol, SxList};

#[derive(Debug)]
pub enum EvalError {
    Undefined(SxSymbol),
    Redefine(SxSymbol),

    DefineTooFewArgs,
    DefineTooManyArgs,
    DefineBadSymbol(Sx),

    Unknown(Sx)
}

pub fn eval(env: &mut Env, sx: &Sx) -> Result<Sx, EvalError> {
    match sx {
        Sx::Nil | Sx::Integer(_) | Sx::String(_) => {
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
                Some(Sx::Symbol(name)) if name.as_ref() == "def" => {
                    return do_def(env, l);
                },

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
