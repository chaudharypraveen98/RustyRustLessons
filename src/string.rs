// Primitive str  = Immutable fixed-length string somehwhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data
pub fn run(){
    // You must add mut keyword to push string or character
    let mut hello = String::from("Hello ");
    
    println!("{}",hello);

    //Get length
    println!("Length of string is {}",hello.len());
    

    // Push character
    hello.push('W');
    hello.push_str("orld");
    println!("{}",hello);

    // Capacity : The maximum reserve space for allocation in bytes
    println!("Capacity is {}",hello.capacity());

    // Checking is empty
    println!("String is Empty - {}", hello.is_empty());

    // String contains
    println!("Hello World contains World - {}", hello.contains("World"));

    // Looping through the word splitted by whitespace
    for word in hello.split_whitespace() {
        println!("{}",word);
    }

    //Create a string with capacity
    let mut my_name  = String::with_capacity(10);
    my_name.push_str("Prav");
    my_name.push('e');
    my_name.push_str("en");
    println!("My name is {}",my_name);

    // Asseration Test
    assert_eq!(7,my_name.len());

    // checking capacity
    assert_eq!(10,my_name.capacity());
}