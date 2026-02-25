mod cli;
mod io;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::process::exit;
use std::env::args;
fn main() {
    let args: Vec<String>=args().collect();
    
    if(args.len()==1){
        println!("non hai inserito il percorso del file");
        
    }
    else{

    }
    let path=&args[1];
    
}
