use std::rc::Rc;
use std::string::ToString;

type SxInteger = i64;
type SxSymbol = Rc<String>;
type SxString = Rc<String>;
type SxList = Rc<Vec<Sx>>;
type SxQuote = Rc<Sx>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Sx {
    Nil,
    Integer(SxInteger),
    Symbol(SxSymbol),
    String(SxString),
    List(SxList),
    Quote(SxQuote)
}

impl ToString for Sx {
    fn to_string(&self) -> String {
        match self {
            Sx::Nil => format!("nil"),

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
