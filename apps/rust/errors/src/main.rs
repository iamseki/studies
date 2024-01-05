use std::{fs::File, io::{ErrorKind, self, Read}};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("more_hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok (_) => Ok(username),
        Err (e) => Err(e)
    }
}

fn read_username_shortcut() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };

    dbg!(&greeting_file);

    let username_result = read_username_from_file();
    let username = match username_result {
        Ok(u) => u,
        Err(_) => "default_name".to_string() 
    };

    println!("username read: {username}");

    let username_result = read_username_shortcut();
    let username = match username_result {
        Ok(u) => u,
        Err(_) => "default_name".to_string()
    };

    println!("username read from shortner error handler?: {username}");
    println!("we can even chaining method calls after ?");
    println!("we can even chaining method calls after ?");

    println!("A less verbose version of error handling explicit why and how the call must succeed, using expect:");

    let _another_file = File::open("another_hello.txt").expect("another_hello.txt should be included in this project");
}