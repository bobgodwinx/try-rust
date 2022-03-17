// Vectors are just resizable array 
use core::num;

// Arrays are fixed in rust and has the same elemets as data types 
// same as in `Swift` when declared with a `let` 
pub fn run() {
    println!("Welcome to vectors");

    let nums: Vec<i32> = vec![0, 1, 2, 3, 4];
    // Print all
    println!("nums of vectors: {:?}", nums);
    // Get a single value
    println!("first index: {}", nums[0]);
    // Get the length
    // but you can only do this for mutable arrays
    let mut num_arr:Vec<i32> = vec![1, 2, 3, 9];
    println!("Vector lenght: {}", num_arr.len());
    // arrays are stack allocated
    println!("Memory: {} bytes", std::mem::size_of_val(&num_arr)); 
    // Get Slice
    let slice: &[i32] = &num_arr;
    println!("Slice: {:?}", slice);
    // SLice in a range
    let slice2: &[i32] = &num_arr[0..2];
    println!("Slice 2: {:?}", slice2);
    // push and pop
    num_arr.push(12);
    println!("pushed 12 into nums of vectors: {:?}", num_arr);
    num_arr.push(100);
    println!("pushed 100 into nums of vectors: {:?}", num_arr);
    num_arr.pop();
    println!("pop 100 into nums of vectors: {:?}", num_arr);
    // append other vectors 
    let mut num_arr2:Vec<i32> = vec![1, 2, 3, 9];
    num_arr.append(&mut num_arr2);
    println!("appended num_arr2 into nums of vectors: {:?}", num_arr);

    // loop through vectors 

    for element in nums.iter() {
        println!("Num: {}", element);
    }

    for (index, element) in num_arr.iter().enumerate() {
        println!("Num index: {} = {}", index, element);
    }

    println!("NOT-Mutated num_arr: {:?}", num_arr);
    for element in num_arr.iter_mut() {
        *element *= 3;
    }

    println!("Mutated num_arr: {:?}", num_arr);
}