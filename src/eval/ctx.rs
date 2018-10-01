use std::sync::Arc;

use im;

use ::eval::builtin::{BUILTIN_MODULE_NAME, BUILTIN_TABLE};
use ::sx::{Sx, SxSymbol};

#[derive(Clone)]
pub struct Ctx {
    pub module_paths:   Vec<String>,
    pub current_module: SxSymbol,
    pub loaded_modules: im::HashSet<SxSymbol>,
    pub definitions:    im::HashMap<(SxSymbol, SxSymbol), Sx>,

    pub core_module:    SxSymbol
}

impl Ctx {
    pub fn new(module_paths: &Vec<String>, current_module: &SxSymbol) -> Ctx {
        let core_module = sx_symbol_unwrapped!(BUILTIN_MODULE_NAME);

        let mut ctx = Ctx {
            module_paths:   module_paths.clone(),
            current_module: current_module.clone(),
            loaded_modules: hashset!(core_module.clone(), current_module.clone()),
            definitions:    hashmap!(),

            core_module:    core_module.clone()
        };

        for builtin in BUILTIN_TABLE.iter() {
            let symbol = sx_symbol_unwrapped!(builtin.name);
            let value = Sx::Builtin(*builtin);
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
}
