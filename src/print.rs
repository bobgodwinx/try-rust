use std::{str, i8};
use crate::print;

pub fn run() {
    println!("Hello from the print.rs file");
    let x: i32 = 5;
    println!("Number where x = {}", x);

    let b: String = "B is a string".to_string();
    let c: f32 = 1.290;

    // Basic formating 
    println!("{} where c is a number and it equals {}", b, c);

    // Positional Arguments 
    let vodafone: &str = "Vodafone";
    let crvsh: &str = "Crvsh develops the smart app";
    println!("Who develops the app for {0}?, I would say {1} for {0}", vodafone, crvsh);

    // Named Arguments 
    let maria_age: i8 = 8;
    let maria_birth_place: &str = "Andalusia";
    let maira_country = "Spain"; //infered as &str
    println!(
        "Maria's age {age}, and her birth place is {birth_place} which is in {country}", 
        age = maria_age, 
        birth_place = maria_birth_place, 
        country = maira_country
    );

    // Placeholder traits // Binary, Hex, Octal
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 2, 2, 2);

    // Debug traits 
    let an_array = ["me", "them"];
    println!("{:?} \n {:?} \n\n ", (100, true, "blabla"), an_array);

    for value in an_array {
        println!("The content {}", value);
        println!("index: {}", value);
    }

    let a_vector = vec![100, 300];

    for (index, element) in a_vector.iter().enumerate() {
        println!("Element at index: {}: is {:?} \n when multiplied by 3", 
        index, element * 3);
        if index == 1 {
            println!("we are at the end");
        }
    }
}