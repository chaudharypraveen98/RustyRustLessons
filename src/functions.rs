pub fn run(){
    greet("Hello", "Praveen");
    add_number(3,4);
    let result = add_number_with_return(3,4);
    println!("addition by function with return is {}", result);

    // Closure or lambda function
    let sum_of_numbers = |num1:i32, num2:i32| num1+num2;
    println!("Addition by Closure of 3 and 4 is {}",sum_of_numbers(3,4));
}

fn greet(greet:&str,person:&str){
    println!("{}, {}", greet,person)
}

fn add_number(num1:i32,num2:i32){
    println!("Print of two numbers {} and {} is {} without return",num1,num2,num1+num2);
}
fn add_number_with_return(num1:i32,num2:i32) -> i32 {
    return num1+num2;
}
