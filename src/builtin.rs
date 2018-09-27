use ::env::Env;
use ::eval::{eval, apply_builtin, EvalResult, EvalError};
use ::sx::{*};

pub static BUILTIN_ARRAY: &'static [&SxBuiltin] = &[
    &SPECIAL_DEF,
    &SPECIAL_IF,
    &SPECIAL_QUOTE,

    &PRIMITIVE_APPLY,
    &PRIMITIVE_PLUS,
    &PRIMITIVE_PRODUCT
];

static SPECIAL_DEF: SxBuiltin = SxBuiltin {
    name:       "def",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Special(special_def)
};

static SPECIAL_IF: SxBuiltin = SxBuiltin {
    name:       "if",
    min_arity:  3,
    max_arity:  Some(3),
    callback:   SxBuiltinCallback::Special(special_if)
};

static SPECIAL_QUOTE: SxBuiltin = SxBuiltin {
    name:       "quote",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Special(special_quote)
};

static PRIMITIVE_APPLY: SxBuiltin = SxBuiltin {
    name:       "apply",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Primitive(primitive_apply)
};

static PRIMITIVE_PLUS: SxBuiltin = SxBuiltin {
    name:       "+",
    min_arity:  0,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_plus)
};

static PRIMITIVE_PRODUCT: SxBuiltin = SxBuiltin {
    name:       "*",
    min_arity:  0,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_product)
};

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

fn special_quote(_env: &mut Env, args: &Vec<&Sx>) -> EvalResult {
    return Ok(args[0].clone());
}

// TODO: call apply in eval.rs
fn primitive_apply(env: &mut Env, args: &Vec<Sx>) -> EvalResult {
    let head = &args[0];
    let value = &args[1];
    match (eval(env, head), value) {
        (Ok(Sx::Builtin(builtin)), Sx::List(sub_args)) => {
            return apply_builtin(builtin, env, sub_args);
        },

        (Ok(Sx::Builtin(_)), value) => {
            return Err(EvalError::BuiltinBadArg(PRIMITIVE_APPLY.name, value.clone()));
        },

        (Ok(v), _) => {
            return Err(EvalError::NotAFunction(v.clone()));
        },

        (error @ Err(_), _) => {
            return error;
        }
    }
}

fn primitive_plus(_env: &mut Env, args: &Vec<Sx>) -> EvalResult {
    let mut sum = 0;
    for arg in args {
        match arg {
            Sx::Integer(n) => {
                sum += n;
            },

            _ => {
                return Err(EvalError::BuiltinBadArg(PRIMITIVE_PLUS.name, arg.clone()));
            }
        }
    }

    // TODO: overflow
    return Ok(sx_integer!(sum));
}

fn primitive_product(_env: &mut Env, args: &Vec<Sx>) -> EvalResult {
    let mut product = 1;
    for arg in args {
        match arg {
            Sx::Integer(n) => {
                product *= n;
            },

            _ => {
                return Err(EvalError::BuiltinBadArg(PRIMITIVE_PRODUCT.name, arg.clone()));
            }
        }
    }

    // TODO: overflow
    return Ok(sx_integer!(product));
}
