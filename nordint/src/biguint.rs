// author:  Erik Nordin
// created: 07/14/2018
// updated: 08/04/2018
// version: 0.1.0
// contact: aeketn@gmail.com

use std::cmp::min;
use std::cmp::max;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::str::from_utf8;
use std::str::FromStr;
use ParseBigIntError;
use std::ops::Mul;
use std::ops::Rem;
use std::vec::Vec;


pub const LIMIT: i64 = 100; // under 10 digits
pub const DIGITS_PER_BUCKET: usize = 2;

/// An unbounded, unsigned integer.
///
/// # Internal Representation
/// `BigUint` is represnted internally by a `Vector<i64>`.  
/// Each index of the vector (referred to as a `bucket`) contains
/// up to 9 digits of a number, with the highest-order digits stored at the tail.  
///
/// *Example if bucket-size were 3 digits:*  
/// Number: `123_000_000_000_000_004_560`  
/// Internal: `BigUint { [560, 4, 0, 0, 0, 0, 123] }`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigUint {
    buckets: Vec<i64>,
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
    ///
    /// # How does this differ from the FromStr trait?
    ///
    /// This function will strip out all non-digit characters and never return a `ParseIntError`  
    /// This allows strings with more flexible formatting to be passed in:  
    ///
    /// # Examples
    ///
    /// Each of these strings produces the same `BigUint`.  
    ///
    /// `"123456789123456789_123456789123456789"` : Separated by internal bucket size using an underscore.  
    /// `"000123456789123456789123456789123456789"` : Leading zeros are ignored.  
    /// `"abc123456789123456789LMNOP123456789123456789xyz"` : Letters are ignored.  
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

    pub fn from_i64(mut number: i64) -> BigUint {
        if number == 0 {
            return BigUint::zero();
        }
        let mut buckets = Vec::new();
        while number > 0 {
            buckets.push(number % 100);
            number /= 100;
        }
        BigUint { buckets }
    }

    /// Returns the current capacity in `buckets`.  
    /// Filling the `BigUint` beyond capacity will cause it to resize.
    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    /// # Description
    ///
    /// Calculates the traditional Fibonacci sequence up to the nth element.BigUint
    ///
    /// # Example
    /// ```
    /// extern crate nordint;
    /// use nordint::*;
    /// assert_eq!(BigUint::new("8"), BigUint::fib(6));
    /// // 1, 1, (1+1)=2, (1+2)=3, (2+3)=5, (3+5)=8
    /// ```
    pub fn fib(n: usize) -> BigUint {
        BigUint::fib_generic(BigUint::one(), BigUint::one(), n)
    }

    /// Calculates a generic Fibonacci sequence up to the nth element, provided two starting values.
    ///
    /// # Example
    /// ```
    /// extern crate nordint;
    /// use nordint::*;
    /// let first = BigUint::new("5");
    /// let second = BigUint::new("6");
    /// assert_eq!(BigUint::new("28"), BigUint::fib_generic(first, second, 5));
    /// // 5,   6,   (5+6)=11, (6+11)=17,  (11+17)=28
    /// ```
    pub fn fib_generic(mut first: BigUint, mut second: BigUint, n: usize) -> BigUint {
        match n {
            0 => BigUint::empty(),
            1 => first,
            2 => second,
            _ => {
                for i in 3..=n {
                    if 1 == i & 1 {
                        first += &second;
                    } else {
                        second += &first;
                    }
                }

                if 1 == n & 1 {
                    first
                } else {
                    second
                }
            }
        }
    }

    pub fn fac(n: usize) -> BigUint {
        let mut result = BigUint::one();
        (1..n + 1).rev().for_each(|x| {
            result *= x as i64;
        });
        result
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
        let mut buckets = num_as_str
            .as_bytes()
            .iter()
            .cloned()
            .skip_while(|byte| *byte == b'0')
            .collect::<Vec<u8>>();

        buckets.reverse();
        buckets.chunks_mut(DIGITS_PER_BUCKET).for_each(|chunk| {
            chunk.reverse();
            let bucket = from_utf8(chunk).unwrap();
            number.buckets.push(<i64>::from_str(bucket).unwrap());
        });
        Ok(number)
    }
}

impl ToString for BigUint {
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

impl<'a> AddAssign<&'a BigUint> for BigUint {
    fn add_assign(&mut self, rhs: &BigUint) {
        let lhs = &mut self.buckets;
        let rhs = &rhs.buckets;
        if lhs.is_empty() || rhs.is_empty() { 
            return;
        }
        let lhs_len = lhs.len();
        let rhs_len = rhs.len();
        let mut carry = add_slices(&mut lhs[..], &rhs[..]);
        if lhs_len < rhs_len {
            lhs.extend_from_slice(&rhs[lhs_len..]);
        }
        if carry == 1 {
            for index in min(lhs_len, rhs_len)..lhs.len() {
                if carry == 0 {
                    break;
                }
                carry = add_slices(&mut lhs[index..], &[carry]);
            }
            if carry == 1 {
                lhs.push(1);
            }
        }
    }
}

#[inline]
fn add_slices(lhs: &mut [i64], rhs: &[i64]) -> i64 {
    let mut carry = 0;
    lhs.iter_mut().zip(rhs.iter()).for_each(|(lx, rx)| {
        *lx += rx + carry;
        carry = if *lx >= LIMIT {
            *lx %= LIMIT;
            1
        } else {
            0
        }
    });
    carry
}

impl MulAssign<i64> for BigUint {
    fn mul_assign(&mut self, rhs: i64) {
        if rhs == 1 || self.buckets.is_empty() { 
           return;
        }
        if rhs == 0 {
            self.buckets = vec![0];
            return;
        }
        let mut carry = 0;
        for bucket in &mut self.buckets {
            *bucket *= rhs;
            *bucket += carry;
            carry = *bucket / LIMIT;
            *bucket %= LIMIT;
        }
        if 0 < carry {
            self.buckets.extend_from_slice(&BigUint::from_i64(carry).buckets[..]);
        }
    }
}



/// Number-Theoretic Transform Error
type NttError<T> = Result<T, String>;

/// This trait represents x^y % m
trait PowMod<E, M>
where
    Self: Copy + Mul<E> + Rem<M>,
{
    type Return;
    fn pow_mod(self, exponent: E, modulus: M) -> Self::Return;
}

/// This trait represents the modular inverse of x^y % m
trait InversePowMod<E, M>
where
    Self: Copy + Mul<E> + Rem<M>,
{
    type Return;
    fn inv_pow_mod(self, exponent: E, modulus: M) -> Self::Return;
}

/// Defining x^y % m for the i64 type.
/// This code fails when modulus is larger than sqrt(std::i64::MAX):
///    Allowing self to get that large will wrap the integer when squaring.
///    This will either panic!() in debug mode, or wrap and produce and incorrect
///    result in production mode. I have not yet found an algorithm that will
///    account for the wrapping. For now, I am limited to modlulus < sqrt(std::i64::MAX)
impl PowMod<i64, i64> for i64 {
    type Return = Self;
    fn pow_mod(mut self, mut exponent: i64, modulus: i64) -> Self {
        let mut result = 1;
        self %= modulus;
        while 0 < exponent {
            if 1 == exponent & 1 {
                result = result * self % modulus;
            }
            self = self.pow(2) % modulus;
            exponent >>= 1;
        }
        result
    }
}

/// Defining modular inverse of x^y % m for the i64 type.
/// This is a naive algorithm sufficient for the purpose of this exercise.
impl InversePowMod<i64, i64> for i64 {
    type Return = Self;
    fn inv_pow_mod(self, exponent: i64, modulus: i64) -> Self {
        let pow_mod = self.pow_mod(exponent, modulus);
        for i in 0..modulus {
            if 1 == pow_mod * i % modulus {
                return i;
            }
        }
        unreachable!();
    }
}

/// Naive primality test:
///   Handle cases: 0, 1, 2, even numbers
///   Step through the odd numbers up to sqrt(number) looking for divisors.
fn is_prime(number: i64) -> bool {
    if 2 == number {
        return true;
    } // 2
    if 1 >= number {
        return false;
    } // 0 or 1
    if 0 == number & 1 {
        return false;
    } // even, not 2

    let sqrt = (number as f64).sqrt() as i64 + 1;
    for divisor in (3..=sqrt).step_by(2) {
        if 0 == number % divisor {
            return false;
        }
    }
    true
}

/// Returns the set of prime factors of a given number.
fn prime_factors_of(mut number: i64) -> Vec<i64> {
    if is_prime(number) {
        return vec![number];
    }

    let mut factors = Vec::new();
    let half = number / 2 + 1;
    let mut pushed = false;

    for factor in 2..half {
        if number == 1 {
            break;
        }
        while 0 == number % factor {
            number /= factor;
            if !pushed {
                factors.push(factor);
                pushed = true;
            }
        }
        pushed = false;
    }

    factors
}

/// Finds a modulus M such that:
///   M is a prime number.
///   M is larger than the number of elements
///   M is is larger than the value of any element
fn find_convolution_modulus(n: i64, lhs: &[i64], rhs: &[i64]) -> NttError<i64> {
    let max_elem = 
    max(
        *lhs.iter()
            .max()
            .expect("[NttError]: Could not define a maximum element."),
        *rhs.iter()
            .max()
            .expect("[NttError]: Could not define a maximum element.")
    );
    let minimum_modulus = max_elem.pow(2) * n + 1;
    let start = (minimum_modulus - 1) / n;

    for k in start.. {
        let modulus = k * n + 1;
        if modulus > minimum_modulus && is_prime(modulus) {
            return Ok(modulus);
        }
    }
    Err("[NttError]: Could not find working modulus for the provided vector.".to_string())

}

/// Finds a generator under the given modulus:
/// Some number g is a generator for a modulus M if for each
/// prime factor of (M - 1), g^((M - 1) / factor) mod M != 1
/// I still do not fully understand why this works.
fn find_generator(modulus: i64) -> NttError<i64> {
    let max_value = modulus - 1;
    let prime_factors = prime_factors_of(max_value);
    for generator in 1..modulus {
        if prime_factors
            .iter()
            .map(|factor| 1 != generator.pow_mod(max_value / factor, modulus))
            .all(|not_one| not_one)
        {
            return Ok(generator);
        }
    }
    Err(format!(
        "[NttError]: No generator exists under the modulus `{}`",
        modulus
    ))
}

/// Finds the value of omega for the Number-Theore9803tic Transform under a given modulus M.
/// Omega is defined as the following:
///   Let g = a generator under the modulus M
///   Let k = (M - 1) / (the number of elements that will be transformed)
///   Let omega = g^k mod M
fn find_omega(n: i64, modulus: i64) -> NttError<i64> {
    let k = (modulus - 1) / n;
    let generator = find_generator(modulus)?;
    Ok(generator.pow_mod(k, modulus))
}


/// Collects every other element of a slice of [i64] given a starting index
///     Example: [1, 5, 3, 5, 2, 6, 9] where start_index == 0
///     Returns: [1, 3, 2, 9]
/// 
///     Example: [1, 5, 3, 5, 2, 6, 9, 4] where start_index == 1
///     Returns: [5, 5, 6, 4]
fn every_other_element_starting_at(start_index: usize, elements: &[i64]) -> Vec<i64> {
    (start_index..elements.len())
        .step_by(2)
        .map(|i| elements[i])
        .collect()
}

/// Recursively performs the Cooley-Tukey O(n log(n)) algorithm on a slice of [i64].
/// The algorithm is performed using a number-theoretic transform, 
/// where omega is the first of nth roots of unity under the provided modulus.
/// n must be a power of two.
/// 
/// This function takes a modular exponentiation function defined on i64. 
/// The alrogirthm is the same whether doing regular modular exponentiation 
/// (used in the FFT) or inverse modular exponentiation (used in the IFFT).
/// 
/// https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
fn cooley_tukey<'a, F>(
    n: i64,
    omega: i64,
    modulus: i64,
    elements: &'a mut [i64],
    mod_exp_fn: &F,
) -> &'a mut [i64]
where
    F: Fn(i64, i64, i64) -> i64,
{
    let len = elements.len();
    if len == 1 {
        return elements;
    }

    let mut even_index_elements = every_other_element_starting_at(0, &elements);
    let mut odd_index_elements  = every_other_element_starting_at(1, &elements);
    let even_transformed = cooley_tukey(n, omega, modulus, &mut even_index_elements, mod_exp_fn);
    let odd_transformed  = cooley_tukey(n, omega, modulus, &mut odd_index_elements,  mod_exp_fn);
    let multiplier = n / len as i64;

    for i in 0..len / 2 {
        let lhs = even_transformed[i];
        let rhs = mod_exp_fn(omega, multiplier * i as i64, modulus) * odd_transformed[i] % modulus;
        elements[i] = (lhs + rhs) % modulus;
        elements[i + len / 2] = ((lhs - rhs) + modulus) % modulus;
    }

    elements
}

/// The Fast Fourier Transform is implemented as a Number-Theoretic version
/// of the Cooley-Tukey algorithm. The cooley_tukey function is called
/// using modular exponentiation. 
/// 
/// The vector that is passed in is adjusted to be a power of two
/// that is equal to or larger than the original length.
/// 
/// This function returns the omega and modulus used as a tuple. 
/// It is necessary that the IFFT algorithm uses the same omega and modulus.
fn fft_convolution(lhs: &mut Vec<i64>, rhs: &mut Vec<i64>) -> NttError<(i64, i64, Vec<i64>)> {
    let n = ((lhs.len() + rhs.len()) as f64).log2().ceil().exp2() as i64;
    let modulus = find_convolution_modulus(n, &lhs, &rhs)?;
    let omega = find_omega(n, modulus)?;
    let left_difference =  n - lhs.len() as i64;
    let right_difference = n - rhs.len() as i64;

    for _ in 0..left_difference {
        lhs.push(0);
    }
    for _ in 0..right_difference {
        rhs.push(0);
    }

    cooley_tukey(n, omega, modulus, lhs, &<i64>::pow_mod);
    cooley_tukey(n, omega, modulus, rhs, &<i64>::pow_mod);


    Ok((omega, modulus,
        lhs
        .iter()
        .zip(rhs)
        .map(|(lx, rx)| *lx * *rx % modulus)
        .collect::<Vec<i64>>()
    ))
}

/// The Fast Fourier Transform is implemented as a Number-Theoretic version
/// of the Cooley-Tukey algorithm. The cooley_tukey function is called
/// using inverse modular exponentiation. 
/// 
/// The IFFT must use the same omega and modulus that was used in the FFT.
/// Additionally, each element is scaled by a power of the modular inverse of n.
/// This is necessary to reverse the forward transform. 
/// 
/// Afterward, any extraneous elements that were added to make the original vector's 
/// length a power of two are removed. The vector will be the same length as it was
/// before being transformed.
fn ifft(omega: i64, modulus: i64, elements: &mut Vec<i64>) -> NttError<()> {
    let n = elements.len() as i64;

    cooley_tukey(n, omega, modulus, elements, &<i64>::inv_pow_mod)
        .iter_mut()
        .for_each(|x| {
            *x *= n.inv_pow_mod(1, modulus);
            *x %= modulus;
        });

    while elements.ends_with(&[0]) {
        elements.pop();
    }

    Ok(())
}

impl<'a, 'b> Mul<&'b BigUint> for &'a BigUint {
    type Output = BigUint;
    fn mul(self, rhs: &'b BigUint) -> BigUint {
        let mut lhs = self.buckets.clone();
        let mut rhs = rhs.buckets.clone();

        let (omega, modulus, mut convolution) = fft_convolution(&mut lhs, &mut rhs).unwrap();
        ifft(omega, modulus, &mut convolution).unwrap();
        apply_carries(&mut convolution);

        BigUint { buckets: convolution }
    }
}

pub fn apply_carries(buckets: &mut Vec<i64>) {
    let mut carry = 0;
    println!("buckets before = {:?}", buckets);
    for bucket in buckets.iter_mut() {
        *bucket += carry;
        carry = *bucket / LIMIT;
        *bucket %= LIMIT;
    }
    while carry > 0 {
        buckets.push(carry % 100);
        carry /= 100;
    }
    println!("buckets after = {:?}", buckets);
}
