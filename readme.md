<img src="rust.png">

# RustyRustLessons
## Learn Basics of Rust in Five Lessons Only
It contains all the basics concepts of rust in just five lesson only and you can learn each lesson by changing branch to each lesson.

## Lesson Included 📖: - 
* [Lesson 1 - Print Vars Types Strings](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_1_Print_Vars_Types_Strings/src)

* [Lesson 2 - Array Tuples Vectors](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_2_Array_Tuples_Vectors/src)

* [Lesson 3 - Conditionals Loops](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_3_Conditionals_Loops/src)

* [Lesson 4 - Functions Pointers References](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_4_Functions_Pointers_References/src)

* [Lesson 5 - Structs Enums Cli](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_5_Structs_Enums_Cli/src)
  
* [Lesson 6 - Structs Impl Traits](https://github.com/chaudharypraveen98/RustyRustLessons/tree/Lesson_6_Structs_Impl_Traits/src)


## Rust - Interior mutability 
Interior mutability provides the ability to mutate the value even in case of immutable reference.

There is no legal way to convert the &T(exclusive reference) to the &mut T(mutable reference) and this is called undefined behavior.
But we have UnsafeCell which helps to cope with the immutability constraint of &T, and provides a shared reference &UnsafeCell<T> which points to the value to be mutated called **Interior mutability**.

**UnsafeCell**
It is the building block of interior mutability. It is used to eliminate some Rust optimizations that assume exclusive mutable access (or aliasing) inside the cell.

<img src="./readme_assets/Interior%20Mutability.png">

### [Cell](https://doc.rust-lang.org/std/cell/)  
It **abstraction** over the **UnsafeCell** (which is unsafe in nature), So it is unsafe, but taking some measures when we ensure the safety.

It is not suitable for vec, String, or anything that stores data in heap memory as it is expensive to use **Copy** trait.

**Why to use Cell?**  
Suppose we have a cyclic data structure, we are using the mutable reference for accessing the value. Once we reach the end, we start from the first element, then we have two mutable references to a single value. Suppose now we used the mem::swap then bad thing will happen. It is undefined behaviour to ever have multiple mutable references to a single value concurrently, no matter whether you're in a single-threaded or multi-threaded context.

**Why to use UnsafeCell with Cell?**  
If we try to mutate the value directly through the shared pointer without unsafeCell then it will always result in undefined behaviour because the compiler may try to optimize based on the current assumption that the pointer isn't shared, which would be invalid if it was shared. 

It can be done using the UnsafeCell if you know it was truly not shared (even on the same thread), because that prevents some of those optimizations such is it safe to cache when we have `&self `and when you have `&mut self` it makes sure that no two `&mut T` never points to same `T`.

How to mutate in Rust

1. **Immutability** can be possible by `&T` reference known as **aliasing**

2. **Mutability** can be only possible by having `&mut T` reference. This type of reference is exclusive in nature.

**So what does shareable mutable reference means?**  
It means we have shared references(i.e `&T` type) but with the extra power to mutate in a controlled manner.

**How does shared references allow mutation?**  
Interior mutability types which uses the shared references let you do because they maintain additional , special-case invariants that the compiler cannot check, that ensure that no conflicts arise even if you modify through a shared reference. There are lots of ways to do so with the help of CPU atomics, Mutexes, Cell, RefCell, etc.

#### How to ensure that we are using it in a controlled manner?  

<span>**1.**</span> Not using **Sync** trait  
Cell **must not implement** the **Sync** trait as it enables the usage of sharing references across threads which can result in adverse conditions as they can try to overwrite the value at the same time which leads to corrupted results.


**Base code to understand the explanation** 

```
use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
```

Let's implement some changes in the code to understand the working.

**Implementing Sync Method for testing**
```
unsafe impl<T> Sync for Cell<T>{}
```

**Writing Test Case**
```
#[cfg(test)]
mod test {

  use super::Cell;

  #[test]
  fn bad2(){
    use std::sync::Arc;
    let x = Arc::new(Cell::new(0));
    let x1 = Arc::clone(&x);
    let j1 = std::thread::spawn(move || {
      for _ in 0..1000000{
        let x = x1.get();
        x1.set(x+1);
      }
    });
    let x2 = Arc::clone(&x);
    let j2 = std::thread::spawn(move || {
      for _ in 0..1000000{
        let x = x2.get();
        x2.set(x+1);
      }
    });
    j1.join().unwrap();
    j2.join().unwrap();
    assert_eq!(x.get(),2000000)
  }
}
```
**Arc** is used to share the reference between the threads.

In the above code, we are spawning two threads and they are simultaneously mutating the value of x in each iteration 0 to 1000000.

* **Let's run test**  

```
running 1 test
test cell::test::bad2 ... FAILED

failures:

---- cell::test::bad2 stdout ----
thread 'cell::test::bad2' panicked at 'assertion failed: `(left == right)`
  left: `1170776`,
 right: `2000000`', src/cell.rs:96:5
```

**Why does the assertion fail?**  
Instead, of getting 2000000, we get only 1170776. Because one or another thread read the old value of x and incremented and set it to a new value obtained.

<span>**2.**</span> The **get** method must implement the **Copy** trait which will give the cloned value, not the exclusive reference to it. If we don't use the Copy trait, then what happens?

For example:- 

Let's try to understand by code

Returning the pointer to the value inside the Cell

```
pub fn get(&self)->&T{
  unsafe {&*self.value.get()}
}
```

Let's write some test
```
#[test]
fn bad1() {
  let x = Cell::new(true);
  let y = x.get();
  x.set(false);
  eprintln!("{}",y);
}
```
We have something in Cell and let it assign to a variable x then store the reference to y by making some change in **get** method. Then we try to change the value by **set** method. 

Now, if we try to access the y. It should fail because once we set a new value, then the previous memory must be released.

**⇒** If the test doesn't fail, it might be because the system might not free the memory instantly.

**Conclusion:**  
* Always use Cell when you have an immutable struct with numerous fields, and you want to change only 1-2 two fields.

* It can be used for setting a flag in a single thread to know the status of something.

Special shoutout to [Jon Gjengset
](https://www.youtube.com/watch?v=8O0Nt9qY_vo). It inspires me to write simple cron scheduler.

Reference taken:
1. [Jon Gjengset
](https://www.youtube.com/watch?v=8O0Nt9qY_vo)
2. [Rust Org](https://doc.rust-lang.org/std/cell/)

Feel free to ask queries. You can connect to me on [LinkedIn].(https://www.linkedin.com/in/chaudharypraveen98/)

**Happy Hacking**
Rustaceans!

#### Reference Video 🎥
[Rust Crash Course | Rustlang with Brad](https://www.youtube.com/watch?v=zF34dRivLOw)

#### Resources 📘
[Rust Docs](https://www.rust-lang.org/learn)

#### I don't have Rust, Where to get started ? 😕
##### Why not to try [Rust Playground](https://play.rust-lang.org/)