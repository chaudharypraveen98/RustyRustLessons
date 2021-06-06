// First arg at 0 index is the path to executable
use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Praveen";
    let status = "100%";

    if command =="hello"{
        println!("hello,{}. How are you?",name);
    } else if command == "status" {
        println!("status is {}",status);
    } else {
        println!("Invalid Command");
    }
}