use std::sync::Arc;

use im;
use time;

use ::eval::ctx::Ctx;
use ::eval::{module, Result, Error};
use ::eval::eval::{eval, apply_builtin, apply_function};
use ::sx::{*};
use ::util::pretty::pretty;

pub static BUILTIN_MODULE_NAME: &'static str = "core";

pub static BUILTIN_TABLE: &'static [&SxBuiltinInfo] = &[
    // Specials
    &SPECIAL_DEF,
    &SPECIAL_FN,
    &SPECIAL_IF,
    &SPECIAL_MODULE,
    &SPECIAL_QUOTE,
    &SPECIAL_USE,

    // General
    &PRIMITIVE_APPLY,
    &PRIMITIVE_CTX,
    &PRIMITIVE_TRACE,

    // Collections
    &PRIMITIVE_CONS,
    &PRIMITIVE_HEAD,
    &PRIMITIVE_TAIL,

    // Logic
    &PRIMITIVE_EQ,

    // Numbers
    &PRIMITIVE_PLUS,
    &PRIMITIVE_MINUS,
    &PRIMITIVE_PRODUCT,
    &PRIMITIVE_RANGE
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

static SPECIAL_MODULE: SxBuiltinInfo = SxBuiltinInfo {
    name:       "module",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Special(special_module)
};

static SPECIAL_QUOTE: SxBuiltinInfo = SxBuiltinInfo {
    name:       "quote",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Special(special_quote)
};

static SPECIAL_USE: SxBuiltinInfo = SxBuiltinInfo {
    name:       "use",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Special(special_use)
};

static PRIMITIVE_APPLY: SxBuiltinInfo = SxBuiltinInfo {
    name:       "apply",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Primitive(primitive_apply)
};

static PRIMITIVE_CTX: SxBuiltinInfo = SxBuiltinInfo {
    name:       "ctx",
    min_arity:  0,
    max_arity:  Some(0),
    callback:   SxBuiltinCallback::Primitive(primitive_ctx)
};

static PRIMITIVE_TRACE: SxBuiltinInfo = SxBuiltinInfo {
    name:       "trace",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Primitive(primitive_trace)
};

static PRIMITIVE_CONS: SxBuiltinInfo = SxBuiltinInfo {
    name:       "cons",
    min_arity:  2,
    max_arity:  Some(2),
    callback:   SxBuiltinCallback::Primitive(primitive_cons)
};

static PRIMITIVE_HEAD: SxBuiltinInfo = SxBuiltinInfo {
    name:       "head",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Primitive(primitive_head)
};

static PRIMITIVE_TAIL: SxBuiltinInfo = SxBuiltinInfo {
    name:       "tail",
    min_arity:  1,
    max_arity:  Some(1),
    callback:   SxBuiltinCallback::Primitive(primitive_tail)
};

static PRIMITIVE_EQ: SxBuiltinInfo = SxBuiltinInfo {
    name:       "=",
    min_arity:  1,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_eq)
};

static PRIMITIVE_PLUS: SxBuiltinInfo = SxBuiltinInfo {
    name:       "+",
    min_arity:  0,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_plus)
};

static PRIMITIVE_MINUS: SxBuiltinInfo = SxBuiltinInfo {
    name:       "-",
    min_arity:  1,
    max_arity:  None,
    callback:   SxBuiltinCallback::Primitive(primitive_minus)
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

fn special_def(ctx: &mut Ctx, args: &[Sx]) -> Result {
    let binding = &args[0];
    match binding {
        Sx::Symbol(ref symbol) => {
            match ctx.lookup_core(symbol) {
                Some(_) => return Err(Error::RedefineCore(symbol.clone())),
                None    => ()
            }

            match ctx.lookup_current(symbol) {
                None => {
                    let value = &args[1];
                    match eval(ctx, value) {
                        Ok(result) => {
                            ctx.define_current(symbol, &result);
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
fn special_fn(ctx: &mut Ctx, args: &[Sx]) -> Result {
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
                module:     ctx.current_module.clone(),
                arity:      bindings.len(),
                bindings:   bindings_vec,
                body:       Arc::new(body)
            };

            return Ok(Sx::Function(Arc::new(f)));
        },

        _ => {
            return Err(Error::BuiltinBadArg(SPECIAL_FN.name, binding_list.clone()));
        }
    }
}

fn special_if(ctx: &mut Ctx, args: &[Sx]) -> Result {
    let cond = &args[0];
    let true_path = &args[1];
    let false_path = &args[2];

    match eval(ctx, cond) {
        Ok(Sx::Nil) | Ok(Sx::Boolean(false)) => {
            return eval(ctx, false_path);
        },

        Ok(_) => {
            return eval(ctx, true_path);
        },

        error @ Err(_) => {
            return error;
        }
    }
}

fn special_module(ctx: &mut Ctx, args: &[Sx]) -> Result {
    let module_name_arg = &args[0];
    let module_name = match module_name_arg {
        Sx::Symbol(module_name) => module_name,
        _ => return Err(Error::BuiltinBadArg(SPECIAL_MODULE.name, module_name_arg.clone()))
    };

    ctx.loaded_modules.insert(module_name.clone());
    ctx.current_module = module_name.clone();

    return Ok(Sx::Symbol(module_name.clone()));
}

fn special_quote(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    return Ok(args[0].clone());
}

fn special_use(ctx: &mut Ctx, args: &[Sx]) -> Result {
    let module_arg = &args[0];
    match module_arg {
        Sx::Symbol(ref module_name) => {
            return module::load_use(ctx, module_name);
        },

        _ => {
            return Err(Error::BuiltinBadArg(SPECIAL_USE.name, module_arg.clone()));
        }
    }
}

// TODO: generic iterators
fn primitive_apply(ctx: &mut Ctx, args: &[Sx]) -> Result {
    let head = &args[0];
    let value = &args[1];
    match (eval(ctx, head), value) {
        (Ok(Sx::Builtin(builtin)), Sx::List(sub_args)) => {
            return apply_builtin(builtin, ctx, sub_args);
        },

        (Ok(Sx::Function(ref f)), Sx::List(sub_args)) => {
            return apply_function(f, ctx, sub_args);
        }

        (Ok(Sx::Builtin(_)), value) => {
            return Err(Error::BuiltinBadArg(PRIMITIVE_APPLY.name, value.clone()));
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
fn primitive_ctx(ctx: &mut Ctx, _args: &[Sx]) -> Result {
    let mut module_paths = Vec::new();
    for module_path in ctx.module_paths.iter() {
        module_paths.push(sx_string!(module_path));
    }

    let current_module = ctx.current_module.clone();

    let mut loaded_modules = Vec::new();
    for module in ctx.loaded_modules.iter() {
        loaded_modules.push(Sx::Symbol(module.clone()));
    }

    let mut defs = Vec::new();
    for ((module, symbol), value) in ctx.definitions.iter() {
        defs.push(sx_vector![Sx::Symbol(module.clone()), Sx::Symbol(symbol.clone()), value.clone()]);
    }

    return Ok(sx_vector![
        sx_vector![sx_symbol!("module-paths"), sx_vector_from_vec!(module_paths)],
        sx_vector![sx_symbol!("current-module"), Sx::Symbol(current_module)],
        sx_vector![sx_symbol!("loaded-modules"), sx_vector_from_vec!(loaded_modules)],
        sx_vector![sx_symbol!("definitions"), sx_vector_from_vec!(defs)]
    ]);
}

fn primitive_trace(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    let ts = time::now();

    let label_arg = &args[0];
    let label = match label_arg {
        Sx::String(s) => s,
        _ => return Err(Error::BuiltinBadArg(PRIMITIVE_TRACE.name, label_arg.clone()))
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
fn primitive_cons(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    let value = &args[0];
    let list_arg = &args[1];

    match list_arg {
        Sx::List(list) => {
            let mut new_list = list.as_slice().to_vec();
            new_list.insert(0, value.clone());
            return Ok(Sx::List(Arc::new(new_list)));
        },

        _ => {
            return Err(Error::BuiltinBadArg(PRIMITIVE_CONS.name, list_arg.clone()));
        }
    }
}

// TODO: vector
fn primitive_head(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    let list_arg = &args[0];
    match list_arg {
        Sx::List(list) => {
            match list.first() {
                Some(value) => {
                    return Ok(value.clone());
                },

                None => {
                    return Err(Error::BuiltinBadArg(PRIMITIVE_HEAD.name, list_arg.clone()));
                }
            }
        },

        _ => {
            return Err(Error::BuiltinBadArg(PRIMITIVE_HEAD.name, list_arg.clone()));
        }
    }
}

// TODO: vector
fn primitive_tail(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    let list_arg = &args[0];
    match list_arg {
        Sx::List(list) => {
            match list.split_first() {
                Some((_, tail_slice)) => {
                    let tail = tail_slice.to_vec();
                    return Ok(Sx::List(Arc::new(tail)));
                },

                None => {
                    return Err(Error::BuiltinBadArg(PRIMITIVE_TAIL.name, list_arg.clone()));
                }
            }
        },

        _ => {
            return Err(Error::BuiltinBadArg(PRIMITIVE_TAIL.name, list_arg.clone()));
        }
    }
}

fn primitive_eq(_ctx: &mut Ctx, args: &[Sx]) -> Result {
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

fn primitive_plus(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    let mut sum = 0i64;
    for arg in args.iter() {
        match arg {
            Sx::Integer(n) => {
                sum += n;
            },

            _ => {
                return Err(Error::BuiltinBadArg(PRIMITIVE_PLUS.name, arg.clone()));
            }
        }
    }

    // TODO: overflow
    return Ok(sx_integer!(sum));
}

fn primitive_minus(_ctx: &mut Ctx, args: &[Sx]) -> Result {
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
                        return Err(Error::BuiltinBadArg(PRIMITIVE_MINUS.name, arg.clone()));
                    }
                }
            }

            // TODO: overflow
            return Ok(sx_integer!(diff));
        },

        _ => {
            return Err(Error::BuiltinBadArg(PRIMITIVE_MINUS.name, diff_arg.clone()));
        }
    }
}

fn primitive_product(_ctx: &mut Ctx, args: &[Sx]) -> Result {
    let mut product = 1;
    for arg in args {
        match arg {
            Sx::Integer(n) => {
                product *= n;
            },

            _ => {
                return Err(Error::BuiltinBadArg(PRIMITIVE_PRODUCT.name, arg.clone()));
            }
        }
    }

    // TODO: overflow
    return Ok(sx_integer!(product));
}

// TODO: vectors
fn primitive_range(_ctx: &mut Ctx, args: &[Sx]) -> Result {
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
            return Err(Error::BuiltinBadArg(PRIMITIVE_RANGE.name, arg.clone()));
        },

        [_, ref arg] => {
            return Err(Error::BuiltinBadArg(PRIMITIVE_RANGE.name, arg.clone()));
        },

        _ => {
            assert!(false);
        }
    }

    return Ok(Sx::Vector(Arc::new(numbers)));
}
