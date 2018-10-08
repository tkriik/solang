#[macro_use] extern crate clap;
#[macro_use] extern crate im;
extern crate rustyline;
extern crate time;
extern crate unicode_segmentation;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

#[macro_use] mod sx;
#[macro_use] mod builtin;
mod eval;
mod init;
mod module;
mod read;
mod repl;
mod util;

use std::sync::Arc;
use clap::App;

use ::eval::Context;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let interactive = matches.is_present("interactive");

    match matches.value_of("INPUT") {
        Some(input) => {
            init::run(input, interactive);
        },

        None => {
            let module_paths = vec![
                "./".to_string()
            ];

            let current_module = sx_symbol_unwrapped!("repl");
            let mut ctx = Context::new(&module_paths, &current_module);
            ctx.import_core();

            repl::enter(&mut ctx);
        }
    }
}
