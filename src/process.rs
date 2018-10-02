use ::sx::{Sx, SxSymbol};

#[derive(Clone)]
pub struct Process {
    module_name:    SxSymbol,
    function_name:  SxSymbol,
    args:           Vec<Sx>
}

impl Process {
    fn new(module_name: &SxSymbol, function_name: SxSymbol, args: &Vec<Sx>) -> Process {
        return Process {
            module_name:    module_name.clone(),
            function_name:  function_name.clone(),
            args:           args.clone()
        }
    }
}