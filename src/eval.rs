use ::rpds::List;

use ::env::Env;
use ::sx::{*};

#[derive(Eq, PartialEq, Debug)]
pub enum EvalError {
    Undefined(SxSymbol),
    Redefine(SxSymbol),
    DefineBadSymbol(Sx),

    NotAFunction(Sx),
    InvalidBinding(Sx),
    DuplicateBinding(SxSymbol),

    BuiltinBadArg(&'static str, Sx),
    BuiltinTooFewArgs(&'static str, usize, usize),
    BuiltinTooManyArgs(&'static str, usize, usize),

    TooFewArgs(SxFunction, usize, usize),
    TooManyArgs(SxFunction, usize, usize),

    Unknown(Sx)
}

pub type EvalResult = Result<Sx, EvalError>;

pub fn eval(env: &mut Env, sx: &Sx) -> EvalResult {
    match sx {
        Sx::Nil | Sx::Boolean(_) | Sx::Integer(_) | Sx::String(_) | Sx::Builtin(_) | Sx::Function(_) => {
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
                (None, _) => {
                    return Ok(sx.clone());
                },

                (Some(ref head), Some(ref args)) => {
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
                },

                _ => {
                    return Err(EvalError::Unknown(sx.clone()));
                }
            }
        }
    }
}

pub fn apply_builtin(builtin: &SxBuiltinInfo, env: &mut Env, arglist: &List<Sx>) -> EvalResult {
    match builtin.callback {
        SxBuiltinCallback::Special(special_fn) => {
            return apply_special(builtin, special_fn, env, arglist);
        },

        SxBuiltinCallback::Primitive(primitive_fn) => {
            return apply_primitive(builtin, primitive_fn, env, arglist);
        }
    }
}

fn apply_special(builtin: &SxBuiltinInfo,
                 special_fn: SxSpecialFn,
                 env: &mut Env,
                 arglist: &List<Sx>) -> EvalResult {
    let mut args = Vec::new();
    for arg in arglist.iter() {
        args.push(arg);
    }

    if args.len() < builtin.min_arity {
        return Err(EvalError::BuiltinTooFewArgs(builtin.name, builtin.min_arity, args.len()));
    }

    match builtin.max_arity {
        Some(arity) if arity < args.len() => {
            return Err(EvalError::BuiltinTooManyArgs(builtin.name, arity, args.len()));
        },

        Some(_) | None => ()
    }

    return special_fn(env, &args);
}

fn apply_primitive(builtin: &SxBuiltinInfo,
                   primitive_fn: SxPrimitiveFn,
                   env: &mut Env,
                   arglist: &List<Sx>) -> EvalResult {
    let mut args = Vec::new();
    for arg in arglist.iter() {
        match eval(env, arg) {
            Ok(result) => args.push(result),
            error @ Err(_) => return error
        }
    }

    if args.len() < builtin.min_arity {
        return Err(EvalError::BuiltinTooFewArgs(builtin.name, builtin.min_arity, args.len()));
    }

    match builtin.max_arity {
        Some(arity) if arity < args.len() => {
            return Err(EvalError::BuiltinTooManyArgs(builtin.name, arity, args.len()));
        },

        Some(_) | None => ()
    }

    return primitive_fn(env, &args);
}

pub fn apply_function(f: &SxFunction, env: &mut Env, arglist: &List<Sx>) -> EvalResult {
    let arity = arglist.len();
    if arity < f.arity {
        return Err(EvalError::TooFewArgs(f.clone(), f.arity, arity));
    }

    if f.arity < arity {
        return Err(EvalError::TooManyArgs(f.clone(), f.arity, arity));
    }

    let mut sub_env = env.clone();
    for (binding, sx) in f.bindings.iter().zip(arglist.iter()) {
        match eval(env, sx) {
            Ok(ref result) => sub_env.define(binding, result),
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use ::read::read;

    fn test_eval(input_source: &str, output_source: &str) {
        let mut env = Env::new();

        let input = read(input_source).expect("invalid input source");
        let output = read(output_source).expect("invalid output source");

        match input {
            Sx::List(sxs) => {
                let mut results = List::new();
                for sx in sxs.iter() {
                    results.push_front_mut(eval(&mut env, &sx).expect("eval error"));
                }
                results.reverse_mut();

                assert_eq!(Sx::List(Arc::new(results)).to_string(), output.to_string());
            },

            _ => {
                assert!(false);
            }
        }
    }

    fn test_eval_results(input_source: &str, exp_results: Vec<EvalResult>) {
        let mut env = Env::new();

        let input = read(input_source).expect("invalid input source");
        match input {
            Sx::List(sxs) => {
                let mut results = Vec::new();
                for sx in sxs.iter() {
                    results.push(eval(&mut env, &sx));
                }

                assert_eq!(results, exp_results);
            },

            _ => {
                assert!(false);
            }
        }
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
            arity:      1,
            bindings:   vec![sx_symbol_unwrapped!("x")],
            body:       Arc::new(list![sx_symbol!("x")])
        });

        let f2 = Arc::new(SxFunctionInfo {
            arity:      2,
            bindings:   vec![sx_symbol_unwrapped!("x"), sx_symbol_unwrapped!("y")],
            body:       Arc::new(list![sx_symbol!("x")])
        });

        test_eval_results(r#"
            ((fn (x) x))
            ((fn (x y) x) 1)
        "#, vec![
            Err(EvalError::TooFewArgs(f1.clone(), 1, 0)),
            Err(EvalError::TooFewArgs(f2.clone(), 2, 1))
        ]);
    }

    #[test]
    fn test_special_fn_too_many_args() {
        let f1 = Arc::new(SxFunctionInfo {
            arity:      0,
            bindings:   vec![],
            body:       Arc::new(list![sx_nil!()])
        });

        let f2 = Arc::new(SxFunctionInfo {
            arity:      1,
            bindings:   vec![sx_symbol_unwrapped!("x")],
            body:       Arc::new(list![sx_symbol!("x")])
        });

        test_eval_results(r#"
            ((fn () nil) 1)
            ((fn (x) x) 1 2)
        "#, vec![
            Err(EvalError::TooManyArgs(f1.clone(), 0, 1)),
            Err(EvalError::TooManyArgs(f2.clone(), 1, 2))
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
}
