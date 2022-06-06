// 1. Traits are something called Interfaces on other languages like java. They provide guarantee of methods that the struct needs to implement. IT connect the outside world without bothering about the inner code. They are public by nature. A trait defines functionality a particular type has and can share with other types.
// 2. Impl struct contains the all the methods that are not common or to not be shared with other structs.
// 3. Struct visibility defaults to private, and can be overridden with the pub modifier. 


// Inheriting the traits
trait Foo {
    fn foo(&self);
}
trait FooBar: Foo {
    fn foobar(&self);
}
struct Baz;

impl Foo for Baz {
    fn foo(&self) {
        println!("foo");
    }
}

impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar");
    }
}

// Default method to as trait
trait Validity {
    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}
struct UseDefault;

impl Validity for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Called UseDefault.is_valid.");
        true
    }
}

struct OverrideDefault;

impl Validity for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("Called OverrideDefault.is_valid.");
        true
    }

    fn is_invalid(&self) -> bool {
        println!("Called OverrideDefault.is_invalid!");
        true // Overrides the expected value of `is_invalid()`.
    }
}

// Better common way of sharing traits
struct Dog {
    hunger: Hunger,
    wagging: bool,
}

struct Cat {
    hunger: Hunger,
    meowing: bool,
}
struct Hunger {
    level: u8,
}

trait Hungry {
    fn is_hungry(&self) -> bool;
}

impl Hunger {
    fn is_hungry(&self) -> bool {
        self.level > 100
    }
}
pub fn run() {
    let a = Baz;
    a.foobar();

    // Default methods to a trait
    let default = UseDefault;
    assert!(!default.is_invalid()); // Prints "Called UseDefault.is_valid."

    let over = OverrideDefault;
    assert!(over.is_invalid()); // Prints "Called OverrideDefault.is_invalid!"

    // Better traits sharing
    let cat = Cat{meowing:true,hunger:Hunger { level: 101 }};
    let dog = Dog{wagging:true,hunger:Hunger { level: 80 }};
    println!("{:?}",cat.hunger.is_hungry());
    println!("{:?}",dog.hunger.is_hungry());
}
