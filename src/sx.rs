use std::fmt;
use std::clone::Clone;
use std::string::ToString;
use std::sync::Arc;

use rpds::List;

use primitive::PrimitiveFn;

pub type SxBoolean      = bool;
pub type SxInteger      = i64;
pub type SxString       = Arc<String>;
pub type SxSymbol       = Arc<String>;
pub type SxList         = Arc<List<Sx>>;
pub type SxQuote        = Arc<Sx>;
pub type SxPrimitive    = Arc<SxPrimitiveInfo>;

pub struct SxPrimitiveInfo {
    pub name:       &'static str,
    pub min_arity:  usize,
    pub max_arity:  Option<usize>,
    pub callback:   PrimitiveFn
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Sx {
    Nil,
    Boolean(SxBoolean),
    Integer(SxInteger),
    Symbol(SxSymbol),
    String(SxString),
    List(SxList),
    Quote(SxQuote),
    SxPrimitive(SxPrimitive)
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
    [ $( $e:expr ),*] => (Sx::List(Arc::new(list![$($e),*])));
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

            Sx::SxPrimitive(p) => p.to_string()
        }
    }
}

impl Copy for SxPrimitiveInfo {}

impl Clone for SxPrimitiveInfo {
    fn clone(&self) -> Self {
        return SxPrimitiveInfo {
            name:       self.name.clone(),
            min_arity:  self.min_arity,
            max_arity:  self.max_arity,
            callback:   self.callback
        }
    }
}
impl fmt::Debug for SxPrimitiveInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Eq for SxPrimitiveInfo {}

impl PartialEq for SxPrimitiveInfo {
    fn eq(&self, other: &SxPrimitiveInfo) -> bool {
        return self.name == other.name
            && self.min_arity == other.min_arity
            && self.max_arity == other.max_arity
            && self.callback as usize == other.callback as usize;
    }
}

impl ToString for SxPrimitiveInfo {
    fn to_string(&self) -> String {
        let arity_str = match (self.min_arity, self.max_arity) {
            (min_arity, Some(max_arity)) if min_arity == max_arity => format!("{}", min_arity),
            (min_arity, Some(max_arity)) => format!("{}..{}", min_arity, max_arity),
            (min_arity, None) => format!("{}..", min_arity)
        };

        return format!("#primitive<name: {}, arity: {}>", self.name, arity_str);
    }
}
