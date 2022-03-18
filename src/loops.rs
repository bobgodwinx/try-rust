// loops are used to iterate until a condition is met
// But you also have infinite loops

pub fn run() {
    println!("Welcome to loops");

    let mut count = 0;

    // Infinite Loop
    loop {
         count += 1;
         println!("Number: {}", count);

        // this could go ahead if we don't have a condition
        if count == 10 {
            break;
        }
    }

    // While loop (FizzBuzz)
    count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0  {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("Number: {}", count);
        }

        count += 1;
    }

    // For range 

    for x in 0..100 {
        if x % 15 == 0 {
            println!("For: FizzBuzz");
        } else if x % 3 == 0  {
            println!("For: Fizz");
        } else if x % 5 == 0 {
            println!("For: Buzz");
        } else {
            println!("For: Number: {}", x);
        }
    }
}