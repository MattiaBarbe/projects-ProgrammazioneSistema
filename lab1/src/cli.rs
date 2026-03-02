use std::env::args;
pub fn parseArg()->Result<(String,i32),String>{
    let args: Vec<String> = args().collect();
    if args.len()<2{
        return Err("manca il nome del file csv".into());
    }    
    else{
        let path=&args[1];
        let mut n=10;
        OK((path,n))
    }
}