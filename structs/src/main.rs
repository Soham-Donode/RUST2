struct User {
    // follows Pascal cse - first word capital
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8 , u8 , u8);
struct Point(u8 , u8 , u8);
fn main() {
    let user_1 = User {
        //instance
        email: String::from("Soham213@gamil.com"),
        username: String::from("Soham"),
        active: true,
        sign_in_count: 77,
    };

    println!("The name of user is : {}", user_1.username);
    println!("The active of user is : {}", user_1.active);

    // user_1.username = String::from("somthing else");
    // println!("The name of user is : {}" ,user_1.username);

    let user_2: User = build_user(String::from("Sohamd"), String::from("23423@gmail.com"));

    let user_3 = User {
        email : String::from("hello"),
        ..user_1  // other fields are copied from user 1 , user1 nhi use kar sakte abb , because value is moved 
    };

    
    println!("The name of user3 is : {}", user_3.username);



    let red: Color = Color (100,0,0); 
    set_bg2(red);

}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,// sirf username likh sakte hain 
        email, // if key is same 
        active: true,
        sign_in_count: 0,
    }
}

// tuple structs

fn set_bg( color : (u8 , u8 , u8)){
    println!(
        "Setting Background color R={} , G ={} , B = {}" , color.0 , color.1 , color.2
    ); 
}
fn set_bg2( color : Color){
    println!(
        "Setting Background color R={} , G ={} , B = {}" , color.0 , color.1 , color.2
    ); 
}
