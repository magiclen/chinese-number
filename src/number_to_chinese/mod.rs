mod functions;
mod naive;
mod number_to_chinese_error;
mod traits;

mod functions_test;

use alloc::string::String;

use functions::*;
pub use naive::*;
pub use number_to_chinese_error::*;
pub use traits::*;

use crate::{ChineseCase, ChineseCountMethod, ChineseSign, ChineseVariant};

// TODO unsigned integer

/// 將 `u8` 整數轉成中文數字。
#[inline]
pub fn from_u8_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u8,
) -> String {
    from_u128_to_chinese_low(chinese_variant, chinese_case, value as u128).unwrap()
}

/// 將 `u16` 整數轉成中文數字。
#[inline]
pub fn from_u16_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u16,
) -> String {
    from_u128_to_chinese_low(chinese_variant, chinese_case, value as u128).unwrap()
}

/// 將 `u32` 整數轉成中文數字，使用 **「下數」**。
#[inline]
pub fn from_u32_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u32,
) -> String {
    from_u128_to_chinese_low(chinese_variant, chinese_case, value as u128).unwrap()
}

/// 將 `u32` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_u32_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u32,
) -> String {
    from_u128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as u128)
}

/// 將 `u32` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_u32_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u32,
) -> String {
    from_u128_to_chinese_middle(chinese_variant, chinese_case, value as u128)
}

/// 將 `u32` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_u32_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u32,
) -> String {
    from_u128_to_chinese_high(chinese_variant, chinese_case, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「下數」**。數值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_u64_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u64,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_low(chinese_variant, chinese_case, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_u64_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u64,
) -> String {
    from_u128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_u64_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u64,
) -> String {
    from_u128_to_chinese_middle(chinese_variant, chinese_case, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_u64_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u64,
) -> String {
    from_u128_to_chinese_high(chinese_variant, chinese_case, value as u128)
}

/// 將 `u128` 整數轉成中文數字，使用 **「下數」**。數值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_u128_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u128,
) -> Result<String, NumberToChineseError> {
    if value >= 1_0000_0000_0000_0000 {
        return Err(NumberToChineseError::Overflow);
    }

    Ok(unsigned_integer_to_chinese_low(chinese_variant, chinese_case, false, value))
}

/// 將 `u128` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_u128_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u128,
) -> String {
    unsigned_integer_to_chinese_ten_thousand(chinese_variant, chinese_case, false, value)
}

/// 將 `u128` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_u128_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u128,
) -> String {
    unsigned_integer_to_chinese_middle(chinese_variant, chinese_case, false, value)
}

/// 將 `u128` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_u128_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u128,
) -> String {
    unsigned_integer_to_chinese_high(chinese_variant, chinese_case, false, value)
}

/// 將 `usize` 整數轉成中文數字，使用 **「下數」**。數值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_usize_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: usize,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_low(chinese_variant, chinese_case, value as u128)
}

/// 將 `usize` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_usize_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: usize,
) -> String {
    from_u128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as u128)
}

/// 將 `usize` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_usize_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: usize,
) -> String {
    from_u128_to_chinese_middle(chinese_variant, chinese_case, value as u128)
}

/// 將 `usize` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_usize_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: usize,
) -> String {
    from_u128_to_chinese_high(chinese_variant, chinese_case, value as u128)
}

// TODO signed integer

/// 將 `i8` 整數轉成中文數字。
#[inline]
pub fn from_i8_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i8,
) -> String {
    from_i128_to_chinese_low(chinese_variant, chinese_case, value as i128).unwrap()
}

/// 將 `i16` 整數轉成中文數字。
#[inline]
pub fn from_i16_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i16,
) -> String {
    from_i128_to_chinese_low(chinese_variant, chinese_case, value as i128).unwrap()
}

/// 將 `i32` 整數轉成中文數字，使用 **「下數」**。
#[inline]
pub fn from_i32_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i32,
) -> String {
    from_i128_to_chinese_low(chinese_variant, chinese_case, value as i128).unwrap()
}

/// 將 `i32` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_i32_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i32,
) -> String {
    from_i128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as i128)
}

/// 將 `i32` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_i32_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i32,
) -> String {
    from_i128_to_chinese_middle(chinese_variant, chinese_case, value as i128)
}

/// 將 `i32` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_i32_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i32,
) -> String {
    from_i128_to_chinese_high(chinese_variant, chinese_case, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_i64_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i64,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_low(chinese_variant, chinese_case, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_i64_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i64,
) -> String {
    from_i128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_i64_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i64,
) -> String {
    from_i128_to_chinese_middle(chinese_variant, chinese_case, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_i64_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i64,
) -> String {
    from_i128_to_chinese_high(chinese_variant, chinese_case, value as i128)
}

/// 將 `i128` 整數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_i128_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i128,
) -> Result<String, NumberToChineseError> {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_low(chinese_variant, chinese_case, -(value + 1) as u128 + 1)
                .map_err(|err| match err {
                    NumberToChineseError::Overflow => NumberToChineseError::Underflow,
                    _ => err,
                })?;

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        Ok(s)
    } else {
        from_u128_to_chinese_low(chinese_variant, chinese_case, value as u128)
    }
}

/// 將 `i128` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_i128_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i128,
) -> String {
    if value < 0 {
        let mut s = from_u128_to_chinese_ten_thousand(
            chinese_variant,
            chinese_case,
            -(value + 1) as u128 + 1,
        );

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        s
    } else {
        from_u128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as u128)
    }
}

/// 將 `i128` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_i128_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i128,
) -> String {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_middle(chinese_variant, chinese_case, -(value + 1) as u128 + 1);

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        s
    } else {
        from_u128_to_chinese_middle(chinese_variant, chinese_case, value as u128)
    }
}

/// 將 `i128` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_i128_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i128,
) -> String {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_high(chinese_variant, chinese_case, -(value + 1) as u128 + 1);

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        s
    } else {
        from_u128_to_chinese_high(chinese_variant, chinese_case, value as u128)
    }
}

/// 將 `isize` 整數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_isize_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: isize,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_low(chinese_variant, chinese_case, value as i128)
}

/// 將 `isize` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_isize_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: isize,
) -> String {
    from_i128_to_chinese_ten_thousand(chinese_variant, chinese_case, value as i128)
}

/// 將 `isize` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_isize_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: isize,
) -> String {
    from_i128_to_chinese_middle(chinese_variant, chinese_case, value as i128)
}

/// 將 `isize` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_isize_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: isize,
) -> String {
    from_i128_to_chinese_high(chinese_variant, chinese_case, value as i128)
}

// TODO float

/// 將 `f32` 浮點數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_f32_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f32,
) -> Result<String, NumberToChineseError> {
    from_f64_to_chinese_low(chinese_variant, chinese_case, value as f64)
}

/// 將 `f32` 浮點數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_f32_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f32,
) -> String {
    from_f64_to_chinese_ten_thousand(chinese_variant, chinese_case, value as f64).unwrap()
}

/// 將 `f32` 浮點數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_f32_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f32,
) -> String {
    from_f64_to_chinese_middle(chinese_variant, chinese_case, value as f64).unwrap()
}

/// 將 `f32` 浮點數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_f32_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f32,
) -> String {
    from_f64_to_chinese_high(chinese_variant, chinese_case, value as f64)
}

#[inline]
fn from_f64_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    method: ChineseCountMethod,
    value: f64,
) -> String {
    if value < 0.0 {
        let mut s = positive_float_to_chinese(chinese_variant, chinese_case, method, -value);

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        s
    } else {
        positive_float_to_chinese(chinese_variant, chinese_case, method, value)
    }
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_f64_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> Result<String, NumberToChineseError> {
    if value >= 1_0000_0000_0000_0000f64 {
        return Err(NumberToChineseError::Overflow);
    } else if value <= -1_0000_0000_0000_0000f64 {
        return Err(NumberToChineseError::Underflow);
    }

    Ok(from_f64_to_chinese(chinese_variant, chinese_case, ChineseCountMethod::Low, value))
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「萬進」**。數值的絕對值不能大於或等於 `1e52`。
#[inline]
pub fn from_f64_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> Result<String, NumberToChineseError> {
    if value >= 1e52 {
        return Err(NumberToChineseError::Overflow);
    } else if value <= -1e52 {
        return Err(NumberToChineseError::Underflow);
    }

    Ok(from_f64_to_chinese(chinese_variant, chinese_case, ChineseCountMethod::TenThousand, value))
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「中數」**。數值的絕對值不能大於或等於 `1e96`。
#[inline]
pub fn from_f64_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> Result<String, NumberToChineseError> {
    if value >= 1e96 {
        return Err(NumberToChineseError::Overflow);
    } else if value <= -1e96 {
        return Err(NumberToChineseError::Underflow);
    }

    Ok(from_f64_to_chinese(chinese_variant, chinese_case, ChineseCountMethod::Middle, value))
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_f64_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> String {
    from_f64_to_chinese(chinese_variant, chinese_case, ChineseCountMethod::High, value)
}
