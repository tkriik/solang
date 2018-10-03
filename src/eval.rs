use std::result;
use std::sync::Arc;

use im::{HashMap, HashSet, Vector};

use ::builtin::core::{BUILTIN_MODULE_NAME, BUILTIN_TABLE};
use ::module;
use ::read;
use ::sx::{*};

#[derive(Clone)]
pub struct Context {
    pub module_paths:   Vector<String>,
    pub current_module: SxSymbol,
    pub loaded_modules: HashSet<SxSymbol>,
    pub definitions:    HashMap<(SxSymbol, SxSymbol), Sx>,

    pub core_module:    SxSymbol
}

impl Context {
    pub fn new(module_paths: &Vec<String>, current_module: &SxSymbol) -> Context {
        let core_module = sx_symbol_unwrapped!(BUILTIN_MODULE_NAME);

        let mut ctx = Context {
            module_paths:   Vector::from(module_paths),
            current_module: current_module.clone(),
            loaded_modules: hashset!(core_module.clone(), current_module.clone()),
            definitions:    hashmap!(),

            core_module:    core_module.clone()
        };

        for builtin in BUILTIN_TABLE.iter() {
            let symbol = sx_symbol_unwrapped!(builtin.name);
            let value = Sx::Builtin(builtin);
            ctx.define(&core_module, &symbol, &value);
        }

        return ctx;
    }

    pub fn define(&mut self, module: &SxSymbol, symbol: &SxSymbol, value: &Sx) {
        self.definitions.insert((module.clone(), symbol.clone()), value.clone());
    }

    pub fn define_current(&mut self, symbol: &SxSymbol, value: &Sx) {
        let module = self.current_module.clone();
        self.define(&module, symbol, value);
    }

    pub fn lookup(&self, module: &SxSymbol, symbol: &SxSymbol) -> Option<&Sx> {
        return self.definitions.get(&(module.clone(), symbol.clone()));
    }

    pub fn lookup_core(&self, symbol: &SxSymbol) -> Option<&Sx> {
        let module = self.core_module.clone();
        return self.lookup(&module, symbol);
    }

    pub fn lookup_current(&self, symbol: &SxSymbol) -> Option<&Sx> {
        return self.lookup(&self.current_module, symbol);
    }

    pub fn eval(self: &mut Context, sx: &Sx) -> Result {
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
                let mut effective_module = self.core_module.clone();
                let mut effective_symbol = symbol.clone();

                match module::entry_from_symbol(symbol)[..] {
                    [ref other_module, ref sub_symbol] => {
                        effective_module = other_module.clone();
                        effective_symbol = sub_symbol.clone();
                    },

                    [_] => (),

                    _ => return Err(Error::SymbolBadModuleFormat(symbol.clone()))
                }

                if !self.loaded_modules.contains(&effective_module) {
                    return Err(Error::ModuleNotLoaded(effective_module.clone()));
                }

                match self.lookup(&effective_module, &effective_symbol) {
                    Some(value) => return Ok(value.clone()),
                    None        => ()
                }

                match self.lookup_current(symbol) {
                    Some(value) => return Ok(value.clone()),
                    None        => return Err(Error::Undefined(symbol.clone()))
                }
            },

            Sx::List(l) => {
                match l.split_first() {
                    None => {
                        return Ok(sx.clone());
                    },

                    Some((head, args)) => {
                        match self.eval(head) {
                            Ok(Sx::Builtin(builtin)) => {
                                return apply_builtin(builtin, self, args);
                            },

                            Ok(Sx::Function(ref f)) => {
                                return apply_function(f, self, args);
                            },

                            Ok(v) => {
                                return Err(Error::NotAFunction(v.clone()));
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
                    match self.eval(sx) {
                        Ok(result) => *sx = result,
                        error @ Err(_) => return error
                    }
                }

                return Ok(Sx::Vector(Arc::new(w)));
            }
        }
    }

}

pub type Result = result::Result<Sx, Error>;

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
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
    ModuleNotFound(SxSymbol, Vector<String>),
    ModuleMultipleOptions(SxSymbol, Vec<String>),
    ModuleIoOpenError(SxSymbol, String),
    ModuleIoReadError(SxSymbol, String),
    ModuleReadErrors(SxSymbol, Vec<read::Error>),
    ModuleEvalErrors(SxSymbol, Vec<Error>),
    ModuleNotLoaded(SxSymbol)
}

pub fn apply_builtin(builtin: &SxBuiltinInfo, ctx: &mut Context, arglist: &[Sx]) -> Result {
    match builtin.callback {
        SxBuiltinCallback::Special(special_fn) => {
            return apply_special(builtin, special_fn, ctx, arglist);
        },

        SxBuiltinCallback::Primitive(primitive_fn) => {
            return apply_primitive(builtin, primitive_fn, ctx, arglist);
        }
    }
}

fn apply_special(builtin: &SxBuiltinInfo, special_fn: SxBuiltinFn, ctx: &mut Context, args: &[Sx]) -> Result {
    if args.len() < builtin.min_arity {
        return Err(Error::BuiltinTooFewArgs(builtin.name, builtin.min_arity, args.len()));
    }

    match builtin.max_arity {
        Some(arity) if arity < args.len() => {
            return Err(Error::BuiltinTooManyArgs(builtin.name, arity, args.len()));
        },

        Some(_) | None => ()
    }

    return special_fn(ctx, args);
}

fn apply_primitive(builtin: &SxBuiltinInfo, primitive_fn: SxBuiltinFn, ctx: &mut Context, args: &[Sx]) -> Result {
    if args.len() < builtin.min_arity {
        return Err(Error::BuiltinTooFewArgs(builtin.name, builtin.min_arity, args.len()));
    }

    match builtin.max_arity {
        Some(arity) if arity < args.len() => {
            return Err(Error::BuiltinTooManyArgs(builtin.name, arity, args.len()));
        },

        Some(_) | None => ()
    }

    let mut result_args = args.to_vec();
    for arg in result_args.iter_mut() {
        match ctx.eval(arg) {
            Ok(result) => *arg = result,
            error @ Err(_) => return error
        }
    }

    return primitive_fn(ctx, &result_args);
}

pub fn apply_function(f: &SxFunction, ctx: &mut Context, args: &[Sx]) -> Result {
    let arity = args.len();
    if arity < f.arity {
        return Err(Error::FnTooFewArgs(f.clone(), f.arity, arity));
    }

    if f.arity < arity {
        return Err(Error::FnTooManyArgs(f.clone(), f.arity, arity));
    }

    let mut sub_ctx = ctx.clone();
    sub_ctx.current_module = f.module.clone();
    for (binding, sx) in f.bindings.iter().zip(args.iter()) {
        match ctx.eval(sx) {
            Ok(ref result) => sub_ctx.define_current(binding, result),
            error @ Err(_) => return error
        }
    }

    let exprs = f.body.clone();
    let mut result = sx_nil!();
    for expr in exprs.iter() {
        match sub_ctx.eval(expr) {
            Ok(sub_result) => result = sub_result,
            error @ Err(_) => return error
        }
    }

    return Ok(result);
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::Undefined(sx) => {
                return format!("undefined symbol: {}", sx.to_string());
            }

            Error::Redefine(symbol) => {
                return format!("cannot redefine symbol {}", symbol.to_string());
            }

            Error::RedefineCore(symbol) => {
                return format!("cannot redefine core symbol {}", symbol.to_string());
            }

            Error::DefineBadSymbol(sx) => {
                return format!("first argument to def must be a symbol, got {}", sx.to_string());
            }

            Error::SymbolBadModuleFormat(symbol) => {
                return format!("badly formatted symbol {}, expected something like my-module/my-val", symbol.clone());
            }

            Error::NotAFunction(sx) => {
                return format!("{} does not evaluate to a function", sx.to_string());
            }

            Error::InvalidBinding(sx) => {
                return format!("invalid binding form in function, got {}", sx.to_string());
            }

            Error::DuplicateBinding(symbol) => {
                return format!("cannot bind symbol {} more than once in function definition", symbol);
            }

            Error::BuiltinTooFewArgs(name, min_arity, act_arity) => {
                return format!("{} expects at least {} argument(s), got {}", name, min_arity, act_arity);
            }

            Error::BuiltinTooManyArgs(name, max_arity, act_arity) => {
                return format!("{} expects at most {} argument(s), got {}", name, max_arity, act_arity);
            }

            Error::BuiltinBadArg(name, arg) => {
                return format!("invalid argument to {}, got {}", name, arg.to_string());
            }

            Error::FnTooFewArgs(f, min_arity, act_arity) => {
                return format!("{} expects at least {} argument(s), got {}", f.to_string(), min_arity, act_arity);
            }

            Error::FnTooManyArgs(f, max_arity, act_arity) => {
                return format!("{} expects at most {} argument(s), got {}", f.to_string(), max_arity, act_arity);
            }

            Error::ModuleSelfRefer(module_name) => {
                return format!("cannot use self in module {}", module_name);
            }

            Error::ModulePathError(module_path, module_name) => {
                return format!("failed to compose filename from module path {} and name {}", module_path, module_name);
            }

            Error::ModuleNotFound(module_name, _module_paths) => {
                // TODO
                return format!("could not find module named {} under module paths", module_name);
            }

            Error::ModuleMultipleOptions(module_name, filename_matches) => {
                let filename_matches_str = filename_matches.join(", ");
                return format!("could not load module {} due to multiple options: {}", module_name, filename_matches_str);
            }

            Error::ModuleIoOpenError(module_name, io_error) => {
                return format!("error while opening file for module {}: {}", module_name, io_error);
            }

            Error::ModuleIoReadError(module_name, io_error) => {
                return format!("error while reading file for module {}: {}", module_name, io_error);
            }

            Error::ModuleReadErrors(_module_name, read_errors) => {
                let mut s = read_errors
                    .iter()
                    .fold(String::new(), |acc, e| acc + &e.to_string() + "\n");
                s.pop();

                return s;
            }

            Error::ModuleEvalErrors(_module_name, eval_errors) => {
                let mut s = eval_errors
                    .iter()
                    .fold(String::new(), |acc, e| acc + &e.to_string() + "\n");
                s.pop();

                return s;
            }

            Error::ModuleNotLoaded(module_name) => {
                return format!("module {} is not loaded", module_name);
            }
        }
    }
}

// TODO: relocate primitive and special tests
#[cfg(test)]
pub mod tests {
    use super::*;

    use std::sync::Arc;

    use ::read::read;

    pub fn test_eval(input_source: &str, output_source: &str) {
        let mut ctx = mk_test_ctx();

        let input = read(input_source).expect("invalid input source");
        let output = read(output_source).expect("invalid output source");

        let mut results = Vec::new();
        for sx in input.iter() {
            results.push(ctx.eval(&sx).expect("eval error"));
        }

        assert_eq!(sx_list_from_vec!(results).to_string(), sx_list_from_vec!(output).to_string());
    }

    pub fn test_eval_results(input_source: &str, exp_results: Vec<Result>) {
        let mut ctx = mk_test_ctx();

        let input = read(input_source).expect("invalid input source");

        let mut results = Vec::new();
        for sx in input.iter() {
            results.push(ctx.eval(&sx));
        }

        assert_eq!(results, exp_results);
    }

    fn mk_test_ctx() -> Context {
        let module_paths = vec![
            "./resources/test/eval".to_string()
        ];

        let current_module = sx_symbol_unwrapped!("test-eval");

        return Context::new(&module_paths, &current_module);
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
    fn test_primitive_error_too_few_args() {
        test_eval_results(r#"
            (apply)
            (apply +)
        "#, vec![
            Err(Error::BuiltinTooFewArgs("apply", 2, 0)),
            Err(Error::BuiltinTooFewArgs("apply", 2, 1))
        ]);
    }

    #[test]
    fn test_primitive_error_too_many_args() {
        test_eval_results(r#"
            (apply + '(1 2) 1)
        "#, vec![
            Err(Error::BuiltinTooManyArgs("apply", 2, 3))
        ]);
    }

    #[test]
    fn test_error_not_a_function() {
        test_eval_results(r#"
            (1 2 3)
        "#, vec![
            Err(Error::NotAFunction(sx_integer!(1)))
        ]);
    }

    #[test]
    fn test_error_sub_error() {
        test_eval_results(r#"
            ((1 2 3) 2 3)
        "#, vec![
            Err(Error::NotAFunction(sx_integer!(1)))
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
