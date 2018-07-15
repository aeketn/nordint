// author:  Erik Nordin
// created: 07/14/2018
// updated: 07/14/2018
// version: 0.1.0
// contact: aeketn@gmail.com

//const CAP: u64 = 1_000_000_000_000_000_000; // 18 digits
//const DIGITS_PER_BUCKET: usize = 18;

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
        BigUint { buckets: Vec::with_capacity(10) }
    }

    /// Creates an empty `BigUint` with specified capacity
    pub fn with_capacity(capacity: usize) -> BigUint {
        BigUint { buckets: Vec::with_capacity(capacity) }
    }

    /// Returns the current capacity in `buckets`.  
    /// Filling the `BigUint` beyond capacity will cause it to resize.
    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    /// Creates a `BigUint` with the value 0
    pub fn zero() -> BigUint {
        BigUint { buckets: vec![0] }
    }

    /// Creates a `BigUint` with the value 1
    pub fn one() -> BigUint {
        BigUint { buckets: vec![1] }
    }
}

impl Default for BigUint {
    /// Default `BigUint` is empty.
    fn default() -> BigUint {
        BigUint::empty()
    }
}