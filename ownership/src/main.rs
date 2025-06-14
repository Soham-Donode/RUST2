fn main() {
    let st = "Hello world"; // this is a string literal
    let mut s = String::from("Hello"); // this is a String
    s.push_str("world");
    println!("S = {s}");

    second();
    takes_ownership(s);

    let st = gives_ownership();
    println!("{st}");
    // s1 is associated with 3 values ( pointer to the acutal memory in heap , lenghth , capacity   )

    let abc = String::from("my third string"); 
    let abc = takes_and_gives_back(abc); // shadowing
}
fn takes_ownership(s: String) {
    println!("I am inside the function{}", s);
}

fn gives_ownership() -> String {
    let st = String::from("HEllO");
    st
}

fn takes_and_gives_back(s : String) -> String {
    println!("The value of s is {s}"); 
    s 
}
fn second() {
    let s1 = String::from("i am s1");
    let s2 = s1.clone(); // copy in heap is created , without clone the s2 = s1 movies data from s1 to s2 
    print!("value of s1 {s1} and value of s2{s2}");
}


