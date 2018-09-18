use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Sx {
    Nil,
    Int(i64),
    Symbol(Rc<String>),
    String(Rc<String>),
    List(Rc<Vec<Sx>>)
}
