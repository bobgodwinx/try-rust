// Tuples are just a group of values 
// it doesn't have to be the same type 
// like in an array or vector 
// 12 elements max

pub fn run() {
    println!("Welcome to tuples");

    let person: (&str, &str, i8) = ("Bob", "Obi", 20);
    println!("The person tuple: {:?}", person);
    println!("I am {} and my surname is {}, I am {} years old", person.0, person.1, person.2);
}