use std::path::Path;
use std::sync::Arc;

use ::env::Env;
use ::module;
use ::repl;

pub fn run(path: &str, interactive: bool) {
    let module_name = module::from_filename(path);

    let module_path = Path::new(path).parent().expect("failed to read module path parent");
    let module_paths = vec![
        module_path.to_str().expect("failed to convert module path to string").to_string()
    ];

    let current_module= sx_symbol_unwrapped!("core");
    let mut env = Env::new(&module_paths, &current_module);

    match module::load_use(&mut env, &module_name) {
        Ok(_) => (),
        Err(eval_error) => println!("failed to run file {}: {}", path, eval_error.to_string())
    }

    if interactive {
        env.current_module = module_name;
        repl::enter(&mut env);
    }
}