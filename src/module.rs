use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use ::sx::{Sx, SxSymbol};

#[derive(Clone, Debug)]
pub struct Module {
    pub imports:        HashSet<SxSymbol>,
    pub exports:        HashSet<SxSymbol>,
    pub definitions:    HashMap<SxSymbol, Sx>
}

impl Module {
    pub fn new() -> Module {
        Module {
            imports:        HashSet::new(),
            exports:        HashSet::new(),
            definitions:    HashMap::new()
        }
    }

    pub fn define(&mut self, symbol: &SxSymbol, value: &Sx) -> Result<SxSymbol, Error> {
        let k = symbol.clone();

        if self.definitions.contains_key(&k) {
            return Err(Error::Redefine(k));
        }

        let v = value.clone();
        self.definitions.insert(k.clone(), v);
        return Ok(k.clone());
    }

    pub fn lookup(&self, symbol: &SxSymbol) -> Option<&Sx> {
        return self.definitions.get(symbol);
    }
}

pub fn filename_to_module_name(filename: &str) -> SxSymbol {
    let stem = Path::new(filename)
        .file_stem()
        .expect("invalid filename")
        .to_str()
        .expect("invalid path")
        .to_string();

    return sx_symbol_unwrapped!(stem);
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    Redefine(SxSymbol)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_define_lookup() {
        let mut module = Module::new();

        let k1 = sx_symbol_unwrapped!("k1");
        assert_eq!(None, module.lookup(&k1));

        let v1 = sx_string!("v1");
        assert_eq!(Ok(k1.clone()), module.define(&k1, &v1));
        assert_eq!(Some(&v1), module.lookup(&k1));

        assert_eq!(Err(Error::Redefine(k1.clone())), module.define(&k1, &v1));
        assert_eq!(Some(&v1), module.lookup(&k1));
    }

    #[test]
    fn test_filename_to_module_name_flat() {
        let exp = sx_symbol_unwrapped!("foo-module");
        let act = filename_to_module_name("foo-module.sol");
        assert_eq!(exp, act);
    }

    #[test]
    fn test_filename_to_module_name_deep() {
        let exp = sx_symbol_unwrapped!("baz-module");
        let act = filename_to_module_name("./foo/bar/baz-module.sol");
        assert_eq!(exp, act);
    }
}