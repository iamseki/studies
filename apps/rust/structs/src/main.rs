// Needed to let Rust prints the struct using the :? inside the curly brackets in println!() fn 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );

    println!("The rect1 is {:?}",rect1);

    println!("An easier to read for the rect1 using :#? specifier: {:#?}",rect1);

    println!("The dbg! macro is useful one as well, which will takes ownership of an expression returning the ownership of the value.");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // This prints exactly the line and the file which were printed out to stdout/err !!1!
        height: 50
    };
    // passing as reference to not let dbg! take ownership of rect2
    dbg!(&rect2);
}
