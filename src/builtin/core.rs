use std::sync::Arc;

use im;
use time;

use ::eval::{Env, Result, Error, eval, apply_builtin, apply_function};
use ::module;
use ::sx::{*};
use ::util::pretty::pretty;

pub static BUILTIN_MODULE_NAME: &'static str = "core";

pub static BUILTIN_TABLE: &'static [SxBuiltinInfo] = &[
    // Specials
    special!("def", 2, special_def),
    special_no_arg_limit!("fn", 2, special_fn),
    special!("if", 3, special_if),
    special!("module", 1, special_module),
    special!("quote", 1, special_quote),
    special!("use", 1, special_use),

    // General
    primitive!("apply", 2, primitive_apply),
    primitive!("env", 0, primitive_env),
    primitive!("trace", 2, primitive_trace),

    // Collections
    primitive!("cons", 2, primitive_cons),
    primitive!("head", 1, primitive_head),
    primitive!("tail", 1, primitive_tail),
    primitive_var_arity!("range", 0, 2, primitive_range),

    // Logic
    primitive_no_arg_limit!("eq", 1, primitive_eq),

    // Numbers
    primitive_no_arg_limit!("+", 0, primitive_plus),
    primitive_no_arg_limit!("-", 1, primitive_minus),
    primitive_no_arg_limit!("*", 0, primitive_product)
];

fn special_def(env: &mut Env, args: &[Sx]) -> Result {
    let binding = &args[0];
    match binding {
        Sx::Symbol(ref symbol) => {
            match env.lookup_core(symbol) {
                Some(_) => return Err(Error::RedefineCore(symbol.clone())),
                None    => ()
            }

            match env.lookup_current(symbol) {
                None => {
                    let value = &args[1];
                    match eval(env, value) {
                        Ok(result) => {
                            env.define_current(symbol, &result);
                            return Ok(binding.clone());
                        },

                        error @ Err(_) => {
                            return error;
                        }
                    }
                },

                Some(_) => {
                    return Err(Error::Redefine(symbol.clone()));
                }
            }
        },

        _ => {
            return Err(Error::DefineBadSymbol(binding.clone()));
        }
    }
}

// TODO: vector binding
fn special_fn(env: &mut Env, args: &[Sx]) -> Result {
    let binding_list = &args[0];
    match binding_list {
        Sx::List(bindings) => {
            let mut bindings_vec = Vec::new();
            for binding in bindings.iter() {
                match binding {
                    Sx::Symbol(name) => {
                        if bindings_vec.contains(name) {
                            return Err(Error::DuplicateBinding(name.clone()));
                        }

                        bindings_vec.push(name.clone());
                    },

                    invalid => {
                        return Err(Error::InvalidBinding(invalid.clone()));
                    }
                }
            }

            let mut body = Vec::new();
            for i in 1 .. args.len() {
                body.push(args[i].clone());
            }

            let f = SxFunctionInfo {
                module:     env.current_module.clone(),
                arity:      bindings.len(),
                bindings:   bindings_vec,
                body:       Arc::new(body)
            };

            return Ok(Sx::Function(Arc::new(f)));
        },

        _ => {
            return Err(Error::BuiltinBadArg("fn", binding_list.clone()));
        }
    }
}

fn special_if(env: &mut Env, args: &[Sx]) -> Result {
    let cond = &args[0];
    let true_path = &args[1];
    let false_path = &args[2];

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

fn special_module(env: &mut Env, args: &[Sx]) -> Result {
    let module_name_arg = &args[0];
    let module_name = match module_name_arg {
        Sx::Symbol(module_name) => module_name,
        _ => return Err(Error::BuiltinBadArg("module", module_name_arg.clone()))
    };

    env.loaded_modules.insert(module_name.clone());
    env.current_module = module_name.clone();

    return Ok(Sx::Symbol(module_name.clone()));
}

fn special_quote(_env: &mut Env, args: &[Sx]) -> Result {
    return Ok(args[0].clone());
}

fn special_use(env: &mut Env, args: &[Sx]) -> Result {
    let module_arg = &args[0];
    match module_arg {
        Sx::Symbol(ref module_name) => {
            return module::load_use(env, module_name);
        },

        _ => {
            return Err(Error::BuiltinBadArg("use", module_arg.clone()));
        }
    }
}

// TODO: generic iterators
fn primitive_apply(env: &mut Env, args: &[Sx]) -> Result {
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
            return Err(Error::BuiltinBadArg("apply", value.clone()));
        },

        (Ok(v), _) => {
            return Err(Error::NotAFunction(v.clone()));
        },

        (error @ Err(_), _) => {
            return error;
        }
    }
}

// TODO: iterator magic
fn primitive_env(env: &mut Env, _args: &[Sx]) -> Result {
    let mut module_paths = Vec::new();
    for module_path in env.module_paths.iter() {
        module_paths.push(sx_string!(module_path));
    }

    let current_module = env.current_module.clone();

    let mut loaded_modules = Vec::new();
    for module in env.loaded_modules.iter() {
        loaded_modules.push(Sx::Symbol(module.clone()));
    }

    let mut env_list = Vec::new();
    for ((module, symbol), value) in env.definitions.iter() {
        env_list.push(sx_vector![Sx::Symbol(module.clone()), Sx::Symbol(symbol.clone()), value.clone()]);
    }

    return Ok(sx_vector![
        sx_vector![sx_symbol!("module-paths"), sx_vector_from_vec!(module_paths)],
        sx_vector![sx_symbol!("current-module"), Sx::Symbol(current_module)],
        sx_vector![sx_symbol!("loaded-modules"), sx_vector_from_vec!(loaded_modules)],
        sx_vector![sx_symbol!("definitions"), sx_vector_from_vec!(env_list)]
    ]);
}

fn primitive_trace(_env: &mut Env, args: &[Sx]) -> Result {
    let ts = time::now();

    let label_arg = &args[0];
    let label = match label_arg {
        Sx::String(s) => s,
        _ => return Err(Error::BuiltinBadArg("trace", label_arg.clone()))
    };

    let value = &args[1];

    println!(r#"
-----BEGIN TRACE-----
Label: {}
Timestamp: {}

{}
-----END TRACE-----"#,
             label.as_ref(), ts.rfc3339(), pretty(value));

    return Ok(value.clone());
}

// TODO: vector
fn primitive_cons(_env: &mut Env, args: &[Sx]) -> Result {
    let value = &args[0];
    let list_arg = &args[1];

    match list_arg {
        Sx::List(list) => {
            let mut new_list = list.as_slice().to_vec();
            new_list.insert(0, value.clone());
            return Ok(Sx::List(Arc::new(new_list)));
        },

        _ => {
            return Err(Error::BuiltinBadArg("cons", list_arg.clone()));
        }
    }
}

// TODO: vector
fn primitive_head(_env: &mut Env, args: &[Sx]) -> Result {
    let list_arg = &args[0];
    match list_arg {
        Sx::List(list) => {
            match list.first() {
                Some(value) => {
                    return Ok(value.clone());
                },

                None => {
                    return Err(Error::BuiltinBadArg("head", list_arg.clone()));
                }
            }
        },

        _ => {
            return Err(Error::BuiltinBadArg("head", list_arg.clone()));
        }
    }
}

// TODO: vector
fn primitive_tail(_env: &mut Env, args: &[Sx]) -> Result {
    let list_arg = &args[0];
    match list_arg {
        Sx::List(list) => {
            match list.split_first() {
                Some((_, tail_slice)) => {
                    let tail = tail_slice.to_vec();
                    return Ok(Sx::List(Arc::new(tail)));
                },

                None => {
                    return Err(Error::BuiltinBadArg("tail", list_arg.clone()));
                }
            }
        },

        _ => {
            return Err(Error::BuiltinBadArg("tail", list_arg.clone()));
        }
    }
}

// TODO: vectors
fn primitive_range(_env: &mut Env, args: &[Sx]) -> Result {
    let mut numbers = im::Vector::new();

    match args[..] {
        [Sx::Integer(end)] => {
            for i in 0i64 .. end {
                numbers.push_back(sx_integer!(i));
            }
        },

        [Sx::Integer(start), Sx::Integer(end)] => {
            for i in start .. end {
                numbers.push_back(sx_integer!(i));
            }
        },

        [ref arg, Sx::Integer(_)] => {
            return Err(Error::BuiltinBadArg("range", arg.clone()));
        },

        [_, ref arg] => {
            return Err(Error::BuiltinBadArg("range", arg.clone()));
        },

        _ => {
            assert!(false);
        }
    }

    return Ok(Sx::Vector(Arc::new(numbers)));
}

fn primitive_eq(_env: &mut Env, args: &[Sx]) -> Result {
    if args.len() == 1 {
        return Ok(Sx::Boolean(true));
    }

    let mut eq = true;
    let mut first = true;
    let value = &args[0];
    for arg in args.iter() {
        if first {
            first = false;
            continue;
        }

        if arg != value {
            eq = false;
            break;
        }
    }

    return Ok(Sx::Boolean(eq));
}

fn primitive_plus(_env: &mut Env, args: &[Sx]) -> Result {
    let mut sum = 0i64;
    for arg in args.iter() {
        match arg {
            Sx::Integer(n) => {
                sum += n;
            },

            _ => {
                return Err(Error::BuiltinBadArg("+", arg.clone()));
            }
        }
    }

    // TODO: overflow
    return Ok(sx_integer!(sum));
}

fn primitive_minus(_env: &mut Env, args: &[Sx]) -> Result {
    let diff_arg = &args[0];
    match diff_arg {
        Sx::Integer(x) => {
            if args.len() == 1 {
                return Ok(sx_integer!(-x));
            }

            let mut first = true;
            let mut diff = *x;
            for arg in args.iter() {
                if first {
                    first = false;
                    continue;
                }

                match arg {
                    Sx::Integer(n) => {
                        diff -= n;
                    },

                    _ => {
                        return Err(Error::BuiltinBadArg("-", arg.clone()));
                    }
                }
            }

            // TODO: overflow
            return Ok(sx_integer!(diff));
        },

        _ => {
            return Err(Error::BuiltinBadArg("-", diff_arg.clone()));
        }
    }
}

fn primitive_product(_env: &mut Env, args: &[Sx]) -> Result {
    let mut product = 1;
    for arg in args {
        match arg {
            Sx::Integer(n) => {
                product *= n;
            },

            _ => {
                return Err(Error::BuiltinBadArg("*", arg.clone()));
            }
        }
    }

    // TODO: overflow
    return Ok(sx_integer!(product));
}
