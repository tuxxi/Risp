extern crate clap;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    println!("risp v.{}", clap::crate_version!());
    println!("Press Ctrl-C to exit");
    println!();
    repl()?;
    Ok(())
}

fn repl() -> io::Result<()> {
    loop { 
        print!("risp > ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        println!("  {}", input.trim());
    }
}
