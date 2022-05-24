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

// This is how you add lifetime to a `struct` that has reference types 
#[derive(Debug)]
struct School<'a> {
    address: &'a str,
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
    