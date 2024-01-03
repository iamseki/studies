#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("little bit weird")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { 
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);

    let m = Message::Write(String::from("Testing"));
    m.call();

    dbg!(value_in_cents(Coin::Dime));
    dbg!(value_in_cents(Coin::Penny));
    dbg!(value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five);
    dbg!(six);
    dbg!(none);

    let dice_roll = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }

    println!("Pattern match is awesome!");

    let opt: Option<String> = Some(String::from("Hello World!"));
    println!("We must be carefull with the usage of Match in a enum with non-copyable data like String due to ownership rust stuff!!
we must match &opt if we need to use opt further.
    ");

    match &opt {
        Some(s) => print!("Some: {s}"),
        None => println!("None!")
    };

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("value of max: {}", max),
        _ => ()
    }

    if let Some(max) = config_max {
        println!("if let turns match of single values less verbose. value of max: {}
In exchange to exhaustive checking... its like a syntax sugar for match something with only one handle case! 
", max)
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}