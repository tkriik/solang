use ::env::Env;
use ::sx::Sx;

#[derive(Debug)]
pub enum EvalError {
    Undefined(Sx),
    Arity(Sx, usize, usize),
    Redefine(Sx)
}

pub fn eval(env: &mut Env, sx: &Sx) -> Result<Sx, EvalError> {
    if is_self_eval(sx) {
        return Ok(sx.clone());
    }

    if is_symbol(sx) {
        match env.lookup(sx) {
            Some(v) => {
                return Ok(v.clone());
            },

            None => {
                return Err(EvalError::Undefined(sx.clone()));
            }
        }
    }

    if is_def(sx) {
        return do_def(env, sx);
    }

    return Err(EvalError::Undefined(sx.clone()));
}

fn is_self_eval(sx: &Sx) -> bool {
    match sx {
        Sx::Nil | Sx::Int(_) | Sx::String(_) => true,

        Sx::List(l) => l.is_empty(),

        _ => false
    }
}

fn is_symbol(sx: &Sx) -> bool {
    match sx {
        Sx::Symbol(_) => true,

        _ => false
    }
}

// TODO: refactor
fn is_def(sx: &Sx) -> bool {
    match sx {
        Sx::List(l) => {
            match l.first() {
                Some(Sx::Symbol(name)) => name.as_ref() == "def",

                Some(_) => false,

                None => false
            }
        },

        _ => false
    }
}

// TODO: refactor
fn do_def(env: &mut Env, sx: &Sx) -> Result<Sx, EvalError> {
    match sx {
        Sx::List(l) => {
            let symbol = &l[1];
            let exp_arity = 2;
            let act_arity = l.len() - 1;
            if exp_arity != act_arity {
                return Err(EvalError::Arity(symbol.clone(), exp_arity, act_arity));
            }

            match env.lookup(symbol) {
                Some(_) => {
                    return Err(EvalError::Redefine(symbol.clone()));
                },

                None => {
                    let value = &l[2];
                    env.define(symbol.clone(), value.clone());
                    return Ok(symbol.clone());
                }
            }
        },

        _ => {
            assert!(false);
            return Err(EvalError::Undefined(sx.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nil() {
        assert_eq!(1, 1);
    }
}
