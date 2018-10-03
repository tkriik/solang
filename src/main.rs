#[macro_use] extern crate clap;
#[macro_use] extern crate im;
extern crate rustyline;
extern crate time;
extern crate unicode_segmentation;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

#[macro_use] mod sx;
mod builtin;
mod eval;
mod module;
mod read;
mod repl;
mod script;
mod util;

use std::sync::Arc;
use clap::App;

use ::eval::Env;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let interactive = matches.is_present("interactive");


    match matches.value_of("INPUT") {
        Some(input) => {
            script::run(input, interactive);
        },

        None => {
            let module_paths = vec![
                "./".to_string()
            ];

            let current_module = sx_symbol_unwrapped!("repl");
            let mut env = Env::new(&module_paths, &current_module);
            repl::enter(&mut env);
        }
    }
}
