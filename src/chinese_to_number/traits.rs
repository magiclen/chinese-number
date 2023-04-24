use super::*;

/// 讓 Rust 程式語言的字串型別擁有將中文數字轉成數值的能力。
pub trait ChineseToNumber<T> {
    /// 將中文數字轉成數值。
    ///
    /// ## 範例
    ///
    /// ```rust
    /// use chinese_number::{ChineseCountMethod, ChineseToNumber};
    ///
    /// assert_eq!(1234567890123456789u64, "一百二十三京四千五百六十七兆八千九百零一億二千三百四十五萬六千七百八十九".to_number(ChineseCountMethod::TenThousand).unwrap());
    /// ```
    fn to_number(&self, method: ChineseCountMethod) -> Result<T, ChineseToNumberError>;

    /// 將中文數字直接轉成數值，不進行單位計算。
    ///
    /// ## 範例
    ///
    /// ```rust
    /// use chinese_number::{ChineseCountMethod, ChineseToNumber};
    ///
    /// assert_eq!(123456789u64, "一二三四五六七八九".to_number_naive().unwrap());
    /// ```
    fn to_number_naive(&self) -> Result<T, ChineseToNumberError>;
}

impl<T: AsRef<str>> ChineseToNumber<u8> for T {
    #[inline]
    fn to_number(&self, _method: ChineseCountMethod) -> Result<u8, ChineseToNumberError> {
        from_chinese_to_u8(self)
    }

    #[inline]
    fn to_number_naive(&self) -> Result<u8, ChineseToNumberError> {
        from_chinese_to_u8_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<u16> for T {
    #[inline]
    fn to_number(&self, _method: ChineseCountMethod) -> Result<u16, ChineseToNumberError> {
        from_chinese_to_u16(self)
    }

    #[inline]
    fn to_number_naive(&self) -> Result<u16, ChineseToNumberError> {
        from_chinese_to_u16_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<u32> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<u32, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_u32_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_u32_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_u32_middle(self),
            ChineseCountMethod::High => from_chinese_to_u32_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<u32, ChineseToNumberError> {
        from_chinese_to_u32_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<u64> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<u64, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_u64_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_u64_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_u64_middle(self),
            ChineseCountMethod::High => from_chinese_to_u64_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<u64, ChineseToNumberError> {
        from_chinese_to_u64_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<u128> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<u128, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_u128_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_u128_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_u128_middle(self),
            ChineseCountMethod::High => from_chinese_to_u128_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<u128, ChineseToNumberError> {
        from_chinese_to_u128_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<usize> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<usize, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_usize_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_usize_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_usize_middle(self),
            ChineseCountMethod::High => from_chinese_to_usize_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<usize, ChineseToNumberError> {
        from_chinese_to_usize_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<i8> for T {
    #[inline]
    fn to_number(&self, _method: ChineseCountMethod) -> Result<i8, ChineseToNumberError> {
        from_chinese_to_i8(self)
    }

    #[inline]
    fn to_number_naive(&self) -> Result<i8, ChineseToNumberError> {
        from_chinese_to_i8_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<i16> for T {
    #[inline]
    fn to_number(&self, _method: ChineseCountMethod) -> Result<i16, ChineseToNumberError> {
        from_chinese_to_i16(self)
    }

    #[inline]
    fn to_number_naive(&self) -> Result<i16, ChineseToNumberError> {
        from_chinese_to_i16_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<i32> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<i32, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_i32_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_i32_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_i32_middle(self),
            ChineseCountMethod::High => from_chinese_to_i32_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<i32, ChineseToNumberError> {
        from_chinese_to_i32_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<i64> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<i64, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_i64_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_i64_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_i64_middle(self),
            ChineseCountMethod::High => from_chinese_to_i64_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<i64, ChineseToNumberError> {
        from_chinese_to_i64_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<i128> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<i128, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_i128_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_i128_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_i128_middle(self),
            ChineseCountMethod::High => from_chinese_to_i128_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<i128, ChineseToNumberError> {
        from_chinese_to_i128_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<isize> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<isize, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_isize_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_isize_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_isize_middle(self),
            ChineseCountMethod::High => from_chinese_to_isize_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<isize, ChineseToNumberError> {
        from_chinese_to_isize_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<f32> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<f32, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_f32_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_f32_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_f32_middle(self),
            ChineseCountMethod::High => from_chinese_to_f32_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<f32, ChineseToNumberError> {
        from_chinese_to_f32_naive(self)
    }
}

impl<T: AsRef<str>> ChineseToNumber<f64> for T {
    #[inline]
    fn to_number(&self, method: ChineseCountMethod) -> Result<f64, ChineseToNumberError> {
        match method {
            ChineseCountMethod::Low => from_chinese_to_f64_low(self),
            ChineseCountMethod::TenThousand => from_chinese_to_f64_ten_thousand(self),
            ChineseCountMethod::Middle => from_chinese_to_f64_middle(self),
            ChineseCountMethod::High => from_chinese_to_f64_high(self),
        }
    }

    #[inline]
    fn to_number_naive(&self) -> Result<f64, ChineseToNumberError> {
        from_chinese_to_f64_naive(self)
    }
}
