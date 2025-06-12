fn main() {
    
    let num : i8 = 89 ; // when we explicitly mention dt its called type annotation 
    let a = 2323_i128; 
    let b = 100_000_000; // underscores are ignored 
    println!("{b}"); 

    // scalar types - single value 
    // integers , floating point  , bool and char 


    //integers
    let c = 0xff ; //hexadecimal
    let d = 0o77 ; //octal
    let e = 0b1111 ; //binary
    let f = b'A';  // return ascii value 
    
    //floating pointn 
    let k = 3.0 ; 
    let x: f32 = 8.0 ; 

    // boolean type 
    let bo = true ;
    let fl = false ; 

    // character type 
    let char1 = '1'; 
    let char2 = 'A'; 

    // Compound types - tuples and arrays 

    // tuples 
    let tup: (i32,f64,u8) = (500,6.4,1); 
    let soham: (&'static str, i32, i32) = ("Soham" , 19 , 10000000); // destructuring 
    let (x,y,z) = soham; 
    println!("Name : {x} , age :{y} , salaray : {z} ");
    println!("Name : {} , age :{} , salaray : {} " , soham.0 , soham.1 , soham.2);

    let tupl = (); //unit tuple


    //arrays 
    let ar: [i32; 6] = [1,2,3,4,5,6];
    let br =[10;5]; // ek array 5 size ka with all elements as 10




}
// ascii - american standard code for international interchange 