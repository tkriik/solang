use std::sync::Arc;

use rpds::HashTrieMap;

use builtin::BUILTIN_ARRAY;
use sx::{Sx, SxSymbol};

#[derive(Clone)]
pub struct Env {
    definitions: HashTrieMap<SxSymbol, Sx>
}

impl Env {
    pub fn new() -> Env {
        let mut env = Env {
            definitions: HashTrieMap::new()
        };

        for builtin in BUILTIN_ARRAY.iter() {
            let symbol = sx_symbol_unwrapped!(builtin.name);
            let value = Sx::Builtin(*builtin);
            env.define(&symbol, &value);
        }

        return env;
    }

    pub fn define(&mut self, symbol: &SxSymbol, value: &Sx) {
        self.definitions.insert_mut(symbol.clone(), value.clone());
    }

    pub fn lookup(&self, symbol: &SxSymbol) -> Option<&Sx> {
        return self.definitions.get(symbol.as_ref());
    }
}
