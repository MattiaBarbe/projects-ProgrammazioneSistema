use std::fmt::{Display, Formatter};

//utilizzando derive implementa il metodo in automatico alla struct
#[derive(PartialEq,Debug)]
struct  S{
    v:i32
}

/*impl PartialEq for S {
    fn eq(&self, other:&Self)->bool{
        self.v==other.v
    }
}*/

impl Display for S  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"S{{v:{}}}",self.v)
    }
}
fn main() {
    let s1=S{v:1};
    let s2=S{v:2};
    if s1==s2{
        println!("sono uguali");
    }
    else {
        println!("sono diversi");
    }
    println!("{}",s1);
    println!("{}",s2);
}
