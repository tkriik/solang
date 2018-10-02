use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

use ::module::{Module, filename_to_module_name};
use ::read;
use ::sx::{Sx, SxSymbol};

#[derive(Debug)]
pub struct Context {
    pub module_paths:   Vec<String>,
    pub loaded_modules: HashMap<SxSymbol, LoadResult>,
    pub modules:        HashMap<SxSymbol, Module>,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            module_paths:   Vec::new(),
            loaded_modules: HashMap::new(),
            modules:        HashMap::new()
        };
    }

    pub fn run_file(&mut self, filename: &str) {
        let load_result = self.load_file(filename);
        let module_path = Path::new(filename)
            .parent()
            .expect("failed to read module path parent")
            .to_str()
            .expect("failed to convert module path to string");

        self.add_module_path(module_path);

        let module_name = filename_to_module_name(filename);
        self.add_pending_module(&module_name, load_result.clone());

        //println!("load result: {:#?}", load_result);
        //println!("context: {:#?}", self);
        let sxs = match load_result {
            Ok(sxs) => sxs,
            Err(e) => {
                self.print_load_errors();
                return;
            }
        };
    }

    fn add_module_path(&mut self, path: &str) {
        let p = path.to_string();
        assert!(!self.module_paths.contains(&p));
        self.module_paths.push(p);
    }

    fn add_pending_module(&mut self, name: &SxSymbol, load_result: LoadResult) {
        assert!(!self.loaded_modules.contains_key(name));
        self.loaded_modules.insert(name.clone(), load_result);
    }

    fn load_module(&self, name: &SxSymbol) -> LoadResult {
        let mut filename_matches = Vec::new();
        for module_path in self.module_paths.iter() {
            let full_path = Path::new(module_path)
                .join(name.as_ref())
                .with_extension("sol");

            let filename = match full_path.to_str() {
                Some(filename) => filename,
                None => return Err(LoadError::ModulePathError(module_path.clone()))
            };

            match fs::metadata(filename) {
                Ok(ref meta) if meta.is_file() => filename_matches.push(filename.to_string()),
                _ => ()
            }
        }

        let filename = match filename_matches[..] {
            [ref filename] => filename,
            [] => return Err(LoadError::ModuleNotFound),
            _ => return Err(LoadError::ModuleMultiplePathOptions(filename_matches.clone()))
        };

        return self.load_file(filename.as_ref());
    }

    fn load_file(&self, filename: &str) -> LoadResult {
        let mut file = match File::open(filename) {
            Ok(mut f) => f,
            Err(e) => return Err(LoadError::ModuleIoOpenError(filename.to_string(), e.to_string()))
        };

        let mut source = String::new();
        match file.read_to_string(&mut source) {
            Ok(_) => (),
            Err(e) => return Err(LoadError::ModuleIoReadError(filename.to_string(), e.to_string()))
        };

        let sxs = match read::from_str(source.as_ref()) {
            Ok(sxs) => sxs,
            Err(read_errors) => return Err(LoadError::ModuleReadErrors(filename.to_string(), read_errors))
        };

        return Ok(sxs);
    }

    fn eval_module_top(&mut self, name: &SxSymbol) -> LoadResult {
        return Ok(Vec::new());
    }

    fn print_load_errors(&self) {
        for (module_name, load_result) in self.loaded_modules.iter() {
            let load_error = match load_result {
                Ok(_) => continue,
                Err(load_error) => load_error
            };

            eprintln!("Failed to load module {}:", module_name.as_ref());
            match load_error {
                LoadError::ModuleIoOpenError(filename, err_msg) => {
                    eprintln!("  - failed to open file at '{}':", filename);
                    eprintln!("    * {}", err_msg);
                }

                LoadError::ModuleIoReadError(filename, err_msg) => {
                    eprintln!("  - failed to read file at '{}':", filename);
                    eprintln!("    * {}", err_msg);
                }

                LoadError::ModuleReadErrors(filename, read_errors) => {
                    eprintln!("  - syntax errors in '{}':", filename);
                    for e in read_errors.iter() {
                        eprintln!("    * {}", e.to_string())
                    }
                }

                _ => {
                    // TODO
                    eprintln!("  - {:?}", load_error);
                }
            }

            eprint!("\n");
        }
    }
}

pub type LoadResult = Result<Vec<Sx>, LoadError>;

#[derive(Debug, Clone)]
pub enum LoadError {
    ModulePathError(String),
    ModuleNotFound,
    ModuleMultiplePathOptions(Vec<String>),
    ModuleIoOpenError(String, String),
    ModuleIoReadError(String, String),
    ModuleReadErrors(String, Vec<read::Error>)
}
