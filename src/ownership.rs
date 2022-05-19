use std::{ops::{DerefMut, Deref}, clone};

#[allow(unused_variables)] //This is to highlight warnings 

pub fn run() {
    println!("About ownership and borrowing");
    let var_a = String::from("Hello");
    let var_b = var_a;
    // - This will Error because it's "BORROWED" meaning `var_a` assigned to `var_b`
    // - `Rust` literally kills `var_a` asap assigned to `var_b`
    // println!("{}", var_a); 😡

    //STACK
    // - Fast memory creation and retrieval ... speed 
    // - These are all FIXED size!!!
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.0;
    let stack_bool: bool = true;
    let stack_char :char = 'a';

    // - The above will be deallocated at the end of the scope. 
    // - A Scope is anything that has a curly bracket. 
    // - Take a look at the `if` scope below

    if stack_i8 == 3 {
        let inside_scope_var = 9;
        println!("{}", inside_scope_var);
    }

    //println!("{}", inside_scope_var);  // Error because it's out of scope 

    //HEAP
    // - Flexibilty 
    // - Memory that can grow in size (Vector, Hashmap, Strings)
    // - Runtime performance cost (slow speed) 
    // - Memory can live beyond the scope that created it
    // - Memory is automatically recaptured when the last OWNER goes out of scope (deallocated)
    let heap_vector: Vec<i8> = Vec::new(); // same as `vec![]`
    let heap_string: String = String::from("hello");
    let heap_i8: Box<i8> = Box::new(30);

    // - `Rust` does a `Copy on Write` because it's so cheap to do. 
    // - So bellow code will not error. Same thing in `Swift` it's a copy on write
    let stack_i8_2 = stack_i8;
    println!("stack_i8: {}", stack_i8);
    println!("stack_i8_2: {}", stack_i8_2);

    let heap_i8_2 = heap_i8;
    println!("print_i8_2: {}", heap_i8_2);
    // - This will error because it has already been assigned to another person
    // - meaning only one OWNER at a time for HEAP based variables.
    // println!("stack_i8_2: {}", heap_i8); 😩 😡 😩 😡

    // - There is though another way to make two objects point to a single space in memery
    // - by using the `&` meaning borrowing 🤯
    let heap_i8_3 = &heap_i8_2;
    println!("print_i8_2: {}", heap_i8_2);
    println!("print_i8_3: {}", heap_i8_3);

    // - Alternatively you can clone a variable
    // - But this will make the second varible completely different from the first.
    let heap_i8_4 = heap_i8_3.clone(); // This is memory expensive!!!
    println!("print_i8_4: {}", heap_i8_4);
    println!("print_i8_3: {}", heap_i8_3);

    // - Scope based memory cleanup
    let stack_f64: f64 = 1.0;
    // - `stack_do_some_thing` will copy `stack_f64` into `param`
    // - so it a whole new variable 
    stack_do_some_thing(stack_f64);
    println!("Original `stack_f64` {}", stack_f64);


    // Even though I declared this as a `mut`
    // the `heap_do_some_thing` function cannot
    // change because it is passed into it as
    // reference. So you cannot do anything 100% 
    // guarranteed!!!
    let mut heap_f64: Box<f64> = Box::new(6.0);

    // - The `heap_do_some_thing` BORROWS `heap_f64`
    // - `Rust` makes sure you cannot do anything stupid
    // - After the method finishes runing it will return OWNERSHIP back to the original OWNER
    // - There can be One and Only one, OWNER of a memory at a time. 
    heap_do_some_thing(&heap_f64);
    println!("Pretty same value not updated `heap_f64` {}", heap_f64);

    // to be able to mutate the value you 
    // pass it as a non reference. 
    // meaning derefering it
    heap_f64 = heap_do_some_thing_change_param(heap_f64);
    println!("Now the value has been changed and returned back to us: {}", heap_f64); 

    // - Strings are always in the heap 
    let some_string: String = String::from("Hello");
    // - You can create a reference to a piece of memory
    // - it never owns it in the memory but borrows it literally.
    let some_str: &str = "Hellow me"; // this is a better practise! Always use string slices 
    // - as you can see below you have to borrow `some_string` 
    some_string_str(&some_string, &some_str);
    println!("{} {}", some_string, some_str); 

    // - When you BORROW an object you CANNOT mutate it
    // - Only the OWNER can mutate it's OWN value and there can 
    // - be only one single OWNER at a time.

    let var_a = String::from("Owned by var_a");
    let mut var_b = &var_a;
    // - Now if `var_b` tries to modify the 
    // - referenced value from `var_a`
    // - you get error 👇🏾 "cannot borrow `*var_b` as mutable, as it is behind a `&` reference"
    //  var_b.push('c');

    let mut var_c = String::from("Owned by var_c");
    let mut var_d = &var_c;
    let var_f = var_c;
    // - below you still get a compiler error because 
    // - `var_f` is an immutable reference which has 
    // - BORROWED `var_c`. Even though `var_c` has the 
    // - power to modify it's own value.. `Rust` will NOT 
    // - allow it because it's being refrenced by a `var_f`👇🏾  
    // - var_c.push('d');

    let mut var_g = String::from("Owned by var_g");
    let mut var_h = &var_g;
    let mut var_k = &var_g;
    // - `Rust` will allow this 👇🏾 because all references are mutable
    var_g.push('g');
    println!("{}", &var_g);
 }

 fn stack_do_some_thing(mut param: f64) {
     param += 10.9;
     println!("In stack with {}", param);
 }

 fn heap_do_some_thing(mut param: &Box<f64>) {
     // this how you reference the rawValue 
     // from the boxed property with `*`
    //let unboxed_param: f64 = *param; //error
    //let calculate = unboxed_param * 5.0; // error 
    //param = Box::new(calculate); // error
    
    println!("In heap with {}", param);
 }

 fn heap_do_some_thing_change_param(mut param: Box<f64>) -> Box<f64> {
    // this how you reference the rawValue 
    // from the boxed property with `*`
    let unboxed_param: f64 = *param; // dereferenced
   let calculate = unboxed_param * 5.0;
   param = Box::new(calculate); // works because param was dereferenced
   
   println!("In heap with updated and dereferenced param: {}", param);
   return param
}

 fn some_string_str(param_a: &String, param_b: &str) {
     println!("some_string_str: {} {}", param_a, param_b);
 }



