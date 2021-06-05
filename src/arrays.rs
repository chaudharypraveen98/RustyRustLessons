//  Array - Fixed list where elements are of same data type

use std::mem;

pub fn run() {
    // To make array mutable use mut keyword
    let mut number: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", number);
    // Re-assigning Value
    number[1] = 88;

    // Get single array
    println!("{} is the single element of number array", number[0]);

    // Get length of array
    println!("Araay length is {}", number.len());

    // Get memory used
    println!("Memory used by the num array is {} bytes", mem::size_of_val(&number));

    // slice
    let slice : &[i32] = &number[0..2];
    println!("Slice is {:?}",slice);
}
