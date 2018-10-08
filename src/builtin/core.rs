use std::sync::Arc;
use im::Vector;
use time;

use ::eval::{Context, Visibility, Result, Error};
use ::module;
use ::sx::{*};
use ::util::pretty::pretty;

pub static MODULE_NAME: &'static str = "core";

pub static TABLE: &'static [SxBuiltinInfo] = &[
    // Specials
    special!("def", 2, special_def),
    special_no_arg_limit!("fn", 2, special_fn),
    special!("if", 3, special_if),
    special!("module", 1, special_module),
    special!("quote", 1, special_quote),

    // Modules
    special!("import", 1, special_import),
    //special!("export", 1, special_export),

    // General
    primitive!("apply", 2, primitive_apply),
    primitive!("context", 0, primitive_context),
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

fn special_def(ctx: &mut Context, args: &[Sx]) -> Result {
    let binding = &args[0];
    match binding {
        Sx::Symbol(ref symbol) => {
            match ctx.lookup_current(symbol) {
                None => {
                    let value = &args[1];
                    match ctx.eval(value) {
                        Ok(result) => {
                            ctx.define_current(symbol, &result, Visibility::Private);
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
fn special_fn(ctx: &mut Context, args: &[Sx]) -> Result {
    let binding_list = &args[0];
    match binding_list {
        Sx::List(bindings) => {
            let mut bindings_vec = Vector::new();
            for binding in bindings.iter() {
                match binding {
                    Sx::Symbol(name) => {
                        if bindings_vec.contains(name) {
                            return Err(Error::DuplicateBinding(name.clone()));
                        }

                        bindings_vec.push_back(name.clone());
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
            return Err(Error::BuiltinBadArg("fn", binding_list.clone()));
        }
    }
}

fn special_if(ctx: &mut Context, args: &[Sx]) -> Result {
    let cond = &args[0];
    let true_path = &args[1];
    let false_path = &args[2];

    match ctx.eval(cond) {
        Ok(Sx::Nil) | Ok(Sx::Boolean(false)) => {
            return ctx.eval(false_path);
        },

        Ok(_) => {
            return ctx.eval(true_path);
        },

        error @ Err(_) => {
            return error;
        }
    }
}

fn special_module(ctx: &mut Context, args: &[Sx]) -> Result {
    let module_name_arg = &args[0];
    let module_name = match module_name_arg {
        Sx::Symbol(module_name) => module_name,
        _ => return Err(Error::BuiltinBadArg("module", module_name_arg.clone()))
    };

    ctx.loaded_modules.insert(module_name.clone());
    ctx.current_module = module_name.clone();
    ctx.load_core();

    return Ok(Sx::Symbol(module_name.clone()));
}

fn special_quote(_ctx: &mut Context, args: &[Sx]) -> Result {
    return Ok(args[0].clone());
}

fn special_import(ctx: &mut Context, args: &[Sx]) -> Result {
    let module_arg = &args[0];
    match module_arg {
        Sx::Symbol(ref module_name) => {
            return module::load_import(ctx, module_name);
        },

        _ => {
            return Err(Error::BuiltinBadArg("import", module_arg.clone()));
        }
    }
}

// TODO: generic iterators
fn primitive_apply(ctx: &mut Context, args: &[Sx]) -> Result {
    let head = &args[0];
    let value = &args[1];
    match (ctx.eval(head), value) {
        (Ok(Sx::Builtin(builtin)), Sx::List(sub_args)) => {
            return ctx.apply_builtin(builtin, sub_args);
        },

        (Ok(Sx::Function(ref f)), Sx::List(sub_args)) => {
            return ctx.apply_function(f, sub_args);
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
fn primitive_context(ctx: &mut Context, _args: &[Sx]) -> Result {
    let mut module_paths = Vec::new();
    for module_path in ctx.module_paths.iter() {
        module_paths.push(sx_string!(module_path));
    }

    let current_module = ctx.current_module.clone();

    let mut loaded_modules = Vec::new();
    for module in ctx.loaded_modules.iter() {
        loaded_modules.push(Sx::Symbol(module.clone()));
    }

    let mut ctx_list = Vec::new();
    for ((module, symbol), (value, visibility)) in ctx.definitions.iter() {
        ctx_list.push(sx_vector![
            Sx::Symbol(module.clone()),
            Sx::Symbol(symbol.clone()),
            value.clone(),
            sx_string!(visibility.to_string())
        ]);
    }

    return Ok(sx_vector![
        sx_vector![sx_symbol!("module-paths"), sx_vector_from_vec!(module_paths)],
        sx_vector![sx_symbol!("current-module"), Sx::Symbol(current_module)],
        sx_vector![sx_symbol!("loaded-modules"), sx_vector_from_vec!(loaded_modules)],
        sx_vector![sx_symbol!("definitions"), sx_vector_from_vec!(ctx_list)]
    ]);
}

fn primitive_trace(_ctx: &mut Context, args: &[Sx]) -> Result {
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
fn primitive_cons(_ctx: &mut Context, args: &[Sx]) -> Result {
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
fn primitive_head(_ctx: &mut Context, args: &[Sx]) -> Result {
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
fn primitive_tail(_ctx: &mut Context, args: &[Sx]) -> Result {
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
fn primitive_range(_ctx: &mut Context, args: &[Sx]) -> Result {
    let mut numbers = Vector::new();

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

fn primitive_eq(_ctx: &mut Context, args: &[Sx]) -> Result {
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

fn primitive_plus(_ctx: &mut Context, args: &[Sx]) -> Result {
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

fn primitive_minus(_ctx: &mut Context, args: &[Sx]) -> Result {
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

fn primitive_product(_ctx: &mut Context, args: &[Sx]) -> Result {
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

// TODO: specs to avoid writing repetitive type check tests
// TODO: proptests
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ::eval::Error;
    use ::eval::tests::{test_eval, test_eval_results};
    use ::sx::{Sx, SxFunctionInfo};

    #[test]
    fn test_def() {
        test_eval(r#"
            (def foo 1)
            foo
        "#, r#"
            foo
            1
        "#);
    }

    #[test]
    fn test_error_def() {
        test_eval_results(r#"
            (def)
            (def foo)
            (def foo 1 2)
            (def "foo" 1)
            (def foo 1)
            (def foo 2)
        "#, vec![
            Err(Error::BuiltinTooFewArgs("def", 2, 0)),
            Err(Error::BuiltinTooFewArgs("def", 2, 1)),
            Err(Error::BuiltinTooManyArgs("def", 2, 3)),
            Err(Error::DefineBadSymbol(sx_string!("foo"))),
            Ok(sx_symbol!("foo")),
            Err(Error::Redefine(sx_symbol_unwrapped!("foo")))
        ]);
    }

    #[test]
    fn test_fn_invoke() {
        test_eval(r#"
            ((fn () nil))
            ((fn (x) (* x x)) 3)
            ((fn (pred x y) (if pred x y)) true "happy" "sad")
        "#, r#"
            nil
            9
            "happy"
        "#);
    }

    #[test]
    fn test_fn_invalid_binding() {
        test_eval_results(r#"
            (fn (x 1) nil)
        "#, vec![
            Err(Error::InvalidBinding(sx_integer!(1)))
        ]);
    }

    #[test]
    fn test_fn_duplicate_binding() {
        test_eval_results(r#"
            (fn (x x) (+ x x))
        "#, vec![
            Err(Error::DuplicateBinding(sx_symbol_unwrapped!("x")))
        ]);
    }

    #[test]
    fn test_fn_too_few_args() {
        let f1 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      1,
            bindings:   vector![sx_symbol_unwrapped!("x")],
            body:       Arc::new(vec![sx_symbol!("x")])
        });

        let f2 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      2,
            bindings:   vector![sx_symbol_unwrapped!("x"), sx_symbol_unwrapped!("y")],
            body:       Arc::new(vec![sx_symbol!("x")])
        });

        test_eval_results(r#"
            ((fn (x) x))
            ((fn (x y) x) 1)
        "#, vec![
            Err(Error::FnTooFewArgs(f1.clone(), 1, 0)),
            Err(Error::FnTooFewArgs(f2.clone(), 2, 1))
        ]);
    }

    #[test]
    fn test_fn_too_many_args() {
        let f1 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      0,
            bindings:   vector![],
            body:       Arc::new(vec![sx_nil!()])
        });

        let f2 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      1,
            bindings:   vector![sx_symbol_unwrapped!("x")],
            body:       Arc::new(vec![sx_symbol!("x")])
        });

        test_eval_results(r#"
            ((fn () nil) 1)
            ((fn (x) x) 1 2)
        "#, vec![
            Err(Error::FnTooManyArgs(f1.clone(), 0, 1)),
            Err(Error::FnTooManyArgs(f2.clone(), 1, 2))
        ]);
    }

    #[test]
    fn test_if_direct() {
        test_eval(r#"
            (if true "happy" "sad")
            (if "yay!" "happy" "sad")
            (if false "happy" "sad")
            (if nil "happy" "sad")
        "#, r#"
            "happy"
            "happy"
            "sad"
            "sad"
        "#);
    }

    #[test]
    fn test_if_short_circuit() {
        test_eval(r#"
            (if true "happy" undefined-symbol)
            (if false undefined-symbol "sad")
        "#, r#"
            "happy"
            "sad"
        "#);
    }

    #[test]
    fn test_error_if() {
        test_eval_results(r#"
            (if foo "happy" "sad")
            (if true foo "sad")
            (if false "happy" foo)
        "#, vec![
            Err(Error::Undefined(sx_symbol_unwrapped!("foo"))),
            Err(Error::Undefined(sx_symbol_unwrapped!("foo"))),
            Err(Error::Undefined(sx_symbol_unwrapped!("foo")))
        ])
    }

    #[test]
    fn test_if_indirect() {
        test_eval(r#"
            (def is-happy? true)
            (def is-not-happy? false)
            (if is-happy? "happy" "sad")
            (if is-not-happy? "happy" "sad")
        "#, r#"
            is-happy?
            is-not-happy?
            "happy"
            "sad"
        "#);
    }

    #[test]
    fn test_quote() {
        test_eval(r#"
            (quote 1)
            (quote foo)
            (quote (1 2 3))
        "#, r#"
            1
            foo
            (1 2 3)
        "#);
    }

    #[test]
    fn test_apply() {
        test_eval(r#"
            (apply + '())
            (apply + '(1 2 3))
        "#, r#"
            0
            6
        "#);
    }

    #[test]
    fn test_cons() {
        test_eval(r#"
            (cons 1 ())
            (cons 1 '(2))
            (cons 1 (cons 2 (cons 3 ())))
        "#, r#"
            (1)
            (1 2)
            (1 2 3)
        "#);
    }

    #[test]
    fn test_head() {
        test_eval(r#"
            (head '(1))
            (head '(2 1))
            (head '(3 2 1))
        "#, r#"
            1
            2
            3
        "#);
    }

    #[test]
    fn test_tail() {
        test_eval(r#"
            (tail '(1))
            (tail '(1 2 3))
            (tail (tail '(1 2 3)))
        "#, r#"
            ()
            (2 3)
            (3)
        "#);
    }

    #[test]
    fn test_plus() {
        test_eval(r#"
            (+)
            (+ 1)
            (+ 0 0)
            (+ -1 1)
            (+ 1 1)
            (+ 999 1)
            (+ (+ 1 1) (+ 1 1))
            (+ 1 2 3)
            (+ 1 2 (+ 1 2) 4)
        "#, r#"
            0
            1
            0
            0
            2
            1000
            4
            6
            10
        "#);
    }

    #[test]
    fn test_product() {
        test_eval(r#"
            (*)
            (* 1)
            (* 1 2)
            (* 1 2 3)
        "#, r#"
            1
            1
            2
            6
        "#);
    }
}