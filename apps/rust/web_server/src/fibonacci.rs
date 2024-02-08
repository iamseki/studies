/// Calculates the fibonacci sequence using the iterative method.
fn iterative(nth: u128) -> u128 {
    if nth == 0 {
        return 0;
    } else if nth == 1 {
        return 1;
    }

    let mut number = 0;
    let mut x = 0;
    let mut y = 1;

    for _ in 0..nth {
        number = x + y;
        x = y;
        y = number;
    }

    number
}

/// Returns the nth number of fibonacci sequence, starts from 0 and 1.
///
/// The Fibonacci sequence is a sequence in which each number is the sum of the two preceding ones.
/// Sample of first eleven numbers in the sequence: 0 1 1 2 3 5 8 13 21 34 55 89 144
///
/// # Arguments
/// * `nth` - A nth number of the fibonacci sequence to pick.
///
/// # Examples
///
/// ```
/// use _rust_web_server::fibonacci;
///
/// let sequence_number = 11;
/// let expected_fibonacci_number = 144;
/// let fibonacci_number = fibonacci::calculate(sequence_number);
///
/// assert_eq!(fibonacci_number, expected_fibonacci_number);
/// ```
pub fn calculate(nth: u128) -> u128 {
    iterative(nth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fibonacci_of_zero() {
        let number = iterative(0);
        assert_eq!(number, 0);
    }

    #[test]
    fn calculate_fibonnaci_of_one() {
        let number = iterative(1);
        assert_eq!(number, 1)
    }

    #[test]
    fn calculate_fibonacci_of_two() {
        let number = iterative(2);
        assert_eq!(number, 2)
    }

    #[test]
    fn calculate_fibonacci_of_three() {
        let number = iterative(3);
        assert_eq!(number, 3)
    }

    #[test]
    fn calculate_fibonacci_of_four() {
        let number = iterative(4);
        assert_eq!(number, 5)
    }

    #[test]
    fn calculate_fibonacci_of_five() {
        let number = iterative(5);
        assert_eq!(number, 8)
    }

    #[test]
    fn calculate_fibonacci_of_six() {
        let number = iterative(6);
        assert_eq!(number, 13);
    }

    #[test]
    fn calculate_fibonacci_of_seven() {
        let number = iterative(7);
        assert_eq!(number, 21)
    }

    #[test]
    fn calcate_fibonacci_of_eight() {
        let number = iterative(8);
        assert_eq!(number, 34);
    }

    #[test]
    fn calculate_fibonacci_of_nine() {
        let number = iterative(9);
        assert_eq!(number, 55);
    }

    #[test]
    fn calculate_fibonacci_of_ten() {
        let number = iterative(10);
        assert_eq!(number, 89);
    }
}
