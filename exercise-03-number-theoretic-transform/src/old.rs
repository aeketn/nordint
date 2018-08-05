// author:  Erik Nordin
// created: 08/04/2018
// updated: 08/04/2018
// contact: aeketn@gmail.com

use std::cmp::max;
use std::vec::Vec;
use std::ops::Mul;
use std::ops::Rem;

fn is_prime(number: u64) -> bool {
    if 2 == number     { return true;  } 
    if 1 >= number     { return false; } 
    if 0 == number & 1 { return false; } 

    let sqrt = (number as f64).sqrt() as u64 + 1;
    for divisor in (3..=sqrt).step_by(2) {
        if 0 == number % divisor { return false; }
    }
    
    true
}

fn minimum_working_modulus(n: u64, max_elem: u64) -> u64 {
    let largest = max(n, max_elem);
    let start = if 0 == largest & 1 { largest + 1 } else { largest + 2 };
    for mwm in (start..).step_by(2) {
        if is_prime(mwm) { return mwm; }
    }
    unreachable!();
}

fn find_omega(n: u64, working_modulus: &mut u64) -> u64 {
    let start = (*working_modulus - 1) / n;
    let start = if start < 10 { 0 } else { start };
    ///////\println!("n = {}, mwm = {}, start = {}", n, working_modulus, start);
    for k in start.. {
        let prime = k * n + 1;
        if prime >= *working_modulus && is_prime(prime) { 
            if let Some(g) = find_generator(prime, *working_modulus) {
                let omega = g.pow_mod(k, prime);
                ///////\println!("g = {}, k = {}, omega = {}, prime = {}", g, k, omega, prime);
                if 0 <= omega {
                    ///////\println!("g = {}, k = {}", g, k);
                    *working_modulus = prime;
                    return g.pow_mod(k, prime);
                } else {
                    continue;
                }
            } 
        }
    }
    unreachable!();
}

fn prime_factors_of(mut number: u64) -> Vec<u64> {
    if is_prime(number) { return vec![number]; }
    let mut factors = Vec::new();
    let half = number / 2 + 1;
    let mut pushed = false;
    for factor in 2..=half {
        if number == 1 { break; }
        while 0 == number % factor {
            if !pushed {
                factors.push(factor);
                pushed = true;
            }
            number /= factor;
        }
        pushed = false;
    }
    factors
}

fn find_generator(prime: u64, working_modulus: u64) -> Option<u64> {
    let one_less = prime - 1;
    for generator in 1..prime {
        let mut is_generator = true;
        //println!("pf of {}  = {:?}", one_less, prime_factors_of(one_less));
        for factor in &prime_factors_of(one_less) {
            let exponent = (one_less / factor);
            //println!("generator = {:<5} one_less = {:<5} factor = {:<5} exponent = {:<5}, after_mod = {:<5}",
            //generator, one_less, factor, exponent, generator.pow_mod(exponent, prime));
            if 1 == generator.pow_mod(exponent, prime) {
                is_generator = false;
                break;
            }
        }
        if is_generator { 
            return Some(generator);
        }
    }
    None
}

trait PowMod<E, M> 
where Self: Copy + Mul<E> + Rem<M> {
    type Return;
    fn pow_mod(self, exponent: E, modulus: M) -> Self::Return;
}

impl PowMod<u64, u64> for u64 {
    type Return = Self;
    fn pow_mod(mut self, mut exponent: u64, modulus: u64) -> Self {
        let mut result = 1;
        while exponent > 0 {
            if 1 == exponent & 1 {
                result *= self;
                result %= modulus;
            }
            self *= self;
            self %= modulus;
            exponent >>= 1;
        }
        result
    }
}

trait PowModInverse<E, M> 
where Self: Copy + Mul<E> + Rem<M> {
    type Return;
    fn pow_mod_inverse(self, exponent: E, modulus: M) -> Self::Return;
}

impl PowModInverse<u64, u64> for u64 {
    type Return = Self;
    fn pow_mod_inverse(self, exponent: u64, modulus: u64) -> Self {
         let pow_mod = self.pow_mod(exponent, modulus);
         for i in 0..=modulus {
             if 1 == (pow_mod * i) % modulus {
                 return i;
             }
         }
         unreachable!();
    }
}

fn dft_table(n: u64) -> Vec<Vec<u64>> {
    let cap = n as usize - 1;
    let mut table = Vec::with_capacity(cap);
    for i in 0..n {
        let mut row = Vec::with_capacity(cap);
        for j in 0..n {
            row.push(i * j % n);
        }
        table.push(row);
    }
    table
}

fn main() {
//    for n in 1..1_000 {
//        let mwm = minimum_working_modulus(n, 10);
//        let omega = find_omega(n, mwm);
//        ///////\println!("\
//            n = {:<10}\
//            mwm = {:<10}\
//            omega = {:<10}",
//            n, mwm, omega,
//        );
//    }
    for i in 2..=1_000 {
        let a: Vec<u64> = (1..=i).collect();
        let n = a.len() as u64;
        let max_elem = *a.iter().max().unwrap();
        let mut mwm = minimum_working_modulus(n, max_elem);
        ///////\println!("\
            //n = {:<10}\
            //mwm = {:<10}\
            //max = {:<10}",
            //n, mwm, max_elem,
        ///////\);    
        let omega = find_omega(n, &mut mwm);
        ///////\println!("\
            //n = {:<10}\
            //mwm = {:<10}\
            //max = {:<10}\
            //omega = {:<10}",
            //n, mwm, max_elem, omega,
        ///////\);

        ///////\println!("a = {:?}", a);

        let table = dft_table(n);
        ///////\println!("table = {:?}", table);
        
        let transformed = dft_table(n)
            .iter().map(|row| {
                a.iter().zip(row).map(|(x, exponent)| {
                    x * omega.pow_mod(*exponent, mwm)
                })
                .sum::<u64>() % mwm
            })
            .collect::<Vec<u64>>();

        ///////\println!("transformed = {:?}", transformed);

        let original = dft_table(n)
            .iter().map(|row| {
                transformed.iter().zip(row).map(|(x, exponent)| {
                    x * omega.pow_mod_inverse(*exponent, mwm)
                })
                .sum::<u64>() % mwm * n.pow_mod_inverse(1, mwm) % mwm
            })
            .collect::<Vec<u64>>();

        ///////\println!("original = {:?}", original);
        if a != original {
            println!("FAIL:{{{}}}", i);
        } else {
            println!("\t\tPASS:{{{}}}", i);
        }
    }    
}






//    for n in 2..1_000 {
//        ///////\println!("Prime Factors of {:_<20}{:?}", n, prime_factors_of(n));
//    }


//    for n in 1..1_000_000 {
//        let mwm = minimum_working_modulus(n, 10);
//        let k = find_k(n, mwm);
//        assert!(k * n + 1 >= mwm);
//    }