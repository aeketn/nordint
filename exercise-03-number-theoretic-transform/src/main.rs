// author:  Erik Nordin
// created: 08/04/2018
// updated: 08/05/2018
// contact: aeketn@gmail.com

use std::cmp::max;
use std::ops::Mul;
use std::ops::Rem;
use std::vec::Vec;

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

/// Defining x^y % m for the u64 type.
impl PowMod<u64, u64> for u64 {
    type Return = Self;
    fn pow_mod(mut self, mut exponent: u64, modulus: u64) -> Self {
        let mut result = 1;
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

/// Defining modular inverse of x^y % m for the u64 type.
/// This is a Naive implementation, but sufficient for this example.
/// https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-inverses
impl InversePowMod<u64, u64> for u64 {
    type Return = Self;
    fn inv_pow_mod(self, exponent: u64, modulus: u64) -> Self {
        let pow_mod = self.pow_mod(exponent, modulus);
        for i in 0..=modulus {
            if 1 == (pow_mod * i) % modulus {
                return i;
            }
        }
        unreachable!();
    }
}

/// Naive primality test:
///   Handle cases: 0, 1, 2, even numbers
///   Step through the odd numbers up to sqrt(number) looking for divisors. 
fn is_prime(number: u64) -> bool {
    if 2 == number     { return true;  } // 2
    if 1 >= number     { return false; } // 0 or 1
    if 0 == number & 1 { return false; } // even, not 2

    let sqrt = (number as f64).sqrt() as u64 + 1;
    for divisor in (3..=sqrt).step_by(2) {
        if 0 == number % divisor {
            return false;
        }
    }
    true
}

/// Returns the set of prime factors of a given number.
fn prime_factors_of(mut number: u64) -> Vec<u64> {
    if is_prime(number) { return vec![number]; }

    let mut factors = Vec::new();
    let half = number / 2 + 1;
    let mut pushed = false;

    for factor in 2..half {
        if number == 1 { break; }
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

/// Returns the Multiplication Matrix of the Integers mod n:
/// (Example, n = 5): http://www.wolframalpha.com/input/?i=integers+mod+5
fn dft_matrix(n: u64) -> Vec<Vec<u64>> {
    let cap = n as usize - 1;
    let mut matrix = Vec::with_capacity(cap);
    for i in 0..n {
        let mut row = Vec::with_capacity(cap);
        for j in 0..n {
            row.push(i * j % n);
        }
        matrix.push(row);
    }
    matrix
}

/// Finds a modulus M such that:
///   M is a prime number.
///   M is larger than the number of elements
///   M is is larger than the value of any element
fn find_modulus(elements: &[u64]) -> NttError<u64> {
    let n = elements.len() as u64;
    if 0 == n { 
        return Err("[NttError]: Attempt to transform nothing".to_string());
    }

    let max_elem = *elements
        .iter()
        .max()
        .expect("[NttError]: Could not define a maximum element.");
    let largest = max(n, max_elem);
    let start = (largest - 1) / n;

    for k in start.. {
        let modulus = k * n + 1;
        if modulus > largest && is_prime(modulus) {
            return Ok(modulus);
        }
    }
    Err("[NttError]: Could not find working modulus for the provided vector.".to_string())
}

/// Finds a generator under the given modulus:
/// Some number g is a generator for a modulus M if for each
/// prime factor of (M - 1), g^((M - 1) / factor) mod M != 1
/// I still do not fully understand why this works.
fn find_generator(modulus: u64) -> NttError<u64> {
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
    Err(format!("[NttError]: No generator exists under the modulus `{}`", modulus))
}

/// Finds the value of omega for the Number-Theoretic Transform under a given modulus M.
/// Omega is defined as the following:
///   Let g = a generator under the modulus M
///   Let k = (M - 1) / (the number of elements that will be transformed)
///   Let omega = g^k mod M
fn find_omega(n: u64, modulus: u64) -> NttError<u64> {
    let k = (modulus - 1) / n;
    let generator = find_generator(modulus)?;
    Ok(generator.pow_mod(k, modulus))
}

/// Number-Theoretic Transform
fn ntt(elements: &[u64]) -> NttError<Vec<u64>> {
    let n = elements.len() as u64;
    let modulus = find_modulus(elements)?;
    let omega = find_omega(n, modulus)?;
    Ok(
        dft_matrix(n)
        .iter()
        .map(|row| {
            elements
                .iter()
                .zip(row)
                .map(|(elem, ij)| elem * omega.pow_mod(*ij, modulus))
                .sum::<u64>()
                % modulus
        }).collect::<Vec<u64>>()
    )
}

/// Inverse Number-Theoretic Transform
fn intt(elements: &[u64]) -> NttError<Vec<u64>> {
    let n = elements.len() as u64;
    let modulus = find_modulus(elements)?;
    let omega = find_omega(n, modulus)?;
    Ok(
        dft_matrix(n)
        .iter()
        .map(|row| {
            elements
                .iter()
                .zip(row)
                .map(|(elem, ij)| elem * omega.inv_pow_mod(*ij, modulus))
                .sum::<u64>()
                * n.inv_pow_mod(1, modulus)
                % modulus
        }).collect::<Vec<u64>>()
    )
}

/// Computes the NTT on multiple input vectors, then computes the INTT and verifies 
/// that each output of the INTT matches each corresponding original input vector.
fn main() -> NttError<()> {
    for n in (3..=33).step_by(3) {
        let input: Vec<u64> = (0..=n).step_by(3).collect();
        println!("input vec  = {:?}", input);

        let output = ntt(&input)?;
        println!("after ntt  = {:?}", output);
        assert_ne!(input, output);

        let original = intt(&output)?;
        println!("after intt = {:?}\n\n", original);
        assert_eq!(input, original);
    }
    Ok(())
}