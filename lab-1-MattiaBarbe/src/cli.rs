use clap::Parser;

#[derive(Parser,Debug)]
pub struct Args{
    pub file:String,
    #[arg(default_value_t=10)]
    pub righe:i32,
}

pub fn parse_args()->Args{
    let  args=Args::parse();
    args
}
