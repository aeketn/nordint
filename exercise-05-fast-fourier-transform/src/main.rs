// author:  Erik Nordin
// created: 08/10/2018
// updated: 08/11/2018
// contact: aeketn@gmail.com

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
fn find_modulus(n: i64, elements: &[i64]) -> NttError<i64> {
    let max_elem = *elements.iter()
            .max()
            .expect("[NttError]: Could not define a maximum element.");
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
fn fft(elements: &mut Vec<i64>) -> NttError<(i64, i64)> {
    let n = (elements.len() as f64).log2().ceil().exp2() as i64;
    let modulus = find_modulus(n, &elements)?;
    let omega = find_omega(n, modulus)?;
    println!("
    omega = {}
    modulus = {}",
    omega, modulus);
    let difference = n - elements.len() as i64;

    for _ in 0..difference {
        elements.push(0);
    }

    cooley_tukey(n, omega, modulus, elements, &<i64>::pow_mod);
    Ok((omega, modulus))
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

/// Displays a vector before applying the FFT.
/// Displays the vector after applying the FFT.
/// Displays the fector after applying the IFFT.
/// 
/// The vector should ultimately be transformed back to 
/// its original state.
fn display_fft(mut elements: Vec<i64>) -> NttError<()> {
    println!("\n\telements before FFT: {:>6?}", elements);

    let (omega, modulus) = fft(&mut elements)?;
    println!("\n\telements after  FFT: {:>6?}", elements);

    ifft(omega, modulus, &mut elements)?;
    println!("\n\telements after IFFT: {:>6?}\n", elements);

    Ok(())
}


fn main() -> NttError<()> {
    println!("FFT length  1");
    display_fft(vec![11])?;

    println!("FFT length  2");
    display_fft(vec![00, 11])?;

    println!("FFT length  4");
    display_fft(vec![00, 11, 22])?;

    println!("FFT length  8");
    display_fft(vec![00, 11, 22, 33, 44])?;

    println!("FFT length 16");
    display_fft(vec![00, 11, 22, 33, 44, 55, 66, 77, 88, 99])?;
    Ok(())
}
