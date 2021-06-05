pub fn run(){
    let age: i32 = 24;
    let know_age = true;
    let knows_english = false;

    if age>=21 && know_age || knows_english {
        println!("Bartender : what would you like to drink?");
    }
    else if age<21 {
        println!("Bartender: Would you like soft drink or juice");
    }
    else {
        println!("Sorry we dont have anything for you");
    }

    // Shorthand
    let valid_age : bool = if age>=21 {true} else {false};
    if valid_age {
        println!("Bartender : You are a valid  customer. what would you like to drink?");
    }
}