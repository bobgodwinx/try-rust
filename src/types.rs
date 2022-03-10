/*
 Primitive types 
 Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize
 Floats: f32, f64
 Boolean: (bool)
 Characters: (char)
 Tuples
 Arrays
 */

use std::primitive;

pub fn run() {
    // Default is "i32"
    let x = 1; 
    println!("Integer 32 bits: {}", x);
    
    // Default is "f64"
    let y = 0.5;
    println!("Float 64 bits: {}", y);

    // Add explicit type
    let c: i64 = 4888484858585;
    println!("Huge integer 64 bits: {}", c);

    // find max size of "i32" integer
    let i32_max = std::i32::MAX;
    println!("Max i32: {}", i32_max);

    // Boolean
    let is_active = true;
    let is_not_active: bool = false; 
    let is_greater_than = c > (y as i64);
    println!("is_active: {} and is_not_active is {:?}, is_greater_than: {2}", is_active, is_not_active, is_greater_than);

    // Char
    let a1 = 'a';
    println!("A single character: {}", a1);

    // Unicode
    let a2 = '\u{1F300}';
    println!("Show me an emoji: {}", a2);

    // String --- str is not a primitive type.. but just adding the example here. 
    // Default is &str
    let ab = "ab"; 
    println!("A string type: {}", ab);
    // Explicit Setting of String
    let bc: &str = "Hellow";
    println!("Print me an explicit string: {}", bc);
}