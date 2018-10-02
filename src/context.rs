use std::collections::{HashMap, HashSet};

use ::module::Module;
use ::sx::SxSymbol;

pub struct Context {
    pub module_paths:       Vec<String>,

    pub pending_modules:    HashSet<SxSymbol>,
    pub loaded_modules:     HashMap<SxSymbol, Module>,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            module_paths:       Vec::new(),

            pending_modules:    HashSet::new(),
            loaded_modules:     HashMap::new()
        };
    }

    pub fn add_module_path(&mut self, path: &str) {
        let p = path.to_string();
        assert!(!self.module_paths.contains(&p));

        self.module_paths.push(p);
    }

    fn add_pending_module(&mut self, name: &SxSymbol) {
        self.pending_modules.insert(name.clone());
    }

    fn add_loaded_module(&mut self, name: &SxSymbol, module: &Module) {
        assert!(self.pending_modules.contains(name));

        self.loaded_modules.insert(name.clone(), module.clone());
    }

    fn lookup_module(&self, name: &SxSymbol) -> Option<&Module> {
        return self.loaded_modules.get(name);
    }
}
