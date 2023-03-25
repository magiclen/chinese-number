#![cfg(test)]

use super::*;

#[test]
fn test_unsigned_integer_to_chinese_low() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                unsigned_integer_to_chinese_low(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    $value
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("二十", 20);
    test!("五十五", 55);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("一億", 10_0000);
    test!("一億零一", 10_0001);
}

#[test]
fn test_unsigned_integer_to_chinese_ten_thousand() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                unsigned_integer_to_chinese_ten_thousand(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    $value
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("十萬", 10_0000);
    test!("十萬零一", 10_0001);
    test!("一億", 1_0000_0000);
    test!("一億零一", 1_0000_0001);
    test!("一億零一萬", 1_0001_0000);
}

#[test]
fn test_big_unsigned_integer_to_chinese_ten_thousand() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                big_unsigned_integer_to_chinese_ten_thousand(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    ($value as u128).into()
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("十萬", 10_0000);
    test!("十萬零一", 10_0001);
    test!("一億", 1_0000_0000);
    test!("一億零一", 1_0000_0001);
    test!("一億零一萬", 1_0001_0000);
}

#[test]
fn test_unsigned_integer_to_chinese_middle() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                unsigned_integer_to_chinese_middle(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    $value
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("十萬", 10_0000);
    test!("十萬零一", 10_0001);
    test!("一億", 1_0000_0000);
    test!("一萬億", 1_0000_0000_0000);
    test!("一萬零一億", 1_0001_0000_0000);
}

#[test]
fn test_big_unsigned_integer_to_chinese_middle() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                big_unsigned_integer_to_chinese_middle(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    ($value as u128).into()
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("十萬", 10_0000);
    test!("十萬零一", 10_0001);
    test!("一億", 1_0000_0000);
    test!("一萬億", 1_0000_0000_0000);
    test!("一萬零一億", 1_0001_0000_0000);
}

#[test]
fn test_unsigned_integer_to_chinese_high() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                unsigned_integer_to_chinese_high(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    $value
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("十萬", 10_0000);
    test!("十萬零一", 10_0001);
    test!("一億", 1_0000_0000);
    test!("一萬億", 1_0000_0000_0000);
    test!("一兆", 1_0000_0000_0000_0000);
    test!("一兆零一萬億", 1_0001_0000_0000_0000);
    test!("一萬兆", 1_0000_0000_0000_0000_0000);
    test!("一億兆", 1_0000_0000_0000_0000_0000_0000);
    test!("一萬億兆", 1_0000_0000_0000_0000_0000_0000_0000);
    test!("一京", 1_0000_0000_0000_0000_0000_0000_0000_0000);
}

#[test]
fn test_big_unsigned_integer_to_chinese_high() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                big_unsigned_integer_to_chinese_high(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    false,
                    ($value as u128).into()
                )
            );
        };
    }

    test!("零", 0);
    test!("一", 1);
    test!("十", 10);
    test!("一萬", 1_0000);
    test!("一萬零一", 1_0001);
    test!("一萬零一十", 1_0010);
    test!("十萬", 10_0000);
    test!("十萬零一", 10_0001);
    test!("一億", 1_0000_0000);
    test!("一萬億", 1_0000_0000_0000);
    test!("一兆", 1_0000_0000_0000_0000);
    test!("一兆零一萬億", 1_0001_0000_0000_0000);
    test!("一萬兆", 1_0000_0000_0000_0000_0000);
    test!("一億兆", 1_0000_0000_0000_0000_0000_0000);
    test!("一萬億兆", 1_0000_0000_0000_0000_0000_0000_0000);
    test!("一京", 1_0000_0000_0000_0000_0000_0000_0000_0000);
}

#[test]
fn test_fraction_compat() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                positive_float_to_chinese(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    ChineseCountMethod::TenThousand,
                    $value
                )
            );
        };
    }

    test!("零", 0.0);
    test!("一分", 0.01);
    test!("一角", 0.1);
    test!("五角五分", 0.55);
    test!("九十九九角九分", 99.99);
}
