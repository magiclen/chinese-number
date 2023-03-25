#![cfg(test)]

use super::*;

#[test]
fn test_chinese_to_unsigned_integer_low() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_unsigned_integer(ChineseCountMethod::Low, &to_chars_vec($value))
                    .unwrap()
            );
        };
    }

    test!(0, "零");
    test!(1, "一");
    test!(2, "貳");
    test!(10, "十");
    test!(11, "十一");
    test!(20, "二十");
    test!(100, "一百");
    test!(101, "一百零一");
    test!(110, "一百一十");
    test!(111, "一百一十一");
    test!(200, "兩百");
    test!(1000, "一千");
    test!(1001, "一千零一");
    test!(1010, "一千零一十");
    test!(1011, "一千零一十一");
    test!(1100, "一千一百");
    test!(1101, "一千一百零一");
    test!(1110, "一千一百一十");
    test!(2000, "二千");
    test!(1_0000, "一萬");
    test!(1_0001, "一萬零一");
    test!(10_0000, "一億");
    test!(10_0100, "一億零一百");
    test!(1_0000_0000, "一垓");
    test!(1_0000_1000, "一垓零一千");
    test!(10_0000_0000, "一秭");
    test!(1_0000_0000_0000, "一澗");
    test!(1_0000_1000_0000, "一澗零一京");
    test!(10_0000_0000_0000, "一正");
    test!(1000_0000_0000_0000, "一極");
    test!(1000_1000_0000_0000, "一極零一溝");
    test!(9000_0900_0090_0009, "九極零九穰零九億零九");
}

#[test]
fn test_chinese_to_unsigned_integer_ten_thousand() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_unsigned_integer(ChineseCountMethod::TenThousand, &to_chars_vec($value))
                    .unwrap()
            );
        };
    }

    test!(0, "零");
    test!(1, "一");
    test!(2, "貳");
    test!(10, "十");
    test!(11, "十一");
    test!(20, "二十");
    test!(100, "一百");
    test!(101, "一百零一");
    test!(110, "一百一十");
    test!(111, "一百一十一");
    test!(200, "兩百");
    test!(1000, "一千");
    test!(1001, "一千零一");
    test!(1010, "一千零一十");
    test!(1011, "一千零一十一");
    test!(1100, "一千一百");
    test!(1101, "一千一百零一");
    test!(1110, "一千一百一十");
    test!(2000, "二千");
    test!(1_0000, "一萬");
    test!(1_0001, "一萬零一");
    test!(10_0000, "十萬");
    test!(10_0100, "十萬零一百");
    test!(1_0000_0000, "一億");
    test!(1_0000_1000, "一億零一千");
    test!(10_0000_0000, "十億");
    test!(1_0000_0000_0000, "一兆");
    test!(1_0000_1000_0000, "一兆零一千萬");
    test!(10_0000_0000_0000, "十兆");
    test!(1_0000_0000_0000_0000, "一京");
    test!(1_0000_1000_0000_0000, "一京零一千億");
    test!(9_9000_0900_0090_0009, "九京九千兆零九百億零九十萬零九");
}

#[test]
fn test_chinese_to_unsigned_integer_middle() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_unsigned_integer(ChineseCountMethod::Middle, &to_chars_vec($value))
                    .unwrap()
            );
        };
    }

    test!(0, "零");
    test!(1, "一");
    test!(2, "貳");
    test!(10, "十");
    test!(11, "十一");
    test!(20, "二十");
    test!(100, "一百");
    test!(101, "一百零一");
    test!(110, "一百一十");
    test!(111, "一百一十一");
    test!(200, "兩百");
    test!(1000, "一千");
    test!(1001, "一千零一");
    test!(1010, "一千零一十");
    test!(1011, "一千零一十一");
    test!(1100, "一千一百");
    test!(1101, "一千一百零一");
    test!(1110, "一千一百一十");
    test!(2000, "二千");
    test!(1_0000, "一萬");
    test!(1_0001, "一萬零一");
    test!(10_0000, "十萬");
    test!(10_0100, "十萬零一百");
    test!(1_0000_0000, "一億");
    test!(1_0000_1000, "一億零一千");
    test!(10_0000_0000, "十億");
    test!(1_0000_0000_0000, "一萬億");
    test!(1_0000_1000_0000, "一萬億零一千萬");
    test!(10_0000_0000_0000, "十萬億");
    test!(1_0000_0000_0000_0000, "一兆");
    test!(1_0000_1000_0000_0000, "一兆零一千億");
    test!(10_0000_0000_0000_0000, "十兆");
    test!(1_0000_0000_0000_0000_0000, "一萬兆");
    test!(1_0000_0000_0000_0000_0000_0000, "一京");
    test!(9_0000_0000_9000_0900_0090_0009, "九京零九千萬零九百億零九十萬零九");
}

#[test]
fn test_chinese_to_unsigned_integer_high() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_unsigned_integer(ChineseCountMethod::High, &to_chars_vec($value))
                    .unwrap()
            );
        };
    }

    test!(0, "零");
    test!(1, "一");
    test!(2, "貳");
    test!(10, "十");
    test!(11, "十一");
    test!(20, "二十");
    test!(100, "一百");
    test!(101, "一百零一");
    test!(110, "一百一十");
    test!(111, "一百一十一");
    test!(200, "兩百");
    test!(1000, "一千");
    test!(1001, "一千零一");
    test!(1010, "一千零一十");
    test!(1011, "一千零一十一");
    test!(1100, "一千一百");
    test!(1101, "一千一百零一");
    test!(1110, "一千一百一十");
    test!(2000, "二千");
    test!(1_0000, "一萬");
    test!(1_0001, "一萬零一");
    test!(10_0000, "十萬");
    test!(10_0100, "十萬零一百");
    test!(1_0000_0000, "一億");
    test!(1_0000_1000, "一億零一千");
    test!(10_0000_0000, "十億");
    test!(1_0000_0000_0000, "一萬億");
    test!(1_0000_1000_0000, "一萬億零一千萬");
    test!(10_0000_0000_0000, "十萬億");
    test!(1_0000_0000_0000_0000, "一兆");
    test!(1_0000_1000_0000_0000, "一兆零一千億");
    test!(10_0000_0000_0000_0000, "十兆");
    test!(1_0000_0000_0000_0000_0000, "一萬兆");
    test!(1_0000_0000_0000_0000_0000_0000, "一億兆");
    test!(1_0000_0000_0000_0000_0000_0000_0000, "一萬億兆");
    test!(1_0000_0000_0000_0000_0000_0000_0000_0000, "一京");
    test!(9_0000_0000_0000_0000_9000_0900_0090_0009, "九京零九千萬零九百億零九十萬零九");
}

#[test]
fn test_chinese_to_signed_integer_low() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_signed_integer(ChineseCountMethod::Low, &to_chars_vec($value)).unwrap()
            );
        };
    }

    test!(9000_0900_0090_0009, "九極零九穰零九億零九");
    test!(-9000_0900_0090_0009, "負九極零九穰零九億零九");
}

#[test]
fn test_chinese_to_signed_integer_ten_thousand() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_signed_integer(ChineseCountMethod::TenThousand, &to_chars_vec($value))
                    .unwrap()
            );
        };
    }
    test!(9_9000_0900_0090_0009, "九京零九千萬零九百億零九十萬零九");
    test!(-9_9000_0900_0090_0009, "負九京零九千萬零九百億零九十萬零九");
}

#[test]
fn test_chinese_to_signed_integer_middle() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_signed_integer(ChineseCountMethod::Middle, &to_chars_vec($value))
                    .unwrap()
            );
        };
    }

    test!(9_0000_0000_9000_0900_0090_0009, "九京零九千萬零九百億零九十萬零九");
    test!(-9_0000_0000_9000_0900_0090_0009, "負九京零九千萬零九百億零九十萬零九");
}

#[test]
fn test_chinese_to_signed_integer_high() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_signed_integer(ChineseCountMethod::High, &to_chars_vec($value)).unwrap()
            );
        };
    }
    test!(9_0000_0000_0000_0000_9000_0900_0090_0009, "九京零九千萬零九百億零九十萬零九");
    test!(-9_0000_0000_0000_0000_9000_0900_0090_0009, "負九京零九千萬零九百億零九十萬零九");
}

// TODO f64

#[test]
fn test_chinese_to_f64_low() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_f64(ChineseCountMethod::Low, &to_chars_vec($value)).unwrap()
            );
        };
    }

    test!(0f64, "零");
    test!(0.01f64, "一分");
    test!(0.1f64, "一角");
    test!(0.11f64, "一角一分");
    test!(1f64, "一");
    test!(1.23f64, "一二角三分");
    test!(2f64, "貳");
    test!(10f64, "十");
    test!(11f64, "十一");
    test!(20f64, "二十");
    test!(100f64, "一百");
    test!(101f64, "一百零一");
    test!(110f64, "一百一十");
    test!(111f64, "一百一十一");
    test!(200f64, "兩百");
    test!(1000f64, "一千");
    test!(1001f64, "一千零一");
    test!(1010f64, "一千零一十");
    test!(1011f64, "一千零一十一");
    test!(1100f64, "一千一百");
    test!(1101f64, "一千一百零一");
    test!(1110f64, "一千一百一十");
    test!(2000f64, "二千");
    test!(1_0000f64, "一萬");
    test!(1_0001f64, "一萬零一");
    test!(10_0000f64, "一億");
    test!(10_0100f64, "一億零一百");
    test!(1_0000_0000f64, "一垓");
    test!(1_0000_1000f64, "一垓零一千");
    test!(10_0000_0000f64, "一秭");
    test!(1_0000_0000_0000f64, "一澗");
    test!(1_0000_1000_0000f64, "一澗零一京");
    test!(10_0000_0000_0000f64, "一正");
    test!(1000_0000_0000_0000f64, "一極");
    test!(1000_1000_0000_0000f64, "一極零一溝");
    test!(-1000_1000_0000_0000f64, "負一極零一溝");
}

#[test]
fn test_chinese_to_f64_ten_thousand() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_f64(ChineseCountMethod::TenThousand, &to_chars_vec($value)).unwrap()
            );
        };
    }

    test!(0f64, "零");
    test!(0.01f64, "一分");
    test!(0.1f64, "一角");
    test!(0.11f64, "一角一分");
    test!(1f64, "一");
    test!(1.23f64, "一二角三分");
    test!(2f64, "貳");
    test!(10f64, "十");
    test!(11f64, "十一");
    test!(20f64, "二十");
    test!(100f64, "一百");
    test!(101f64, "一百零一");
    test!(110f64, "一百一十");
    test!(111f64, "一百一十一");
    test!(200f64, "兩百");
    test!(1000f64, "一千");
    test!(1001f64, "一千零一");
    test!(1010f64, "一千零一十");
    test!(1011f64, "一千零一十一");
    test!(1100f64, "一千一百");
    test!(1101f64, "一千一百零一");
    test!(1110f64, "一千一百一十");
    test!(2000f64, "二千");
    test!(1_0000f64, "一萬");
    test!(1_0001f64, "一萬零一");
    test!(10_0000f64, "十萬");
    test!(10_0100f64, "十萬零一百");
    test!(1_0000_0000f64, "一億");
    test!(1_0000_1000f64, "一億零一千");
    test!(10_0000_0000f64, "十億");
    test!(1_0000_0000_0000f64, "一兆");
    test!(1_0000_1000_0000f64, "一兆零一千萬");
    test!(10_0000_0000_0000f64, "十兆");
    test!(1_0000_0000_0000_0000f64, "一京");
    test!(1_0000_1000_0000_0000f64, "一京零一千億");
    test!(-1_0000_1000_0000_0000f64, "負一京零一千億");
}

#[test]
fn test_chinese_to_f64_middle() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_f64(ChineseCountMethod::Middle, &to_chars_vec($value)).unwrap()
            );
        };
    }

    test!(0f64, "零");
    test!(1f64, "一");
    test!(2f64, "貳");
    test!(10f64, "十");
    test!(11f64, "十一");
    test!(20f64, "二十");
    test!(100f64, "一百");
    test!(101f64, "一百零一");
    test!(110f64, "一百一十");
    test!(111f64, "一百一十一");
    test!(200f64, "兩百");
    test!(1000f64, "一千");
    test!(1001f64, "一千零一");
    test!(1010f64, "一千零一十");
    test!(1011f64, "一千零一十一");
    test!(1100f64, "一千一百");
    test!(1101f64, "一千一百零一");
    test!(1110f64, "一千一百一十");
    test!(2000f64, "二千");
    test!(1_0000f64, "一萬");
    test!(1_0001f64, "一萬零一");
    test!(10_0000f64, "十萬");
    test!(10_0100f64, "十萬零一百");
    test!(1_0000_0000f64, "一億");
    test!(1_0000_1000f64, "一億零一千");
    test!(10_0000_0000f64, "十億");
    test!(1_0000_0000_0000f64, "一萬億");
    test!(1_0000_1000_0000f64, "一萬億零一千萬");
    test!(10_0000_0000_0000f64, "十萬億");
    test!(1_0000_0000_0000_0000f64, "一兆");
    test!(1_0000_1000_0000_0000f64, "一兆零一千億");
    test!(10_0000_0000_0000_0000f64, "十兆");
    test!(1_0000_0000_0000_0000_0000f64, "一萬兆");
    test!(1_0000_0000_0000_0000_0000_0000f64, "一京");
    test!(-1_0000_0000_0000_0000_0000_0000f64, "負一京");
}

#[test]
fn test_chinese_to_f64_high() {
    macro_rules! test {
        ($expect:expr, $value:expr) => {
            assert_eq!(
                $expect,
                chinese_to_f64(ChineseCountMethod::High, &to_chars_vec($value)).unwrap()
            );
        };
    }

    test!(0f64, "零");
    test!(1f64, "一");
    test!(2f64, "貳");
    test!(10f64, "十");
    test!(11f64, "十一");
    test!(20f64, "二十");
    test!(100f64, "一百");
    test!(101f64, "一百零一");
    test!(110f64, "一百一十");
    test!(111f64, "一百一十一");
    test!(200f64, "兩百");
    test!(1000f64, "一千");
    test!(1001f64, "一千零一");
    test!(1010f64, "一千零一十");
    test!(1011f64, "一千零一十一");
    test!(1100f64, "一千一百");
    test!(1101f64, "一千一百零一");
    test!(1110f64, "一千一百一十");
    test!(2000f64, "二千");
    test!(1_0000f64, "一萬");
    test!(1_0001f64, "一萬零一");
    test!(10_0000f64, "十萬");
    test!(10_0100f64, "十萬零一百");
    test!(1_0000_0000f64, "一億");
    test!(1_0000_1000f64, "一億零一千");
    test!(10_0000_0000f64, "十億");
    test!(1_0000_0000_0000f64, "一萬億");
    test!(1_0000_1000_0000f64, "一萬億零一千萬");
    test!(10_0000_0000_0000f64, "十萬億");
    test!(1_0000_0000_0000_0000f64, "一兆");
    test!(1_0000_1000_0000_0000f64, "一兆零一千億");
    test!(10_0000_0000_0000_0000f64, "十兆");
    test!(1_0000_0000_0000_0000_0000f64, "一萬兆");
    test!(1_0000_0000_0000_0000_0000_0000f64, "一億兆");
    test!(1_0000_0000_0000_0000_0000_0000_0000f64, "一萬億兆");
    test!(1_0000_0000_0000_0000_0000_0000_0000_0000f64, "一京");
}
