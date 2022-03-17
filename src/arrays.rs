use core::num;

// Arrays are fixed in rust and has the same elemets as data types 
// same as in `Swift` when declared with a `let` 
pub fn run() {
    println!("Welcome to arrays");

    let nums: [i32; 5] = [0, 1, 2, 3, 4];
    // Print all
    println!("nums: {:?}", nums);
    // Get a single value
    println!("first index: {}", nums[0]);
    // Get the length
    // but you can only do this for mutable arrays
    let mut num_arr = [1, 2, 3, 9];
    println!("Array lenght: {}", num_arr.len());
    // arrays are stack allocated
    println!("Memory: {} bytes", std::mem::size_of_val(&num_arr)); 
    // Get Slice
    let slice: &[i32] = &num_arr;
    println!("Slice: {:?}", slice);
    // SLice in a range
    let slice2: &[i32] = &num_arr[0..2];
    println!("Slice 2: {:?}", slice2);
}