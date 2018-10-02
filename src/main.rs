#[macro_use] extern crate clap;
#[macro_use] extern crate im;
extern crate rustyline;
extern crate time;
extern crate unicode_segmentation;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

#[macro_use] mod sx;
mod context;
mod eval;
mod module;
mod process;
mod read;
mod repl;
mod init;
mod script;
mod util;

use std::sync::Arc;
use clap::App;

use init::Context;

fn main() {
    let mut ctx = Context::new();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let interactive = matches.is_present("interactive");

    match matches.value_of("INPUT") {
        Some(input) => {
            //script::run(input, interactive);
            ctx.run_file(input);
        },

        None => {
            let module_paths = vec![
                "./".to_string()
            ];

            let current_module = sx_symbol_unwrapped!("repl");
            let mut eval_ctx = eval::ctx::Ctx::new(&module_paths, &current_module);
            repl::enter(&mut eval_ctx);
        }
    }
}
