pub mod env;
pub mod eval;
pub mod module;

mod builtin;

use std::result;

use ::read;
use ::sx::{Sx, SxSymbol, SxFunction};

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
    ModuleNotFound(SxSymbol, Vec<String>),
    ModuleMultipleOptions(SxSymbol, Vec<String>),
    ModuleIoOpenError(SxSymbol, String),
    ModuleIoReadError(SxSymbol, String),
    ModuleReadErrors(SxSymbol, Vec<read::Error>),
    ModuleEvalErrors(SxSymbol, Vec<Error>),
    ModuleNotLoaded(SxSymbol)
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

            Error::ModuleNotFound(module_name, module_paths) => {
                let module_paths_str = module_paths.join(", ");
                return format!("could not find module named {} under following module paths: {}", module_name, module_paths_str);
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
