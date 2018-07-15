// author:  Erik Nordin
// created: 07/14/2018
// updated: 07/14/2018
// version: 0.1.0
// contact: aeketn@gmail.com

use std::str::FromStr;
use ParseBigIntError;

//const CAP: u64 = 1_000_000_000_000_000_000; // 18 digits
const DIGITS_PER_BUCKET: usize = 18;

/// An unbounded, unsigned integer.
///
/// # Internal Representation
/// `BigUint` is represnted internally by a `Vector<u64>`.  
/// Each index of the vector (referred to as a `bucket`) contains
/// up to 18 digits of a number, with the highest-order digits stored at the tail.  
///
/// *Example:*  
/// Number: `123_000_000_000_000_004_560`  
/// Internal: `BigUint { [4560, 123] }`
#[derive(Debug)]
pub struct BigUint {
    buckets: Vec<u64>,
}

impl BigUint {
    /// Creates an empty `BigUint` with default capacity 10
    pub fn empty() -> BigUint {
        BigUint {
            buckets: Vec::with_capacity(10),
        }
    }

    /// Creates an empbity `BigUint` with specified capacity
    pub fn with_capacity(capacity: usize) -> BigUint {
        BigUint {
            buckets: Vec::with_capacity(capacity),
        }
    }

    /// Creates a `BigUint` with the value 0
    pub fn zero() -> BigUint {
        BigUint { buckets: vec![0] }
    }

    /// Creates a `BigUint` with the value 1
    pub fn one() -> BigUint {
        BigUint { buckets: vec![1] }
    }

    /// Creates a `BigUint` from a string.
    /// # FromStr Difference
    /// This function will strip out all non-digit characters and never return a `ParseIntError`  
    /// This allows strings with more flexible formatting to be passed in:  
    /// # Examples
    /// Each of these strings produces the same `BigUint`.  
    ///
    /// `"123456789123456789_123456789123456789"` : Separated by internal bucket size using an underscore.  
    /// `"000123456789123456789123456789123456789"` : Leading zeros are ignored.  
    /// `"abc123456789123456789LMNOP123456789123456789xyz" : Letters are ignored.
    /// `"123,456,789,123,456,789,123,456,789,123,456,789"` : Represented using commas as separators.  
    pub fn new(num_as_str: &str) -> BigUint {
        // Safe to unwrap() because all invalid characters will be filtered out.
        if num_as_str.is_empty() {
            BigUint::empty()
        } else {
            BigUint::from_str(
                &num_as_str
                    .chars()
                    .filter(|character| character.is_digit(10))
                    .collect::<String>(),
            ).unwrap()
        }
    }

    /// Returns the current capacity in `buckets`.  
    /// Filling the `BigUint` beyond capacity will cause it to resize.
    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }
}

impl Default for BigUint {
    /// Default `BigUint` is empty.
    fn default() -> BigUint {
        BigUint::empty()
    }
}

impl FromStr for BigUint {
    type Err = ParseBigIntError;

    /// Creates a `BigUint` from a provided string.  
    /// This function will throw a `ParseIntError` if the provided string is not entirely numerical.
    /// *Note:* If you want more flexible formatting, use `BigUing::new()`
    fn from_str(num_as_str: &str) -> Result<Self, Self::Err> {
        if num_as_str.is_empty() {
            return Err(Self::Err::empty());
        }

        for digit in num_as_str.chars() {
            if !digit.is_digit(10) {
                return Err(Self::Err::invalid());
            }
        }

        let mut number = BigUint::with_capacity(num_as_str.len() / DIGITS_PER_BUCKET + 1);
        let mut bucket = String::new();

        num_as_str.chars()
            .skip_while(|digit| '0' == *digit)
            .collect::<String>().chars().rev()
            // Fill the BigUint 
            .enumerate()
            .for_each(|(i, digit)| {
                bucket.push(digit);
                if 0 == (i + 1) % DIGITS_PER_BUCKET {
                    let reverse = bucket.chars().rev().collect::<String>();
                    number.buckets.push(<u64>::from_str(&reverse).unwrap());
                    bucket.clear();
                }
            });

        // Push on the last bucket
        if !bucket.is_empty() {
            let reverse = bucket.chars().rev().collect::<String>();
            number.buckets.push(<u64>::from_str(&reverse).unwrap());
        }

        Ok(number)
    }
}

impl ToString for BigUint {
    /// Converts a BigUint to a string.
    fn to_string(&self) -> String {
        if self.buckets.is_empty() {
            return String::new();
        }
        // Avoid generating leading zeros on the highest-order bucket.
        let mut num_as_string = self.buckets[self.buckets.len() - 1].to_string();
        // Add each bucket to the string with potential leading zeros
        for bucket in self.buckets.iter().rev().skip(1) {
            let number = &bucket.to_string();
            for _ in number.len()..DIGITS_PER_BUCKET {
                num_as_string.push('0');
            }
            num_as_string += number;
        }
        num_as_string
    }
}
