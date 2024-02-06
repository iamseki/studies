fn while_let() {
    let mut stack = Vec::new();

    for v in 0..5 {
        stack.push(v);
    }

    while let Some(v) = stack.pop() {
        println!("value: {v}");
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c', 'd'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn simple_match_scope() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // This is a new scope so y used here is any value given to x and not the variable y = 10 in outter scope.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn simple_match_range() {
    let a = 1;
    match a {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        // Range is just allowed to char or numeric values
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let l = 'c';
    match l {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        // Remember: x and y variables only exists in the match scope
        // and after executing right side from the arm it'll be released from memory
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

fn destructuring_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    };

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}");
        }
    }
}

fn ignoring_patterns() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value")
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        // using .. is quicker than y: _, z: _
        Point { x, .. } => println!("x is {}", x),
    }
}

// A match guard is an additional if condition, specified after the pattern in a match arm;
fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case: x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let a = 4;
    let b = false;
    match a {
        4 | 5 | 6 if b => println!("yes"),
        _ => println!("no"),
    }
}

// the at operator @ lets us create a variable that holds a value at the same time as we're testing that value for a pattern match.
fn bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn main() {
    println!("=== PATTERNS MATCHING ===");

    while_let();
    for_loops();

    simple_match_scope();
    simple_match_range();

    destructuring_structs();
    destructuring_enums();

    ignoring_patterns();

    match_guard();

    bindings();
}
