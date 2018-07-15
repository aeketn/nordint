extern crate nordint;

mod tests {
    use nordint::BigUint;

    #[test]
    fn biguint_empty() {
        let number = BigUint::empty();
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
        assert_eq!(10, number.capacity());
    }

    #[test]
    fn biguint_with_capacity() {
        let number = BigUint::with_capacity(50);
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
        assert_eq!(50, number.capacity());
    }

    #[test]
    fn biguint_zero() {
        let number = BigUint::zero();
        assert_eq!("BigUint { buckets: [0] }", format!("{:?}", number));
    }

    #[test]
    fn biguint_one() {
        let number = BigUint::one();
        assert_eq!("BigUint { buckets: [1] }", format!("{:?}", number));
    }

    #[test]
    fn biguint_default() {
        let number = BigUint::default();
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
    }

}