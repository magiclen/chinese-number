#![allow(clippy::unreadable_literal, clippy::cognitive_complexity)]

extern crate chinese_number;

#[macro_use]
extern crate assert_approx_eq;

use chinese_number::{ChineseNumberCountMethod, ChineseNumberToNumber};

#[test]
fn i8() {
    assert_eq!(0i8, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1i8, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i8, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i8, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12i8, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108i8,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127i8,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        -128i8,
        "負一百二十八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
}

#[test]
fn u8() {
    assert_eq!(0u8, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1u8, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u8, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u8, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12u8, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108u8,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127u8,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        255u8,
        "二百五十五".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
}

#[test]
fn i16() {
    assert_eq!(0i16, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1i16, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i16, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i16, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12i16, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108i16,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127i16,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000i16,
        "一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1001i16,
        "一千零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1010i16,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1011i16,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1100i16,
        "一千一百".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1101i16,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1110i16,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1111i16,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000i16,
        "一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        32767i16,
        "三萬二千七百六十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        -32768i16,
        "負三萬二千七百六十八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
}

#[test]
fn u16() {
    assert_eq!(0u16, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1u16, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u16, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u16, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12u16, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108u16,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127u16,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000u16,
        "一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1001u16,
        "一千零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1010u16,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1011u16,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1100u16,
        "一千一百".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1101u16,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1110u16,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1111u16,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000u16,
        "一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        65535u16,
        "六萬五千五百三十五".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
}

#[test]
fn i32_low() {
    assert_eq!(0i32, "零".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1i32, "一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10i32, "十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10i32, "一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(12i32, "十二".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(108i32, "一百零八".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(127i32, "一百二十七".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000i32, "一千".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1001i32, "一千零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1010i32, "一千零一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1011i32,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(1100i32, "一千一百".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1101i32,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1110i32,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1111i32,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(10000i32, "一萬".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000i32, "一億".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000i32, "一兆".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10000000i32, "一京".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000000i32, "一垓".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000000i32, "一秭".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        2147483647i32,
        "二秭一垓四京七兆四億八萬三千六百四十七"
            .parse_chinese_number(ChineseNumberCountMethod::Low)
            .unwrap()
    );
    assert_eq!(
        -2147483648i32,
        "負二秭一垓四京七兆四億八萬三千六百四十八"
            .parse_chinese_number(ChineseNumberCountMethod::Low)
            .unwrap()
    );
}

#[test]
fn u32_low() {
    assert_eq!(0u32, "零".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1u32, "一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10u32, "十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10u32, "一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(12u32, "十二".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(108u32, "一百零八".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(127u32, "一百二十七".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000u32, "一千".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1001u32, "一千零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1010u32, "一千零一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1011u32,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(1100u32, "一千一百".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1101u32,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1110u32,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1111u32,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(10000u32, "一萬".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000u32, "一億".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000u32, "一兆".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10000000u32, "一京".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000000u32, "一垓".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000000u32, "一秭".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        4294967295u32,
        "四秭二垓九京四兆九億六萬七千二百九十五"
            .parse_chinese_number(ChineseNumberCountMethod::Low)
            .unwrap()
    );
}

#[test]
fn i32_ten_thousand() {
    assert_eq!(0i32, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1i32, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i32, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i32, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12i32, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108i32,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127i32,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000i32,
        "一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1001i32,
        "一千零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1010i32,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1011i32,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1100i32,
        "一千一百".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1101i32,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1110i32,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1111i32,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000i32,
        "一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000i32,
        "十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000i32,
        "一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000001i32,
        "一千萬零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10010000i32,
        "一千零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10100000i32,
        "一千零一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10110000i32,
        "一千零一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11000000i32,
        "一千一百萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11010000i32,
        "一千一百零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11100000i32,
        "一千一百一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11110000i32,
        "一千一百一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        2147483647i32,
        "二十一億四千七百四十八萬三千六百四十七"
            .parse_chinese_number(ChineseNumberCountMethod::TenThousand)
            .unwrap()
    );
    assert_eq!(
        -2147483648i32,
        "負二十一億四千七百四十八萬三千六百四十八"
            .parse_chinese_number(ChineseNumberCountMethod::TenThousand)
            .unwrap()
    );
}

#[test]
fn u32_ten_thousand() {
    assert_eq!(0u32, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1u32, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u32, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u32, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12u32, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108u32,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127u32,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000u32,
        "一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1001u32,
        "一千零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1010u32,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1011u32,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1100u32,
        "一千一百".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1101u32,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1110u32,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1111u32,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000u32,
        "一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000u32,
        "十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000u32,
        "一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000001u32,
        "一千萬零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10010000u32,
        "一千零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10100000u32,
        "一千零一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10110000u32,
        "一千零一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11000000u32,
        "一千一百萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11010000u32,
        "一千一百零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11100000u32,
        "一千一百一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11110000u32,
        "一千一百一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        4294967295u32,
        "四十二億九千四百九十六萬七千二百九十五"
            .parse_chinese_number(ChineseNumberCountMethod::TenThousand)
            .unwrap()
    );
}

#[test]
fn i64_low() {
    assert_eq!(0i64, "零".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1i64, "一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10i64, "十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10i64, "一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(12i64, "十二".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(108i64, "一百零八".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(127i64, "一百二十七".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000i64, "一千".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1001i64, "一千零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1010i64, "一千零一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1011i64,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(1100i64, "一千一百".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1101i64,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1110i64,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1111i64,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(10000i64, "一萬".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000i64, "一億".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000i64, "一兆".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10000000i64, "一京".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000000i64, "一垓".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000000i64, "一秭".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10000000000i64, "一穰".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        100000000000i64,
        "一溝".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1000000000000i64,
        "一澗".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        10000000000000i64,
        "一正".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        100000000000000i64,
        "一載".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        -1000000000000000i64,
        "負一極".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
}

#[test]
fn u64_low() {
    assert_eq!(0u64, "零".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1u64, "一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10u64, "十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10u64, "一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(12u64, "十二".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(108u64, "一百零八".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(127u64, "一百二十七".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000u64, "一千".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1001u64, "一千零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1010u64, "一千零一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1011u64,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(1100u64, "一千一百".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        1101u64,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1110u64,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1111u64,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(10000u64, "一萬".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000u64, "一億".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10000000u64, "一京".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(100000000u64, "一垓".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(1000000000u64, "一秭".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(10000000000u64, "一穰".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
    assert_eq!(
        100000000000u64,
        "一溝".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1000000000000u64,
        "一澗".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        10000000000000u64,
        "一正".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        100000000000000u64,
        "一載".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
    assert_eq!(
        1000000000000000u64,
        "一極".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap()
    );
}

#[test]
fn i64_ten_thousand() {
    assert_eq!(0i64, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1i64, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i64, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10i64, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12i64, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108i64,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127i64,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000i64,
        "一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1001i64,
        "一千零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1010i64,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1011i64,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1100i64,
        "一千一百".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1101i64,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1110i64,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1111i64,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000i64,
        "一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000i64,
        "十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000i64,
        "一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000001i64,
        "一千萬零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10010000i64,
        "一千零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10100000i64,
        "一千零一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10110000i64,
        "一千零一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11000000i64,
        "一千一百萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11010000i64,
        "一千一百零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11100000i64,
        "一千一百一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11110000i64,
        "一千一百一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000000i64,
        "一億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000001i64,
        "一億零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000i64,
        "十億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000i64,
        "一百億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000000000i64,
        "一千億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000000i64,
        "一兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000001i64,
        "一兆零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000000i64,
        "十兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000000000000i64,
        "一百兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000000000i64,
        "一千兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000000000i64,
        "一京".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000000001i64,
        "一京零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        9223372036854775807i64,
        "九百二十二京三千三百七十二兆零三百六十八億五千四百七十七萬五千八百零七"
            .parse_chinese_number(ChineseNumberCountMethod::TenThousand)
            .unwrap()
    );
    assert_eq!(
        -9223372036854775808i64,
        "負九百二十二京三千三百七十二兆零三百六十八億五千四百七十七萬五千八百零八"
            .parse_chinese_number(ChineseNumberCountMethod::TenThousand)
            .unwrap()
    );
}

#[test]
fn u64_ten_thousand() {
    assert_eq!(0u64, "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(1u64, "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u64, "十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(10u64, "一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(12u64, "十二".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
    assert_eq!(
        108u64,
        "一百零八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        127u64,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000u64,
        "一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1001u64,
        "一千零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1010u64,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1011u64,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1100u64,
        "一千一百".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1101u64,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1110u64,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1111u64,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000u64,
        "一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000u64,
        "十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000u64,
        "一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000001u64,
        "一千萬零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10010000u64,
        "一千零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10100000u64,
        "一千零一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10110000u64,
        "一千零一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11000000u64,
        "一千一百萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11010000u64,
        "一千一百零一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11100000u64,
        "一千一百一十萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        11110000u64,
        "一千一百一十一萬".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000000u64,
        "一億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000001u64,
        "一億零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000u64,
        "十億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000u64,
        "一百億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000000000u64,
        "一千億".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000000u64,
        "一兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000001u64,
        "一兆零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000000u64,
        "十兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        100000000000000u64,
        "一百兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        1000000000000000u64,
        "一千兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000000000u64,
        "一京".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        10000000000000001u64,
        "一京零一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap()
    );
    assert_eq!(
        18446744073709551615u64,
        "一千八百四十四京六千七百四十四兆零七百三十七億零九百五十五萬一千六百一十五"
            .parse_chinese_number(ChineseNumberCountMethod::TenThousand)
            .unwrap()
    );
}

#[test]
fn i64_middle() {
    assert_eq!(0i64, "零".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(1i64, "一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(10i64, "十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(10i64, "一十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(12i64, "十二".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(108i64, "一百零八".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        127i64,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(1000i64, "一千".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(1001i64, "一千零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        1010i64,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1011i64,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(1100i64, "一千一百".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        1101i64,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1110i64,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1111i64,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(10000i64, "一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(100000i64, "十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(100000i64, "一十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        10000001i64,
        "一千萬零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10010000i64,
        "一千零一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10100000i64,
        "一千零一十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10110000i64,
        "一千零一十一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11000000i64,
        "一千一百萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11010000i64,
        "一千一百零一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11100000i64,
        "一千一百一十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11110000i64,
        "一千一百一十一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000000i64,
        "一億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000001i64,
        "一億零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000i64,
        "十億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000i64,
        "一百億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000000000i64,
        "一千億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000000i64,
        "一萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000001i64,
        "一萬億零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000000i64,
        "十萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000000000000i64,
        "一百萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000000000i64,
        "一千萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000000000i64,
        "一兆".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000000001i64,
        "一兆零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        9223372036854775807i64,
        "九百二十二兆三千三百七十二萬零三百六十八億五千四百七十七萬五千八百零七"
            .parse_chinese_number(ChineseNumberCountMethod::Middle)
            .unwrap()
    );
    assert_eq!(
        -9223372036854775808i64,
        "負九百二十二兆三千三百七十二萬零三百六十八億五千四百七十七萬五千八百零八"
            .parse_chinese_number(ChineseNumberCountMethod::Middle)
            .unwrap()
    );
}

#[test]
fn u64_middle() {
    assert_eq!(0u64, "零".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(1u64, "一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(10u64, "十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(10u64, "一十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(12u64, "十二".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(108u64, "一百零八".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        127u64,
        "一百二十七".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(1000u64, "一千".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(1001u64, "一千零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        1010u64,
        "一千零一十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1011u64,
        "一千零一十一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(1100u64, "一千一百".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        1101u64,
        "一千一百零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1110u64,
        "一千一百一十".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1111u64,
        "一千一百一十一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(10000u64, "一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(100000u64, "十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(100000u64, "一十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
    assert_eq!(
        10000001u64,
        "一千萬零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10010000u64,
        "一千零一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10100000u64,
        "一千零一十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10110000u64,
        "一千零一十一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11000000u64,
        "一千一百萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11010000u64,
        "一千一百零一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11100000u64,
        "一千一百一十萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        11110000u64,
        "一千一百一十一萬".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000000u64,
        "一億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000001u64,
        "一億零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000u64,
        "十億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000u64,
        "一百億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000000000u64,
        "一千億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000000u64,
        "一萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000001u64,
        "一萬億零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000000u64,
        "十萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        100000000000000u64,
        "一百萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        1000000000000000u64,
        "一千萬億".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000000000u64,
        "一兆".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        10000000000000001u64,
        "一兆零一".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap()
    );
    assert_eq!(
        18446744073709551615u64,
        "一千八百四十四兆六千七百四十四萬零七百三十七億零九百五十五萬一千六百一十五"
            .parse_chinese_number(ChineseNumberCountMethod::Middle)
            .unwrap()
    );
}

#[test]
fn f64_ten_thousand() {
    assert_approx_eq!(0f64, {
        let a: f64 = "零".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap();

        a
    });
    assert_approx_eq!(1f64, {
        let a: f64 = "一".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap();
        a
    });
    assert_approx_eq!(1.2f64, {
        let a: f64 = "一二角".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap();
        a
    });
    assert_approx_eq!(1.23f64, {
        let a: f64 =
            "一二角三分".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap();
        a
    });
    assert_approx_eq!(1000.03f64, {
        let a: f64 =
            "一千三分".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap();
        a
    });
}
