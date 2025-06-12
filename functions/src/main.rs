fn main() {
    let a = 10; // statement - instr which dosent return anything
    println!("Hello, world!"); // expression 
    add_two(10, 90, true); //expression - returns something

    //expressions do not include ending semicolons
    let y = {
        let mut x = 89;
        x += 1;
        x // return value
    };

    println!("{y}");

    let k = subtract(10, 8);
    println!("The value of subtraction is : {k}");
}

fn add_two(x: i32, z: i32, y: bool) {
    // function name should be written in snake_case
    println!("my second func {} {y}", x + z);
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y // return x-y ; is equivalent to x-y 
}
