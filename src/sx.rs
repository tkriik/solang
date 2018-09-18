use std::rc::Rc;
use std::string::ToString;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Sx {
    Nil,
    Int(i64),
    Symbol(Rc<String>),
    String(Rc<String>),
    List(Rc<Vec<Sx>>)
}

impl ToString for Sx {
    fn to_string(&self) -> String {
        match self {
            Sx::Nil => format!("nil"),

            Sx::Int(i) => format!("{}", i),

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
            }
        }
    }
}
