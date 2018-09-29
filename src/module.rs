use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

use ::env::Env;
use ::eval::{eval, EvalResult, EvalError};
use ::read::read;
use ::sx::{Sx, SxSymbol};

pub fn from_filename(filename: &str) -> SxSymbol {
    let stem = Path::new(filename)
        .file_stem().expect("invalid filename")
        .to_str().expect("invalid path");

    return sx_symbol_unwrapped!(stem.to_string());
}

pub fn entry_from_symbol(symbol: &SxSymbol) -> Vec<SxSymbol> {
    return symbol
        .as_ref()
        .split('/')
        .map(|x| sx_symbol_unwrapped!(x))
        .collect::<Vec<_>>();
}

pub fn load_use(env: &mut Env, module_name: &SxSymbol) -> EvalResult {
    if module_name.as_ref() == env.current_module.as_ref() {
        return Err(EvalError::ModuleSelfRefer(module_name.clone()));
    }

    if env.loaded_modules.contains(module_name) {
        return Ok(Sx::Symbol(module_name.clone()));
    }

    let mut filename_matches = Vec::new();

    for module_path in env.module_paths.iter() {
        let filename_path = Path::new(module_path)
            .join(Path::new(module_name.as_ref()))
            .with_extension("sol");

        match filename_path.to_str() {
            Some(filename) => {
                match fs::metadata(filename) {
                    Ok(ref file_meta) if file_meta.is_file() => {
                        filename_matches.push(filename.to_string());
                    },

                    _ => ()
                }
            },

            None => {
                return Err(EvalError::ModulePathError(module_path.clone(), module_name.as_ref().clone()));
            }
        }
    }

    let filename = match filename_matches[..] {
        [ref f] => f,
        []      => return Err(EvalError::ModuleNotFound(module_name.clone(), env.module_paths.clone())),
        _       => return Err(EvalError::ModuleMultipleOptions(module_name.clone(), filename_matches.clone()))
    };

    let mut file = match File::open(filename) {
        Ok(mut f)   => f,
        Err(e)      => return Err(EvalError::ModuleIoOpenError(module_name.clone(), e.to_string()))
    };

    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Ok(_)   => (),
        Err(e)  => return Err(EvalError::ModuleIoReadError(module_name.clone(), e.to_string()))
    }

    let sxs = match read(source.as_ref()) {
        Ok(xs) => xs,
        Err(read_errors) => {
            return Err(EvalError::ModuleReadErrors(module_name.clone(), read_errors))
        }
    };

    let mut new_env = env.clone();
    new_env.current_module = module_name.clone();
    new_env.loaded_modules.insert(module_name.clone());

    let mut eval_errors = Vec::new();
    for sx in sxs.iter() {
        match eval(&mut new_env, sx) {
            Ok(_)           => (),
            Err(eval_error) => eval_errors.push(eval_error)
        }
    }

    if !eval_errors.is_empty() {
        return Err(EvalError::ModuleEvalErrors(module_name.clone(), eval_errors));
    }

    new_env.current_module = env.current_module.clone();
    *env = new_env;

    return Ok(Sx::Symbol(module_name.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_filename_flat() {
        let exp = sx_symbol_unwrapped!("foo-module");
        let act = from_filename("foo-module.sol");
        assert_eq!(exp, act);
    }

    #[test]
    fn test_entry_from_symbol_flat() {
        let exp = vec![sx_symbol_unwrapped!("foo"), sx_symbol_unwrapped!("fooval")];
        let act = entry_from_symbol(&sx_symbol_unwrapped!("foo/fooval"));
        assert_eq!(exp, act);
    }
}
