
// -----------------------------------------------------
//  Section 5.1 - Defining and instantiating structs
// -----------------------------------------------------

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn print_user(user:&User){
    // Orderly print to the terminal the user's properties
    println!("");
    println!("Username      :{}",user.username);
    println!("Email         :{}",user.email);
    println!("Log in count  :{}",user.sign_in_count);
    println!("Status        :{}",user.active);
    println!("");
}

fn build_user(username: String ,email: String) -> User {
        User{
            username: username,
            email: email,
            sign_in_count: 1,
            active: true,
        }
}

fn user_struc_test(){
   
    let mut user1 = User { 
        username: String::from("someusername"), 
        email: String::from("somename@mail.com"), 
        sign_in_count: 0, 
        active: true };

    println!("");
    println!("After init.");
    // Pass value by reference so that the variable can be used still within scope
    print_user(&user1);
    
    user1.username = String::from("tacolord");
    println!("");
    println!("After modifying name.");
    print_user(&user1);

}

struct Color(u32,u32,u32);

// -----------------------------------------------------
//  Section 5.2 - An Example Program Using Structs
// -----------------------------------------------------

fn square_area_calc(){
    let width1 = 30;
    let height1 = 50;
    
    println!("The area of the rectangle is {} square pixels.",
                area(width1, height1));
    println!("");
}

fn area(width: u32, height: u32) -> u32 {
        width * height
}

fn square_area_tuples_calc(){
    let rect1 = (30,50);

    println!("The area of the rectangle is {} square pixels.",
    area_tuple(rect1));
    println!("");
}

fn area_tuple(dimensions:(u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn main() {

    // Section 5.1 code
    println!("");
    println!("Section 5.1:");
    println!("");
    // Test the User struct
    user_struc_test();

    let user = build_user(String::from("Aname"), String::from("AnEmail@mail.com"));

    println!("");
    println!("Making a custom user using a Struct");
    println!("");
    print_user(&user);


    println!("");
    println!("Making a custom color using a tuple struct");
    println!("");
    
    let color = Color(125,23,255);
    
    println!("R:{} G:{} B:{}",color.0, color.1, color.2);

    // Section 5.1 code
    println!("");
    println!("Section 5.2:");
    println!(""); 

    square_area_calc();
    
    square_area_tuples_calc();
}
