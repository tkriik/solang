use std::rc::Rc;

use ::sexp::Sexp;

#[derive(Debug)]
pub enum EvalError {
    Undefined(Sexp)
}

pub fn eval(sexp: Sexp) -> Result<Sexp, EvalError> {
    match sexp {
        Sexp::Nil => Ok(sexp.clone()),

        Sexp::Int(_) => Ok(sexp.clone()),

        Sexp::String(_) => Ok(sexp.clone()),

        Sexp::List(sexps) => {
            let mut results = Vec::new();
            for s in sexps.iter() {
                match eval(s.clone()) {
                    Ok(result) => {
                        results.push(result);
                    },

                    Err(eval_error) => {
                        return Err(eval_error);
                    }
                }
            }

            return Ok(Sexp::List(Rc::new(results)));
        }

        _ => Err(EvalError::Undefined(sexp))
    }
}