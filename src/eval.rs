use std::rc::Rc;

use ::sexp::{Sexp, SexpError};

pub fn eval(sexp: Sexp) -> Sexp {
    match sexp {
        Sexp::Nil => sexp.clone(),

        Sexp::Int(_) => sexp.clone(),

        Sexp::String(_) => sexp.clone(),

        Sexp::List(sexps) => {
            let mut results = Vec::new();
            for s in sexps.iter() {
                let result = eval(s.clone());
                results.push(result);
            }

            return Sexp::List(Rc::new(results));
        }

        _ => Sexp::Error(Rc::new(SexpError::Undefined))
    }
}