// Variable are immuntable by default
// Rust is a block-scoped language
// Variables hold primitive data or references to data

pub fn run() {
    let name = "Raghav";
    // can reintialize varibale
    // like name = "ram";
    println!("My name is {}", name);

    let mut age = 18;
    // If we are incrementing/ changing the variable value with using it then it will give warning
    println!("{} is {}", name, age);
    age = 20;
    println!("{} is {}", name, age);

    // Assigning multiple values
    let (name, age) = ("Ram", 56);
    println!("{} is {}",name,age);

    // Define constant
    const ID: i32 = 001;
    println!("constant is {}",ID);
}
