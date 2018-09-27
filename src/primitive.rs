use sx::{Sx, SxPrimitiveInfo};

pub type PrimitiveFn = fn(&Vec<Sx>) -> PrimitiveResult;

pub type PrimitiveResult = Result<Sx, PrimitiveError>;

pub enum PrimitiveError {
    BadArg
}

pub static PRIMITIVES: &'static [&SxPrimitiveInfo] = &[
    &PLUS
];

static PLUS: SxPrimitiveInfo = SxPrimitiveInfo {
    name:       "+",
    min_arity:  0,
    max_arity:  None,
    callback:   plus
};

fn plus(args: &Vec<Sx>) -> PrimitiveResult {
    let mut sum = 0;
    for arg in args {
        match arg {
            Sx::Integer(n) => {
                sum += n;
            },

            _ => {
                return Err(PrimitiveError::BadArg);
            }
        }
    }

    return Ok(sx_integer!(sum));
}
