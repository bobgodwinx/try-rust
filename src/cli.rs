use std::env;

pub fn  run() {
    println!("welcom Rust command line interface");
    let args: Vec<String>= env::args().collect();
    let name = "Bob";
    let status = "100%";
    let command = args[1].clone();

    //println!("Args: {:?}", args);

    if command == "hellow" {
        println!("Hi {} how are you ?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Not a valid command");
    }
}