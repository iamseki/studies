use std::collections::HashMap;

fn main() {
    // must have type annotation because Vec is implemented using Generics under the hood
    let mut v1: Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);

    // macro that creates a Vec<i32> with 1, 2 ,3 values
    let mut v2 = vec![1, 2, 3];

    v2.push(4);

    dbg!(v1);
    dbg!(v2);

    let v3 = vec![4, 5, 6];

    let third: &i32 = &v3[2];
    println!("The third element of v3 is: {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    // the type can be infered as well
    let fifth = v3.get(5);
    match fifth {
        Some(value) => println!("The fifth value is: {value}"),
        None => println!("There is no fifth value on v3!")
    }
    
    println!("
Due to how Vector were implemented we can't read and mutate(push value at the end) at the same scope!!
Vector always try to put values next to each other in memory and can potentialy copying data to a new space...
This may can cause undefined behavior in read and write situations...
");

    let v4 = vec![1, 2, 4];
    for n_ref in &v4 {
        let n_plust_one = n_ref + 1;
        println!("n_plus_one iterating over v4 => {n_plust_one}");
    }
    dbg!(v4);

    let mut v5 = vec![3, 2, 1];
    for n_ref in &mut v5 {
        *n_ref += 50;
    }
    dbg!(v5);
    
    for i in 1..3 {
        println!("value in 1..3, the interval is [1, 3): {i}");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.23),
        SpreadsheetCell::Text(String::from("hello"))
    ];

    println!("vector with diff types using enum!!1!!");
    // here dbg! take ownership of row and we cant use it again!!!
    dbg!(row);

    for b in "ahuê".bytes() {
        println!("byte for ahuê: {b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // println!("reading from a map like: scores['aseas'] => {}", scores["Yellow"]);
    // to avoid move scores since we gonna used it again further
    dbg!(&scores);

    let team_name = String::from("Black");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {} for team: {}", score, team_name);

    for (key, value) in &scores {
        println!("{key}:{value}");
    }

    scores.entry(String::from("Black")).or_insert(50);
    dbg!(&scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("sum most appeard word using map: *count+=1 to sum the referenced value get from entry");
    println!("{:?}", map);

    // panic!("crash and burn");    
}

