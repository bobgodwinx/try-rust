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

    let b_gen_fn = b_generic_func(4, 6);
    println!("b_generic_func {}", b_gen_fn);

    let c_gen_fn = c_generic_func(3, 3);
    println!("c_gen_fn {}", c_gen_fn);

    let d_gen_fn = d_generic_func(4, 1);
    println!("d_gen_fn {}", d_gen_fn);

    let a_bob = People {
        name: "Bob",
        other_name: "another",
    };

    let my_string1 = a_bob.give_me_a_string("hello", " what's up?");
    println!("BobCustomTrait in action {}", my_string1);
    let my_string2 = do_some_work1(&a_bob);
    let my_string3 = do_some_work2(&a_bob);
    println!("BobCustomTrait in action {}", my_string2);
    println!("BobCustomTrait in action {}", my_string3);

    let a_pupil = Pupils {
        name_t: "bob",
        age_u: 20,
    };
    // - now let's log the generic traits 
    a_pupil.log_variables();

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
// - That's quite nasty of `Rust` 
// - As you can notice you use a `+` sign to add more and more `Constraints`
fn b_generic_func<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T>>(input_a: T, input_b: T) -> T {
    input_a - input_b
}

// - That's quite nasty of `Rust` 
// - As you can notice you use a `+` sign to add more and more `Constraints` + more + more
fn c_generic_func<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>(input_a: T, input_b: T) -> T {
    println!("input_a {:?}", input_a);
    println!("input_b {:?}", input_b);
    input_a - input_b
}

// - But we can use `where` just like in `Swift` 
// - to arrange the code better.. So here I am beginning 
// - to ask myself how much did `Swift` copy `Rust` ? Yeah 🙈 let's use `where`
fn d_generic_func<T>(input_a: T, input_b: T) -> T 
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug {
    println!("input_a {:?}", input_a);
    println!("input_b {:?}", input_b);
    input_a - input_b
}

// - Going with traits
// - You should see `traits` as `Swift` `protocols`
// - It's just another way of saying characteristics
// - that are associated to an object
trait  BobCustomTrait {
    fn give_me_a_string(&self, a: &str, b: &str) -> String;
}
// - This is a Generic `T` but `Constrained` with the `where` clause
fn do_some_work1<T>(some_object: &T) -> String 
where T: BobCustomTrait {
    some_object.give_me_a_string("d", "q")
}
// - Here  `do_some_work2` just recieves a `BobCustomTrait`
fn do_some_work2(some_object: &dyn BobCustomTrait) -> String {
    some_object.give_me_a_string("c", "d")
}

struct People {
    name: &'static str,
    other_name: &'static str,
}
// - Implementation of `BobCustomTrait` for `People` struct
// - in `Swift` terms `People` conforming to `BobCustomTrait`
impl BobCustomTrait for People {
    fn give_me_a_string(&self, a: &str, b: &str) -> String {
    let value = format!("{} {} {} {}", a, b, self.name, self.other_name); 
    return String::from(value);
    }
}

// - Now let's takes this further to Generic Traits 

struct Pupils<T, U> {
    name_t: T,
    age_u: U,
}
// - Here `Rust` will only allow you to use 
// - `Debug` Traits on the implementation but 
// - not on the struct it's self. Which is kind strange
impl <T, U> Pupils<T, U> 
where T: std::fmt::Debug, 
      U: std::fmt::Debug  { 
    fn log_variables(&self) {
        println!("{:?}, {:?}", self.name_t, self.age_u);
    }
    
}