// Enums are types which have a few definite values

pub fn run() {
    // Varients
    enum Color {
        Green,
        Red,
        Blue,
        Custom(String),                 // tuple struct style
        Rgb{ r: i32, g: i32, b: i32 }, // classic struct style
    }

    let favorite: Color = Color::Green;
    let custom: Color = Color::Custom("pink".to_string());

    // chech with if let
    if let Color::Green = favorite {
        println!("Color is green by -if let check-");
    }

    // with match
    match favorite {
        Color::Green => println!("Color is green by - match check-"),
        Color::Blue => println!("Color is blue"),
        // As we don't have covered all the test cases so we define a default one
        _ => {}
    }

    match custom {
        Color::Custom(color) => println!("Custom color is {} by -match check with builtin Option-", color),
        _ => {}
    }

    // built-in Option<i32> enum
    let mut age: Option<i32> = None;
    age = Some(18);

    match age {
        Some(age) => {
            if age >= 21 {
                println!("Your age is {}", age);
                println!("Would you like to have beer?");
            }
        }
        // We had covered all the cases that's why we hadn't use the deafult varient
        None => {}
    }
}
