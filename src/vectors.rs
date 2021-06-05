use std::mem;

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
    for item in number.iter_mut(){
        *item *= 2;
    }
    println!("Numbers are {:?}",number);
}
