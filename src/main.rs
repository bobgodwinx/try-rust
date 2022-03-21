//use std::env;
//mod print;
//mod variables;
//mod types;
//mod strings;
//mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
mod pointers;


const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    println!("Welcome to Rust:  {}", VERSION);
    println!("Hello, world! hello main.rs file");
    //print::run();
    //variables::run();
    //types::run();
    //strings::run();
    //tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run();
    //loops::run();
    //functions::run();
    pointers::run();
}
