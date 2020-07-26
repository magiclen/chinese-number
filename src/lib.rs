/*!
# Chinese Number

Convert primitive numbers to Chinese numbers, or parse Chinese numbers to primitive numbers.

This crate can convert Rust's primitive number data types to Chinese numbers as strings. For example, **123** can be converted into **一百二十三** or **壹佰貳拾參**. It supports both Traditional Chinese and Simple Chinese.

## Example

```rust
extern crate chinese_number;

use chinese_number::{ChineseNumber, ChineseVariant, ChineseNumberToNumber, ChineseNumberCountMethod};

assert_eq!("壹佰貳拾參", 123i8.to_uppercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("壹佰贰拾参", 123i8.to_uppercase_ten_thousand(ChineseVariant::Simple));

assert_eq!("一百二十三", 123i8.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!("十二萬三千四百五十六億七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_high(ChineseVariant::Traditional));
assert_eq!("十二萬三千四百五十六京七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_middle(ChineseVariant::Traditional));
assert_eq!("十二穰三千四百五十六秭七千八百九十垓一千二百三十四京五千六百七十八兆九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("一极二载三正四涧五沟六穰七秭八垓九京零一亿二万三千四百五十六", 1234567890123456i64.to_lowercase_low(ChineseVariant::Simple).unwrap());

assert_eq!("一角二分", 0.12f64.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!(123i8, "一百二十三".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(-30303i16, "負三萬零三百零三".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(3212345678u32, "三十二億一千二百三十四萬五千六百七十八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(10010001001001001000u64, "一千零一京零一兆零一十億零一百萬一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());

assert_eq!(1000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
assert_eq!(1000000000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(10000000000000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
assert_eq!(10000000000000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::High).unwrap());
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.chinese-number]
version = "*"
default-features = false
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

extern crate chinese_variant;

mod chinese_characters;
mod chinese_number_case;
mod chinese_number_count_method;
mod chinese_to_number;
mod number_to_chinese;
mod tests;

use alloc::string::String;

pub use chinese_number_case::*;
pub use chinese_number_count_method::*;
pub use chinese_variant::*;

pub use chinese_to_number::*;
pub use number_to_chinese::*;

/// 讓Rust程式語言的所有基本數值型別擁有中文數字的轉換能力。
pub trait ChineseNumber {
    /// 轉成大寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    #[inline]
    fn to_uppercase_low(
        &self,
        variant: ChineseVariant,
    ) -> Result<String, NumberToChineseNumberError> {
        let mut s = String::new();

        self.to_uppercase_low_to_string(variant, &mut s)?;

        Ok(s)
    }

    /// 轉成大寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_uppercase_low_to_string(
        &self,
        variant: ChineseVariant,
        buffer: &mut String,
    ) -> Result<(), NumberToChineseNumberError>;

    /// 轉成大寫數字，使用 **「萬進」** 作為單位標準。
    #[inline]
    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.to_uppercase_ten_thousand_to_string(variant, &mut s);

        s
    }

    /// 轉成大寫數字，使用 **「萬進」** 作為單位標準。
    fn to_uppercase_ten_thousand_to_string(&self, variant: ChineseVariant, buffer: &mut String);

    /// 轉成大寫數字，使用 **「中數」** 作為單位標準。
    #[inline]
    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.to_uppercase_middle_to_string(variant, &mut s);

        s
    }

    /// 轉成大寫數字，使用 **「中數」** 作為單位標準。
    fn to_uppercase_middle_to_string(&self, variant: ChineseVariant, buffer: &mut String);

    /// 轉成大寫數字，使用 **「上數」** 作為單位標準。
    #[inline]
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.to_uppercase_high_to_string(variant, &mut s);

        s
    }

    /// 轉成大寫數字，使用 **「上數」** 作為單位標準。
    fn to_uppercase_high_to_string(&self, variant: ChineseVariant, buffer: &mut String);

    /// 轉成小寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    #[inline]
    fn to_lowercase_low(
        &self,
        variant: ChineseVariant,
    ) -> Result<String, NumberToChineseNumberError> {
        let mut s = String::new();

        self.to_lowercase_low_to_string(variant, &mut s)?;

        Ok(s)
    }

    /// 轉成小寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_lowercase_low_to_string(
        &self,
        variant: ChineseVariant,
        buffer: &mut String,
    ) -> Result<(), NumberToChineseNumberError>;

    /// 轉成小寫數字，使用 **「萬進」** 作為單位標準。
    #[inline]
    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.to_lowercase_ten_thousand_to_string(variant, &mut s);

        s
    }

    /// 轉成小寫數字，使用 **「萬進」** 作為單位標準。
    fn to_lowercase_ten_thousand_to_string(&self, variant: ChineseVariant, buffer: &mut String);

    /// 轉成小寫數字，使用 **「中數」** 作為單位標準。
    #[inline]
    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.to_lowercase_middle_to_string(variant, &mut s);

        s
    }

    /// 轉成小寫數字，使用 **「中數」** 作為單位標準。
    fn to_lowercase_middle_to_string(&self, variant: ChineseVariant, buffer: &mut String);

    /// 轉成小寫數字，使用 **「上數」** 作為單位標準。
    #[inline]
    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.to_lowercase_high_to_string(variant, &mut s);

        s
    }

    /// 轉成小寫數字，使用 **「上數」** 作為單位標準。
    fn to_lowercase_high_to_string(&self, variant: ChineseVariant, buffer: &mut String);
}

macro_rules! chinese_number_impl_1 {
    (@unit $t:ty, $f:ident) => {
        impl ChineseNumber for $t {
        #[inline]
            fn to_uppercase_low_to_string(&self, variant: ChineseVariant, s: &mut String) -> Result<(), NumberToChineseNumberError> {
                $f(variant, ChineseNumberCase::Upper, *self, s);

                Ok(())
            }

            #[inline]
            fn to_uppercase_ten_thousand_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, *self, s);
            }

            #[inline]
            fn to_uppercase_middle_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, *self, s);
            }

            #[inline]
            fn to_uppercase_high_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, *self, s);
            }

            fn to_lowercase_low_to_string(&self, variant: ChineseVariant, s: &mut String) -> Result<(), NumberToChineseNumberError> {
                $f(variant, ChineseNumberCase::Lower, *self, s);

                Ok(())
            }

            #[inline]
            fn to_lowercase_ten_thousand_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, *self, s);
            }

            #[inline]
            fn to_lowercase_middle_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, *self, s);
            }

            #[inline]
            fn to_lowercase_high_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, *self, s);
            }
        }
    };
    ($($t:ty, $f:ident),* $(,)*) => {
        $(
            chinese_number_impl_1!(@unit $t, $f);
        )*
    };
}

macro_rules! chinese_number_impl_2 {
    (@unit $t:ty, $f:ident) => {
        impl ChineseNumber for $t {
        #[inline]
            fn to_uppercase_low_to_string(&self, variant: ChineseVariant, s: &mut String) -> Result<(), NumberToChineseNumberError> {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self, s);

                Ok(())
            }

            #[inline]
            fn to_uppercase_ten_thousand_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self, s);
            }

            #[inline]
            fn to_uppercase_middle_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self, s);
            }

            #[inline]
            fn to_uppercase_high_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self, s);
            }

            fn to_lowercase_low_to_string(&self, variant: ChineseVariant, s: &mut String) -> Result<(), NumberToChineseNumberError> {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self, s);

                Ok(())
            }

            #[inline]
            fn to_lowercase_ten_thousand_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self, s);
            }

            #[inline]
            fn to_lowercase_middle_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self, s);
            }

            #[inline]
            fn to_lowercase_high_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self, s);
            }
        }
    };
    ($($t:ty, $f:ident),* $(,)*) => {
        $(
            chinese_number_impl_2!(@unit $t, $f);
        )*
    };
}

macro_rules! chinese_number_impl_3 {
    (@unit $t:ty, $f:ident) => {
        impl ChineseNumber for $t {
        #[inline]
            fn to_uppercase_low_to_string(&self, variant: ChineseVariant, s: &mut String) -> Result<(), NumberToChineseNumberError> {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self, s)
            }

            #[inline]
            fn to_uppercase_ten_thousand_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self, s).unwrap();
            }

            #[inline]
            fn to_uppercase_middle_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self, s).unwrap();
            }

            #[inline]
            fn to_uppercase_high_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self, s).unwrap();
            }

            fn to_lowercase_low_to_string(&self, variant: ChineseVariant, s: &mut String) -> Result<(), NumberToChineseNumberError> {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self, s)
            }

            #[inline]
            fn to_lowercase_ten_thousand_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self, s).unwrap();
            }

            #[inline]
            fn to_lowercase_middle_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self, s).unwrap();
            }

            #[inline]
            fn to_lowercase_high_to_string(&self, variant: ChineseVariant, s: &mut String) {
                $f(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self, s).unwrap();
            }
        }
    };
    ($($t:ty, $f:ident),* $(,)*) => {
        $(
            chinese_number_impl_3!(@unit $t, $f);
        )*
    };
}

chinese_number_impl_1!(
    u8,
    from_u8_to_string,
    u16,
    from_u16_to_string,
    i8,
    from_i8_to_string,
    i16,
    from_i16_to_string,
);

chinese_number_impl_2!(u32, from_u32_to_string, i32, from_i32_to_string,);

chinese_number_impl_3!(
    u64,
    from_u64_to_string,
    u128,
    from_u128_to_string,
    usize,
    from_usize_to_string,
    i64,
    from_i64_to_string,
    i128,
    from_i128_to_string,
    isize,
    from_isize_to_string,
    f32,
    from_f32_to_string,
    f64,
    from_f64_to_string
);

// TODO

/// 讓Rust程式語言的字串型別擁有中文數字的轉換能力。
pub trait ChineseNumberToNumber<T> {
    /// 將中文數字轉成基本型別之數值。
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<T, ChineseNumberParseError>;
}

impl<T: AsRef<str>> ChineseNumberToNumber<u8> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<u8, ChineseNumberParseError> {
        parse_chinese_number_to_u8(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u16> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<u16, ChineseNumberParseError> {
        parse_chinese_number_to_u16(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u32> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<u32, ChineseNumberParseError> {
        parse_chinese_number_to_u32(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u64> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<u64, ChineseNumberParseError> {
        parse_chinese_number_to_u64(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u128> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<u128, ChineseNumberParseError> {
        parse_chinese_number_to_u128(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<usize> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<usize, ChineseNumberParseError> {
        parse_chinese_number_to_usize(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i8> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<i8, ChineseNumberParseError> {
        parse_chinese_number_to_i8(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i16> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<i16, ChineseNumberParseError> {
        parse_chinese_number_to_i16(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i32> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<i32, ChineseNumberParseError> {
        parse_chinese_number_to_i32(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i64> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<i64, ChineseNumberParseError> {
        parse_chinese_number_to_i64(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i128> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<i128, ChineseNumberParseError> {
        parse_chinese_number_to_i128(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<isize> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<isize, ChineseNumberParseError> {
        parse_chinese_number_to_isize(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<f32> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<f32, ChineseNumberParseError> {
        parse_chinese_number_to_f32(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<f64> for T {
    #[inline]
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<f64, ChineseNumberParseError> {
        parse_chinese_number_to_f64(method, self)
    }
}
