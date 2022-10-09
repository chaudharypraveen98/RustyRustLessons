// single placeholder type
struct Point<T> {
    x: T,
    y: T,
}

// two placeholder type
struct TwoPoint<T, U> {
    x: T,
    y: U,
}

// Don't use the more placeholder types - min is good
// explicit types with genrics

struct Student<T> {
    name: String,
    roll_no: T,
    result: Result<T>,
}

// Generics with Enums
#[derive(Debug)]
enum Result<T> {
    Passed(T),
    Failed(T),
    Pending,
}

// It can't ensure that a and b can be added

// fn custom_add<T>(a: T, b: T) -> T {
//     a + b
// }

// Rust is smart to handle the output type other than the T, we can should U if result is different from the T but we need to explicity define that
fn custom_add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn custom_add_sub<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
    a: T,
    b: T,
) -> T {
    // Debug trait is need for this print statement
    println!("{:?} {:?}", a, b);
    a + b
}

// using the where to define generic type
fn custom_add_sub_mul<T, E>(a: T, b: T, c: E) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    E: std::fmt::Debug,
{
    println!("{:?}", c);
    a * b
}

// It not about programming to specific types
// It's about programming to capabilities of types via constraints on traits
// Becomes easier to program abstractly if mutiple capabilities or same traits then this fact can be leveraged

// Generics are easy to expand the constraints needed. When you are adding the capabilities, you are actually decreasing the the numnber of types that will be able to comply with the genrics type defination
trait SomeCustomTrait {
    fn bla_bla(&self, a: &str, b: &str) -> String;
}

fn do_this<T>(some_var: &T) -> String
where
    T: SomeCustomTrait + std::fmt::Debug,
{
    println!("{:?}", some_var);
    some_var.bla_bla("first", "second")
}

// with trait object we can have
// fn do_this2(some_var:&dyn SomeCustomTrait)->String{
//     println!("{:?}",some_var);
//     some_var.bla_bla("first", "second")
// }

#[derive(Debug)]
struct TestStruct {
    something: i32,
}

impl SomeCustomTrait for TestStruct {
    fn bla_bla(&self, a: &str, b: &str) -> String {
        self.something.to_string() + "-" + a + "-" + b
    }
}
impl SomeCustomTrait for i32 {
    fn bla_bla(&self, a: &str, b: &str) -> String {
        "i32".to_string() + "-" + a + "-" + b
    }
}

struct ImplStruct<T, U> {
    field1: T,
    field2: U,
}

impl<T, U> ImplStruct<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn log_something(&self) {
        println!("{:?} {:?}", self.field1, self.field2);
    }
}
pub fn run() {
    let random_point = Point { x: 45, y: 90 };
    println!("x is {} and y is {}", random_point.x, random_point.y);

    let explicit_point = TwoPoint { x: 9, y: 90.3 };
    println!(
        "explicit_point x is {} and y is {}",
        explicit_point.x, explicit_point.y
    );

    let random_result = Result::Passed(43);
    match random_result {
        Result::Passed(a) => println!("Student Passed with {}", a),
        Result::Failed(b) => println!("Student Failed with {}", b),
        Result::Pending => println!("Student result is pending"),
    }

    // vector internally uses the generics for holding integers, strings or float
    let random_result2 = Result::Passed(vec![45, 67, 89]);
    println!("using vector {:?}", random_result2);

    let student = Student {
        name: "Praveen".to_string(),
        roll_no: 78,
        result: Result::Passed(98),
    };

    match student.result {
        Result::Passed(a) => println!("Student Passed with {}", a),
        Result::Failed(b) => println!("Student Failed with {}", b),
        Result::Pending => println!("Student result is pending"),
    }
    let a = 4;
    let b = 5;
    println!("sum of {} and {} is {}", a, b, custom_add(a, b));
    println!("sub of {} and {} is {}", a, b, custom_add_sub(a, b));
    println!("mul of {} and {} is {}", a, b, custom_add_sub_mul(a, b, b));

    let test = TestStruct { something: 1000 };
    let result = do_this(&test);

    let impl_test = ImplStruct{
        field1:5.6,
        field2:vec![1,2,3],
    };
    impl_test.log_something();
}


// Generics could be the solid foundation in abatsract programming