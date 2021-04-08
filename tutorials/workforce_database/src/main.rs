use std::{collections::HashMap, io};

fn main(){

    // Create the main "database" where we will store our departments and employees
    let mut workforce:HashMap<String,Vec<String>> = HashMap::new();

 
    // Create reader to handle user input
    let reader = io::stdin();

    loop {

        // Clear terminal 
        print!("{esc}c", esc = 27 as char);

        // Buffer string
        let mut input = String::new();
        
        // Options display
        display_options();
        
        // User input selection
        reader
        .read_line(&mut input)
        .expect("Failed to read");

        // Convert string to number
        let option: u32 = match input.trim().parse() {
            Ok(option) => option,
            Err(_) => continue
        };

        match option {
            1 => enter_employee(&workforce),      // Add employee to database
            2 => display_workforce(),   // Show the worforce in an ordely manner
            3 => break,                 // Break loop and exit program
            _ => println!(""),
        }

        
    }

    println!("");
    println!("Thank you for using the workforce database");
    println!("");
    
}

fn display_options(){
    println!("Welcome to the workforce database");
    println!("");
    println!("(1) Enter employee in department.");
    println!("(2) Display workforce ordered list.");
    println!("(3) Exit database.");
    println!("");
}

fn enter_employee(database: &HashMap<String,Vec<String>>){
    
    // Display example to user
    println!("");
    println!("Enter employee's name and department.");
    println!("Example:");
    println!("  Add John to Sales.");
    println!("");

    // Read input
    let mut line= String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Invalid input, please look at example.");
    
    let new_entry: Vec<&str> = line.split_whitespace().collect();
    // Order input as a 4  

}

fn display_workforce(){
    println!("");
    println!("Display stuff");
    println!("");
}
