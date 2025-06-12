use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {


    println!("Guess the number  game starts!");


    let secret_number = random_range(1..=100);
    loop {
        let mut guess = String::new();

        //standart input in rust 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input"); // providing a mutable reference


        let guess: u32 = match guess.trim().parse() { // variable shadowing + variable type conversion 
            Ok(num) => num , 
            Err(_)=>{
                println!("Please enter a valid input"); 
                continue;
            }
        };  

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("too small"),
        }
    }
}

// semantic versioning major.minor.patch
