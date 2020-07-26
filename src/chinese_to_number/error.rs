use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

/// 將中文數字轉成數值時發生的錯誤。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChineseNumberParseError {
    ChineseNumberEmpty,
    ChineseNumberIncorrect {
        char_index: usize,
    },
    Overflow,
    Underflow,
}

impl Display for ChineseNumberParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ChineseNumberParseError::ChineseNumberEmpty => {
                f.write_str("a chinese number cannot be empty")
            }
            ChineseNumberParseError::ChineseNumberIncorrect {
                char_index,
            } => {
                f.write_fmt(format_args!(
                    "the chinese number is incorrect (whitespace-ignored position: {})",
                    char_index
                ))
            }
            ChineseNumberParseError::Overflow => f.write_str("the chinese number is too large"),
            ChineseNumberParseError::Underflow => f.write_str("the chinese number is too small"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ChineseNumberParseError {}
