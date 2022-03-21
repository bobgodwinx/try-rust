// Functions is just used to store block of code for re-use
pub fn run() {
    println!("Welcome to functions");
    greeting("Hello", "Bob");

    // Binding function 
    let get_sum = add(6, 7);
    println!("The sum is: {}", get_sum);

    // Closure are written with pipes 
    let add_num = |n1: i32, n2: i32| n1 + n2;
    println!("Addtion: {}", add_num(3, 4));

    // cool thing about closures is that add outside variables 
    // which is not possible in functions because it's block scoped
    // in Swift you have something called capture list { [weak self] ... }
    let local_variable = 40;
    let sub_num = |n4: i32, n5: i32| n4 - n5 + local_variable;
    println!("Subtraction and addition of a local variable: {}", sub_num(10, 6));

    // Multi-line closure
    let multi_line = |m1: i32, m2: i32| 
    if m1 == m2 { 
        m1 - m2 
    } else { 
        m2 - local_variable
    };
    // ends here.

    println!("Multi-line Closure: {}", multi_line(6, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}