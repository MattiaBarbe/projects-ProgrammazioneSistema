//use std::fs::File;
//use std::io::{BufRead,BufReader};
//use std::process::exit;
use std::{env::args, process::exit};
mod io;
mod cli;
fn main() {
    let (file, head) = match cli::parseArg() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Errore: {}", e);
            process::exit(1);
        }
    };
    
}
