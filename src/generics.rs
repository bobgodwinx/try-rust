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

    // - Generics give you a ton of flexibity 
    // - Code reduction 
    // - Zero additional run-time costs 
    // - Rust always infers the explicit type at compile time

    // - Here a simple enum with associated type
    // - one thing to notice is that in `Rust`
    // - the `case` keyword is not used in `enums` 
    let optionA = Options::OptionA(3.4);
    match optionA {
        Options::OptionA(a) => println!("a is {}", a),
        Options::OptionB(b) => println!("b is {}", b),
    }

    // - A more complex type 
    let optionB = Options::OptionB(vec![1, 2]);
    match optionB {
        Options::OptionA(_) => { 
            println!("LOL")
        },
        Options::OptionB(vec) => {
            for x in vec {
                println!(" element = {}", x);
            }
        }
    }

    let a_gen_fn = a_generic_func(3, 4);
    println!(" a_generic_func produces {}", a_gen_fn);
}

enum Options<T> {
    OptionA(T),
    OptionB(T),
}

// - Here `Rust` gets a little bit hairy with 
// - Generic `Constraints`.. in the example below
// - We are stating that `T` conforms to `Add`
// - that our `Output` is a `Type` of `T`
// - `Swift` has a better syntax in this case.. 
fn a_generic_func<T: std::ops::Add<Output = T>>(input_a: T, input_b: T) -> T {
    input_a + input_b
}

fn b_generic_func<T: std::ops::Add, U: T>(input_a: T, input_b: T) -> U {
    input_a + input_b
}

