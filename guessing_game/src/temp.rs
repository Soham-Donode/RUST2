use std::io; 
fn main(){
    let mut a = String::new();
    io::stdin().read_line(&mut a ).expect("no");
    let a : u32 = a.trim().parse().expect("failed"); 
    println!("{a}"); 
}