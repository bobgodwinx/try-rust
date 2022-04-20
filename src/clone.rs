// - There are two ways to clone a property 
// - The first is to actaully 

#[derive(Debug)]
struct Address {
    street: String,
    number: i32
}

impl Address {
    fn new(street: &str, number: i32) -> Address {
        Address { 
            street: street.to_string(),
            number: number
        }
    }
}

impl Clone for Address {
    fn clone(&self) -> Self {
        Self { street: self.street.clone(), number: self.number }
    }
}

#[derive(Debug, Clone, Copy)]
struct City {
    name: String, 
    country: String
}

pub fn run() {
    let address_a = Address::new("LilienthalStrasse", 18);
    let mut address_b = address_a.clone();
    address_b.street = String::from("BachStrasse");
    address_b.number = 34;
    
    println!("{:?}", address_a);
    println!("{:?}", address_b);

    let city_a = City { 
        name: String::from("Berlin"),
        country: String::from("Germany")
    };

    let mut city_b = city_a.clone();
    city_b.name = String::from("Turin");
    city_b.country = String::from("Italy");

    println!("{:?}", city_a);
    println!("{:?}", city_b);
}

