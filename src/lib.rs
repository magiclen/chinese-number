/*!
# Chinese Number

Convert primitive numbers to Chinese numbers, or parse Chinese numbers to primitive numbers.

This crate can convert Rust's primitive number data types to Chinese numbers as strings. For example, **123** can be converted into **一百二十三** or **壹佰貳拾參**. It supports both Traditional Chinese and Simple Chinese.

## Example

```rust
extern crate chinese_number;

use chinese_number::{ChineseNumber, ChineseVariant, ChineseNumberToNumber, ChineseBigNumberCountMethod};

assert_eq!("壹佰貳拾參", 123i8.to_uppercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("壹佰贰拾参", 123i8.to_uppercase_ten_thousand(ChineseVariant::Simple));

assert_eq!("一百二十三", 123i8.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!("十二萬三千四百五十六億七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_high(ChineseVariant::Traditional));
assert_eq!("十二萬三千四百五十六京七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_middle(ChineseVariant::Traditional));
assert_eq!("十二穰三千四百五十六秭七千八百九十垓一千二百三十四京五千六百七十八兆九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("一极二载三正四涧五沟六穰七秭八垓九京零一亿二万三千四百五十六", 1234567890123456i64.to_lowercase_low(ChineseVariant::Simple));

assert_eq!("一角二分", 0.12f64.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!(123i8, "一百二十三".parse_chinese_number(ChineseBigNumberCountMethod::TenThousand).unwrap());
assert_eq!(-30303i16, "負三萬零三百零三".parse_chinese_number(ChineseBigNumberCountMethod::TenThousand).unwrap());
assert_eq!(3212345678u32, "三十二億一千二百三十四萬五千六百七十八".parse_chinese_number(ChineseBigNumberCountMethod::TenThousand).unwrap());
assert_eq!(10010001001001001000u64, "一千零一京零一兆零一十億零一百萬一千".parse_chinese_number(ChineseBigNumberCountMethod::TenThousand).unwrap());

assert_eq!(1000000u64, "一兆".parse_chinese_number(ChineseBigNumberCountMethod::Low).unwrap());
assert_eq!(1000000000000u64, "一兆".parse_chinese_number(ChineseBigNumberCountMethod::TenThousand).unwrap());
assert_eq!(10000000000000000u64, "一兆".parse_chinese_number(ChineseBigNumberCountMethod::Middle).unwrap());
assert_eq!(10000000000000000u64, "一兆".parse_chinese_number(ChineseBigNumberCountMethod::High).unwrap());
```
*/
mod constants;

pub(crate) use constants::*;

mod chinese_variant;

pub use self::chinese_variant::ChineseVariant;

mod chinese_number_case;

pub use self::chinese_number_case::ChineseNumberCase;

mod chinese_big_number_count_method;

pub use self::chinese_big_number_count_method::ChineseBigNumberCountMethod;

mod inner_functions;

use self::inner_functions::*;

mod chinese_number_parse_error;

use self::chinese_number_parse_error::ChineseNumberParseError;

/// 將i8整數轉成中文數字。
pub fn from_i8(variant: ChineseVariant, case: ChineseNumberCase, value: i8) -> String {
    let mut s = String::new();

    from_i8_mut(variant, case, value, &mut s);

    s
}

/// 將i8整數轉成中文數字。
pub fn from_i8_mut(variant: ChineseVariant, case: ChineseNumberCase, value: i8, buffer: &mut String) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i8::min_value() {
            from_u8_mut(variant, case, -(value as i16) as u8, buffer)
        } else {
            from_u8_mut(variant, case, -value as u8, buffer)
        }
    } else {
        from_u8_mut(variant, case, value as u8, buffer)
    }
}

/// 將u8整數轉成中文數字。
pub fn from_u8(variant: ChineseVariant, case: ChineseNumberCase, value: u8) -> String {
    let mut s = String::new();

    from_u8_mut(variant, case, value, &mut s);

    s
}

/// 將u8整數轉成中文數字。
pub fn from_u8_mut(variant: ChineseVariant, case: ChineseNumberCase, value: u8, buffer: &mut String) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    if value >= 100 {
        digit_100(chinese_number_index, value as usize, buffer);
    } else if value >= 10 {
        digit_10(chinese_number_index, value as usize, buffer, false);
    } else {
        digit_1(chinese_number_index, value as usize, buffer);
    }
}

/// 將i16整數轉成中文數字。
pub fn from_i16(variant: ChineseVariant, case: ChineseNumberCase, value: i16) -> String {
    let mut s = String::new();

    from_i16_mut(variant, case, value, &mut s);

    s
}

/// 將i16整數轉成中文數字。
pub fn from_i16_mut(variant: ChineseVariant, case: ChineseNumberCase, value: i16, buffer: &mut String) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i16::min_value() {
            from_u16_mut(variant, case, -(value as i32) as u16, buffer)
        } else {
            from_u16_mut(variant, case, -value as u16, buffer)
        }
    } else {
        from_u16_mut(variant, case, value as u16, buffer)
    }
}

/// 將u16整數轉成中文數字。
pub fn from_u16(variant: ChineseVariant, case: ChineseNumberCase, value: u16) -> String {
    let mut s = String::new();

    from_u16_mut(variant, case, value, &mut s);

    s
}

/// 將u16整數轉成中文數字。
pub fn from_u16_mut(variant: ChineseVariant, case: ChineseNumberCase, value: u16, buffer: &mut String) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    digit_10000_compat(chinese_number_index, value as usize, buffer, false);
}

/// 將i32整數轉成中文數字。
pub fn from_i32(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: i32) -> String {
    let mut s = String::new();

    from_i32_mut(variant, case, method, value, &mut s);

    s
}

/// 將i32整數轉成中文數字。
pub fn from_i32_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: i32, buffer: &mut String) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i32::min_value() {
            from_u32_mut(variant, case, method, -(value as i64) as u32, buffer)
        } else {
            from_u32_mut(variant, case, method, -value as u32, buffer)
        }
    } else {
        from_u32_mut(variant, case, method, value as u32, buffer)
    }
}

/// 將u32整數轉成中文數字。
pub fn from_u32(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: u32) -> String {
    let mut s = String::new();

    from_u32_mut(variant, case, method, value, &mut s);

    s
}

/// 將u32整數轉成中文數字。
pub fn from_u32_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: u32, buffer: &mut String) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    match method {
        ChineseBigNumberCountMethod::Low => {
            digit_compat_low_u32(chinese_number_index, value, buffer);
        }
        ChineseBigNumberCountMethod::TenThousand | ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => {
            digit_compat_ten_thousand_u32(chinese_number_index, value, buffer);
        }
    }
}

/// 將i64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i64(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: i64) -> String {
    let mut s = String::new();

    from_i64_mut(variant, case, method, value, &mut s);

    s
}

/// 將i64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i64_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: i64, buffer: &mut String) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i64::min_value() {
            from_u64_mut(variant, case, method, -(value as i128) as u64, buffer)
        } else {
            from_u64_mut(variant, case, method, -value as u64, buffer)
        }
    } else {
        from_u64_mut(variant, case, method, value as u64, buffer)
    }
}

/// 將u64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u64(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: u64) -> String {
    let mut s = String::new();

    from_u64_mut(variant, case, method, value, &mut s);

    s
}

/// 將u64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u64_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: u64, buffer: &mut String) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    match method {
        ChineseBigNumberCountMethod::Low => {
            digit_compat_low_u64(chinese_number_index, value as u64, buffer);
        }
        ChineseBigNumberCountMethod::TenThousand => {
            digit_compat_ten_thousand_u64(chinese_number_index, value as u64, buffer);
        }
        ChineseBigNumberCountMethod::Middle => {
            digit_compat_middle_u64(chinese_number_index, value as u64, buffer);
        }
        ChineseBigNumberCountMethod::High => {
            digit_compat_high_u128(chinese_number_index, value as u128, buffer);
        }
    }
}

/// 將i128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i128(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: i128) -> String {
    let mut s = String::new();

    from_i128_mut(variant, case, method, value, &mut s);

    s
}

/// 將i128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i128_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: i128, buffer: &mut String) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i128::min_value() {
            from_u128_mut(variant, case, method, -((value + 1) as i128) as u128 + 1, buffer)
        } else {
            from_u128_mut(variant, case, method, -value as u128, buffer)
        }
    } else {
        from_u128_mut(variant, case, method, value as u128, buffer)
    }
}

/// 將u128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u128(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: u128) -> String {
    let mut s = String::new();

    from_u128_mut(variant, case, method, value, &mut s);

    s
}

/// 將u128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u128_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: u128, buffer: &mut String) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    match method {
        ChineseBigNumberCountMethod::Low => {
            assert!(value < 10000000000000000); // support to "極"
            digit_compat_low_u64(chinese_number_index, value as u64, buffer);
        }
        ChineseBigNumberCountMethod::TenThousand => {
            digit_compat_ten_thousand_u128(chinese_number_index, value as u128, buffer);
        }
        ChineseBigNumberCountMethod::Middle => {
            digit_compat_middle_u128(chinese_number_index, value as u128, buffer);
        }
        ChineseBigNumberCountMethod::High => {
            digit_compat_high_u128(chinese_number_index, value as u128, buffer);
        }
    }
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_isize(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: isize) -> String {
    from_i8(variant, case, value as i8)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_isize_mut(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: isize, buffer: &mut String) {
    from_i8_mut(variant, case, value as i8, buffer)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_isize(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: isize) -> String {
    from_i16(variant, case, value as i16)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_isize_mut(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: isize, buffer: &mut String) {
    from_i16_mut(variant, case, value as i16, buffer)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_isize(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: isize) -> String {
    from_i32(variant, case, method, value as i32)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_isize_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: isize, buffer: &mut String) {
    from_i32_mut(variant, case, method, value as i32, buffer)
}

/// 將isize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_isize(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: isize) -> String {
    from_i64(variant, case, method, value as i64)
}

/// 將isize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_isize_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: isize, buffer: &mut String) {
    from_i64_mut(variant, case, method, value as i64, buffer)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_usize(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: usize) -> String {
    from_u8(variant, case, value as u8)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_usize_mut(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: usize, buffer: &mut String) {
    from_u8_mut(variant, case, value as u8, buffer)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_usize(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: usize) -> String {
    from_u16(variant, case, value as u16)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_usize_mut(variant: ChineseVariant, case: ChineseNumberCase, _method: ChineseBigNumberCountMethod, value: usize, buffer: &mut String) {
    from_u16_mut(variant, case, value as u16, buffer)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_usize(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: usize) -> String {
    from_u32(variant, case, method, value as u32)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_usize_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: usize, buffer: &mut String) {
    from_u32_mut(variant, case, method, value as u32, buffer)
}

/// 將usize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_usize(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: usize) -> String {
    from_u64(variant, case, method, value as u64)
}

/// 將usize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_usize_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: usize, buffer: &mut String) {
    from_u64_mut(variant, case, method, value as u64, buffer)
}

/// 將f64浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f64(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: f64) -> String {
    let mut s = String::new();

    from_f64_mut(variant, case, method, value, &mut s);

    s
}

/// 將f64浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f64_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, mut value: f64, buffer: &mut String) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    if value < 0.0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);
        value = -value;
    }


    match method {
        ChineseBigNumberCountMethod::Low => {
            fraction_compat_low(chinese_number_index, value, buffer);
        }
        ChineseBigNumberCountMethod::TenThousand => {
            fraction_compat_ten_thousand(chinese_number_index, value, buffer);
        }
        ChineseBigNumberCountMethod::Middle => {
            fraction_compat_middle(chinese_number_index, value, buffer);
        }
        ChineseBigNumberCountMethod::High => {
            fraction_compat_high(chinese_number_index, value, buffer);
        }
    }
}

/// 將f32浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f32(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: f32) -> String {
    from_f64(variant, case, method, value as f64)
}

/// 將f32浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f32_mut(variant: ChineseVariant, case: ChineseNumberCase, method: ChineseBigNumberCountMethod, value: f32, buffer: &mut String) {
    from_f64_mut(variant, case, method, value as f64, buffer);
}

#[cfg(test)]
mod tests;

/// 讓Rust程式語言的所有基本數值型別擁有中文數字的轉換能力。
pub trait ChineseNumber {
    /// 轉成大寫數字，使用 **「上數」** 作為單位標準。
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「上數」** 作為單位標準。
    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成大寫數字，使用 **「中數」** 作為單位標準。
    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「中數」** 作為單位標準。
    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成大寫數字，使用 **「萬進」** 作為單位標準。
    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「萬進」** 作為單位標準。
    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成大寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_uppercase_low(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「上數」** 作為單位標準。
    fn to_lowercase_high(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「上數」** 作為單位標準。
    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「中數」** 作為單位標準。
    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「中數」** 作為單位標準。
    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「萬進」** 作為單位標準。
    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「萬進」** 作為單位標準。
    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_lowercase_low(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String);
}

impl ChineseNumber for i8 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}


impl ChineseNumber for u8 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}


impl ChineseNumber for i16 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}


impl ChineseNumber for u16 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}


impl ChineseNumber for i32 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for u32 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for i64 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for u64 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for i128 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for u128 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for isize {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for usize {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for f64 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

impl ChineseNumber for f32 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Upper, ChineseBigNumberCountMethod::Low, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::High, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Middle, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::TenThousand, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(variant, ChineseNumberCase::Lower, ChineseBigNumberCountMethod::Low, *self, buffer)
    }
}

/// 將中文數字轉成i8數值。
pub fn parse_chinese_number_to_i8<S: AsRef<str>>(chinese_number: S) -> Result<i8, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => match chinese_digit_100_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i8::max_value() as u16 + 1 {
                                Err(ChineseNumberParseError::Underflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 6
                                    })
                                } else {
                                    Ok((number as i16 * -1) as i8)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                    None => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: 1
                    })
                }
            } else {
                match chinese_digit_100_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > i8::max_value() as u16 {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 5
                                })
                            } else {
                                Ok(number as i8)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成u8數值。
pub fn parse_chinese_number_to_u8<S: AsRef<str>>(chinese_number: S) -> Result<u8, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match chinese_digit_100_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next()) {
                Ok(number) => {
                    if number > u8::max_value() as u16 {
                        Err(ChineseNumberParseError::Overflow)
                    } else {
                        if let Some(_) = chars.next() {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: 5
                            })
                        } else {
                            Ok(number as u8)
                        }
                    }
                }
                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                    char_index: err
                })
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成i16數值。
pub fn parse_chinese_number_to_i16<S: AsRef<str>>(chinese_number: S) -> Result<i16, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => match chinese_digit_10000_ten_thousand_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i16::max_value() as u32 + 1 {
                                Err(ChineseNumberParseError::Underflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 16
                                    })
                                } else {
                                    Ok((number as i32 * -1) as i16)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                    None => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: 1
                    })
                }
            } else {
                match chinese_digit_10000_ten_thousand_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > i16::max_value() as u32 {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 15
                                })
                            } else {
                                Ok(number as i16)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成u16數值。
pub fn parse_chinese_number_to_u16<S: AsRef<str>>(chinese_number: S) -> Result<u16, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match chinese_digit_10000_ten_thousand_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                Ok(number) => {
                    if number > u16::max_value() as u32 {
                        Err(ChineseNumberParseError::Overflow)
                    } else {
                        if let Some(_) = chars.next() {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: 15
                            })
                        } else {
                            Ok(number as u16)
                        }
                    }
                }
                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                    char_index: err
                })
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成i32數值。
pub fn parse_chinese_number_to_i32<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<i32, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match method {
                            ChineseBigNumberCountMethod::Low => match chinese_digit_1000000000_low_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                                Ok(number) => {
                                    if number > i32::max_value() as u64 + 1 {
                                        Err(ChineseNumberParseError::Underflow)
                                    } else {
                                        if let Some(_) = chars.next() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 20
                                            })
                                        } else {
                                            Ok((number as i64 * -1) as i32)
                                        }
                                    }
                                }
                                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err + 1
                                })
                            }
                            ChineseBigNumberCountMethod::TenThousand | ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => match chinese_digit_100000000_ten_thousand_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                                Ok(number) => {
                                    if number > i32::max_value() as u64 + 1 {
                                        Err(ChineseNumberParseError::Underflow)
                                    } else {
                                        if let Some(_) = chars.next() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 24
                                            })
                                        } else {
                                            Ok((number as i64 * -1) as i32)
                                        }
                                    }
                                }
                                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err + 1
                                })
                            }
                        }
                    }
                    None => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: 1
                    })
                }
            } else {
                match method {
                    ChineseBigNumberCountMethod::Low => match chinese_digit_1000000000_low_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i32::max_value() as u64 {
                                Err(ChineseNumberParseError::Overflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 19
                                    })
                                } else {
                                    Ok(number as i32)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                    ChineseBigNumberCountMethod::TenThousand | ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => match chinese_digit_100000000_ten_thousand_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i32::max_value() as u64 {
                                Err(ChineseNumberParseError::Overflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 23
                                    })
                                } else {
                                    Ok(number as i32)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                }
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成u32數值。
pub fn parse_chinese_number_to_u32<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<u32, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match method {
                ChineseBigNumberCountMethod::Low => match chinese_digit_1000000000_low_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > u32::max_value() as u64 {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 19
                                })
                            } else {
                                Ok(number as u32)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
                ChineseBigNumberCountMethod::TenThousand | ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => match chinese_digit_100000000_ten_thousand_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > u32::max_value() as u64 {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 23
                                })
                            } else {
                                Ok(number as u32)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成i64數值。
pub fn parse_chinese_number_to_i64<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<i64, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match method {
                            ChineseBigNumberCountMethod::Low => match chinese_digit_1000000000000000_low_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                                Ok(number) => {
                                    if let Some(_) = chars.next() {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: 32
                                        })
                                    } else {
                                        Ok(number as i64 * -1)
                                    }
                                }
                                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err + 1
                                })
                            }
                            ChineseBigNumberCountMethod::TenThousand => match chinese_digit_10000000000000000_ten_thousand_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                                Ok(number) => {
                                    if number > i64::max_value() as u128 + 1 {
                                        Err(ChineseNumberParseError::Underflow)
                                    } else {
                                        if let Some(_) = chars.next() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 40
                                            })
                                        } else {
                                            Ok((number as i128 * -1) as i64)
                                        }
                                    }
                                }
                                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err + 1
                                })
                            }
                            ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => match chinese_digit_10000000000000000_middle_compat(next_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                                Ok(number) => {
                                    if number > i64::max_value() as u128 + 1 {
                                        Err(ChineseNumberParseError::Underflow)
                                    } else {
                                        if let Some(_) = chars.next() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 48
                                            })
                                        } else {
                                            Ok((number as i128 * -1) as i64)
                                        }
                                    }
                                }
                                Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err + 1
                                })
                            }
                        }
                    }
                    None => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: 1
                    })
                }
            } else {
                match method {
                    ChineseBigNumberCountMethod::Low => match chinese_digit_1000000000000000_low_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i64::max_value() as u64 {
                                Err(ChineseNumberParseError::Overflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 31
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                    ChineseBigNumberCountMethod::TenThousand => match chinese_digit_10000000000000000_ten_thousand_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i64::max_value() as u128 {
                                Err(ChineseNumberParseError::Overflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 39
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                    ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => match chinese_digit_10000000000000000_middle_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                        Ok(number) => {
                            if number > i64::max_value() as u128 {
                                Err(ChineseNumberParseError::Overflow)
                            } else {
                                if let Some(_) = chars.next() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 47
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                        }
                        Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err
                        })
                    }
                }
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成u64數值。
pub fn parse_chinese_number_to_u64<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<u64, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match method {
                ChineseBigNumberCountMethod::Low => match chinese_digit_1000000000000000_low_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > u64::max_value() {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 31
                                })
                            } else {
                                Ok(number)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
                ChineseBigNumberCountMethod::TenThousand => match chinese_digit_10000000000000000_ten_thousand_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > u64::max_value() as u128 {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 39
                                })
                            } else {
                                Ok(number as u64)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
                ChineseBigNumberCountMethod::Middle | ChineseBigNumberCountMethod::High => match chinese_digit_10000000000000000_middle_compat(first_char, chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
                    Ok(number) => {
                        if number > u64::max_value() as u128 {
                            Err(ChineseNumberParseError::Overflow)
                        } else {
                            if let Some(_) = chars.next() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 47
                                })
                            } else {
                                Ok(number as u64)
                            }
                        }
                    }
                    Err(err) => Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err
                    })
                }
            }
        }
        None => {
            Err(ChineseNumberParseError::ChineseNumberEmpty)
        }
    }
}

/// 將中文數字轉成i128數值。
pub fn parse_chinese_number_to_i128<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, _chinese_number: S) -> Result<i128, ChineseNumberParseError> {
    unimplemented!()
}

/// 將中文數字轉成u128數值。
pub fn parse_chinese_number_to_u128<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, _chinese_number: S) -> Result<u128, ChineseNumberParseError> {
    unimplemented!()
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "8")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i8(chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "8")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u8(chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "16")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i16(chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "16")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u16(chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "32")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i32(method, chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "32")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u32(method, chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "64")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i64(method, chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "64")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(method: ChineseBigNumberCountMethod, chinese_number: S) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u64(method, chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成f64數值。
pub fn parse_chinese_number_to_f64<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, _chinese_number: S) -> Result<f64, ChineseNumberParseError> {
    unimplemented!()
}

/// 將中文數字轉成f32數值。
pub fn parse_chinese_number_to_f32<S: AsRef<str>>(_method: ChineseBigNumberCountMethod, _chinese_number: S) -> Result<f32, ChineseNumberParseError> {
    unimplemented!()
}

/// 讓Rust程式語言的字串型別擁有中文數字的轉換能力。
pub trait ChineseNumberToNumber<T> {
    /// 將中文數字轉成基本型別之數值。
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<T, ChineseNumberParseError>;
}

impl ChineseNumberToNumber<i8> for String {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<i8, ChineseNumberParseError> {
        parse_chinese_number_to_i8(self)
    }
}

impl ChineseNumberToNumber<u8> for String {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<u8, ChineseNumberParseError> {
        parse_chinese_number_to_u8(self)
    }
}

impl ChineseNumberToNumber<i16> for String {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<i16, ChineseNumberParseError> {
        parse_chinese_number_to_i16(self)
    }
}

impl ChineseNumberToNumber<u16> for String {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<u16, ChineseNumberParseError> {
        parse_chinese_number_to_u16(self)
    }
}

impl ChineseNumberToNumber<i32> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<i32, ChineseNumberParseError> {
        parse_chinese_number_to_i32(method, self)
    }
}

impl ChineseNumberToNumber<u32> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<u32, ChineseNumberParseError> {
        parse_chinese_number_to_u32(method, self)
    }
}

impl ChineseNumberToNumber<i64> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<i64, ChineseNumberParseError> {
        parse_chinese_number_to_i64(method, self)
    }
}

impl ChineseNumberToNumber<u64> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<u64, ChineseNumberParseError> {
        parse_chinese_number_to_u64(method, self)
    }
}

impl ChineseNumberToNumber<i128> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<i128, ChineseNumberParseError> {
        parse_chinese_number_to_i128(method, self)
    }
}

impl ChineseNumberToNumber<u128> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<u128, ChineseNumberParseError> {
        parse_chinese_number_to_u128(method, self)
    }
}

impl ChineseNumberToNumber<isize> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<isize, ChineseNumberParseError> {
        parse_chinese_number_to_isize(method, self)
    }
}

impl ChineseNumberToNumber<usize> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<usize, ChineseNumberParseError> {
        parse_chinese_number_to_usize(method, self)
    }
}

impl ChineseNumberToNumber<f64> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<f64, ChineseNumberParseError> {
        parse_chinese_number_to_f64(method, self)
    }
}

impl ChineseNumberToNumber<f32> for String {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<f32, ChineseNumberParseError> {
        parse_chinese_number_to_f32(method, self)
    }
}

impl<'a> ChineseNumberToNumber<i8> for &'a str {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<i8, ChineseNumberParseError> {
        parse_chinese_number_to_i8(self)
    }
}

impl<'a> ChineseNumberToNumber<u8> for &'a str {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<u8, ChineseNumberParseError> {
        parse_chinese_number_to_u8(self)
    }
}

impl<'a> ChineseNumberToNumber<i16> for &'a str {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<i16, ChineseNumberParseError> {
        parse_chinese_number_to_i16(self)
    }
}

impl<'a> ChineseNumberToNumber<u16> for &'a str {
    fn parse_chinese_number(&self, _method: ChineseBigNumberCountMethod) -> Result<u16, ChineseNumberParseError> {
        parse_chinese_number_to_u16(self)
    }
}

impl<'a> ChineseNumberToNumber<i32> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<i32, ChineseNumberParseError> {
        parse_chinese_number_to_i32(method, self)
    }
}

impl<'a> ChineseNumberToNumber<u32> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<u32, ChineseNumberParseError> {
        parse_chinese_number_to_u32(method, self)
    }
}

impl<'a> ChineseNumberToNumber<i64> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<i64, ChineseNumberParseError> {
        parse_chinese_number_to_i64(method, self)
    }
}

impl<'a> ChineseNumberToNumber<u64> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<u64, ChineseNumberParseError> {
        parse_chinese_number_to_u64(method, self)
    }
}

impl<'a> ChineseNumberToNumber<i128> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<i128, ChineseNumberParseError> {
        parse_chinese_number_to_i128(method, self)
    }
}

impl<'a> ChineseNumberToNumber<u128> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<u128, ChineseNumberParseError> {
        parse_chinese_number_to_u128(method, self)
    }
}

impl<'a> ChineseNumberToNumber<isize> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<isize, ChineseNumberParseError> {
        parse_chinese_number_to_isize(method, self)
    }
}

impl<'a> ChineseNumberToNumber<usize> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<usize, ChineseNumberParseError> {
        parse_chinese_number_to_usize(method, self)
    }
}

impl<'a> ChineseNumberToNumber<f64> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<f64, ChineseNumberParseError> {
        parse_chinese_number_to_f64(method, self)
    }
}

impl<'a> ChineseNumberToNumber<f32> for &'a str {
    fn parse_chinese_number(&self, method: ChineseBigNumberCountMethod) -> Result<f32, ChineseNumberParseError> {
        parse_chinese_number_to_f32(method, self)
    }
}