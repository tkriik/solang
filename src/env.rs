use rpds::HashTrieMap;

use sx::{Sx, SxSymbol};

pub struct Env {
    definitions: HashTrieMap<SxSymbol, Sx>
}

impl Env {
    pub fn new() -> Env {
        return Env {
            definitions: HashTrieMap::new()
        };
    }

    pub fn define(&mut self, symbol: &SxSymbol, value: &Sx) {
        self.definitions.insert_mut(symbol.clone(), value.clone());
    }

    pub fn lookup(&self, symbol: &SxSymbol) -> Option<&Sx> {
        return self.definitions.get(symbol.as_ref());
    }
}
