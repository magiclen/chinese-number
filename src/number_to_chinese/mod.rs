mod error;
mod functions;

use alloc::string::String;

use crate::{ChineseNumberCase, ChineseNumberCountMethod, ChineseVariant};

pub use error::*;
pub(crate) use functions::*;

/// 將u8整數轉成中文數字。
#[inline]
pub fn from_u8_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: u8,
    buffer: &mut String,
) {
    let chinese_number_table = get_chinese_number_table(variant, case);

    digit_100_compat(chinese_number_table, case, false, value as usize, buffer);
}

/// 將u16整數轉成中文數字。
#[inline]
pub fn from_u16_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: u16,
    buffer: &mut String,
) {
    let chinese_number_table = get_chinese_number_table(variant, case);

    digit_10_000_compat(chinese_number_table, case, false, value as usize, buffer);
}

/// 將u32整數轉成中文數字。
#[inline]
pub fn from_u32_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u32,
    buffer: &mut String,
) {
    let chinese_number_table = get_chinese_number_table(variant, case);

    match method {
        ChineseNumberCountMethod::Low => {
            digit_compat_low_u32(chinese_number_table, case, value, buffer);
        }
        ChineseNumberCountMethod::TenThousand
        | ChineseNumberCountMethod::Middle
        | ChineseNumberCountMethod::High => {
            digit_compat_ten_thousand_u32(chinese_number_table, case, value, buffer);
        }
    }
}

/// 將u64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[inline]
pub fn from_u64_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u64,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    match method {
        ChineseNumberCountMethod::Low => {
            if value < 10_000_000_000_000_000 {
                digit_compat_low_u64(get_chinese_number_table(variant, case), case, value, buffer);

                Ok(())
            } else {
                Err(NumberToChineseNumberError::Overflow)
            }
        }
        ChineseNumberCountMethod::TenThousand => {
            digit_compat_ten_thousand_u64(
                get_chinese_number_table(variant, case),
                case,
                value,
                buffer,
            );

            Ok(())
        }
        ChineseNumberCountMethod::Middle => {
            digit_compat_middle_u64(get_chinese_number_table(variant, case), case, value, buffer);

            Ok(())
        }
        ChineseNumberCountMethod::High => {
            digit_compat_high_u64(get_chinese_number_table(variant, case), case, value, buffer);

            Ok(())
        }
    }
}

/// 將u128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[inline]
pub fn from_u128_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u128,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    match method {
        ChineseNumberCountMethod::Low => {
            if value < 10_000_000_000_000_000 {
                digit_compat_low_u64(
                    get_chinese_number_table(variant, case),
                    case,
                    value as u64,
                    buffer,
                );

                Ok(())
            } else {
                Err(NumberToChineseNumberError::Overflow)
            }
        }
        ChineseNumberCountMethod::TenThousand => {
            digit_compat_ten_thousand_u128(
                get_chinese_number_table(variant, case),
                case,
                value,
                buffer,
            );

            Ok(())
        }
        ChineseNumberCountMethod::Middle => {
            digit_compat_middle_u128(get_chinese_number_table(variant, case), case, value, buffer);

            Ok(())
        }
        ChineseNumberCountMethod::High => {
            digit_compat_high_u128(get_chinese_number_table(variant, case), case, value, buffer);

            Ok(())
        }
    }
}

#[cfg(target_pointer_width = "8")]
/// 將usize整數轉成中文數字。
pub fn from_usize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_u8_to_string(variant, case, value as u8, buffer);

    Ok(())
}

#[cfg(target_pointer_width = "16")]
/// 將usize整數轉成中文數字。
pub fn from_usize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_u16_to_string(variant, case, value as u16, buffer);

    Ok(())
}

#[cfg(target_pointer_width = "32")]
/// 將usize整數轉成中文數字。
pub fn from_usize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_u32_to_string(variant, case, method, value as u32, buffer);

    Ok(())
}

#[cfg(target_pointer_width = "64")]
/// 將usize整數轉成中文數字。
pub fn from_usize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_u64_to_string(variant, case, method, value as u64, buffer)
}

#[cfg(not(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
/// 將usize整數轉成中文數字。
pub fn from_usize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_u128_to_string(variant, case, method, value as u128, buffer)
}

/// 將i8整數轉成中文數字。
#[inline]
pub fn from_i8_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: i8,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(get_chinese_negative_str(variant));

        if value == i8::MIN {
            from_u8_to_string(variant, case, -(i16::from(value)) as u8, buffer)
        } else {
            from_u8_to_string(variant, case, -value as u8, buffer)
        }
    } else {
        from_u8_to_string(variant, case, value as u8, buffer)
    }
}

/// 將i16整數轉成中文數字。
#[inline]
pub fn from_i16_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: i16,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(get_chinese_negative_str(variant));

        if value == i16::MIN {
            from_u16_to_string(variant, case, -(i32::from(value)) as u16, buffer)
        } else {
            from_u16_to_string(variant, case, -value as u16, buffer)
        }
    } else {
        from_u16_to_string(variant, case, value as u16, buffer)
    }
}

/// 將i32整數轉成中文數字。
#[inline]
pub fn from_i32_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i32,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(get_chinese_negative_str(variant));

        if value == i32::MIN {
            from_u32_to_string(variant, case, method, -(i64::from(value)) as u32, buffer)
        } else {
            from_u32_to_string(variant, case, method, -value as u32, buffer)
        }
    } else {
        from_u32_to_string(variant, case, method, value as u32, buffer)
    }
}

/// 將i64整數轉成中文數字。
#[inline]
pub fn from_i64_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i64,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    if value < 0 {
        if method == ChineseNumberCountMethod::Low && value <= -10_000_000_000_000_000 {
            return Err(NumberToChineseNumberError::Underflow);
        }

        buffer.push_str(get_chinese_negative_str(variant));

        if value == i64::MIN {
            from_u64_to_string(variant, case, method, -(i128::from(value)) as u64, buffer)
        } else {
            from_u64_to_string(variant, case, method, -value as u64, buffer)
        }
    } else {
        from_u64_to_string(variant, case, method, value as u64, buffer)
    }
}

/// 將i128整數轉成中文數字。
#[inline]
pub fn from_i128_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i128,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    if value < 0 {
        if method == ChineseNumberCountMethod::Low && value <= -10_000_000_000_000_000 {
            return Err(NumberToChineseNumberError::Underflow);
        }

        buffer.push_str(get_chinese_negative_str(variant));

        if value == i128::MIN {
            from_u128_to_string(variant, case, method, -(value + 1) as u128 + 1, buffer)
        } else {
            from_u128_to_string(variant, case, method, -value as u128, buffer)
        }
    } else {
        from_u128_to_string(variant, case, method, value as u128, buffer)
    }
}

#[cfg(target_pointer_width = "8")]
/// 將isize整數轉成中文數字。
pub fn from_isize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_i8_to_string(variant, case, value as i8, buffer);

    Ok(())
}

#[cfg(target_pointer_width = "16")]
/// 將isize整數轉成中文數字。
pub fn from_isize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_i16_to_string(variant, case, value as i16, buffer);

    Ok(())
}

#[cfg(target_pointer_width = "32")]
/// 將isize整數轉成中文數字。
pub fn from_isize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_i32_to_string(variant, case, method, value as i32, buffer);

    Ok(())
}

#[cfg(target_pointer_width = "64")]
/// 將isize整數轉成中文數字。
pub fn from_isize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_i64_to_string(variant, case, method, value as i64, buffer)
}

#[cfg(not(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
/// 將isize整數轉成中文數字。
pub fn from_isize_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_i128_to_string(variant, case, method, value as i128, buffer)
}

/// 將f64浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[inline]
pub fn from_f64_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    mut value: f64,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    match method {
        ChineseNumberCountMethod::Low => {
            if value >= 10_000_000_000_000_000f64 {
                Err(NumberToChineseNumberError::Overflow)
            } else if value <= -10_000_000_000_000_000f64 {
                Err(NumberToChineseNumberError::Underflow)
            } else {
                let chinese_number_table = get_chinese_number_table(variant, case);

                if value < 0.0 {
                    buffer.push_str(get_chinese_negative_str(variant));
                    value = -value;
                }

                fraction_compat_low(chinese_number_table, case, value, buffer);

                Ok(())
            }
        }
        ChineseNumberCountMethod::TenThousand => {
            let chinese_number_table = get_chinese_number_table(variant, case);

            if value < 0.0 {
                buffer.push_str(get_chinese_negative_str(variant));
                value = -value;
            }

            fraction_compat_ten_thousand(chinese_number_table, case, value, buffer);

            Ok(())
        }
        ChineseNumberCountMethod::Middle => {
            let chinese_number_table = get_chinese_number_table(variant, case);

            if value < 0.0 {
                buffer.push_str(get_chinese_negative_str(variant));
                value = -value;
            }

            fraction_compat_middle(chinese_number_table, case, value, buffer);

            Ok(())
        }
        ChineseNumberCountMethod::High => {
            let chinese_number_table = get_chinese_number_table(variant, case);

            if value < 0.0 {
                buffer.push_str(get_chinese_negative_str(variant));
                value = -value;
            }

            fraction_compat_high(chinese_number_table, case, value, buffer);

            Ok(())
        }
    }
}

/// 將f32浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[inline]
pub fn from_f32_to_string(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: f32,
    buffer: &mut String,
) -> Result<(), NumberToChineseNumberError> {
    from_f64_to_string(variant, case, method, f64::from(value), buffer)
}
