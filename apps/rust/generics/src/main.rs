use _rust_generics::{NewsArticle, Summary, Tweet, notify};

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

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y
    }
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
        y: U,
    }

    let both_integer = Point { x: 10, y: 8 };
    let both_float = Point { x: 10.17, y: 17.7 };
    let interger_and_float = Point { x: 17.4, y: 7 };
    // since we moved the following variables to dbg! fn we can used any of them futher 1!!!!
    dbg!(both_integer);
    dbg!(both_float);
    dbg!(interger_and_float);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, peaople"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Since article has an empty impl block summarize turns to default implementation in Summary trait");

    println!("A function can also accept a trait as paramter such as notify (item: &impl Summary), item must be a type that implements the trait Summary");
    notify(&article);
    notify(&tweet);
    
    println!("But it was a syntax sugar to 'trait bounds': pub fn notify<T: Summary>(item: &T)");
    println!("If we want two arguments of same concrete type we must use trait bounds such as: pub fn notify<T: Summary>(item1: &T, item2: &T)");
    println!("We also can have an argument that needs two trait implementations: notify(item: &(impl Summary + Display))");

    println!("
This can be tough to read, for instance:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32

We can use 'where' clause like this to specifying trait bounds:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_str(&string1, &string2);
    println!("Generic lifetimes in functions comparing strings, result => {}", result);


}