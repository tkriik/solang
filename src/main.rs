extern crate clap;
extern crate rustyline;
extern crate unicode_segmentation;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

use clap::{Arg, App};

mod eval;
mod read;
mod repl;
mod sx;
mod token;

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Solang (Solid Language)")
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
            println!("solang (Solid Language) {}", version);
            repl::enter();
        }

        false => {
            // TODO: scripts
            println!("solang (Solid Language) {}", version);
            repl::enter();
        }
    }
}
