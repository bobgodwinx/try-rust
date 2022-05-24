pub fn run() {
    println!("Hello Lifetimes");
    
    // let r: i32 = 7;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("Now we have a dangling referenc r: {}", r);

    // &i32 a reference
    // 'a i32 a reference with an explicit lifetime 
    // &'a mut i32 // a mutable reference with an explicit lifetime  
    let a = 3;
    let x = get_int_ref(&a);
    println!("x : {}", &x);
    let address = "1a Rust avenue";
    let city = "Berlin";
    let school = School::new(address, city);
    println!(" School looks like this: {:?}", &school);

}
// the code below is as same as `Rust` hides the 
// `'a` behind the scenes when it can and when it cannot 
// the compiller just yells at you. 
/*
fn get_int_ref<'a>(param: &'a i32) -> &'a i32 {
    return param;
}
*/
fn get_int_ref(param: &i32) -> &i32 {
    return param;
}

// here is another example of how the `Rust`
// lifetime can get crazy This code will never 
// compile in `Rust` and compiler yelling this:
//
// missing lifetime specifier
// this function's return type contains a borrowed value, 
// but the signature does not say whether it is borrowed 
// from `param_1` or `param_2`rustcE0106
/*
fn get_string_ref(param_1: &str, param_2: &str) -> &str {
    if param_1 == "hello world" {
        return param_1;
    } else {
        return param_2;
    }
}
*/
// here is what the compiler did with the above code 
/*
fn get_string_ref<'a, 'b>(param_1: &'a str, param_2: &'b str) -> &'a str {
    if param_1 == "hello world" {
        return param_1;
    } else {
        return param_2;
    }
}
*/
// here is the fix `<'a, 'b: 'a>` or just make it `<'a>` and `param_2` will 
// have to use `'a` as follows `param_2: &'a str` eith of the solution works!
// Lifetime is just a way `Rust` protects our reference types from accessing 
// garbage memory.  
#[allow(dead_code)] // don't do this in production I am just showing how to add macros
fn get_string_ref<'a, 'b: 'a>(param_1: &'a str, param_2: &'b str) -> &'a str {
    if param_1 == "hello world" {
        return param_1;
    } else {
        return param_2;
    }
}


// This is how you add lifetime to a `struct` that has reference types 
#[derive(Debug)]
struct School<'a> {
    #[allow(dead_code)] // don't do this in production I am just showing how to add macros
    address: &'a str,
    #[allow(dead_code)] // don't do this in production I am just showing how to add macros
    city: &'a str
}
// `impl` is quite weird and people in the `Rust` community voiced
// concerns over this weird syntax which I find funny 
// TL;DR you have to use `'a` to denote the lifetime in the `impl`
// Another thing to notice is the way `Rust` places the angle brackets 
// at the back of the keyword. 
impl<'a> School<'a> {
    fn new(address: &'a str, city: &'a str) -> School<'a> {
        Self { address: address, city: city }
    }
}
    