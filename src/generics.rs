// - Generics in `Rust` works pretty similar to 
// - other programming languages. like `Swift`
// - Below you can see the convention of using a `T` 
// - used to represent the unknown `Type`
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    println!("Welcome to Rust generics");

    let a = Point { x: 100, y: 200 };
    let b = Point { x: 10.0, y: 123.0 };

    println!("a = {:?}", &a);
    println!("b = {:?}", &b);
}