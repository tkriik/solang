extern crate rustyline;

use std::error::Error;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use ::read;

pub fn enter() {
    let history_path = ".solang_history";

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_path);

    loop {
        let readline  = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let sexps = read::sexps(&line);
                println!("got sexps:\n{:#?}", sexps);

                rl.add_history_entry(line.as_ref());
            },

            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }

            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }

            Err(err) => {
                println!("Error while reading line: {:?}", err);
                break;
            }
        }
    }

    match rl.save_history(history_path) {
        Ok(_) => (),

        Err(err) => {
            println!("Failed to save shell history to {}: {}",
                     history_path, err.description());
        }
    }
}