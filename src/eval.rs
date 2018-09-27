use ::rpds::List;

use ::env::Env;
use ::sx::{Sx, SxSymbol};

#[derive(Eq, PartialEq, Debug)]
pub enum EvalError {
    Undefined(SxSymbol),
    Redefine(SxSymbol),

    SpecialTooFewArgs(SxSymbol),
    SpecialTooManyArgs(SxSymbol),

    DefineBadSymbol(Sx),

    Unknown(Sx)
}

pub type EvalResult = Result<Sx, EvalError>;

pub fn eval(env: &mut Env, sx: &Sx) -> EvalResult {
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
            match (l.first(), l.drop_first()) {
                (Some(Sx::Symbol(ref symbol)), Some(ref args)) => {
                    match symbol.as_str() {
                        "def" => {
                            return apply_special(symbol, special_def, 2, env, &args);
                        },

                        "if" => {
                            return apply_special(symbol, special_if, 3, env, &args);
                        },

                        "quote" => {
                            return apply_special(symbol, special_quote, 1, env, &args);
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

type SpecialFn = fn(&mut Env, &Vec<&Sx>) -> EvalResult;

fn apply_special(symbol: &SxSymbol,
                 special_fn: SpecialFn,
                 arity: usize,
                 env: &mut Env,
                 arglist: &List<Sx>) -> EvalResult {
    let mut args = Vec::new();
    for arg in arglist.iter() {
        args.push(arg);
    }

    if args.len() < arity {
        return Err(EvalError::SpecialTooFewArgs(symbol.clone()));
    }

    if arity < args.len() {
        return Err(EvalError::SpecialTooManyArgs(symbol.clone()));
    }

    return special_fn(env, &args);
}

fn special_def(env: &mut Env, args: &Vec<&Sx>) -> EvalResult {
    let binding = args[0];
    match binding {
        Sx::Symbol(symbol) => {
            match env.lookup(symbol) {
                None => {
                    let value = args[1];
                    match eval(env, value) {
                        Ok(result) => {
                            env.define(symbol, &result);
                            return Ok(binding.clone());
                        },

                        error @ Err(_) => {
                            return error;
                        }
                    }
                },

                Some(_) => {
                    return Err(EvalError::Redefine(symbol.clone()));
                }
            }
        },

        _ => {
            return Err(EvalError::DefineBadSymbol(binding.clone()));
        }
    }
}

fn special_if(env: &mut Env, args: &Vec<&Sx>) -> EvalResult {
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
