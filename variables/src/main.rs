fn main() {

  // by default the variables in rust are immutable 
  // to make them mutable we add mut after let 

  //variable declaration
   let mut a = 79; 
   a = 90 ; 
   println!("The value of a is : {a} "); 



  //  let b = true ; 
  //  match b {
  //      true => todo!(),
  //      false => todo!()
  //  }

  enum Direction {
    North, 
    South, 
    East,
    West,
  }

  let dir = Direction::West; 

  match dir {
      Direction::East => print!("The direction is east "),
      _=> print!("Something else")
  }

   let number = 2; 
   match number {
       1 => print!("Your number is 1"),
       2 => print!("Your number is 2"),
       3 => print!("Your number is 3"),
       _ => print!("Its something else")
   }
}
