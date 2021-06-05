pub fn run() {
    println!("hello from print rs file");

    // printing integer
    // println!(1); gives string literal error
    
    println!("{} is from {}","praveen","INIDA");

    // positional argument
    println!("{1} loves {0} but hates {2}","cats","praveen","dogs");

    //named argument
    println!("My name is {name} and i loves {subject}", name="Praveen",subject="Maths");

    // placeholder for debug traits
    println!("Binary is {:b} octal is {:o}",10,10);

    // debug traits
    println!("{:?}",("ram",10,"ravi"));

    // basic maths
    println!("addition of 10 +10 is {}",10+10);
}