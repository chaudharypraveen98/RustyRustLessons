// Ownership Rules
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


pub fn run(){
    let vec1 = vec![1,2,3,4];

    // the below code gives error because it data is moved to vec2 
    // rust free memory by ownership rules listed at the starting  
    // let vec2  = vec1;
    // println!("{:?}",(vec2,vec1));
    let vec2  = &vec1;
    println!("{:?}",(vec2,&vec1));
}
