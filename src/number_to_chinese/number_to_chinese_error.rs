use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// 將數值轉成中文數字時發生的錯誤。
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NumberToChineseError {
    Overflow,
    Underflow,
}

impl Display for NumberToChineseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            NumberToChineseError::Overflow => f.write_str("number is too large"),
            NumberToChineseError::Underflow => f.write_str("number is too small"),
        }
    }
}

impl Error for NumberToChineseError {}
