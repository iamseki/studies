fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    println!("
first_workd(s: &String) -> usize function, must return the first word before a blank space in a string.
This first approach has a flaw returning only a number, because 's' variable still retains write permissions after calling first_word.
So the s.clear(); will cause a runtime bug when trying to read the first word using the 'word'.
Extend the functionality to a second word were also problematic...
");
    /* PROBLEM (It will compile but can cause runtime errors):
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    s.clear();    
     */

    println!("Slices to the WIN!!");

    let example = String::from("Hello World!");
    println!("entire string with: &example[..] => {}", &example[..]);
    println!("until first blank space: &example[0..5] or [..5] => {}", &example[..5]);
    
    let word = first_word(&example);
    println!("
Refactor first_word signature: first_workd(s: &String) -> &str function
'word' found by first_word with slice implementation => {word}
returning the entire word if doesn't find a blank space with: &s[..]
or the word when find the first match with: &s[..index]
also example.clear(); will cause a compile error due to borrowing rules.
tip: &str is an immutable reference!!
");

    println!("
Change signature again to: first_word(s: &str) -> &str
Cause &str agreed with &str and &String.
First word of a string literal: {}
", first_word("Hey Test!"));

    println!("
show up memory usage in bytes by typos:
&String={}, &str={}.
tip: remember that when something like s._as_bytes() produces an immutable reference, it is illegal to mutate i1s value when iterates over it using .iter().", std::mem::size_of::<&String>(), std::mem::size_of::<&str>())
}

