use ::env::Env;
use ::sx::Sx;

#[derive(Debug)]
pub enum EvalError {
    Undefined(Sx),
    Arity(Sx, usize, usize),
    Redefine(Sx)
}

pub fn eval(env: &mut Env, sx: &Sx) -> Result<Sx, EvalError> {
    match sx {
        Sx::Nil | Sx::Integer(_) | Sx::String(_) => {
            return Ok(sx.clone());
        },

        Sx::List(l) if l.is_empty() => {
            return Ok(sx.clone());
        },

        Sx::Quote(v) => {
            return Ok(v.as_ref().clone());
        },

        Sx::Symbol(_) => {
            match env.lookup(sx) {
                Some(v) => {
                    return Ok(v.clone());
                },

                None => {
                    return Err(EvalError::Undefined(sx.clone()));
                }
            }
        },

        _ => {
            assert!(false);
            return Err(EvalError::Undefined(sx.clone()));
        }

        //Sx::List(l) => {
        //    match l.first() {
        //        Some(Sx::Symbol(name)) if name.as_ref() == "def" => {
        //            return do_def(env, sx);
        //        },

        //        _ => {
        //            return Err(EvalError::Undefined(sx.clone()));
        //        }
        //    }
        //}
    }
}


// TODO: refactor
//fn do_def(env: &mut Env, sx: &Sx) -> Result<Sx, EvalError> {
//    match sx {
//        Sx::List(l) => {
//            let symbol = &l[1];
//            let exp_arity = 2;
//            let act_arity = l.len() - 1;
//            if exp_arity != act_arity {
//                return Err(EvalError::Arity(symbol.clone(), exp_arity, act_arity));
//            }
//
//            match env.lookup(symbol) {
//                Some(_) => {
//                    return Err(EvalError::Redefine(symbol.clone()));
//                },
//
//                None => {
//                    let value = &l[2];
//                    match eval(env, value) {
//                        Ok(result) => {
//                            env.define(symbol.clone(), result.clone());
//                            return Ok(symbol.clone());
//                        },
//
//                        error @ Err(_) => {
//                            return error;
//                        }
//                    }
//                }
//            }
//        },
//
//        _ => {
//            assert!(false);
//            return Err(EvalError::Undefined(sx.clone()));
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nil() {
        assert_eq!(1, 1);
    }
}
