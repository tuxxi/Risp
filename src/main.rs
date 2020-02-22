extern crate clap;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::io;

fn main() -> io::Result<()> {
    println!("risp v.{}", clap::crate_version!());
    println!();
    repl()?;
    Ok(())
}

fn repl() -> io::Result<()> {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }
    loop { 
        let readline = rl.readline("risp > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("  {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // rl.save_history("history.txt").unwrap();
    Ok(())
}
