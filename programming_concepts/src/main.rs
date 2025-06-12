fn main() {

    //immutable variables 
    let age = 24 ; 
    println!("Hello, world! {age}");
    println!("Hello, world! {}" , age);

    //to make mutable 
    let mut abc = 323;
    println!("{abc}"); 
    abc = 89 ;  
    println!("{abc}"); 

    //const keyword 
    const PI : f64 = 3.14 * 2.0 ; // const use karte wkt type dena hi padega 
    println!("{PI}"); //  const can be declared in global scope
    const ONE_HOUR : u32 = 60 * 60 * 30 ; 
    println!("{ONE_HOUR}");    


    //shadowing in RUST 
    let apples = 49 ; 
    let apples =  apples* 99 ; // the second declaration shadows the first one 
    println!("{}" , apples); 

}
// i32 for integer , u32 for positive integers
// dfladkf