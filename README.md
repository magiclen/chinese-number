Chinese Number
====================

[![CI](https://github.com/magiclen/chinese-number/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/chinese-number/actions/workflows/ci.yml)

Convert primitive numbers to Chinese numbers, or parse Chinese numbers to primitive numbers.

This crate can convert Rust's primitive number data types to Chinese numbers as strings. For example, **123** can be converted into **一二三**, **一百二十三** or **壹佰貳拾參**. It supports both Traditional Chinese and Simple Chinese, and it supports different methods to count the scale as well. Also, Chinese numbers in strings can be parsed to primitive number data types.

## Example

```rust
use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese, ChineseToNumber};

assert_eq!("一二三", 123i8.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower));

assert_eq!("壹佰貳拾參", 123i8.to_chinese(ChineseVariant::Traditional, ChineseCase::Upper, ChineseCountMethod::TenThousand).unwrap());
assert_eq!("壹佰贰拾参", 123i8.to_chinese(ChineseVariant::Simple, ChineseCase::Upper, ChineseCountMethod::TenThousand).unwrap());

assert_eq!("一百二十三", 123i8.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::TenThousand).unwrap());

assert_eq!("一極二載三正四澗五溝六穰七秭八垓九京零一億二萬三千四百五十六", 1234567890123456i64.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::Low).unwrap());
assert_eq!("十二穰三千四百五十六秭七千八百九十垓一千二百三十四京五千六百七十八兆九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::TenThousand).unwrap());
assert_eq!("十二萬三千四百五十六京七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::Middle).unwrap());
assert_eq!("十二萬三千四百五十六億七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::High).unwrap());

assert_eq!("一角二分", 0.12f64.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::TenThousand).unwrap());

assert_eq!(123i8, "一二三".to_number_naive().unwrap());

assert_eq!(123i8, "一百二十三".to_number(ChineseCountMethod::TenThousand).unwrap());
assert_eq!(-30303i16, "負三萬零三百零三".to_number(ChineseCountMethod::TenThousand).unwrap());
assert_eq!(3212345678u32, "三十二億一千二百三十四萬五千六百七十八".to_number(ChineseCountMethod::TenThousand).unwrap());
assert_eq!(10010001001001001000u64, "一千零一京零一兆零一十億零一百萬一千".to_number(ChineseCountMethod::TenThousand).unwrap());

assert_eq!(1000000u64, "一兆".to_number(ChineseCountMethod::Low).unwrap());
assert_eq!(1000000000000u64, "一兆".to_number(ChineseCountMethod::TenThousand).unwrap());
assert_eq!(10000000000000000u64, "一兆".to_number(ChineseCountMethod::Middle).unwrap());
assert_eq!(10000000000000000u64, "一兆".to_number(ChineseCountMethod::High).unwrap());
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.chinese-number]
version = "*"
default-features = false
features = ["number-to-chinese", "chinese-to-number"]
```

## Crates.io

https://crates.io/crates/chinese-number

## Documentation

https://docs.rs/chinese-number

## License

[MIT](LICENSE)