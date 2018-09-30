extern crate rustyline;

use std::error::Error;

use rustyline::Editor;
use rustyline::error::ReadlineError;
use time;

use ::eval::env::Env;
use ::eval::eval::eval;
use ::read::read;
use ::util::pretty::pretty;

pub fn enter(mut env: &mut Env) {
    let history_path = ".solang_history";

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_path);

    let version = env!("CARGO_PKG_VERSION");
    println!("solang (Solid Language) {}", version);

    loop {
        let prompt = format!("{}=> ", env.current_module);
        let readline  = rl.readline(prompt.as_ref());
        match readline {
            Ok(line) => {
                match read(&line) {
                    Ok(sxs) => {
                        for sx in sxs.iter() {
                            let t0 = time::precise_time_s();
                            match eval(&mut env, sx) {
                                Ok(ref result) => {
                                    let t1 = time::precise_time_s();
                                    println!("{}", pretty(result));
                                    println!("time: {:.6}s", t1 - t0);
                                },

                                Err(eval_error) => {
                                    println!("eval error: {}", eval_error.to_string());
                                }
                            }
                        }
                    },

                    Err(read_errors) => {
                        for read_error in read_errors {
                            println!("read error: {}", read_error.to_string());
                        }
                    }
                }

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
