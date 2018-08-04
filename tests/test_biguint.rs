// author:  Erik Nordin
// created: 07/14/2018
// updated: 08/04/2018
// version: 0.1.0
// contact: aeketn@gmail.com

#![feature(test)]

extern crate nordint;

#[cfg(test)]
mod biguint_simple_constructors {
    use nordint::BigUint;

    #[test]
    fn empty() {
        let number = BigUint::empty();
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
        assert_eq!(10, number.capacity());
    }

    #[test]
    fn with_capacity() {
        let number = BigUint::with_capacity(50);
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
        assert_eq!(50, number.capacity());
    }

    #[test]
    fn zero() {
        let number = BigUint::zero();
        assert_eq!("BigUint { buckets: [0] }", format!("{:?}", number));
    }

    #[test]
    fn one() {
        let number = BigUint::one();
        assert_eq!("BigUint { buckets: [1] }", format!("{:?}", number));
    }

    #[test]
    fn default() {
        let number = BigUint::default();
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
    }
}

#[cfg(test)]
mod biguint_from_str {
    use nordint::BigUint;
    use std::str::FromStr;

    #[test]
    fn empty_string() {
        let number = BigUint::from_str("").unwrap_or_default();
        assert_eq!(format!("{:?}", BigUint::default()), format!("{:?}", number));
    }

    #[test]
    fn digits_only() {
        let number = BigUint::from_str("123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [456789000, 123] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn leading_zeros() {
        let number =
            BigUint::from_str("00000000000000000000000000123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [456789000, 123] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn multiple_full_buckets() {
        let number = BigUint::from_str("123456789123456789123456789123456789").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789, 123456789, 123456789, 123456789] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn middle_zero_buckets() {
        let number = BigUint::from_str("1000000000000000000000000000000000000000000000000000011")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [11, 0, 0, 0, 0, 0, 1] }",
            format!("{:?}", number)
        );
    }

    #[test]
    #[should_panic]
    fn alpha_panics() {
        let _ = BigUint::from_str("Hello, world!").expect("");
    }
}

#[cfg(test)]
mod biguint_new {
    use nordint::BigUint;

    #[test]
    fn empty_string() {
        let number = BigUint::new("");
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
    }

    #[test]
    fn only_digits() {
        let number = BigUint::new("123456789000");
        assert_eq!(
            "BigUint { buckets: [456789000, 123] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn underscore_separated_digits() {
        let number = BigUint::new("123_456_789_000");
        assert_eq!(
            "BigUint { buckets: [456789000, 123] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn comma_separated_digits() {
        let number = BigUint::new("123,456,789,000");
        assert_eq!(
            "BigUint { buckets: [456789000, 123] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn leading_zeros() {
        let number = BigUint::new("00000000000000000000000000123456789000");
        assert_eq!(
            "BigUint { buckets: [456789000, 123] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn multiple_full_buckets() {
        let number = BigUint::new("123456789123456789123456789123456789");
        assert_eq!(
            "BigUint { buckets: [123456789, 123456789, 123456789, 123456789] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn middle_zero_buckets() {
        let number = BigUint::new("1_000000000_000000000_000000000_000000000_000000000_000000011");
        assert_eq!(
            "BigUint { buckets: [11, 0, 0, 0, 0, 0, 1] }",
            format!("{:?}", number)
        );
    }
}


#[cfg(test)]
mod biguint_to_string {
    use nordint::BigUint;

    #[test]
    fn emtpy() {
        let number = BigUint::empty();
        assert_eq!("", number.to_string());
    }

    #[test]
    fn zero() {
        let number = BigUint::zero();
        assert_eq!("0", number.to_string());
    }

    #[test]
    fn one() {
        let number = BigUint::one();
        assert_eq!("1", number.to_string());
    }

    #[test]
    fn multiple_buckets() {
        let number = BigUint::new("123456789_123456789_123456789_123456789");
        assert_eq!("123456789123456789123456789123456789", number.to_string());
    }

    #[test]
    fn middle_zero_buckets() {
        let number = BigUint::new("1_000000000_000000000_000000000_000000000_000000000_000000011");
        assert_eq!(
            "1000000000000000000000000000000000000000000000000000011",
            number.to_string()
        );
    }

    #[test]
    fn large_number() {
        let number = BigUint::new("182378728712487128471000290908069000659082591207591280619256076590761028570000005019587612985614951629857034095761345610495710297851092485709128560100050600120589107540080065409812746509182750981273409812367409812375098312650938127409182750938125609283740918274012937840912378501923865091287401974019235601927850129385701285601298374012984710238471092586091238741209874091286509123874019287409128560912785019285601285655");
        assert_eq!(number.to_string(), "182378728712487128471000290908069000659082591207591280619256076590761028570000005019587612985614951629857034095761345610495710297851092485709128560100050600120589107540080065409812746509182750981273409812367409812375098312650938127409182750938125609283740918274012937840912378501923865091287401974019235601927850129385701285601298374012984710238471092586091238741209874091286509123874019287409128560912785019285601285655");
    }
}


#[cfg(test)]
mod biguint_add_assign {
    use nordint::BigUint;

    #[test] 
    fn undefined_on_empty_lhs() {
        let mut actual = BigUint::empty();
        let rhs = BigUint::one();
        actual += &rhs;
        assert_eq!(BigUint::empty(), actual);
    }

    #[test]
    fn undefined_on_empty_rhs() {
        let mut actual = BigUint::one();
        let rhs = BigUint::empty();
        actual += &rhs;
        assert_eq!(BigUint::one(), actual);
    }

    #[test]
    fn one_plus_one() {
        let mut actual = BigUint::one();
        let rhs = BigUint::one();
        actual += &rhs;
        assert_eq!(BigUint::new("2"), actual);
    }

    #[test]
    fn carry_multiple_buckets_lhs_larger() {
        let mut actual = BigUint::new("999_999_999_999_999_999_999_999_999_999_999_999");
        let rhs = BigUint::one();
        let expected = BigUint::new("1_000_000_000_000_000_000_000_000_000_000_000_000");
        actual += &rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn carry_multiple_buckets_rhs_larger() {
        let mut actual = BigUint::one();
        let rhs = BigUint::new("999_999_999_999_999_999_999_999_999_999_999_999");
        let expected = BigUint::new("1_000_000_000_000_000_000_000_000_000_000_000_000");
        actual += &rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn all_full_buckets() {
        let mut actual = BigUint::new(
            "[999_999_999] [999_999_999] [999_999_999] [999_999_999] [999_999_999] [999_999_999]",
        );
        let rhs = BigUint::new(
            "[999_999_999] [999_999_999] [999_999_999] [999_999_999] [999_999_999] [999_999_999]",
        );
        let expected = BigUint::new(
            "1_999_999_999_999_999_999_999_999_999_999_999_999_999_999_999_999_999_998",
        );
        actual += &rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn fibonacci() {
        let expected = BigUint::empty();
        assert_eq!(BigUint::fib(0), expected);

        let first = BigUint::new("52");
        let second = BigUint::one();
        let expected = BigUint::new("52");
        assert_eq!(expected, BigUint::fib_generic(first, second, 1));

        let first = BigUint::one();
        let second = BigUint::new("102");
        let expected = BigUint::new("102");
        assert_eq!(expected, BigUint::fib_generic(first, second, 2));

        let first = BigUint::new("5");
        let second = BigUint::new("6");
        let expected = BigUint::new("28");
        assert_eq!(expected, BigUint::fib_generic(first, second, 5));

        let expected = BigUint::new("832040");
        assert_eq!(expected, BigUint::fib(30));

        let expected =
            BigUint::new("222232244629420445529739893461909967206666939096499764990979600");
        assert_eq!(expected, BigUint::fib(300));

        let expected = BigUint::new("20793608237133498072112648988642836825087036094015903119682945866528501423455686648927456034305226515591757343297190158010624794267250973176133810179902738038231789748346235556483191431591924532394420028067810320408724414693462849062668387083308048250920654493340878733226377580847446324873797603734794648258113858631550404081017260381202919943892370942852601647398213554479081823593715429566945149312993664846779090437799284773675379284270660175134664833266377698642012106891355791141872776934080803504956794094648292880566056364718187662668970758537383352677420835574155945658542003634765324541006121012446785689171494803262408602693091211601973938229446636049901531963286159699077880427720289235539329671877182915643419079186525118678856821600897520171070499437657067342400871083908811800976259727431820539554256869460815355918458253398234382360435762759823179896116748424269545924633204614137992850814352018738480923581553988990897151469406131695614497783720743461373756218685106856826090696339815490921253714537241866911604250597353747823733268178182198509240226955826416016690084749816072843582488613184829905383150180047844353751554201573833105521980998123833253261228689824051777846588461079790807828367132384798451794011076569057522158680378961532160858387223882974380483931929541222100800313580688585002598879566463221427820448492565073106595808837401648996423563386109782045634122467872921845606409174360635618216883812562321664442822952537577492715365321134204530686742435454505103269768144370118494906390254934942358904031509877369722437053383165360388595116980245927935225901537634925654872380877183008301074569444002426436414756905094535072804764684492105680024739914490555904391369218696387092918189246157103450387050229300603241611410707453960080170928277951834763216705242485820801423866526633816082921442883095463259080471819329201710147828025221385656340207489796317663278872207607791034431700112753558813478888727503825389066823098683355695718137867882982111710796422706778536913192342733364556727928018953989153106047379741280794091639429908796650294603536651238230626");
        assert_eq!(expected, BigUint::fib(9999));
    }

}

#[cfg(test)]
mod biguint_mul_assign_u64 {
    use nordint::BigUint;

    #[test]
    fn undefined_for_empty_lhs() {
        let mut actual = BigUint::empty();
        let rhs: u64 = 999999999;
        let expected = BigUint::empty();
        actual *= rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn rhs_is_zero() {
        let mut actual = BigUint::new("123456789987654321");
        let rhs: u64 = 0;
        let expected = BigUint::zero();
        actual *= rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn rhs_is_one() {
        let mut actual = BigUint::new("123456789987654321");
        let rhs: u64 = 1;
        let expected = actual.clone();
        actual *= rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn rhs_is_max_bucket_lhs_is_not() {
        let mut actual = BigUint::new("123456789987654321");
        let rhs: u64 = 999999999;
        let expected = BigUint::new("123456789864197531012345679");
        actual *= rhs;
        assert_eq!(expected, actual);
    }

    #[test]
    fn fac_500() {
        let expected = BigUint::new("1220136825991110068701238785423046926253574342803192842192413588385845373153881997605496447502203281863013616477148203584163378722078177200480785205159329285477907571939330603772960859086270429174547882424912726344305670173270769461062802310452644218878789465754777149863494367781037644274033827365397471386477878495438489595537537990423241061271326984327745715546309977202781014561081188373709531016356324432987029563896628911658974769572087926928871281780070265174507768410719624390394322536422605234945850129918571501248706961568141625359056693423813008856249246891564126775654481886506593847951775360894005745238940335798476363944905313062323749066445048824665075946735862074637925184200459369692981022263971952597190945217823331756934581508552332820762820023402626907898342451712006207714640979456116127629145951237229913340169552363850942885592018727433795173014586357570828355780158735432768888680120399882384702151467605445407663535984174430480128938313896881639487469658817504506926365338175055478128640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        let actual = BigUint::fac(500);
        assert_eq!(expected, actual);
    }
}