#[macro_export]
macro_rules! special_var_arity {
    ($name:expr, $min_arity:expr, $max_arity: expr, $callback:expr) => {
        SxBuiltinInfo {
            name: $name,
            min_arity:  $min_arity,
            max_arity:  Some($max_arity),
            callback:   SxBuiltinCallback::Special($callback)
        }
    }
}

#[macro_export]
macro_rules! special_no_arg_limit {
    ($name:expr, $arity:expr, $callback:expr) => {
        SxBuiltinInfo {
            name: $name,
            min_arity:  $arity,
            max_arity:  None,
            callback:   SxBuiltinCallback::Special($callback)
        }
    }
}

#[macro_export]
macro_rules! special {
    ($name:expr, $arity:expr, $callback:expr) => (special_var_arity!($name, $arity, $arity, $callback))
}

#[macro_export]
macro_rules! primitive_var_arity {
    ($name:expr, $min_arity:expr, $max_arity: expr, $callback:expr) => {
        SxBuiltinInfo {
            name: $name,
            min_arity:  $min_arity,
            max_arity:  Some($max_arity),
            callback:   SxBuiltinCallback::Primitive($callback)
        }
    }
}

#[macro_export]
macro_rules! primitive_no_arg_limit {
    ($name:expr, $arity:expr, $callback:expr) => {
        SxBuiltinInfo {
            name: $name,
            min_arity:  $arity,
            max_arity:  None,
            callback:   SxBuiltinCallback::Primitive($callback)
        }
    }
}

#[macro_export]
macro_rules! primitive {
    ($name:expr, $arity:expr, $callback:expr) => (primitive_var_arity!($name, $arity, $arity, $callback))
}

pub mod core;
