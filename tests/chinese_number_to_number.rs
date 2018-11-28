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