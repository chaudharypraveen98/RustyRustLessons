/* 
Primitive Types --
Integers : u8,u18,i16,u32,i32,u64,i64,u128,i128 (number of bits they take in memory)
Floats : f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run(){
    // default is i32
    let x = 1;

    // default is f64
    let y = 2.5;

    // Find max range
    println!("Max range of i32 is {}",std::i32::MAX);
    println!("Max range of i64 is {}",std::i64::MAX);

    let is_active: bool = true;
    

    // Get boolean from expression
    let is_greater: bool = 10>5;

    // Character literal contains only 1 char
    let  a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}",(x,y,is_active,is_greater,a1,face));
}