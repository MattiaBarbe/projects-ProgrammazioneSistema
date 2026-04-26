use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::{self, Ok};


pub fn open(path:&String,righe:&i32)->Result<i32,anyhow::Error>{
    let file=File::open(path)?;
    let buffer=BufReader::new(file);
    let mut numlines=0;
    for line in buffer.lines(){
        numlines+=1;
        if *righe>=numlines{
            let line=line?;
            println!("{}",line);
        }
    }
    Ok(numlines)
}