use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use _rust_web_server::{fibonacci, ThreadPool};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /fibonacci HTTP/1.1" => {
            let nth = 150;
            println!("calculating {nth}nth fibonacci number");
            let fibonacci_number = fibonacci::calculate(nth);
            // since i can't calculate fibonacci of 999 999 999 in the meanwhile just sleep
            // we can address this using some big number implementation in the future
            thread::sleep(Duration::from_secs(5));
            println!("calculated fibonacci number: {fibonacci_number}");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let pool = match ThreadPool::build(4) {
        Ok(pool) => pool,
        Err(err) => panic!("Failed build ThreadPool with error => {err}"),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}
