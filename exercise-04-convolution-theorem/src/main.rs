// author:  Erik Nordin
// created: 08/05/2018
// updated: 08/08/2018
// contact: aeketn@gmail.com

//! The functions defined in this file are identical to those in 
//! exercise-03 except except for the following functions:
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
/// This code fails when modulus is larger than sqrt(std::u64::MAX):
///    Allowing self to get that large will wrap the integer when squaring.
///    This will either panic!() in debug mode, or wrap and produce and incorrect
///    result in production mode. I have not yet found an algorithm that will
///    account for the wrapping. For now, I am limited to modlulus < sqrt(std::u64::MAX)
impl PowMod<u64, u64> for u64 {
    type Return = Self;
    fn pow_mod(mut self, mut exponent: u64, modulus: u64) -> Self {
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

/// Defining modular inverse of x^y % m for the u64 type.
/// This is a naive algorithm sufficient for the purpose of this exercise.
impl InversePowMod<u64, u64> for u64 {
    type Return = Self;
    fn inv_pow_mod(self, exponent: u64, modulus: u64) -> Self {
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
///   M is larger than the square of the max element + the combined length + 1
fn find_convolution_modulus(lhs: &[u64], rhs: &[u64]) -> NttError<u64> {
    let conv_len = (lhs.len() + rhs.len()) as u64;
    let max_elem = 
    max(
        *lhs.iter()
            .max()
            .expect("[NttError]: Could not define a maximum element."),
        *rhs.iter()
            .max()
            .expect("[NttError]: Could not define a maximum element.")
    );
    let minimum_modulus = max_elem.pow(2) * conv_len + 1;
    let start = (minimum_modulus - 1) / conv_len;

    for k in start.. {
        let modulus = k * conv_len + 1;
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

/// Finds the value of omega for the Number-Theore9803tic Transform under a given modulus M.
/// Omega is defined as the following:
///   Let g = a generator under the modulus M
///   Let k = (M - 1) / (the number of elements that will be transformed)
///   Let omega = g^k mod M
fn find_omega(n: u64, modulus: u64) -> NttError<u64> {
    let k = (modulus - 1) / n;
    let generator = find_generator(modulus)?;
    println!("generator = {}, k = {}", generator, k);
    Ok(generator.pow_mod(k, modulus))
}

/// Number-Theoretic Transform
fn ntt(elements: &[u64], n: u64, modulus: u64, omega: u64) -> NttError<Vec<u64>> {
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

/// Convolves two slices of u64 by transforming them in the NTT and then multiplying them point-wise
fn ntt_convolution(lhs: &[u64], rhs: &[u64], modulus: u64, omega: u64) -> NttError<Vec<u64>> { 
    let n = (lhs.len() + rhs.len()) as u64;
    let l_transformed = ntt(lhs, n, modulus, omega)?;
    let r_transformed = ntt(rhs, n, modulus, omega)?;
    Ok(
        l_transformed
        .iter()
        .zip(r_transformed)
        .map(|(lx, rx)| *lx * rx % modulus)
        .collect::<Vec<u64>>()
    )
}

/// Inverse Number-Theoretic Transform
fn intt(elements: &[u64], modulus: u64, omega: u64) -> NttError<Vec<u64>> {
    let n = elements.len() as u64;
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


fn main() -> NttError<()> {
    // Example:

    // Numbers are represented with the lowest place value to the left.
    let lhs = [5, 7, 3]; // Represents 375
    let rhs = [9, 5, 8]; // Represents 859


    // Compute 375 * 859 using the convolution theorem.
    let n = (lhs.len() + rhs.len()) as u64;
    let modulus = find_convolution_modulus(&lhs, &rhs)?;
    let omega = find_omega(n, modulus)?;


    // Compute the NTT of both lhs and rhs separately:
    // Then multiply NTT(lhs) * NTT(rhs) point-wise mod modulus:
    //
    //     NTT(lhs) = [15, 404, 59,  1, 440, 109]
    //     NTT(rhs) = [22, 324, 83, 12, 421, 190]
    //
    //     NTT(lhs) * NTT(rhs) 
    //
    //     = [15, 404, 59,  1, 440, 109]
    //         |   |    |   |   |    | 
    //       [22, 324, 83, 12, 421, 190]
    //
    //     = [(15*22), (404*324), (59*83), (1*12), (440*421), (109*190)] mod modulus
    //
    //     = [330, 158, 406, 12, 111, 251]
    //
    // This product is the convolution.
    let convolution = ntt_convolution(&lhs, &rhs, modulus, omega)?;
    println!("convolution = {:?}", convolution);

    
    // Compute the INTT of the convolution using the same modulus and omega:
    //     INTT(convolution) = [45, 88, 102, 71, 24, 0]
    let inverse_convolution = intt(&convolution, modulus, omega)?;
    println!("inverse con = {:?}", inverse_convolution);

    // Take the INTT(convolution) and line the numbers up in the following order:
    //     Take each number from left to right
    //     Write each number from right to left in the following fashion:
    //
    //         - Write each subsequent number under the previous number.
    //
    //         - Align the current number's right-most digit one place value
    //           to the left of the previous number's right-most digit.
    //
    // Example using [45, 88, 102, 71, 24, 0]:
    // 
    //       45    ==         45
    //      88     ==        880
    //    102      ==      10200
    //    71       ==      71000
    //   24        ==     240000
    //   0         ==    0000000 +
    //
    // Now Add the numbers:
    //_________________________________________________________________________________
    //|    Step 1  |    Step 2  |    Step 3  |    Step 4  |    Step 5   |    Step 6   |
    //|------------|------------|------------|------------|-------------|-------------|
    //|            |            |            |            |             |             |
    //|            |    carry   |   carry    |            |  carry      |             |
    //|            |       |    |      |     |            |     |       |             |
    //|            |       |    |      |     |            |     |       |             |
    //|        45  |       145  |      |145  |       145  |     |  145  |        145  |
    //|       88|  |       88   |      188   |      188   |     | 188   |       188   |
    //|     102 |  |     102|   |     102    |     102    |     |102    |      102    |
    //|     71  |  |     71 |   |     71|    |     71     |     171     |     171     |
    //|    24   |  |    24  |   |    24 |    |    24|     |     24      |     24      |
    //|    0    |  |    0   |   |    0  |    |    0 |     |     0|      |     0       |
    //|         |  |        |   |       |    |      |     |      |      |     |       |    
    //|         |  |        |   |       |    |      |     |      |      |     |       | 
    //|_________v__|________v___|_______v____|______v_____|______v______|_____v_______|
    //          5           25          125         2125         22125        322125
    //
    //
    // The result of 375 * 859 = 322,125

  Ok(())
}