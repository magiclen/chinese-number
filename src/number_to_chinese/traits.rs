use super::*;

/// 讓 Rust 程式語言的所有基本數值型別擁有轉成中文數字的能力。
pub trait NumberToChinese {
    /// 將數值轉成中文數字。
    ///
    /// * 如果使用 **「下數」**，則數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
    /// * 如果使用 **「萬進」**，則數值的絕對值不能大於或等於 `1e52`。
    /// * 如果使用 **「中數」**，則數值的絕對值不能大於或等於 `1e96`。
    ///
    /// ## 範例
    ///
    /// ```rust
    /// use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};
    ///
    /// assert_eq!("一百二十三京四千五百六十七兆八千九百零一億二千三百四十五萬六千七百八十九", 1234567890123456789u64.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::TenThousand).unwrap());
    /// ```
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError>;

    /// 將數值直接轉成中文數字，不進行單位計算。
    ///
    /// ## 範例
    ///
    /// ```rust
    /// use chinese_number::{
    ///     ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese,
    /// };
    ///
    /// assert_eq!(
    ///     "一二三四五六七八九",
    ///     123456789
    ///         .to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
    /// );
    /// ```
    fn to_chinese_naive(self, chinese_variant: ChineseVariant, chinese_case: ChineseCase)
        -> String;
}

impl NumberToChinese for u8 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as u128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_u8_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for i8 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as i128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_i8_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for u16 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as u128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_u16_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for i16 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as i128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_i16_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for u32 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as u128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_u32_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for i32 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as i128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_i32_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for u64 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as u128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_u64_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for i64 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as i128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_i64_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for u128 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        match method {
            ChineseCountMethod::Low => {
                from_u128_to_chinese_low(chinese_variant, chinese_case, self)
            },
            ChineseCountMethod::TenThousand => {
                Ok(from_u128_to_chinese_ten_thousand(chinese_variant, chinese_case, self))
            },
            ChineseCountMethod::Middle => {
                Ok(from_u128_to_chinese_middle(chinese_variant, chinese_case, self))
            },
            ChineseCountMethod::High => {
                Ok(from_u128_to_chinese_high(chinese_variant, chinese_case, self))
            },
        }
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_u128_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for i128 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        match method {
            ChineseCountMethod::Low => {
                from_i128_to_chinese_low(chinese_variant, chinese_case, self)
            },
            ChineseCountMethod::TenThousand => {
                Ok(from_i128_to_chinese_ten_thousand(chinese_variant, chinese_case, self))
            },
            ChineseCountMethod::Middle => {
                Ok(from_i128_to_chinese_middle(chinese_variant, chinese_case, self))
            },
            ChineseCountMethod::High => {
                Ok(from_i128_to_chinese_high(chinese_variant, chinese_case, self))
            },
        }
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_i128_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for usize {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as u128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_usize_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for isize {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as i128).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_isize_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for f32 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        (self as f64).to_chinese(chinese_variant, chinese_case, method)
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_f32_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}

impl NumberToChinese for f64 {
    #[inline]
    fn to_chinese(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
        method: ChineseCountMethod,
    ) -> Result<String, NumberToChineseError> {
        match method {
            ChineseCountMethod::Low => from_f64_to_chinese_low(chinese_variant, chinese_case, self),
            ChineseCountMethod::TenThousand => {
                from_f64_to_chinese_ten_thousand(chinese_variant, chinese_case, self)
            },
            ChineseCountMethod::Middle => {
                from_f64_to_chinese_middle(chinese_variant, chinese_case, self)
            },
            ChineseCountMethod::High => {
                Ok(from_f64_to_chinese_high(chinese_variant, chinese_case, self))
            },
        }
    }

    #[inline]
    fn to_chinese_naive(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> String {
        from_f64_to_chinese_naive(chinese_variant, chinese_case, self)
    }
}
