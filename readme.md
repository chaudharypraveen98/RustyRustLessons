<img src="rust.png">

## RustyRustLessons
#### Learn Basics of Rust in Five Lessons Only
It contains all the basics concepts of rust in just five lesson only and you can learn each lesson by changing branch to each lesson.

#### Lesson Included 📖: - 
* [Lesson 1 - Print Vars Types Strings](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_1_Print_Vars_Types_Strings/src)

* [Lesson 2 - Array Tuples Vectors](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_2_Array_Tuples_Vectors/src)

* [Lesson 3 - Conditionals Loops](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_3_Conditionals_Loops/src)

* [Lesson 4 - Functions Pointers References](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_4_Functions_Pointers_References/src)

* [Lesson 5 - Structs Enums Cli](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_5_Structs_Enums_Cli/src)
  
* [Lesson 6 - Structs Impl Traits](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_6_Structs_Impl_Traits/src)

#### Reference Video 🎥
[Rust Crash Course | Rustlang with Brad](https://www.youtube.com/watch?v=zF34dRivLOw)

#### Resources 📘
[Rust Docs](https://www.rust-lang.org/learn)

#### I don't have Rust, Where to get started ? 😕
##### Why not to try [Rust Playground](https://play.rust-lang.org/)

It is a way of writing generalized algorithms in such a way that can be used for the types specified later. It can be initialized for a specified type with help of parameters.

## What we use generics:-
1. **Abstract Types:** Instead of writing explicit type like i32 or float we can use the generics for different types by writing different code
2. **Adds Flexibility:** We can use different data types with the same piece of code.
3. **Reduces Code Duplication:** We can have the same piece of code or both for any two types.
4. **No Runtime Cost:** Unlike Object Traits, generics don't have runtime costs. The compiler generates the different types in the background during compile time.

## Generics
Usually the **generics type parameter** is represented by `<T>`. But you are free to use any.
### Structs
#### 1. Single placeholder type
Defining the struct with the <T> placeholder/generic type. 

```
struct Point<T> {
    x: T,
    y: T,
}
```

Let's try to use this for i32

```
pub fn main() {
    let i32_point = Point { x: 45, y: 90 };
    println!("x is {} and y is {}", i32_point.x, i32_point.y);
}
```
If you hover on the **i32_point** then you can find the type of variable is Point i32 i32. But it is not limited. You can even use the float for both x and y but they must be the same.

So how can we use the different types for the two fields x and y?

It can be done using the two placeholder

#### 2. Two placeholder  
**struct definition** 
```
struct TwoPoint<T, U> {
    x: T,
    y: U,
}
```

let's test

```
pub fn main() {
    let explicit_point = TwoPoint { x: 9, y: 90.3 };
    println!(
        "explicit_point x is {} and y is {}",
        explicit_point.x, explicit_point.y
    );
}
```

Similarly we can two different data types for the x and y, even vectors.

### Enums 
#### 1. Generics with Enum

creating a result enum that will store the result whether pass, fail, or awaiting.
```
#[derive(Debug)]
enum Result<T> {
    Passed(T),
    Failed(T),
    Pending,
}
```

Used the `Debug` trait because the enum is not primitive because all the primitives include the debug capabilities/trait. 

Testing

```
pub fn main() {
  let result = Result::Passed(43);
  match result {
      Result::Passed(a) => println!("Student Passed with {}", a),
      Result::Failed(b) => println!("Student Failed with {}", b),
      Result::Pending => println!("Student result is pending"),
  }
}
```

You can use different generics types for Passed and Failed as shown in the above case.

#### 2. Using struct and enums together with generics
we are using the Result enum from the above example

```
struct Student<T> {
    name: String,
    roll_no: T,
    result: Result<T>,
}
```

Let's use it

```
pub fn main() {
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
}
```

### Functions with Generics 
#### 1. Function with single generic type parameter and single trait

function definition
```
fn custom_add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```

Here **Output=T** ensures that after the `+` or `-` result must be the same. It can be different if you wish too like the addition of two atoms can result in the molecule(`T+T=U`)

```
let a = 4;
let b = 5;
println!("sum of {} and {} is {}", a, b, custom_add(a, b));
```

#### 2. Function with single generic type parameter and multiple traits

```
fn custom_add_sub<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
    a: T,
    b: T,
) -> T {
    // Debug trait is needed for this print statement
    println!("{:?} {:?}", a, b);
    a + b
}
```

Using the type

```
pub fn main() {
  println!("sub of {} and {} is {}", a, b, custom_add_sub(a, b));
}
```

I know it's easy for you now

#### 3.Function with multiple generic type parameters and multiple traits and where keyword
```
// using the where to define a generic type
fn custom_add_sub_mul<T, E>(a: T, b: T, c: E) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    E: std::fmt::Debug,
{
    println!("{:?}", c);
    a * b
}
```

`where` keyword makes our code clearer. It doesn't have any impact on how the code run. 

```
pub fn main() {
  println!("mul of {} and {} is {}", a, b, custom_add_sub_mul(a, b, b));
}
``` 

#### 4. Functions with custom trait
Instead of using the predefined trait, we can use the custom trait as we like to.

```
trait SomeCustomTrait {
    fn bla_bla(&self, a: &str, b: &str) -> String;
}
#[derive(Debug)]
struct TestStruct {
    something: i32,
}

impl SomeCustomTrait for TestStruct {
    fn bla_bla(&self, a: &str, b: &str) -> String {
        self.something.to_string() + "-" + a + "-" + b
    }
}

fn do_this<T>(some_var: &T) -> String
where
    T: SomeCustomTrait + std::fmt::Debug,
{
    println!("{:?}", some_var);
    some_var.bla_bla("first", "second")
}
```
In the function, we have used our `SomeCustomTrait` along with the Debug. We can even add more. 

```
pub fn main() {
  let test = TestStruct { something: 1000 };
  let result = do_this(&test);
}
```

### Impl with Generics

```
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
```

You can notice here the generics type is defined at the `impl block` instead of a struct but you can define it at both blocks but Impl block is a must.

```
pub fn main() {
  let impl_test = ImplStruct{
      field1:5.6,
      field2:vec![1,2,3],
  };
  impl_test.log_something();
```

So, we have covered the basics of Rust Generics but you can do a  lot more with Generics.

**Resources**
1. [rust-lang by example](https://doc.rust-lang.org/rust-by-example/generics.html)
2. [Doug Milford](https://www.youtube.com/watch?v=nvur2Ast8hE)