use std::mem;
#[derive(Debug)]
enum TYPE {
    STRING(String),
    INTEGER(i32),
}

pub fn run() {
    let mut number: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", number);
    // Re-assigning Value
    number[1] = 88;

    // Get single array
    println!("{} is the single element of number vector", number[0]);

    // Get length of array
    println!("Vector length is {}", number.len());

    // Get memory used
    println!(
        "Memory used by the num array is {} bytes",
        mem::size_of_val(&number)
    );

    // slice
    let slice: &[i32] = &number[0..2];
    println!("Slice is {:?}", slice);

    // Vector Looping and mutating value
    for item in number.iter_mut() {
        *item *= 2;
    }
    println!("Numbers are {:?}", number);

    // random types in vectors
    let random_list = vec![
        TYPE::STRING(String::from("Praveen")),
        TYPE::INTEGER(99),
        TYPE::STRING(String::from("Rahul")),
    ];
    println!("{:?}", random_list);

    for i in &random_list {
        match i {
            TYPE::INTEGER(i) => println!("{}", i),
            TYPE::STRING(name) => println!("{}", name),
        }
    }
}
