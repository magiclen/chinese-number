use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

/// 將中文數字轉成數值時發生的錯誤。
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChineseToNumberError {
    ChineseNumberEmpty,
    ChineseNumberIncorrect {
        char_index: usize,
    },
    Overflow,
    Underflow,
}

impl Display for ChineseToNumberError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ChineseToNumberError::ChineseNumberEmpty => {
                f.write_str("a chinese number cannot be empty")
            }
            ChineseToNumberError::ChineseNumberIncorrect {
                char_index,
            } => {
                f.write_fmt(format_args!(
                    "the chinese number is incorrect (position: {})",
                    char_index
                ))
            }
            ChineseToNumberError::Overflow => f.write_str("number is too large"),
            ChineseToNumberError::Underflow => f.write_str("number is too small"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ChineseToNumberError {}
