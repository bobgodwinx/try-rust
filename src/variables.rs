pub fn run() {
    println!("Welcome to valiable"); 
    
    // fixed cannot change. immutable
    let name = "Bob";
    let age = 37; 
    println!("My name is {} and I am {}", name, age); 

    // mutable, can change 
    let mut age1 = 38; 
    println!("My other name is {} and I am {}", name, age1); 
    age1 = 39;
    println!("I am now {}", age1); 

    // const cannot change and immutable 
    // good practice is to use CAPS during declaration od const
    const ID: i32 = 005;
    println!("ID {}", ID);
    let (first_name, last_name, age) = ("Bob", "Obi", 40);
    println!("Firstname: {} Lastname: {} Age: {}", first_name, last_name, age)
}