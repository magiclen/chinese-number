mod chinese_to_number_error;
mod functions;
mod traits;

mod functions_test;

use functions::*;

pub use chinese_to_number_error::*;
pub use traits::*;

use crate::ChineseCountMethod;

/// 將中文數字轉成 `u8` 整數。
#[inline]
pub fn from_chinese_to_u8<S: AsRef<str>>(chinese_number: S) -> Result<u8, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Low, &chars)?;

    if n > u8::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u8)
}

/// 將中文數字轉成 `u16` 整數。
#[inline]
pub fn from_chinese_to_u16<S: AsRef<str>>(chinese_number: S) -> Result<u16, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Low, &chars)?;

    if n > u16::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u16)
}

/// 將中文數字轉成 `u32` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_u32_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Low, &chars)?;

    if n > u32::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u32)
}

/// 將中文數字轉成 `u32` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_u32_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::TenThousand, &chars)?;

    if n > u32::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u32)
}

/// 將中文數字轉成 `u32` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_u32_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Middle, &chars)?;

    if n > u32::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u32)
}

/// 將中文數字轉成 `u32` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_u32_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::High, &chars)?;

    if n > u32::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u32)
}

/// 將中文數字轉成 `u64` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_u64_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Low, &chars)?;

    if n > u64::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u64)
}

/// 將中文數字轉成 `u64` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_u64_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::TenThousand, &chars)?;

    if n > u64::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u64)
}

/// 將中文數字轉成 `u64` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_u64_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Middle, &chars)?;

    if n > u64::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u64)
}

/// 將中文數字轉成 `u64` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_u64_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::High, &chars)?;

    if n > u64::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as u64)
}

/// 將中文數字轉成 `u128` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_u128_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_unsigned_integer(ChineseCountMethod::Low, &chars)
}

/// 將中文數字轉成 `u128` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_u128_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_unsigned_integer(ChineseCountMethod::TenThousand, &chars)
}

/// 將中文數字轉成 `u128` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_u128_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_unsigned_integer(ChineseCountMethod::Middle, &chars)
}

/// 將中文數字轉成 `u128` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_u128_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_unsigned_integer(ChineseCountMethod::High, &chars)
}

/// 將中文數字轉成 `usize` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_usize_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<usize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Low, &chars)?;

    if n > usize::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as usize)
}

/// 將中文數字轉成 `usize` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_usize_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<usize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::TenThousand, &chars)?;

    if n > usize::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as usize)
}

/// 將中文數字轉成 `usize` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_usize_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<usize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::Middle, &chars)?;

    if n > usize::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as usize)
}

/// 將中文數字轉成 `usize` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_usize_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<usize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_unsigned_integer(ChineseCountMethod::High, &chars)?;

    if n > usize::MAX as u128 {
        return Err(ChineseToNumberError::Overflow);
    }

    Ok(n as usize)
}

/// 將中文數字轉成 `i8` 整數。
#[inline]
pub fn from_chinese_to_i8<S: AsRef<str>>(chinese_number: S) -> Result<i8, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Low, &chars)?;

    if n > i8::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i8::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i8)
}

/// 將中文數字轉成 `i16` 整數。
#[inline]
pub fn from_chinese_to_i16<S: AsRef<str>>(chinese_number: S) -> Result<i16, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Low, &chars)?;

    if n > i16::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i16::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i16)
}

/// 將中文數字轉成 `i32` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_i32_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Low, &chars)?;

    if n > i32::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i32::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i32)
}

/// 將中文數字轉成 `i32` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_i32_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::TenThousand, &chars)?;

    if n > i32::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i32::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i32)
}

/// 將中文數字轉成 `i32` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_i32_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Middle, &chars)?;

    if n > i32::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i32::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i32)
}

/// 將中文數字轉成 `i32` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_i32_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::High, &chars)?;

    if n > i32::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i32::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i32)
}

/// 將中文數字轉成 `i64` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_i64_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Low, &chars)?;

    if n > i64::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i64::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i64)
}

/// 將中文數字轉成 `i64` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_i64_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::TenThousand, &chars)?;

    if n > i64::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i64::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i64)
}

/// 將中文數字轉成 `i64` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_i64_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Middle, &chars)?;

    if n > i64::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i64::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i64)
}

/// 將中文數字轉成 `i64` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_i64_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::High, &chars)?;

    if n > i64::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < i64::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as i64)
}

/// 將中文數字轉成 `i128` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_i128_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_signed_integer(ChineseCountMethod::Low, &chars)
}

/// 將中文數字轉成 `i128` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_i128_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_signed_integer(ChineseCountMethod::TenThousand, &chars)
}

/// 將中文數字轉成 `i128` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_i128_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_signed_integer(ChineseCountMethod::Middle, &chars)
}

/// 將中文數字轉成 `i128` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_i128_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i128, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_signed_integer(ChineseCountMethod::High, &chars)
}

/// 將中文數字轉成 `isize` 整數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_isize_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<isize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Low, &chars)?;

    if n > isize::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < isize::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as isize)
}

/// 將中文數字轉成 `isize` 整數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_isize_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<isize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::TenThousand, &chars)?;

    if n > isize::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < isize::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as isize)
}

/// 將中文數字轉成 `isize` 整數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_isize_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<isize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::Middle, &chars)?;

    if n > isize::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < isize::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as isize)
}

/// 將中文數字轉成 `isize` 整數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_isize_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<isize, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    let n = chinese_to_signed_integer(ChineseCountMethod::High, &chars)?;

    if n > isize::MAX as i128 {
        return Err(ChineseToNumberError::Overflow);
    } else if n < isize::MIN as i128 {
        return Err(ChineseToNumberError::Underflow);
    }

    Ok(n as isize)
}

/// 將中文數字轉成 `f32` 浮點數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_f32_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::Low, &chars).map(|f| f as f32)
}

/// 將中文數字轉成 `f32` 浮點數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_f32_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::TenThousand, &chars).map(|f| f as f32)
}

/// 將中文數字轉成 `f32` 浮點數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_f32_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::Middle, &chars).map(|f| f as f32)
}

/// 將中文數字轉成 `f32` 浮點數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_f32_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f32, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::High, &chars).map(|f| f as f32)
}

/// 將中文數字轉成 `f64` 浮點數。使用 **「下數」**。
#[inline]
pub fn from_chinese_to_f64_low<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::Low, &chars)
}

/// 將中文數字轉成 `f64` 浮點數。使用 **「萬進」**。
#[inline]
pub fn from_chinese_to_f64_ten_thousand<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::TenThousand, &chars)
}

/// 將中文數字轉成 `f64` 浮點數。使用 **「中數」**。
#[inline]
pub fn from_chinese_to_f64_middle<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::Middle, &chars)
}

/// 將中文數字轉成 `f64` 浮點數。使用 **「上數」**。
#[inline]
pub fn from_chinese_to_f64_high<S: AsRef<str>>(
    chinese_number: S,
) -> Result<f64, ChineseToNumberError> {
    let chars = to_chars_vec(chinese_number.as_ref());

    chinese_to_f64(ChineseCountMethod::High, &chars)
}
