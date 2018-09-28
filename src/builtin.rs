use std::sync::Arc;

use rpds::List;

use ::env::Env;
use ::eval::{eval, apply_builtin, apply_function, EvalResult, EvalError};
use ::sx::{*};

pub static BUILTIN_ARRAY: &'static [&SxBuiltinInfo] = &[
    &SPECIAL_DEF,
    &SPECIAL_FN,
    &SPECIAL_IF,
    &SPECIAL_QUOTE,

    &PRIMITIVE_APPLY,
    &PRIMITIVE_PLUS,
    &PRIMITIVE_PRODUCT,
    &PRIMITIVE_RANGE,
];

static SPECIAL_DEF: SxBuiltinInfo = SxBuiltinInfo {
    name:       "def",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Special(special_def)
};

static SPECIAL_FN: SxBuiltinInfo = SxBuiltinInfo {
    name:       "fn",
    min_arity:  2,
    max_arity:  None,
    callback:   SxBuiltinCallback::Special(special_fn)
};

static SPECIAL_IF: SxBuiltinInfo = SxBuiltinInfo {
    name:       "if",
    min_arity:  3,
    max_arity:  Some(3),
    callback:   SxBuiltinCallback::Special(special_if)
};

static SPECIAL_QUOTE: SxBuiltinInfo = SxBuiltinInfo {
    name:       "quote",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Special(special_quote)
};

static PRIMITIVE_APPLY: SxBuiltinInfo = SxBuiltinInfo {
    name:       "apply",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Primitive(primitive_apply)
};

static PRIMITIVE_PLUS: SxBuiltinInfo = SxBuiltinInfo {
    name:       "+",
    min_arity:  0,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_plus)
};

static PRIMITIVE_PRODUCT: SxBuiltinInfo = SxBuiltinInfo {
    name:       "*",
    min_arity:  0,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_product)
};

static PRIMITIVE_RANGE: SxBuiltinInfo = SxBuiltinInfo {
    name:       "range",
    min_arity:  0,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Primitive(primitive_range)
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

fn special_fn(_env: &mut Env, args: &Vec<&Sx>) -> EvalResult {
    let binding_list = args[0];
    match binding_list {
        Sx::List(bindings) => {
            let mut bindings_vec = Vec::new();
            for binding in bindings.iter() {
                match binding {
                    Sx::Symbol(name) => {
                        if bindings_vec.contains(name) {
                            return Err(EvalError::DuplicateBinding(name.clone()));
                        }

                        bindings_vec.push(name.clone());
                    },

                    invalid => {
                        return Err(EvalError::InvalidBinding(invalid.clone()));
                    }
                }
            }

            let mut body = List::new();
            for i in 1 .. args.len() {
                body.push_front_mut(args[i].clone());
            }

            body.reverse_mut();

            let f = SxFunctionInfo {
                arity:      bindings.len(),
                bindings:   bindings_vec,
                body:       Arc::new(body.clone())
            };

            return Ok(Sx::Function(Arc::new(f)));
        },

        _ => {
            return Err(EvalError::BuiltinBadArg(SPECIAL_FN.name, binding_list.clone()));
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

        (Ok(Sx::Function(ref f)), Sx::List(sub_args)) => {
            return apply_function(f, env, sub_args);
        }

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

// TODO: vectors
fn primitive_range(_env: &mut Env, args: &Vec<Sx>) -> EvalResult {
    let mut numbers = List::new();

    match args[..] {
        [Sx::Integer(end)] => {
            for i in 0i64 .. end {
                numbers.push_front_mut(sx_integer!(i));
            }
        },

        [Sx::Integer(start), Sx::Integer(end)] => {
            for i in start .. end {
                numbers.push_front_mut(sx_integer!(i));
            }
        },

        [ref arg, Sx::Integer(_)] => {
            return Err(EvalError::BuiltinBadArg(PRIMITIVE_RANGE.name, arg.clone()));
        },

        [_, ref arg] => {
            return Err(EvalError::BuiltinBadArg(PRIMITIVE_RANGE.name, arg.clone()));
        },

        _ => {
            assert!(false);
        }
    }

    numbers.reverse_mut();

    return Ok(Sx::List(Arc::new(numbers)));
}
