use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn run() {
    println!("=== SHARED STATE EXAMPLE START ===");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for v in 0..10 {
        println!("handle number: {v}");
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
    println!("=== SHARED STATE EXAMPLE END ===");
}
