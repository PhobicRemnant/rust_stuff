use std::collections::HashMap;

/*  ---Exercise 1---
Given a list of integers, use a vector and return the mean (the average value),
median (when sorted, the value in the middle position), and mode (the value that
occurs most often; a hash map will be helpful here) of the list.
*/

fn exercise_1(){
    
    let v = vec![1,1,3,4,2,2,1];
    let mut sort_v = v.clone();
    sort_v.sort();
   
    println!(" The vector: {:?}",v);
    println!(" The sorted vector: {:?}",sort_v);

    println!("The mean is   : {}.", mean(&v));
    println!("The median is : {}.", median(&v));
    println!("The mode is   : {}.", mode(&v));
    
    
}

fn mean(v:&Vec<i32>) -> f64{

    let mut mean:f64 = 0.0;

    // Iterate over vector to sum values
    for i in v{
        mean += *i as f64;
    }

    // Calculate the average
    mean = mean/ v.len() as f64;

    // Return the result
    mean
}

fn median(v: &Vec<i32>) -> i32 {

    let mut sorted_vec = v.clone();

    sorted_vec.sort();
    
    let  median = sorted_vec.len()/2;
    
    sorted_vec[median]
}


fn mode(v:&Vec<i32>)-> i32{

    // Create new hash map 
    let mut vector_map: HashMap<&i32,i32> = HashMap::new();

    // Add the count of elements within the vector into the hash map
    for element in v{
        let count = vector_map.entry(element).or_insert(0);
        *count += 1;
    }

    // Get the greater value

    let mut mode = 0;
    let mut count = 0;
    for (key,value) in vector_map{
        if(value > count){
            mode = *key;
            count = value;
        }
    }

    // Return mode 
    mode
}

/* ---Exercise 2---
Convert strings to pig latin. The first consonant of each word is moved to the 
end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start
 with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
Keep in mind the details about UTF-8 encoding!
*/


/* --- Exercise 3---
Using a hash map and vectors, create a text interface to allow a user to add employee 
names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir
to Sales.” Then let the user retrieve a list of all people in a department or all people
in the company by department, sorted alphabetically
*/
fn main() {

    println!("Exercise 1:");
    exercise_1();

}
