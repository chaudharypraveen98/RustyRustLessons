// Tuples group together different values
// Max 12 elements

pub fn run(){
    let person : (&str,&str,i8) = ("Praveen","Abhishek",2);
    println!("{} is older than {} by {} years", person.0,person.1,person.2);
}