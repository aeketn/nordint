// author:  Erik Nordin
// created: 07/14/2018
// updated: 08/04/2018
// version: 0.1.0
// contact: aeketn@gmail.com

#![feature(test)]

extern crate nordint;
extern crate num_bigint;
extern crate test;

#[cfg(test)]
mod local_biguint_benchmarks {
    use nordint::BigUint;
    use test::Bencher;

    #[bench]
    fn bench_fib_27_local(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fib(27);
        });
    }

    #[bench]
    fn bench_fib_272_local(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fib(272);
        });
    }

    #[bench]
    fn bench_fib_2727_local(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fib(2_727);
        });
    }

    #[bench]
    fn bench_fac_27_local(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fac(27);
        });
    }

    #[bench]
    fn bench_fac_272_local(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fac(272);
        });
    }

    #[bench]
    fn bench_fac_2727_local(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fac(2727);
        });
    }
}

#[cfg(test)]
mod crate_biguint_benchmarks {
    use num_bigint::BigUint;
    use test::Bencher;

    fn fib_generic(mut first: BigUint, mut second: BigUint, n: usize) -> BigUint {
        match n {
            0 => BigUint::new(vec![0]),
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

    #[bench]
    fn bench_fib_27_crate(b: &mut Bencher) {
        b.iter(|| {
            fib_generic(BigUint::new(vec![1]), BigUint::new(vec![1]), 27);
        });
    }

    #[bench]
    fn bench_fib_272_crate(b: &mut Bencher) {
        b.iter(|| {
            fib_generic(BigUint::new(vec![1]), BigUint::new(vec![1]), 272);
        });
    }

    #[bench]
    fn bench_fib_2727_crate(b: &mut Bencher) {
        b.iter(|| {
            fib_generic(BigUint::new(vec![1]), BigUint::new(vec![1]), 2_727);
        });
    }

    fn fac(n: usize) -> BigUint {
        let mut result = BigUint::new(vec![1]);
        (1..n + 1).rev().for_each(|x| {
            result *= x as u64;
        });
        result
    }

    #[bench]
    fn bench_fac_27_crate(b: &mut Bencher) {
        b.iter(|| {
            fac(27);
        });
    }

    #[bench]
    fn bench_fac_272_crate(b: &mut Bencher) {
        b.iter(|| {
            fac(272);
        });
    }

    #[bench]
    fn bench_fac_2727_crate(b: &mut Bencher) {
        b.iter(|| {
            fac(2727);
        });
    }

}