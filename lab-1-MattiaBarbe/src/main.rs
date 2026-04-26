mod cli;
mod csv;
fn main(){
    let args=cli::parse_args();
    match csv::open(&args.file, &args.righe){
        Ok(o)=>println!("righe lette {}",o),
        Err(e)=>println!("{}",e),
    }
}