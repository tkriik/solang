use std::rc::Rc;

use ::sx::Sx;

#[derive(Debug)]
pub enum EvalError {
    Undefined(Sx)
}

pub fn eval(sx: Sx) -> Result<Sx, EvalError> {
    match sx {
        Sx::Nil => Ok(sx.clone()),

        Sx::Int(_) => Ok(sx.clone()),

        Sx::String(_) => Ok(sx.clone()),

        Sx::List(sxs) => {
            let mut results = Vec::new();
            for s in sxs.iter() {
                match eval(s.clone()) {
                    Ok(result) => {
                        results.push(result);
                    },

                    Err(eval_error) => {
                        return Err(eval_error);
                    }
                }
            }

            return Ok(Sx::List(Rc::new(results)));
        }

        _ => Err(EvalError::Undefined(sx))
    }
}