fn main() {
    explain_if();
    if_as_an_expression();
    learn_loops();
    labeling_loops();
    learning_while();
    learning_for();
}

fn explain_if() {
    let number = 3;
    if number < 5 {
        println!("{number} is less than 5");
    } else {
        println!("{number} is greater than 5");
    }

    if true {
        println!("Hello world");
    }
}

fn if_as_an_expression() {
    let a = 12;
    let x = if a < 10 {
        10
    } else if a > 79 {
        32
    } else {
        30
    }; // kinda ternary op
    println!("{x}");
}

fn learn_loops() {
    let mut num = 1;
    let result = loop {
        println!("value of num is {num}");
        num += num;
        if num == 8 {
            break 823; // continue is also available
        }
    }; 

    println!("The value of result is {result}"); 
}


fn labeling_loops(){
    let num = 10 ; 
    let res = 'my_loop: loop {
        if num == 10 {
            break 20 ; // break the innermost loop 
        }
        loop {
            if num > 80 {
                break 'my_loop 60;// this break statement will break the loop labeled as name 
            }
        }
    }; 
    println!("The value of res is : {res}"); 
}

fn learning_while(){
    let mut a =  0 ; 
    while a < 10 {
        println!("{a}"); 
        a+=1 ; 
    }
}

fn learning_for(){
    let arr: [i32; 6] = [1 ,2 ,3 ,4 ,5 ,89]; 
    for ele in arr {
        println!("{ele}"); 
    }

    for x in (1..=67).rev(){
        print!("{x} "); 
    }
}