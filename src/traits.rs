use crate::model::employee::Employee;
use crate::model::company::Company;
#[allow(dead_code)] // don't do this in production.
#[derive(Clone, Copy, Debug)]
struct BobData {
    some_bool: bool,
    some_float: f32,
    some_int: i32,
}

impl BobData {
    fn new(some_bool: bool, some_float: f32, some_int: i32) -> Self { Self { some_bool, some_float, some_int } }
}

pub fn run() {
    println!("Welcom to traits");

    // - This creates `bob_data_0` a type of `BobData`
    let bob_data_0 = BobData::new(true, 10.3, 80);
    // - This creates `bob_data_1` and fill it with values from ` bob_data_0`
    let bob_data_1 = BobData { ..bob_data_0 };
    // - This creates `bob_data_2` with partial values from `bob_data_0`
    let bob_data_2 = BobData {
         some_bool: false, 
        some_float: 20.0, ..bob_data_1
    };
    
    let employee_01 = Employee { 
        first_name: String::from("Bob"), 
        last_name: String::from("obi"), 
        age: 23, 
    };

    let employee_02 = Employee { 
        first_name: String::from("Sergi"), 
        last_name: String::from("drapiko"), 
        age: 23,
    };

    println!("{:?}", &bob_data_2);
    println!("{:?}", &employee_01);

    // - Since there is no inheritance in `Rust` 
    // - You can `Compose` data types as show below 👇🏾
    let mut company = Company { 
        call_count: 0,
        name: String::from("Vodafone"), 
        employees: vec![employee_01, employee_02],
    };
    println!("{:?}", &company);
    // - All methods attached to a struct are associatives 
    // - you can see this as `Swift` extensions... literally the same thing.
    println!("Number of Employee: {}", company.count_employee());
    println!("call_clount {} : ", company.call_count);
}