use std::collections::HashMap;

use sx::Sx;

pub struct Env {
    definitions: HashMap<Sx, Sx>
}

impl Env {
    pub fn new() -> Env {
        return Env {
            definitions: HashMap::new()
        };
    }

    pub fn define(&mut self, symbol: Sx, value: Sx) {
        self.definitions.insert(symbol, value);
    }

    pub fn lookup(&self, symbol: &Sx) -> Option<&Sx> {
        return self.definitions.get(&symbol);
    }
}
