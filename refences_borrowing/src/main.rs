fn main() {
  let mut s= String::from("Soham Donode");
  let lt = calc_len(&s);  // instead of ownership transfer a reference is made 
  println!("{lt}"); 

  appen(&mut s); 
  println!(" new string \n {s}"); 
}

fn calc_len( s : &String ) -> usize { // taking ref as paramerter is called borrowing
  let ln = s.len(); 
  ln 
}

fn appen(s: &mut String) { // mutable references can change original value 
  s.push_str(" is a good boy"); 
}

// a refences's scope starts from where it is introduces a dncontinues through the last time that ref is used