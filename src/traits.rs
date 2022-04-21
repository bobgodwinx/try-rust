use crate::model::employee::Employee;
use crate::model::company::Company;
// - `Traits` could be seen as `Swift` protocols
pub trait EmployeeTrait { 
    fn is_valid(&self, company: &Company) -> bool;  
}

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
    let employee_03 = Employee { 
        first_name: employee_01.first_name.clone(),
        last_name: employee_01.last_name.clone(),
        age: employee_01.age,
     }; 

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
    println!("has_employee_named {}: ", company.has_employee_named(String::from("Bob")));
    println!("Is a valid employee: {}", employee_03.is_valid(&company));
    print_valid(&employee_03, &company);
}

// - So you pass a `trait` using the `&dyn` in front of the type 
// - `Traits` are very much like protocols in `Swift`
fn print_valid(check: &dyn EmployeeTrait, company: &Company) {
    if check.is_valid(company) {
        println!("Yes it is valid");
    } else {
        println!("It is not valid");
    }
}

// - It is actually a cool feature to have 
// - implementation outside the body of a 
// - struct. You can customise what you need 
// - at any where in the code because you are 
// - not overriding anything but composing your needs 
// - kudos to `Rust` for this pattern.
impl Company {
    pub fn has_employee_named(&self, name: String) -> bool {
       if let Some(_str) = self.employees.iter().find(|&s| *s.first_name == name) {
           return  true;
       }
       return  false;
    }
}

impl EmployeeTrait for Employee {
    fn is_valid(&self, company: &Company) -> bool {
        let name = self.first_name.clone();
        return  company.has_employee_named(name);
    }
}

impl  Default for BobData {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 43.0,
            some_int: 44,
        }
    }
    
}