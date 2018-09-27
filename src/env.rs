use std::sync::Arc;

use rpds::HashTrieMap;

use primitive::PRIMITIVES;
use sx::{Sx, SxSymbol};

pub struct Env {
    definitions: HashTrieMap<SxSymbol, Sx>
}

impl Env {
    pub fn new() -> Env {
        let mut env = Env {
            definitions: HashTrieMap::new()
        };

        for primitive_info in PRIMITIVES.iter() {
            let symbol = sx_symbol_unwrapped!(primitive_info.name);
            let value = Sx::SxPrimitive(Arc::new(*primitive_info.clone()));
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
