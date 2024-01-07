use _rust_generics::{ Summary, Tweet};

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let number_list = vec![32, 140, 50, 23, 100];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'c', 'e'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![32, 140, 50, 23, 100];
    let result = largest(&number_list);
    println!("The largest using generic fn number is {}", result);

    let char_list = vec!['y', 'm', 'c', 'e'];
    let result = largest(&char_list);
    println!("The largest using generic fn char is {}", result);

    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U
    }
    
    let both_integer = Point { x: 10, y: 8};
    let both_float = Point { x: 10.17, y: 17.7};
    let interger_and_float = Point { x: 17.4, y: 7};
    // since we moved the following variables to dbg! fn we can used any of them futher 1!!!!
    dbg!(both_integer);
    dbg!(both_float);
    dbg!(interger_and_float);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, peaople"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
    
}

