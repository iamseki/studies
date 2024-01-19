//! # Rust Math
//! '_rust_math' it's an example of how to create a library and also used to explore rust stuff.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = _rust_math::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_call_internal_adder() {
        assert_eq!(2, internal_adder(1, 1));
    }
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Kakah");
        assert!(
            result.contains("Kakah"),
            "Greeting did not contain name, values was `{}`",
            result
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 8, width: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
