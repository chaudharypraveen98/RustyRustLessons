pub fn run() {
    let mut count = 0;

    // infinite loop

    println!("------Infinite Loop -----");

    loop {
        count += 1;

        if count == 20 {
            println!("breaking as count become {}",count);
            break;
        }
    }

    // While Loop - Example FizzBuzz

    println!("------While Loop -----");

    while count < 32 {
        if count%15==0 {
            println!("fizzbuzz");
        }
        else if count%3 == 0 {
            println!("fizz");
        }
        else if  count%5==0{
            println!("buzz");
        }
        else {
            println!("count is {}",count);
        }
        count +=1;
    }

    // For range

    println!("------For Range -----");

    for i in 0..count {
        if i%15==0 {
            println!("fizzbuzz");
        }
        else if i%3 == 0 {
            println!("fizz");
        }
        else if  i%5==0{
            println!("buzz");
        }
        else {
            println!("count is {}",i);
        }
    }
}