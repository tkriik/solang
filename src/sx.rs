use std::string::ToString;
use std::sync::Arc;

use rpds::List;

pub type SxBoolean  = bool;
pub type SxInteger  = i64;
pub type SxString   = Arc<String>;
pub type SxSymbol   = Arc<String>;
pub type SxList     = Arc<List<Sx>>;
pub type SxQuote    = Arc<Sx>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Sx {
    Nil,
    Boolean(SxBoolean),
    Integer(SxInteger),
    Symbol(SxSymbol),
    String(SxString),
    List(SxList),
    Quote(SxQuote)
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

            Sx::Quote(sx) => format!("'{}", sx.to_string())
        }
    }
}
