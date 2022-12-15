//Structs used to create custom data types
//struct names require upper camel case
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple Struct 
struct Colors (u8, u8, u8);

struct Person {
    firstname: String,
    lastname: String,
}

//implementing functional structures

impl Person {
    fn new(first: &str, last: &str)-> Person {
        Person { firstname: first.to_string(), lastname: last.to_string() }
    }

    //get fullname
    fn fullname(&self)->String{
        format!("{} {}", self.firstname, self.lastname)
    }
}

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 255,
        blue: 255
    };

    println!("Color values: {} {} {}", c.blue, c.green, c.red);

    c.blue = 200;
    println!("Color values: {} {} {}", c.blue, c.green, c.red);

    let mut c = Colors(255, 128, 150);
    println!("Color values RGB: {} {} {}", c.0, c.1, c.2);

    let mut p = Person :: new("Syed", "Anees");
    println!("Person: {} {}", p.firstname,p.lastname);

    println!("Person: {}", p.fullname());
}