//! # Counter
//!
//! Accepts numeric input, then counts the number of occurances of
//! each number.

use std::collections::HashMap;
use std::io::{self, Write};
use itertools::Itertools;

/// Prints a message, then gets a line of input.
///
/// # Errors
/// Returns `io::error` if it fails to read from `stdin`.
///
/// # Examples
/// ```no_run
/// loop {
///     let num = input("Enter a number:");
///     // do something with num
/// }
/// ```
fn input(message: &str) -> Result<String, io::Error>
{
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut ret = String::new();
    io::stdin().read_line(&mut ret)?;
    Ok(ret)
}

/// Gets numbers from stdin, stopping on an empty string.
/// Returns a vector of the numbers.
///
/// # Errors
/// Returns `io::error` if it fails to read from `stdin`.
///
/// # Examples
/// ```no_run
/// let nums = get_numbers().expect("Invalid number");
/// ```
fn get_numbers() -> Result<Vec<i32>, io::Error> {
    let mut result = Vec::new();
    loop {
        let line = input("Enter a number (blank to stop): ")?;
        match line.trim().len() {
            0 => break,
            _ => match line.trim().parse::<i32>() {
                Ok(x) => { result.push(x); }
                Err(e) => { println!("   That was not a number!\n{}", e); }
            }
        }
    }
    Ok(result)
}

/// Count occurances of a number, and return a map of the count.
///
/// # Examples
/// ```
/// let nums = vec![1,2,3,4,5,3,2,4,2,1,3];
/// let count = counter(&nums);
/// ```
fn counter(numbers: &Vec<i32>) -> HashMap<i32,usize> {
    let mut result = HashMap::new();
    for num in numbers.iter() {
        let count = result.entry(*num).or_insert(0);
        *count += 1;
    }
    result
}


fn main() {
    let numbers = get_numbers().expect("Invalid input!");
    let count = counter(&numbers);
    println!("\nCounts:");
    for num in count.keys().sorted() {
        println!("{}: {}", num, count[num]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_empty() {
        let nums: Vec<i32> = Vec::new();
        let count = counter(&nums);
        assert_eq!(count, HashMap::<i32,usize>::new());
    }

    #[test]
    fn counter_positive() {
        let nums = vec![1,2,3,4,5,3,2,4,2,1,2];
        let count = counter(&nums);
        let mut expected = HashMap::new();
        expected.insert(1, 2);
        expected.insert(2, 4);
        expected.insert(3, 2);
        expected.insert(4, 2);
        expected.insert(5, 1);
        assert_eq!(count, expected);
    }

    #[test]
    fn counter_negative() {
        let nums = vec![-1,-2,-1,0,1,0];
        let count = counter(&nums);
        let mut expected = HashMap::new();
        expected.insert(-2, 1);
        expected.insert(-1, 2);
        expected.insert(0, 2);
        expected.insert(1, 1);
        assert_eq!(count, expected);
    }
}

