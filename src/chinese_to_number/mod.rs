mod crazy_functions;
mod error;
mod functions;

use alloc::string::String;
use alloc::vec::Vec;

use crate::chinese_characters::*;
use crate::ChineseNumberCountMethod;

pub(crate) use crazy_functions::*;
pub use error::*;
pub(crate) use functions::*;

/// 將中文數字轉成u8數值。
#[inline]
pub fn parse_chinese_number_to_u8<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u8, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match chinese_digit_100_compat(
                first_char,
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
            ) {
                Ok(number) => {
                    if number > u16::from(u8::max_value()) {
                        Err(ChineseNumberParseError::Overflow)
                    } else if chars.next().is_some() {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 5,
                        })
                    } else {
                        Ok(number as u8)
                    }
                }
                Err(err) => {
                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err,
                    })
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u16數值。
pub fn parse_chinese_number_to_u16<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u16, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match chinese_digit_10_000_ten_thousand_compat(
                first_char,
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
            ) {
                Ok(number) => {
                    if number > u32::from(u16::max_value()) {
                        Err(ChineseNumberParseError::Overflow)
                    } else if chars.next().is_some() {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 15,
                        })
                    } else {
                        Ok(number as u16)
                    }
                }
                Err(err) => {
                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err,
                    })
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u32數值。
pub fn parse_chinese_number_to_u32<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<u32, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match method {
                ChineseNumberCountMethod::Low => {
                    match chinese_digit_1_000_000_000_low_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u64::from(u32::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 19,
                                })
                            } else {
                                Ok(number as u32)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
                ChineseNumberCountMethod::TenThousand
                | ChineseNumberCountMethod::Middle
                | ChineseNumberCountMethod::High => {
                    match chinese_digit_100_000_000_ten_thousand_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u64::from(u32::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 23,
                                })
                            } else {
                                Ok(number as u32)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u64數值。
pub fn parse_chinese_number_to_u64<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<u64, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match method {
                ChineseNumberCountMethod::Low => {
                    match chinese_digit_1_000_000_000_000_000_low_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u64::max_value() {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 31,
                                })
                            } else {
                                Ok(number)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
                ChineseNumberCountMethod::TenThousand => {
                    match chinese_digit_10_000_000_000_000_000_ten_thousand_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u128::from(u64::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 39,
                                })
                            } else {
                                Ok(number as u64)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
                ChineseNumberCountMethod::Middle | ChineseNumberCountMethod::High => {
                    match chinese_digit_10_000_000_000_000_000_middle_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u128::from(u64::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 47,
                                })
                            } else {
                                Ok(number as u64)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u128數值。
pub fn parse_chinese_number_to_u128<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    _chinese_number: S,
) -> Result<u128, ChineseNumberParseError> {
    unimplemented!()
}

#[cfg(target_pointer_width = "8")]
/// 將中文數字轉成usize數值。
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u8(chinese_number).map(|n| n as usize)
}

#[cfg(target_pointer_width = "16")]
/// 將中文數字轉成usize數值。
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u16(chinese_number).map(|n| n as usize)
}

#[cfg(target_pointer_width = "32")]
/// 將中文數字轉成usize數值。
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u32(method, chinese_number).map(|n| n as usize)
}

#[cfg(target_pointer_width = "64")]
/// 將中文數字轉成usize數值。
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u64(method, chinese_number).map(|n| n as usize)
}

#[cfg(not(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
/// 將中文數字轉成usize數值。
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u128(method, chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成i8數值。
pub fn parse_chinese_number_to_i8<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i8, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match chinese_digit_100_compat(
                            next_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i8::max_value() as u16 + 1 {
                                    Err(ChineseNumberParseError::Underflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 6,
                                    })
                                } else {
                                    Ok(-(number as i16) as i8)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match chinese_digit_100_compat(
                    first_char,
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                ) {
                    Ok(number) => {
                        if number > i8::max_value() as u16 {
                            Err(ChineseNumberParseError::Overflow)
                        } else if chars.next().is_some() {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: 5,
                            })
                        } else {
                            Ok(number as i8)
                        }
                    }
                    Err(err) => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err,
                        })
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i16數值。
pub fn parse_chinese_number_to_i16<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i16, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match chinese_digit_10_000_ten_thousand_compat(
                            next_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i16::max_value() as u32 + 1 {
                                    Err(ChineseNumberParseError::Underflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 16,
                                    })
                                } else {
                                    Ok(-(number as i32) as i16)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match chinese_digit_10_000_ten_thousand_compat(
                    first_char,
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                ) {
                    Ok(number) => {
                        if number > i16::max_value() as u32 {
                            Err(ChineseNumberParseError::Overflow)
                        } else if chars.next().is_some() {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: 15,
                            })
                        } else {
                            Ok(number as i16)
                        }
                    }
                    Err(err) => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err,
                        })
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i32數值。
pub fn parse_chinese_number_to_i32<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<i32, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match method {
                            ChineseNumberCountMethod::Low => {
                                match chinese_digit_1_000_000_000_low_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i32::max_value() as u64 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 20,
                                            })
                                        } else {
                                            Ok(-(number as i64) as i32)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                            ChineseNumberCountMethod::TenThousand
                            | ChineseNumberCountMethod::Middle
                            | ChineseNumberCountMethod::High => {
                                match chinese_digit_100_000_000_ten_thousand_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i32::max_value() as u64 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 24,
                                            })
                                        } else {
                                            Ok(-(number as i64) as i32)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match method {
                    ChineseNumberCountMethod::Low => {
                        match chinese_digit_1_000_000_000_low_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i32::max_value() as u64 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 19,
                                    })
                                } else {
                                    Ok(number as i32)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    ChineseNumberCountMethod::TenThousand
                    | ChineseNumberCountMethod::Middle
                    | ChineseNumberCountMethod::High => {
                        match chinese_digit_100_000_000_ten_thousand_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i32::max_value() as u64 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 23,
                                    })
                                } else {
                                    Ok(number as i32)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i64數值。
pub fn parse_chinese_number_to_i64<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<i64, ChineseNumberParseError> {
    let mut chars = to_char_iter(&chinese_number);

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match method {
                            ChineseNumberCountMethod::Low => {
                                match chinese_digit_1_000_000_000_000_000_low_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 32,
                                            })
                                        } else {
                                            Ok(-(number as i64))
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                            ChineseNumberCountMethod::TenThousand => {
                                match chinese_digit_10_000_000_000_000_000_ten_thousand_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i64::max_value() as u128 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 40,
                                            })
                                        } else {
                                            Ok(-(number as i128) as i64)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                            ChineseNumberCountMethod::Middle | ChineseNumberCountMethod::High => {
                                match chinese_digit_10_000_000_000_000_000_middle_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i64::max_value() as u128 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 48,
                                            })
                                        } else {
                                            Ok(-(number as i128) as i64)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match method {
                    ChineseNumberCountMethod::Low => {
                        match chinese_digit_1_000_000_000_000_000_low_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i64::max_value() as u64 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 31,
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    ChineseNumberCountMethod::TenThousand => {
                        match chinese_digit_10_000_000_000_000_000_ten_thousand_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i64::max_value() as u128 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 39,
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    ChineseNumberCountMethod::Middle | ChineseNumberCountMethod::High => {
                        match chinese_digit_10_000_000_000_000_000_middle_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i64::max_value() as u128 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 47,
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i128數值。
pub fn parse_chinese_number_to_i128<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    _chinese_number: S,
) -> Result<i128, ChineseNumberParseError> {
    unimplemented!()
}

#[cfg(target_pointer_width = "8")]
/// 將中文數字轉成isize數值。
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i8(chinese_number).map(|n| n as isize)
}

#[cfg(target_pointer_width = "16")]
/// 將中文數字轉成isize數值。
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i16(chinese_number).map(|n| n as isize)
}

#[cfg(target_pointer_width = "32")]
/// 將中文數字轉成isize數值。
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i32(method, chinese_number).map(|n| n as isize)
}

#[cfg(target_pointer_width = "64")]
/// 將中文數字轉成isize數值。
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i64(method, chinese_number).map(|n| n as isize)
}

#[cfg(not(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
/// 將中文數字轉成isize數值。
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i128(method, chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成f64數值。
pub fn parse_chinese_number_to_f64<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<f64, ChineseNumberParseError> {
    let chinese_number: String = to_char_iter(&chinese_number).collect();

    let len = chinese_number.len();

    if len == 0 {
        return Err(ChineseNumberParseError::ChineseNumberEmpty);
    }

    let mut integer_index = len;

    if let Some(index) = chinese_number.find(ONE_TENTH_CHAR) {
        integer_index = index;
    }

    if integer_index == len {
        if let Some(index) = chinese_number.find(ONE_HUNDRED_PERCENT) {
            integer_index = index;
        }
    }

    if integer_index == 0 {
        Err(ChineseNumberParseError::ChineseNumberIncorrect {
            char_index: 0,
        })
    } else if integer_index == len {
        parse_chinese_number_to_i64(method, chinese_number).map(|result| result as f64)
    } else {
        let mut integer_chars: Vec<char> = chinese_number[..integer_index].chars().collect();

        let integer_chars_len_dec = integer_chars.len() - 1;

        let last_char = integer_chars.remove(integer_chars_len_dec);

        let integer_part: String = integer_chars.iter().collect();

        let integer_number = parse_chinese_number_to_i64(method, integer_part)?;

        let fd1 = chinese_digit_1(last_char).map_err(|_| {
            ChineseNumberParseError::ChineseNumberIncorrect {
                char_index: integer_chars_len_dec,
            }
        })?;

        let mut fraction_chars = chinese_number[integer_index..].chars();

        let unit1 = fraction_chars.next().unwrap();

        let next_char = fraction_chars.next();

        if ONE_TENTH_CHAR == unit1 {
            match next_char {
                Some(next_char) => {
                    let fd2 = chinese_digit_1(next_char).map_err(|_| {
                        ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: integer_chars_len_dec + 2,
                        }
                    })?;

                    let unit2 = fraction_chars.next();

                    match unit2 {
                        Some(unit2) => {
                            if ONE_HUNDRED_PERCENT == unit2 {
                                if integer_number >= 0 {
                                    Ok(integer_number as f64
                                        + f64::from(fd1) * 0.1
                                        + f64::from(fd2) * 0.01)
                                } else {
                                    Ok(integer_number as f64
                                        - f64::from(fd1) * 0.1
                                        - f64::from(fd2) * 0.01)
                                }
                            } else {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: integer_chars_len_dec + 3,
                                })
                            }
                        }
                        None => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: integer_chars_len_dec + 3,
                            })
                        }
                    }
                }
                None => {
                    if integer_number >= 0 {
                        Ok(integer_number as f64 + f64::from(fd1) * 0.1)
                    } else {
                        Ok(integer_number as f64 - f64::from(fd1) * 0.1)
                    }
                }
            }
        } else if ONE_HUNDRED_PERCENT == unit1 {
            if next_char.is_some() {
                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                    char_index: integer_chars_len_dec + 2,
                })
            } else if integer_number >= 0 {
                Ok(integer_number as f64 + f64::from(fd1) * 0.01)
            } else {
                Ok(integer_number as f64 - f64::from(fd1) * 0.01)
            }
        } else {
            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                char_index: integer_chars_len_dec + 1,
            })
        }
    }
}

/// 將中文數字轉成f32數值。
#[inline]
pub fn parse_chinese_number_to_f32<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<f32, ChineseNumberParseError> {
    parse_chinese_number_to_f64(method, chinese_number).map(|result| result as f32)
}
