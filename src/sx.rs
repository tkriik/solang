use std::fmt;
use std::clone::Clone;
use std::string::ToString;
use std::sync::Arc;

use ::env::Env;
use ::eval::EvalResult;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Sx {
    Nil,
    Boolean(SxBoolean),
    Integer(SxInteger),
    Symbol(SxSymbol),
    String(SxString),
    List(SxList),
    Quote(SxQuote),
    Builtin(SxBuiltin),
    Function(SxFunction)
}

pub type SxBoolean      = bool;
pub type SxInteger      = i64;
pub type SxString       = Arc<String>;
pub type SxSymbol       = Arc<String>;
pub type SxList         = Arc<Vec<Sx>>;
pub type SxQuote        = Arc<Sx>;
pub type SxBuiltin      = &'static SxBuiltinInfo;
pub type SxFunction     = Arc<SxFunctionInfo>;

pub struct SxBuiltinInfo {
    pub name:       &'static str,
    pub min_arity:  usize,
    pub max_arity:  Option<usize>,
    pub callback:   SxBuiltinCallback
}

pub enum SxBuiltinCallback {
    Special(SxSpecialFn),
    Primitive(SxPrimitiveFn)
}

pub type SxSpecialFn = fn(&mut Env, &Vec<&Sx>) -> EvalResult;
pub type SxPrimitiveFn = fn(&mut Env, &Vec<Sx>) -> EvalResult;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SxFunctionInfo {
    pub arity:      usize,
    pub bindings:   Vec<SxSymbol>,
    pub body:       SxList
}

#[macro_export]
macro_rules! sx_nil {
    () => (Sx::Nil);
}

#[macro_export]
macro_rules! sx_boolean {
    ($e:expr) => (Sx::Boolean($e));
}

#[macro_export]
macro_rules! sx_integer {
    ($e:expr) => (Sx::Integer($e));
}

#[macro_export]
macro_rules! sx_symbol {
    ($e:expr) => (Sx::Symbol(Arc::new($e.to_string())));
}

#[macro_export]
macro_rules! sx_symbol_unwrapped {
    ($e:expr) => (Arc::new($e.to_string()));
}

#[macro_export]
macro_rules! sx_string {
    ($e:expr) => (Sx::String(Arc::new($e.to_string())));
}

#[macro_export]
macro_rules! sx_list {
    [ $( $e:expr ),*] => (Sx::List(Arc::new(vec![$($e),*])));
}

#[macro_export]
macro_rules! sx_quote {
    ($e:expr) => (Sx::Quote(Arc::new($e)));
}

impl ToString for Sx {
    fn to_string(&self) -> String {
        match self {
            Sx::Nil => format!("nil"),

            Sx::Boolean(b) => format!("{}", b),

            Sx::Integer(i) => format!("{}", i),

            Sx::Symbol(s) => format!("{}", s),

            Sx::String(s) => format!("\"{}\"", s),

            Sx::List(sxs) => {
                let mut s = String::new();
                let mut first = true;

                s.push('(');
                for sx in sxs.iter() {
                    if !first {
                        s.push(' ');
                    }

                    first = false;
                    let sub = sx.to_string();
                    s.push_str(sub.as_ref());
                }
                s.push(')');

                return s;
            },

            Sx::Quote(sx) => format!("'{}", sx.to_string()),

            Sx::Builtin(b) => b.to_string(),

            Sx::Function(f) => format!("{}", f.to_string())
        }
    }
}

impl Copy for SxBuiltinCallback {}

impl Clone for SxBuiltinCallback {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Debug for SxBuiltinInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Eq for SxBuiltinInfo {}

impl PartialEq for SxBuiltinInfo {
    fn eq(&self, other: &SxBuiltinInfo) -> bool {
        let info_eq = self.name == other.name
            && self.min_arity == other.min_arity
            && self.max_arity == other.max_arity;

        let callback_eq = match (self.callback, other.callback) {
            (SxBuiltinCallback::Special(_), SxBuiltinCallback::Primitive(_)) => false,

            (SxBuiltinCallback::Primitive(_), SxBuiltinCallback::Special(_)) => false,

            (SxBuiltinCallback::Special(a), SxBuiltinCallback::Special(b)) => a as usize == b as usize,

            (SxBuiltinCallback::Primitive(a), SxBuiltinCallback::Primitive(b)) => a as usize == b as usize
        };

        return info_eq && callback_eq;
    }
}

impl ToString for SxBuiltinInfo {
    fn to_string(&self) -> String {
        let arity_str = match (self.min_arity, self.max_arity) {
            (min_arity, Some(max_arity)) if min_arity == max_arity => format!("{}", min_arity),
            (min_arity, Some(max_arity)) => format!("{}..{}", min_arity, max_arity),
            (min_arity, None) => format!("{}..", min_arity)
        };

        let info_str = format!("name: {}, arity: {}", self.name, arity_str);

        match self.callback {
            SxBuiltinCallback::Special(_) => {
                return format!("#special<{}>", info_str);
            },

            SxBuiltinCallback::Primitive(_) => {
                return format!("#primitive<{}>", info_str);
            }
        }
    }
}

impl ToString for SxFunctionInfo {
    fn to_string(&self) -> String {
        let mut bindings_str = String::new();
        let mut first = true;

        bindings_str.push('(');
        for symbol in self.bindings.iter() {
            if !first {
                bindings_str.push(' ');
            }

            first = false;
            bindings_str.push_str(symbol.as_ref());
        }
        bindings_str.push(')');

        return format!("#function<arity: {}, bindings: {}>", self.arity, bindings_str);
    }
}
