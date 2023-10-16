use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implemented by UnsafeCell
// impl<T> !Sync for Cell<T> {}
// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any reference, because we never give any out
        unsafe { *self.value.get() = value };
    }
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Cell;

//     #[test]
//     fn bad_if_cell_is_sync() {
//         use std::sync::Arc;
//         let x = Arc::new(Cell::new(0));
//         let x1 = x.clone();
//         let jh1 = std::thread::spawn(move || {
//             for _ in 0..100000 {
//                 let x = x1.get();
//                 x1.set(x + 1);
//             }
//         });
//         let x2 = x.clone();
//         let jh2 = std::thread::spawn(move || {
//             for _ in 0..100000 {
//                 let x = x2.get();
//                 x2.set(x + 1);
//             }
//         });
//         jh1.join().unwrap();
//         jh2.join().unwrap();
//         assert_eq!(x.get(), 200000);
//     }

//     #[test]
//     fn bad_if_cell_give_out_ref() {
//         let x = Cell::new(String::from("hello"));
//         let first = x.get();
//         x.set(String::from("world"));
//         eprintln!("first: {}", first); // first should not pointing to the new string "world"
//     }
// }
