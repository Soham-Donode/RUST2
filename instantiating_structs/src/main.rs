#[derive(Debug)] // for printing debug value of structs
struct Rectangle {
    width:u32 , 
    height:u32
}


fn main() {
    let rect = Rectangle {
        width : 32 , 
        height: 50 , 
    };

    let area = calculate_area(&rect); 
    println! ("The area of rectangle is {}" , area); 
    println! ("The Rectangle is {rect:?}"); 
}

fn calculate_area( rect : &Rectangle) ->u32 {
    rect.width * rect.height
}
