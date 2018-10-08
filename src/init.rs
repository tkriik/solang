use std::path::Path;
use std::sync::Arc;

use ::eval::Context;
use ::module;
use ::repl;

pub fn run(path: &str, interactive: bool) {
    let module_name = module::filename_to_name(path);

    let module_path = Path::new(path)
        .parent().expect("failed to read module path parent")
        .to_str().expect("failed to convert module path to string")
        .to_string();

    let current_module= sx_symbol_unwrapped!("core");
    let mut ctx = Context::new(&current_module);
    ctx.add_module_path(&module_path);
    ctx.load_core();
    ctx.import_core();

    match module::load_import(&mut ctx, &module_name) {
        Ok(_) => (),
        Err(eval_error) => println!("failed to run file {}: {}", path, eval_error.to_string())
    }

    if interactive {
        ctx.current_module = module_name;
        repl::enter(&mut ctx);
    }
}