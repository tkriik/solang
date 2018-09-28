use std::sync::Arc;

use rpds::HashTrieMap;
use im;

use builtin::{BUILTIN_MODULE_NAME, BUILTIN_TABLE};
use sx::{Sx, SxSymbol};

#[derive(Clone)]
pub struct Env {
    pub module_paths:   Vec<String>,
    pub current_module: SxSymbol,
    pub loaded_modules: im::HashSet<SxSymbol>,
    pub definitions:    HashTrieMap<(SxSymbol, SxSymbol), Sx>, // TODO: immutable.rs

    pub core_module:    SxSymbol
}

impl Env {
    pub fn new(module_paths: &Vec<String>, current_module: &SxSymbol) -> Env {
        let core_module = sx_symbol_unwrapped!(BUILTIN_MODULE_NAME);

        let mut env = Env {
            module_paths:   module_paths.clone(),
            current_module: current_module.clone(),
            loaded_modules: hashset!(core_module.clone(), current_module.clone()),
            definitions:    HashTrieMap::new(),

            core_module:    core_module.clone()
        };

        for builtin in BUILTIN_TABLE.iter() {
            let symbol = sx_symbol_unwrapped!(builtin.name);
            let value = Sx::Builtin(*builtin);
            env.define(&core_module, &symbol, &value);
        }

        return env;
    }

    pub fn define(&mut self, module: &SxSymbol, symbol: &SxSymbol, value: &Sx) {
        self.definitions.insert_mut((module.clone(), symbol.clone()), value.clone());
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
}
