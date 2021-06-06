// Structs used to create custom data types
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Person {
    first_name : String,
    last_name : String,
}

impl Person {
    fn new(first:&str,last:&str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name function
    fn get_full_name(&self)-> String {
        return format!("{} {}",self.first_name,self.last_name);
    }

    // Change last name
    fn change_last_name(&mut self, last:&str) {
        self.last_name = last.to_string();
    }

    // Return Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name,self.last_name)
    }
}
pub fn run(){
    let color  = Color {
        red: 255,
        green: 0,
        blue: 25,
    };
    println!("Colors are {}, {} and {}",color.red,color.green,color.blue);

    // Tuple Struct
    struct Color2(i32,i32);
    let mut color2 = Color2(255,212);
    color2.0 = 212;
    println!("Colors are {} and {}",color2.0,color2.1);

    // Person object
    let mut person = Person::new("Praveen","Chaudhary");
    println!("My name is {}",person.get_full_name());
    person.change_last_name("Sharma");
    println!("Name is {} after changing",person.get_full_name());
    println!("Name tuple is {:?}",person.to_tuple());
}