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

    // Pass value by reference so that the variable can be used still within scope
    print_user(&user1);
    
    user1.username = String::from("tacolord");

    print_user(&user1);

}

fn main() {
    
    // Test the User struct
    //user_struc_test();

    let user = build_user(String::from("Aname"), String::from("AnEmail@mail.com"));

    print_user(&user);

}
