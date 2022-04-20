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
//mod pointers;
//mod structs;
//mod enums;
//mod cli;
//mod ownership;
//mod clone;
mod traits;
mod model { 
    pub mod employee;
    pub mod company;
}
//pub use crate::employee::Employee;




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
    //pointers::run();
    //structs::run();
    //enums::run();
    //cli::run();
    //ownership::run();
    //clone::run();
    traits::run();
    //let employee_01 = Employee { first_name: String::from("Bob"), last_name: String::from("ob
    //i"), age: 23};
}
