//use std::env;
//mod print;
mod variables;
fn main() {
    let x: i32;
    x = 4;
    println!("Hello, world! hello main.rs file");
    println!("Where x = {}", x);
    //print::run();
    variables::run();
}
