// Structs - Used to create custom data
// They are literally almost the same as in `Swift`

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}
// Tuple Struct 
struct ColorT(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Contructor
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.last_name, self.first_name)
    }
}

pub fn run() {
    println!("Welocome to structs");

    let mut c = Color {
        red: 255, 
        green: 0, 
        blue: 0
    };
    c.green = 200;
    println!("Color: {} {} {}", c.red, c.blue, c.green);

    let mut t = ColorT(255, 0, 0);
    t.2 = 30;
    println!("Color: {} {} {}", t.0, t.1, t.2);

    let mut p = Person::new("Bob", "Obi");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Full name: {}", p.full_name());
    p.set_last_name("MullMann");
    println!("Full name: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}