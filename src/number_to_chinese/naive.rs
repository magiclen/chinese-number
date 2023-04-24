use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use chinese_variant::ChineseVariant;
use num_bigint::BigUint;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::float::FloatCore;
use num_traits::{FromPrimitive, ToPrimitive, Zero};

use crate::{
    chinese_characters::{ChineseNumber, ChinesePoint, ChineseSign},
    ChineseCase,
};

fn unsigned_integer_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    mut value: u128,
) -> String {
    if value == 0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    }

    let mut numbers: Vec<ChineseNumber> = Vec::with_capacity(1);

    while value > 0 {
        let n = (value % 10) as u8;
        value /= 10;

        numbers.push(unsafe { ChineseNumber::from_ordinal_unsafe(n) });
    }

    numbers.into_iter().rev().map(|cn| cn.to_str(chinese_variant, chinese_case)).collect()
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

    let mut numbers: Vec<ChineseNumber> = Vec::with_capacity(1);

    while value > big_0 {
        let n = (value.clone() % &big_10).to_u8().unwrap();
        value /= &big_10;

        numbers.push(unsafe { ChineseNumber::from_ordinal_unsafe(n) });
    }

    numbers.into_iter().rev().map(|cn| cn.to_str(chinese_variant, chinese_case)).collect()
}

fn positive_float_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> String {
    let (integer, fraction) = {
        let integer = BigUint::from_f64(value.trunc()).unwrap();
        let fraction = ((value.fract() * 100.0).round() % 100f64) as u8;

        (integer, fraction)
    };

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

    s
}

/// 將 `u8` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u8_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u8,
) -> String {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u16` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u16_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u16,
) -> String {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u32_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u32,
) -> String {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u64_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u64,
) -> String {
    from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
}

/// 將 `u128` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u128_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: u128,
) -> String {
    unsigned_integer_to_chinese(chinese_variant, chinese_case, value)
}

/// 將 `usize` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_usize_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: usize,
) -> String {
    unsigned_integer_to_chinese(chinese_variant, chinese_case, value as u128)
}

/// 將 `i8` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i8_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i8,
) -> String {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i16` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i16_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i16,
) -> String {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i32` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i32_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i32,
) -> String {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i64` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i64_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i64,
) -> String {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `i128` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_i128_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: i128,
) -> String {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_naive(chinese_variant, chinese_case, -(value + 1) as u128 + 1);

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        s
    } else {
        from_u128_to_chinese_naive(chinese_variant, chinese_case, value as u128)
    }
}

/// 將 `isize` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_isize_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: isize,
) -> String {
    from_i128_to_chinese_naive(chinese_variant, chinese_case, value as i128)
}

/// 將 `f32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_f32_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f32,
) -> String {
    from_f64_to_chinese_naive(chinese_variant, chinese_case, value as f64)
}

#[inline]
fn from_f64_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> String {
    if value < 0.0 {
        let mut s = positive_float_to_chinese(chinese_variant, chinese_case, -value);

        s.insert_str(0, ChineseSign::負.to_str(chinese_variant));

        s
    } else {
        positive_float_to_chinese(chinese_variant, chinese_case, value)
    }
}

/// 將 `f64` 浮點數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_f64_to_chinese_naive(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    value: f64,
) -> String {
    from_f64_to_chinese(chinese_variant, chinese_case, value)
}
