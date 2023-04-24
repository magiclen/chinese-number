#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::float::FloatCore;

use super::to_chars_vec;
use crate::{
    chinese_characters::{ChineseNumber, ChinesePoint, ChineseSign},
    ChineseToNumberError,
};

fn chinese_to_unsigned_integer(chars: &[char]) -> Result<u128, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let mut sum = 0u128;

    for (char_index, &char) in chars.iter().enumerate() {
        let d = match ChineseNumber::from_char(char) {
            Some(cn) if cn != ChineseNumber::十 => cn.ordinal() as u128,
            _ => {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index,
                });
            },
        };

        sum = sum.checked_mul(10).ok_or(ChineseToNumberError::Overflow)?;

        sum = sum.checked_add(d).ok_or(ChineseToNumberError::Overflow)?;
    }

    Ok(sum)
}

fn chinese_to_signed_integer(chars: &[char]) -> Result<i128, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let c = chars[0];

    let (sign, offset) = match ChineseSign::from_char(c) {
        Some(sign) => (sign, 1),
        None => (ChineseSign::正, 0),
    };

    let uint = match chinese_to_unsigned_integer(&chars[offset..]) {
        Ok(n) => n,
        Err(error) => {
            return match error {
                ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: index,
                } => Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: index + offset,
                }),
                ChineseToNumberError::Overflow if sign == ChineseSign::負 => {
                    Err(ChineseToNumberError::Underflow)
                },
                _ => Err(error),
            };
        },
    };

    match sign {
        ChineseSign::正 => {
            if uint > i128::MAX as u128 {
                return Err(ChineseToNumberError::Overflow);
            }

            Ok(uint as i128)
        },
        ChineseSign::負 => {
            let m = i128::MAX as u128 + 1;

            if uint > m {
                return Err(ChineseToNumberError::Underflow);
            }

            if uint == m {
                Ok(i128::MIN)
            } else {
                Ok(-(uint as i128))
            }
        },
    }
}

/// 將中文數字轉成 `u8` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_u8_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u8, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(&chars)?;

    if n > u8::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u8)
}

/// 將中文數字轉成 `u16` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_u16_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u16, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(&chars)?;

    if n > u16::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u16)
}

/// 將中文數字轉成 `u32` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_u32_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(&chars)?;

    if n > u32::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u32)
}

/// 將中文數字轉成 `u64` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_u64_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(&chars)?;

    if n > u64::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u64)
}

/// 將中文數字轉成 `u128` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_u128_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_unsigned_integer(&chars)
}

/// 將中文數字轉成 `usize` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_usize_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<usize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(&chars)?;

    if n > usize::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as usize)
}

/// 將中文數字轉成 `i8` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_i8_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i8, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(&chars)?;

    if n > i8::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i8::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i8)
}

/// 將中文數字轉成 `i16` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_i16_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i16, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(&chars)?;

    if n > i16::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i16::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i16)
}

/// 將中文數字轉成 `i32` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_i32_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(&chars)?;

    if n > i32::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i32::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i32)
}

/// 將中文數字轉成 `i64` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_i64_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(&chars)?;

    if n > i64::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i64::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i64)
}

/// 將中文數字轉成 `i128` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_i128_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_signed_integer(&chars)
}

/// 將中文數字轉成 `isize` 整數。不進行單位計算。
#[inline]
pub fn from_chinese_to_isize_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<isize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(&chars)?;

    if n > isize::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < isize::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as isize)
}

// TODO f64

fn chinese_to_f64(chars: &[char]) -> Result<f64, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let (sign, offset) = match ChineseSign::from_char(chars[0]) {
        Some(sign) => (sign, 1),
        None => (ChineseSign::正, 0),
    };

    let mut sum = 0f64;

    let mut iter = chars[offset..].iter().enumerate();

    for (i, &char) in iter.by_ref() {
        let d = match ChineseNumber::from_char(char) {
            Some(cn) if cn != ChineseNumber::十 => cn.ordinal() as f64,
            _ => match ChinesePoint::from_char(char) {
                Some(_) => break,
                None => {
                    return Err(ChineseToNumberError::ChineseNumberIncorrect {
                        char_index: i + offset,
                    })
                },
            },
        };

        sum *= 10.0;

        sum += d;
    }

    let mut c = 1i32;

    for (i, &char) in iter {
        let d = match ChineseNumber::from_char(char) {
            Some(cn) if cn != ChineseNumber::十 => cn.ordinal() as f64,
            _ => {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: i + offset
                })
            },
        };

        sum += d * 0.1f64.powi(c);
        c += 1;
    }

    match sign {
        ChineseSign::正 => Ok(sum),
        ChineseSign::負 => Ok(-sum),
    }
}

/// 將中文數字轉成 `f32` 浮點數。不進行單位計算。
#[inline]
pub fn from_chinese_to_f32_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(&chars).map(|f| f as f32)
}

/// 將中文數字轉成 `f64` 浮點數。不進行單位計算。
#[inline]
pub fn from_chinese_to_f64_naive<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(&chars)
}
