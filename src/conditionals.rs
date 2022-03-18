pub fn run() {
    println!("Welcome to Conditionals");

    let age = 18; 
    let check_id: bool = false; 
    let knows_person_of_age = true;

    if age >= 21 && (check_id || knows_person_of_age) {
        println!("Ok qualified");
    } else if age < 21 && (check_id || knows_person_of_age) {
        println!("ok allowed");
    } else {
         println!("Not yet of age");
    }

    // short handing 
    // there is no tenary operator in Rust.
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age {}", is_of_age);
}