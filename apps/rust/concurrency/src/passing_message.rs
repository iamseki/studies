use std::sync::mpsc;
use std::thread;

pub fn run() {
    println!("=== PASSING MESSAGE EXAMPLE START ===");
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("Hello"),
            String::from("Mtf"),
            String::from("World"),
        ];

        for v in values {
            let v_clone = v.clone();
            let res = tx.send(v);
            match res {
                Ok(()) => println!("successfully send: {}", v_clone),
                Err(err) => println!("sending error: {}", err),
            }
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("Hello"),
            String::from("Mtf"),
            String::from("World"),
        ];

        for v in values {
            let v_clone = v.clone();
            let res = tx_clone.send(v);
            match res {
                Ok(()) => println!("successfully send: {}", v_clone),
                Err(err) => println!("sending error: {}", err),
            }
        }
    });

    for r in rx {
        println!("successfully received value: {}", r);
    }

    println!("=== PASSING MESSAGE EXAMPLE END ===\n");
}
