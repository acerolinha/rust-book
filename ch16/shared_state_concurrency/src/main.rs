// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Using mutex with multiple threads
fn main() {
    // let counter = Mutex::new(0); // This won't work because of multiple ownership
    // let counter = Rc::new(Mutex::new(0)); // This is not thread safe
    let counter = Arc::new(Mutex::new(0)); // This is thread safe

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Using mutex in a single thread
// use std::sync::Mutex;

// fn main() {
//     // create mutex
//     let m = Mutex::new(5);

//     {
//         // acquire the lock
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     } // lock is released

//     println!("m = {:?}", m);
// }
