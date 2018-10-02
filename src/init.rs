use std::collections::{BTreeMap, HashMap, HashSet};
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
    pub loaded_modules: HashMap<SxSymbol, Vec<Sx>>,
    pub failed_modules: BTreeMap<SxSymbol, LoadError>,
    pub modules:        HashMap<SxSymbol, Module>,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            module_paths:   Vec::new(),
            loaded_modules: HashMap::new(),
            failed_modules: BTreeMap::new(),
            modules:        HashMap::new()
        };
    }

    pub fn run_file(&mut self, filename: &str) {
        let module_path = Path::new(filename)
            .parent()
            .expect("failed to read module path parent")
            .to_str()
            .expect("failed to convert module path to string");

        self.module_paths.push(module_path.to_string());

        let module_name = filename_to_module_name(filename);
        let load_result = self.load_file(filename);

        let sxs = match load_result {
            Ok(ref sxs) => sxs,
            Err(load_error) => {
                eprintln!("-----Solang startup failed!-----\n");
                self.failed_modules.insert(module_name, load_error);
                self.print_load_errors();
                return;
            }
        };

        self.loaded_modules.insert(module_name.clone(), sxs.clone());

        self.load_imports(sxs);

        if !self.failed_modules.is_empty() {
            eprintln!("-----Solang startup failed!-----\n");
            self.print_load_errors();
        }
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

    fn load_imports(&mut self, sxs: &Vec<Sx>) {
        let mut imports = Vec::new();
        let mut invalid_imports = Vec::new();
        for sx in sxs.iter() {
            let sub_sxs = match sx {
                Sx::List(ref sub_sxs) if sub_sxs.len() >= 2 => sub_sxs,
                _ => continue
            };

            match &sub_sxs[0] {
                Sx::Symbol(s) if s.as_ref() == "import" => (),
                _ => continue
            }

            for i in 1 .. sub_sxs.len() {
                let import_name = match &sub_sxs[i] {
                    Sx::Symbol(s) => s,
                    invalid_name @ _ => {
                        invalid_imports.push(invalid_name.clone());
                        continue;
                    }
                };

                eprintln!("import: {}", import_name.to_string());
                imports.push(import_name.clone());
            }
        }

        for import in imports.iter() {
            if self.loaded_modules.contains_key(import) || self.failed_modules.contains_key(import) {
                continue;
            }

            match self.load_module(import) {
                Ok(ref sxs) => {
                    self.loaded_modules.insert(import.clone(), sxs.clone());
                    self.load_imports(sxs);
                }

                Err(load_error) => {
                    self.failed_modules.insert(import.clone(), load_error);
                }
            }
        }
  }

    fn print_load_errors(&self) {
        for (module_name, load_error) in self.failed_modules.iter() {
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
    ModuleReadErrors(String, Vec<read::Error>),
    ModuleInvalidImports(Vec<Sx>)
}
