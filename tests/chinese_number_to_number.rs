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
    assert_eq!(65535u16, "六萬五千五百三十五".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
}