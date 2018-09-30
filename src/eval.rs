use std::string::ToString;
use std::sync::Arc;

use ::env::Env;
use ::read::ReadError;
use ::module;
use ::sx::{*};

#[derive(Eq, PartialEq, Debug)]
pub enum EvalError {
    Undefined(SxSymbol),
    Redefine(SxSymbol),
    RedefineCore(SxSymbol),
    DefineBadSymbol(Sx),
    SymbolBadModuleFormat(SxSymbol),

    NotAFunction(Sx),
    InvalidBinding(Sx),
    DuplicateBinding(SxSymbol),
    // TODO: top-level shadow error

    // TODO: BadArg expected info
    BuiltinBadArg(&'static str, Sx),
    BuiltinTooFewArgs(&'static str, usize, usize),
    BuiltinTooManyArgs(&'static str, usize, usize),

    FnTooFewArgs(SxFunction, usize, usize),
    FnTooManyArgs(SxFunction, usize, usize),

    ModuleSelfRefer(SxSymbol),
    ModulePathError(String, String),
    ModuleNotFound(SxSymbol, Vec<String>),
    ModuleMultipleOptions(SxSymbol, Vec<String>),
    ModuleIoOpenError(SxSymbol, String),
    ModuleIoReadError(SxSymbol, String),
    ModuleReadErrors(SxSymbol, Vec<ReadError>),
    ModuleEvalErrors(SxSymbol, Vec<EvalError>),
    ModuleNotLoaded(SxSymbol)
}

pub type EvalResult = Result<Sx, EvalError>;

pub fn eval(env: &mut Env, sx: &Sx) -> EvalResult {
    match sx {
        Sx::Nil         |
        Sx::Boolean(_)  |
        Sx::Integer(_)  |
        Sx::String(_)   |
        Sx::Builtin(_)  |
        Sx::Function(_) => {
            return Ok(sx.clone());
        },

        Sx::Quote(value) => {
            return Ok(value.as_ref().clone());
        },

        Sx::Symbol(ref symbol) => {
            let mut effective_module = env.core_module.clone();
            let mut effective_symbol = symbol.clone();

            match module::entry_from_symbol(symbol)[..] {
                [ref other_module, ref sub_symbol] => {
                    effective_module = other_module.clone();
                    effective_symbol = sub_symbol.clone();
                },

                [_] => (),

                _ => return Err(EvalError::SymbolBadModuleFormat(symbol.clone()))
            }

            if !env.loaded_modules.contains(&effective_module) {
                return Err(EvalError::ModuleNotLoaded(effective_module.clone()));
            }

            match env.lookup(&effective_module, &effective_symbol) {
                Some(value) => return Ok(value.clone()),
                None        => ()
            }

            match env.lookup_current(symbol) {
                Some(value) => return Ok(value.clone()),
                None        => return Err(EvalError::Undefined(symbol.clone()))
            }
        },

        Sx::List(l) => {
            match l.split_first() {
                None => {
                    return Ok(sx.clone());
                },

                Some((head, args)) => {
                    match eval(env, head) {
                        Ok(Sx::Builtin(builtin)) => {
                            return apply_builtin(builtin, env, args);
                        },

                        Ok(Sx::Function(ref f)) => {
                            return apply_function(f, env, args);
                        },

                        Ok(v) => {
                            return Err(EvalError::NotAFunction(v.clone()));
                        },

                        error @ Err(_) => {
                            return error;
                        }
                    }
                }
            }
        },

        Sx::Vector(v) => {
            let mut w = v.as_ref().clone();
            for sx in w.iter_mut() {
                match eval(env, sx) {
                    Ok(result) => *sx = result,
                    error @ Err(_) => return error
                }
            }

            return Ok(Sx::Vector(Arc::new(w)));
        }
    }
}

pub fn apply_builtin(builtin: &SxBuiltinInfo, env: &mut Env, arglist: &[Sx]) -> EvalResult {
    match builtin.callback {
        SxBuiltinCallback::Special(special_fn) => {
            return apply_special(builtin, special_fn, env, arglist);
        },

        SxBuiltinCallback::Primitive(primitive_fn) => {
            return apply_primitive(builtin, primitive_fn, env, arglist);
        }
    }
}

fn apply_special(builtin: &SxBuiltinInfo, special_fn: SxBuiltinFn, env: &mut Env, args: &[Sx]) -> EvalResult {
    if args.len() < builtin.min_arity {
        return Err(EvalError::BuiltinTooFewArgs(builtin.name, builtin.min_arity, args.len()));
    }

    match builtin.max_arity {
        Some(arity) if arity < args.len() => {
            return Err(EvalError::BuiltinTooManyArgs(builtin.name, arity, args.len()));
        },

        Some(_) | None => ()
    }

    return special_fn(env, args);
}

fn apply_primitive(builtin: &SxBuiltinInfo, primitive_fn: SxBuiltinFn, env: &mut Env, args: &[Sx]) -> EvalResult {
    if args.len() < builtin.min_arity {
        return Err(EvalError::BuiltinTooFewArgs(builtin.name, builtin.min_arity, args.len()));
    }

    match builtin.max_arity {
        Some(arity) if arity < args.len() => {
            return Err(EvalError::BuiltinTooManyArgs(builtin.name, arity, args.len()));
        },

        Some(_) | None => ()
    }

    let mut result_args = args.to_vec();
    for arg in result_args.iter_mut() {
        match eval(env, arg) {
            Ok(result) => *arg = result,
            error @ Err(_) => return error
        }
    }

    return primitive_fn(env, &result_args);
}

pub fn apply_function(f: &SxFunction, env: &mut Env, args: &[Sx]) -> EvalResult {
    let arity = args.len();
    if arity < f.arity {
        return Err(EvalError::FnTooFewArgs(f.clone(), f.arity, arity));
    }

    if f.arity < arity {
        return Err(EvalError::FnTooManyArgs(f.clone(), f.arity, arity));
    }

    let mut sub_env = env.clone();
    sub_env.current_module = f.module.clone();
    for (binding, sx) in f.bindings.iter().zip(args.iter()) {
        match eval(env, sx) {
            Ok(ref result) => sub_env.define_current(binding, result),
            error @ Err(_) => return error
        }
    }

    let exprs = f.body.clone();
    let mut result = sx_nil!();
    for expr in exprs.iter() {
        match eval(&mut sub_env, expr) {
            Ok(sub_result) => result = sub_result,
            error @ Err(_) => return error
        }
    }

    return Ok(result);
}

impl ToString for EvalError {
    fn to_string(&self) -> String {
        match self {
            EvalError::Undefined(sx) => {
                return format!("undefined symbol: {}", sx.to_string());
            }

            EvalError::Redefine(symbol) => {
                return format!("cannot redefine symbol {}", symbol.to_string());
            }

            EvalError::RedefineCore(symbol) => {
                return format!("cannot redefine core symbol {}", symbol.to_string());
            }

            EvalError::DefineBadSymbol(sx) => {
                return format!("first argument to def must be a symbol, got {}", sx.to_string());
            }

            EvalError::SymbolBadModuleFormat(symbol) => {
                return format!("badly formatted symbol {}, expected something like my-module/my-val", symbol.clone());
            }

            EvalError::NotAFunction(sx) => {
                return format!("{} does not evaluate to a function", sx.to_string());
            }

            EvalError::InvalidBinding(sx) => {
                return format!("invalid binding form in function, got {}", sx.to_string());
            }

            EvalError::DuplicateBinding(symbol) => {
                return format!("cannot bind symbol {} more than once in function definition", symbol);
            }

            EvalError::BuiltinTooFewArgs(name, min_arity, act_arity) => {
                return format!("{} expects at least {} argument(s), got {}", name, min_arity, act_arity);
            }

            EvalError::BuiltinTooManyArgs(name, max_arity, act_arity) => {
                return format!("{} expects at most {} argument(s), got {}", name, max_arity, act_arity);
            }

            EvalError::BuiltinBadArg(name, arg) => {
                return format!("invalid argument to {}, got {}", name, arg.to_string());
            }

            EvalError::FnTooFewArgs(f, min_arity, act_arity) => {
                return format!("{} expects at least {} argument(s), got {}", f.to_string(), min_arity, act_arity);
            }

            EvalError::FnTooManyArgs(f, max_arity, act_arity) => {
                return format!("{} expects at most {} argument(s), got {}", f.to_string(), max_arity, act_arity);
            }

            EvalError::ModuleSelfRefer(module_name) => {
                return format!("cannot use self in module {}", module_name);
            }

            EvalError::ModulePathError(module_path, module_name) => {
                return format!("failed to compose filename from module path {} and name {}", module_path, module_name);
            }

            EvalError::ModuleNotFound(module_name, module_paths) => {
                let module_paths_str = module_paths.join(", ");
                return format!("could not find module named {} under following module paths: {}", module_name, module_paths_str);
            }

            EvalError::ModuleMultipleOptions(module_name, filename_matches) => {
                let filename_matches_str = filename_matches.join(", ");
                return format!("could not load module {} due to multiple options: {}", module_name, filename_matches_str);
            }

            EvalError::ModuleIoOpenError(module_name, io_error) => {
                return format!("error while opening file for module {}: {}", module_name, io_error);
            }

            EvalError::ModuleIoReadError(module_name, io_error) => {
                return format!("error while reading file for module {}: {}", module_name, io_error);
            }

            EvalError::ModuleReadErrors(_module_name, read_errors) => {
                let mut s = read_errors
                    .iter()
                    .fold(String::new(), |acc, e| acc + &e.to_string() + "\n");
                s.pop();

                return s;
            }

            EvalError::ModuleEvalErrors(_module_name, eval_errors) => {
                let mut s = eval_errors
                    .iter()
                    .fold(String::new(), |acc, e| acc + &e.to_string() + "\n");
                s.pop();

                return s;
            }

            EvalError::ModuleNotLoaded(module_name) => {
                return format!("module {} is not loaded", module_name);
            }
        }
    }
}

// TODO: relocate primitive and special tests
#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use ::read::read;

    fn mk_test_env() -> Env {
        let module_paths = vec![
            "./resources/test/eval".to_string()
        ];

        let current_module = sx_symbol_unwrapped!("test-eval");

        return Env::new(&module_paths, &current_module);
    }

    fn test_eval(input_source: &str, output_source: &str) {
        let mut env = mk_test_env();

        let input = read(input_source).expect("invalid input source");
        let output = read(output_source).expect("invalid output source");

        let mut results = Vec::new();
        for sx in input.iter() {
            results.push(eval(&mut env, &sx).expect("eval error"));
        }

        assert_eq!(sx_list_from_vec!(results).to_string(), sx_list_from_vec!(output).to_string());
    }

    fn test_eval_results(input_source: &str, exp_results: Vec<EvalResult>) {
        let mut env = mk_test_env();

        let input = read(input_source).expect("invalid input source");

        let mut results = Vec::new();
        for sx in input.iter() {
            results.push(eval(&mut env, &sx));
        }

        assert_eq!(results, exp_results);
    }

    #[test]
    fn test_self_eval() {
        test_eval("nil", "nil");
        test_eval("true", "true");
        test_eval("false", "false");
        test_eval("0", "0");
        test_eval("-999", "-999");
        test_eval("\"Yellow submarine\"", "\"Yellow submarine\"");
        test_eval("\"北京市\"", "\"北京市\"");
        test_eval("()", "()");
    }

    #[test]
    fn test_single_quoted() {
        test_eval("'nil", "nil");
        test_eval("'true", "true");
        test_eval("'false", "false");
        test_eval("'0", "0");
        test_eval("'-999", "-999");
        test_eval("'\"Yellow submarine\"", "\"Yellow submarine\"");
        test_eval("'\"北京市\"", "\"北京市\"");
        test_eval("'()", "()");
        test_eval("'(1 2 3)", "(1 2 3)");
        test_eval("'[1 2 3]", "[1 2 3]");
        test_eval("'foo", "foo");
    }

    #[test]
    fn test_double_quoted() {
        test_eval("''nil", "'nil");
        test_eval("''true", "'true");
        test_eval("''false", "'false");
        test_eval("''0", "'0");
        test_eval("''-999", "'-999");
        test_eval("''\"Yellow submarine\"", "'\"Yellow submarine\"");
        test_eval("''\"北京市\"", "'\"北京市\"");
        test_eval("''()", "'()");
        test_eval("''(1 2 3)", "'(1 2 3)");
        test_eval("''[1 2 3]", "'[1 2 3]");
        test_eval("''foo", "'foo");
    }

    #[test]
    fn test_special_def() {
        test_eval(r#"
            (def foo 1)
            foo
        "#, r#"
            foo
            1
        "#);
    }

    #[test]
    fn test_special_error_def() {
        test_eval_results(r#"
            (def)
            (def foo)
            (def foo 1 2)
            (def "foo" 1)
            (def foo 1)
            (def foo 2)
        "#, vec![
            Err(EvalError::BuiltinTooFewArgs("def", 2, 0)),
            Err(EvalError::BuiltinTooFewArgs("def", 2, 1)),
            Err(EvalError::BuiltinTooManyArgs("def", 2, 3)),
            Err(EvalError::DefineBadSymbol(sx_string!("foo"))),
            Ok(sx_symbol!("foo")),
            Err(EvalError::Redefine(sx_symbol_unwrapped!("foo")))
        ]);
    }

    #[test]
    fn test_special_fn_invoke() {
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
    fn test_special_fn_invalid_binding() {
        test_eval_results(r#"
            (fn (x 1) nil)
        "#, vec![
            Err(EvalError::InvalidBinding(sx_integer!(1)))
        ]);
    }

    #[test]
    fn test_special_fn_duplicate_binding() {
        test_eval_results(r#"
            (fn (x x) (+ x x))
        "#, vec![
            Err(EvalError::DuplicateBinding(sx_symbol_unwrapped!("x")))
        ]);
    }

    #[test]
    fn test_special_fn_too_few_args() {
        let f1 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      1,
            bindings:   vec![sx_symbol_unwrapped!("x")],
            body:       Arc::new(vec![sx_symbol!("x")])
        });

        let f2 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      2,
            bindings:   vec![sx_symbol_unwrapped!("x"), sx_symbol_unwrapped!("y")],
            body:       Arc::new(vec![sx_symbol!("x")])
        });

        test_eval_results(r#"
            ((fn (x) x))
            ((fn (x y) x) 1)
        "#, vec![
            Err(EvalError::FnTooFewArgs(f1.clone(), 1, 0)),
            Err(EvalError::FnTooFewArgs(f2.clone(), 2, 1))
        ]);
    }

    #[test]
    fn test_special_fn_too_many_args() {
        let f1 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      0,
            bindings:   vec![],
            body:       Arc::new(vec![sx_nil!()])
        });

        let f2 = Arc::new(SxFunctionInfo {
            module:     sx_symbol_unwrapped!("test-eval"),
            arity:      1,
            bindings:   vec![sx_symbol_unwrapped!("x")],
            body:       Arc::new(vec![sx_symbol!("x")])
        });

        test_eval_results(r#"
            ((fn () nil) 1)
            ((fn (x) x) 1 2)
        "#, vec![
            Err(EvalError::FnTooManyArgs(f1.clone(), 0, 1)),
            Err(EvalError::FnTooManyArgs(f2.clone(), 1, 2))
        ]);
    }

    #[test]
    fn test_special_if_direct() {
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
    fn test_special_if_short_circuit() {
        test_eval(r#"
            (if true "happy" undefined-symbol)
            (if false undefined-symbol "sad")
        "#, r#"
            "happy"
            "sad"
        "#);
    }

    #[test]
    fn test_special_error_if() {
        test_eval_results(r#"
            (if foo "happy" "sad")
            (if true foo "sad")
            (if false "happy" foo)
        "#, vec![
            Err(EvalError::Undefined(sx_symbol_unwrapped!("foo"))),
            Err(EvalError::Undefined(sx_symbol_unwrapped!("foo"))),
            Err(EvalError::Undefined(sx_symbol_unwrapped!("foo")))
        ])
    }

    #[test]
    fn test_special_if_indirect() {
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
    fn test_special_quote() {
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
    fn test_primitive_apply() {
        test_eval(r#"
            (apply + '())
            (apply + '(1 2 3))
        "#, r#"
            0
            6
        "#);
    }

    #[test]
    fn test_primitive_error_apply() {
        test_eval_results(r#"
            (apply + true)
            (apply 1 '(1 2 3))
            (apply foo '(1 2 3))
        "#, vec![
            Err(EvalError::BuiltinBadArg("apply", sx_boolean!(true))),
            Err(EvalError::NotAFunction(sx_integer!(1))),
            Err(EvalError::Undefined(sx_symbol_unwrapped!("foo")))
        ]);
    }

    #[test]
    fn test_primitive_cons() {
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
    fn test_primitive_head() {
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
    fn test_primitive_tail() {
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
    fn test_primitive_plus() {
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
    fn test_primitive_error_plus() {
        test_eval_results(r#"
            (+ 1 nil)
        "#, vec![
            Err(EvalError::BuiltinBadArg("+", sx_nil!()))
        ]);
    }

    #[test]
    fn test_primitive_product() {
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

    #[test]
    fn test_primitive_error_too_few_args() {
        test_eval_results(r#"
            (apply)
            (apply +)
        "#, vec![
            Err(EvalError::BuiltinTooFewArgs("apply", 2, 0)),
            Err(EvalError::BuiltinTooFewArgs("apply", 2, 1))
        ]);
    }

    #[test]
    fn test_primitive_error_too_many_args() {
        test_eval_results(r#"
            (apply + '(1 2) 1)
        "#, vec![
            Err(EvalError::BuiltinTooManyArgs("apply", 2, 3))
        ]);
    }

    #[test]
    fn test_error_not_a_function() {
        test_eval_results(r#"
            (1 2 3)
        "#, vec![
            Err(EvalError::NotAFunction(sx_integer!(1)))
        ]);
    }

    #[test]
    fn test_error_sub_error() {
        test_eval_results(r#"
            ((1 2 3) 2 3)
        "#, vec![
            Err(EvalError::NotAFunction(sx_integer!(1)))
        ]);
    }

    #[test]
    fn test_vector_eval() {
        test_eval(r#"
            (def foo 2)
            [1 foo 3 (+ 2 2)]
        "#, r#"
            foo
            [1 2 3 4]
        "#)
    }
}
