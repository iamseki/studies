use std::{thread, time::Duration};

pub fn run() {
    println!("==== SIMPLE EXAMPLE START ====\n");
    let v = vec![1, 2, 3];

    // move forces thread to take ownership of v!
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            println!("vec: {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("==== SIMPLE EXAMPLE END ====\n")
}
