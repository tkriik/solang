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
            let module_path = "./".to_string();

            let current_module = sx_symbol_unwrapped!("repl");
            let mut ctx = Context::new(&current_module);
            ctx.add_module_path(&module_path);
            ctx.load_core();
            ctx.import_core();

            repl::enter(&mut ctx);
        }
    }
}
