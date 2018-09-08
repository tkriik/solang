extern crate clap;
extern crate rustyline;

use clap::{Arg, App};

mod parse;
mod repl;

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
