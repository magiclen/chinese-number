extern crate chinese_number;

use chinese_number::{ChineseNumberToNumber, ChineseBigNumberCountMethod};

#[test]
fn i8() {
    assert_eq!(0i8, "零".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1i8, "一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10i8, "十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10i8, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(12i8, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(108i8, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(127i8, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(-128i8, "負一百二十八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}

#[test]
fn u8() {
    assert_eq!(0u8, "零".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1u8, "一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10u8, "十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10u8, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(12u8, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(108u8, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(127u8, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(255u8, "二百五十五".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}

#[test]
fn i16() {
    assert_eq!(0i16, "零".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1i16, "一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10i16, "十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10i16, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(12i16, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(108i16, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(127i16, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1000i16, "一千".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1001i16, "一千零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1010i16, "一千零一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1011i16, "一千零一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1100i16, "一千一百".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1101i16, "一千一百零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1110i16, "一千一百一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1111i16, "一千一百一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10000i16, "一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(32767i16, "三萬二千七百六十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(-32768i16, "負三萬二千七百六十八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}

#[test]
fn u16() {
    assert_eq!(0u16, "零".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1u16, "一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10u16, "十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10u16, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(12u16, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(108u16, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(127u16, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1000u16, "一千".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1001u16, "一千零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1010u16, "一千零一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1011u16, "一千零一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1100u16, "一千一百".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1101u16, "一千一百零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1110u16, "一千一百一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1111u16, "一千一百一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10000u16, "一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(65535u16, "六萬五千五百三十五".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}

#[test]
fn i32_low() {
    assert_eq!(0i32, "零".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1i32, "一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10i32, "十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10i32, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(12i32, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(108i32, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(127i32, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1000i32, "一千".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1001i32, "一千零一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1010i32, "一千零一十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1011i32, "一千零一十一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1100i32, "一千一百".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1101i32, "一千一百零一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1110i32, "一千一百一十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1111i32, "一千一百一十一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10000i32, "一萬".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(100000i32, "一億".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1000000i32, "一兆".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10000000i32, "一京".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(100000000i32, "一垓".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1000000000i32, "一秭".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(2147483647i32, "二秭一垓四京七兆四億八萬三千六百四十七".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(-2147483648i32, "負二秭一垓四京七兆四億八萬三千六百四十八".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
}

#[test]
fn u32_low() {
    assert_eq!(0u32, "零".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1u32, "一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10u32, "十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10u32, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(12u32, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(108u32, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(127u32, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1000u32, "一千".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1001u32, "一千零一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1010u32, "一千零一十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1011u32, "一千零一十一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1100u32, "一千一百".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1101u32, "一千一百零一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1110u32, "一千一百一十".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1111u32, "一千一百一十一".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10000u32, "一萬".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(100000u32, "一億".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1000000u32, "一兆".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(10000000u32, "一京".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(100000000u32, "一垓".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(1000000000u32, "一秭".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
    assert_eq!(4294967295u32, "四秭二垓九京四兆九億六萬七千二百九十五".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
}

#[test]
fn i32_middle() {
    assert_eq!(0i32, "零".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1i32, "一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10i32, "十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10i32, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(12i32, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(108i32, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(127i32, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1000i32, "一千".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1001i32, "一千零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1010i32, "一千零一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1011i32, "一千零一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1100i32, "一千一百".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1101i32, "一千一百零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1110i32, "一千一百一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1111i32, "一千一百一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10000i32, "一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(100000i32, "十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(100000i32, "一十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10000001i32, "一千萬零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10010000i32, "一千零一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10100000i32, "一千零一十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10110000i32, "一千零一十一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11000000i32, "一千一百萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11010000i32, "一千一百零一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11100000i32, "一千一百一十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11110000i32, "一千一百一十一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(2147483647i32, "二十一億四千七百四十八萬三千六百四十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(-2147483648i32, "負二十一億四千七百四十八萬三千六百四十八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}

#[test]
fn u32_middle() {
    assert_eq!(0u32, "零".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1u32, "一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10u32, "十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10u32, "一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(12u32, "十二".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(108u32, "一百零八".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(127u32, "一百二十七".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1000u32, "一千".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1001u32, "一千零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1010u32, "一千零一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1011u32, "一千零一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1100u32, "一千一百".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1101u32, "一千一百零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1110u32, "一千一百一十".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(1111u32, "一千一百一十一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10000u32, "一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(100000u32, "十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(100000u32, "一十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10000001u32, "一千萬零一".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10010000u32, "一千零一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10100000u32, "一千零一十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(10110000u32, "一千零一十一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11000000u32, "一千一百萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11010000u32, "一千一百零一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11100000u32, "一千一百一十萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(11110000u32, "一千一百一十一萬".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
    assert_eq!(4294967295u32, "四十二億九千四百九十六萬七千二百九十五".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}