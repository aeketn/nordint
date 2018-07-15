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

    #[test]
    fn biguint_to_string() {
        let number = BigUint::empty();
        assert_eq!("", number.to_string());

        let number = BigUint::zero();
        assert_eq!("0", number.to_string());

        let number = BigUint::one();
        assert_eq!("1", number.to_string());

        let number = BigUint::new("123,456,789,000");
        assert_eq!("123456789000", number.to_string());

        let number = BigUint::new("123456789123456789_123456789123456789");
        assert_eq!("123456789123456789123456789123456789", number.to_string());

        let number = BigUint::new("123456789123456789_023456789023456789_000000000000000001");
        assert_eq!(
            "123456789123456789023456789023456789000000000000000001",
            number.to_string()
        );

        let number = BigUint::new("100000000000000000_000000000000000000_000000000000000001");
        assert_eq!(
            "100000000000000000000000000000000000000000000000000001",
            number.to_string()
        );

        let number = BigUint::new("1_000000000000000000_000000000000000000_000000000000000011");
        assert_eq!(
            "1000000000000000000000000000000000000000000000000000011",
            number.to_string()
        );

        let number = BigUint::new("182378728712487128471297918269128659182591287591285619256976595761928576192785619587612985614951629857634895761345610495710297851092485709128560192854609128589127540981265409812746509182750981273409812367409812375098312650938127409182750938125609283740918274012937840912378501923865091287401974019235601927850129385701285601298374012984710238471092586091238741209874091286509123874019287409128560912785019285601285655");
        assert_eq!(number.to_string(), "182378728712487128471297918269128659182591287591285619256976595761928576192785619587612985614951629857634895761345610495710297851092485709128560192854609128589127540981265409812746509182750981273409812367409812375098312650938127409182750938125609283740918274012937840912378501923865091287401974019235601927850129385701285601298374012984710238471092586091238741209874091286509123874019287409128560912785019285601285655");
    }

}
