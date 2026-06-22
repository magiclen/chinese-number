use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use chinese_variant::ChineseVariant;
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

use super::{prepend_negative_sign, signed_f64_to_chinese, split_i128_sign, split_positive_f64};
use crate::{
    ChineseCase, NumberToChineseError,
    chinese_characters::{ChineseNumber, ChinesePoint},
};

fn unsigned_integer_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    mut value: u128,
) -> String {
    if value == 0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    }

    let mut numbers: Vec<&'static str> = Vec::with_capacity(32);
    let mut bytes = 0usize;

    while value > 0 {
        let n = (value % 10) as u8;
        value /= 10;

        let chunk =
            unsafe { ChineseNumber::from_ordinal_unsafe(n) }.to_str(chinese_variant, chinese_case);

        bytes += chunk.len();
        numbers.push(chunk);
    }

    let mut s = String::with_capacity(bytes);

    for number in numbers.into_iter().rev() {
        s.push_str(number);
    }

    s
}

fn big_unsigned_integer_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    mut value: BigUint,
) -> String {
    let big_0 = BigUint::zero();
    let big_10 = BigUint::from(10u8);

    if value == big_0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    }

    let mut numbers: Vec<&'static str> = Vec::with_capacity(32);
    let mut bytes = 0usize;

    while value > big_0 {
        let n = (value.clone() % &big_10).to_u8().unwrap();
        value /= &big_10;

        let chunk =
            unsafe { ChineseNumber::from_ordinal_unsafe(n) }.to_str(chinese_variant, chinese_case);

        bytes += chunk.len();
        numbers.push(chunk);
    }

    let mut s = String::with_capacity(bytes);

    for number in numbers.into_iter().rev() {
        s.push_str(number);
    }

    s
}

fn positive_float_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> Result<String, NumberToChineseError> {
    let (integer, fraction) = split_positive_f64(value)?;

    let mut s = big_unsigned_integer_to_chinese(chinese_variant, chinese_case, integer);

    if fraction > 0 {
        s.push_str(ChinesePoint::to_str(chinese_variant));

        s.push_str(
            unsafe { ChineseNumber::from_ordinal_unsafe(fraction / 10) }
                .to_str(chinese_variant, chinese_case),
        );

        let d = fraction % 10;

        if d > 0 {
            s.push_str(
                unsafe { ChineseNumber::from_ordinal_unsafe(d) }
                    .to_str(chinese_variant, chinese_case),
            );
        }
    }

    Ok(s)
}

/// 將 `u8` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u8_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u8,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u16` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u16_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u16,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u32_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u32,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u64_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u64,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u128` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u128_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u128,
) -> Result<String, NumberToChineseError> {
    Ok(unsigned_integer_to_chinese(chinese_variant, chinese_case, value))
}

/// 將 `usize` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_usize_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: usize,
) -> Result<String, NumberToChineseError> {
    Ok(unsigned_integer_to_chinese(chinese_variant, chinese_case, value as u128))
}

/// 將 `i8` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i8_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i8,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i16` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i16_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i16,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i32` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i32_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i32,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i64` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i64_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i64,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i128` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_i128_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i128,
) -> Result<String, NumberToChineseError> {
    let (negative, magnitude) = split_i128_sign(value);

    if negative {
        let mut s = from_u128_to_chinese_naive(chinese_variant, chinese_case, magnitude)?;

        prepend_negative_sign(chinese_variant, &mut s);

        Ok(s)
    } else {
        from_u128_to_chinese_naive(chinese_variant, chinese_case, magnitude)
    }
}

/// 將 `isize` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_isize_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: isize,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `f32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_f32_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f32,
) -> Result<String, NumberToChineseError> {
    from_f64_to_chinese_naive(chinese_variant, chinese_case, value as f64)
}

#[inline]
fn from_f64_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> Result<String, NumberToChineseError> {
    signed_f64_to_chinese(chinese_variant, value, |value| {
        positive_float_to_chinese(chinese_variant, chinese_case, value)
    })
}

/// 將 `f64` 浮點數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_f64_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> Result<String, NumberToChineseError> {
    from_f64_to_chinese(chinese_variant, chinese_case, value)
}
