// Primitive str are fixed-length and immutable 
// Strings are mutable and growable 
pub fn run() {
    // Fixed-length Strings 'immutable'
    let fixed_string = "Immutable";
    println!("Here is a fixed length: {}", fixed_string);
    
    // Mutable Strings 
    let mut mutable_string = String::from("Mutable");
    println!("Here is a mutable string: {}", mutable_string);
    println!("Length of string: {}", mutable_string.len());
    // Push 
    mutable_string.push('s');
    println!("Here is a mutable string after push: {}", mutable_string);
    println!("Length of string after push: {}", mutable_string.len());
    // Push str
    mutable_string.push_str(" -> Adding more strings");
    println!("Here is a mutable string after push_str: {}", mutable_string);
    println!("Length of string after push_str: {}", mutable_string.len());

    // Capacity in bytes
    println!("Capacity in bytes: {}", mutable_string.capacity());

    // Empty
    println!("Is string Empty: {}", mutable_string.is_empty());

    // Contains
    println!("Contains `M utable`: {}", mutable_string.contains("Mutable"));

    // Replace 
    println!("Replace a string or a subset of a string -> {}", mutable_string.replace("Mutable", "Replaced Mutable")); 

    //Looping Strings 

    let mut phrase = String::from("Hello there from London");

    for word in phrase.split_whitespace() {
        println!("value = {}", word);
    } 

    for (index, element) in phrase.split_whitespace().into_iter().enumerate() {
        println!("index = {} word = {}", index, element);
    }

    // Create string with capacity 
    let mut string_capacity = String::with_capacity(10);
    string_capacity.push('q');
    string_capacity.push('4');
    println!("String Capacity: {}", string_capacity);
    string_capacity.pop();
    println!("String Capacity: {}", string_capacity);
    
    //Assertion testing 
    assert_eq!(string_capacity, "q");
    assert_eq!(1, string_capacity.len());
}