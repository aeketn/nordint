#![feature(test)]

extern crate nordint;
extern crate test;
extern crate num_bigint;

#[cfg(test)]
mod local_biguint_benchmarks {
    use nordint::BigUint;
    use test::Bencher;

    #[bench]
    fn bench_fib(b: &mut Bencher) {
        b.iter(|| {
            BigUint::fib(500);
            BigUint::fib(5_000);
            BigUint::fib(50_000);
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
    fn bench_fib(b: &mut Bencher) {
        b.iter(|| {
            fib_generic(BigUint::new(vec![1]), BigUint::new(vec![1]), 500);
            fib_generic(BigUint::new(vec![1]), BigUint::new(vec![1]), 5_000);
            fib_generic(BigUint::new(vec![1]), BigUint::new(vec![1]), 50_000);
        });
    }
}