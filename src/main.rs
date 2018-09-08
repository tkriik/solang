extern crate clap;
extern crate rustyline;
extern crate unicode_segmentation;

use clap::{Arg, App};

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

mod read;
mod repl;
mod sexp;
mod token;

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Solang (Soft Language)")
        .version(version)
        .author("Tanel Kriik <tanel.kriik@gmail.com>")
        .about("LISP attempt")
        .arg(Arg::with_name("i")
                 .short("i")
                 .long("interactive")
                 .help("Run shell"))
        .get_matches();

    let interactive = matches.is_present("i");

    match interactive {
        true => {
            println!("solang (Soft Language) {}", version);
            repl::enter();
        }

        false => {
            // TODO: scripts
            println!("solang (Soft Language) {}", version);
            repl::enter();
        }
    }
}
