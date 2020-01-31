//struct Color {
    //red: u8,
    //green: u8,
    //blue: u8
//}

struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    //let cyan = Color {
        //red: 0,
        //green: 255,
        //blue: 255
    //};

    let cyan = Color(0, 255, 255);

    println!("Color: {} {} {}", cyan.0, cyan.1, cyan.2);

    let mut rachael = Person::new("Rachael", "Nelson");
    println!("{}, {}", rachael.last_name, rachael.first_name);
    println!("or");
    println!("{}", rachael.full_name());
}
