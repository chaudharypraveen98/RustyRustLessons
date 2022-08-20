use std::cell::UnsafeCell;

pub struct Cell<T> {
    // WE will use the unsafecell to mutate shared reference
    value: UnsafeCell<T>,
}

// Hence we are unsafecell which implies code is unsafe but we can make sure code is safe by not implementing the Sync trait
// unsafe impl<T> Sync for Cell<T>{}
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    // It will take the shared refrence and give the raw exculsive pointer to T
    // compiler doesn't know that some other thread is mutating the value but we know as we will disable the sync method
    pub fn set(&self, value: T) {
        // SAFETY : we know no-one else is concurrently mutatingself.value (because !Sync)
        // SAFETY : we know we're not invalidating any references, because we never give any out
        unsafe { *self.value.get() = value };
    }

    // pub fn get(&self)->&T{
    //   unsafe {&*self.value.get()}
    // }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY : we know no-one else is modifying this value, sinc only this thread can mutate because !Sync and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    // use std::vec;

    use super::Cell;

    // fn bad(){
    //   use std::sync::Arc;
    //   let x = Arc::new(Cell::new(67));
    //   let x1 = Arc::clone(&x);
    //   std::thread::spawn(|| {
    //     x1.set(89)
    //   });
    //   let x2 = Arc::clone(&x);
    //   std::thread::spawn(|| {
    //     x2.set(88)
    //   });
    // }

    // #[test]
    // fn bad1() {
    //   let x = Cell::new(true);
    //   // because we are providing the copy trait not the reference.
    //   let y = x.get();
    //   x.set(false);
    //   eprintln!("--- ++{}",y);
    // }

    // #[test]
    // fn bad2(){
    //   use std::sync::Arc;
    //   let x = Arc::new(Cell::new(0));
    //   let x1 = Arc::clone(&x);
    //   let j1 = std::thread::spawn(move || {
    //     for _ in 0..1000000{
    //       let x = x1.get();
    //       x1.set(x+1);
    //     }
    //   });
    //   let x2 = Arc::clone(&x);
    //   let j2 = std::thread::spawn(move || {
    //     for _ in 0..1000000{
    //       let x = x2.get();
    //       x2.set(x+1);
    //     }
    //   });
    //   j1.join().unwrap();
    //   j2.join().unwrap();
    //   // we get left: `1170776`, right: `2000000`' which means that one or another thread read the old value of x and incremented and set to new value obtained.
    //   assert_eq!(x.get(),2000000)
    // }
}