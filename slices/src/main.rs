fn main() {
    let mut s = String::from("Hello world"); 
    let res: usize = find_first_word(&s); 
    println!("The result is :{res}"); 


    let hello = &s[..5];
    let world = &s[6..]; 

    println!("{hello} and {world}"); 

    let res2 = find_first_word2(&s); 
    println!("The result is :{res2}"); 
    s.clear();
}

// fn learn_slicec

fn find_first_word(input : &String) -> usize {
    let s: &[u8] = input.as_bytes(); // string -> array of bytes
    for ( i , &item) in s.iter().enumerate(){
        if item == b' ' {
            return i ; 
        }
    }
    s.len()
}

fn find_first_word2(input : &String) -> &str {
    let s: &[u8] = input.as_bytes(); // string -> array of bytes
    for ( i , &item) in s.iter().enumerate(){
        if item == b' ' {
            return &input[..i] ; 
        }
    }
    &input[..]
}
