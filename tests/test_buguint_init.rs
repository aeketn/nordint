extern crate nordint;

mod tests {
    use nordint::BigUint;
    use std::str::FromStr;

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

    #[test]
    fn biguint_new() {
        let number = BigUint::new("");
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));

        let number = BigUint::new("123456789000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123_456_789_000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123,456,789,000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("000123456789000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("00000000000000000000000000123456789000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123456789123456789_123456789123456789");
        assert_eq!(
            "BigUint { buckets: [123456789123456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123456789123456789_023456789023456789_000000000000000001");
        assert_eq!(
            "BigUint { buckets: [1, 23456789023456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("100000000000000000_000000000000000000_000000000000000001");
        assert_eq!(
            "BigUint { buckets: [1, 0, 100000000000000000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("1_000000000000000000_000000000000000000_000000000000000011");
        assert_eq!(
            "BigUint { buckets: [11, 0, 0, 1] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn biguint_from_str() {
        let number = BigUint::from_str("").unwrap_or_default();
        assert_eq!(format!("{:?}", BigUint::default()), format!("{:?}", number));

        let number = BigUint::from_str("123,456,789,000").unwrap_or_default();
        assert_eq!(format!("{:?}", BigUint::default()), format!("{:?}", number));

        let number = BigUint::from_str("123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("000123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number =
            BigUint::from_str("00000000000000000000000000123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("123456789123456789123456789123456789").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789123456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("123456789123456789023456789023456789000000000000000001")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [1, 23456789023456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("100000000000000000000000000000000000000000000000000001")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [1, 0, 100000000000000000] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("1000000000000000000000000000000000000000000000000000011")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [11, 0, 0, 1] }",
            format!("{:?}", number)
        );
    }

}
