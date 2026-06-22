use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use num_bigint::BigUint;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::float::FloatCore;
use num_traits::{FromPrimitive, ToPrimitive, Zero};

use crate::{
    ChineseCase, ChineseCountMethod, ChineseExponent, ChineseNumber, ChineseSign, ChineseVariant,
    NumberToChineseError,
};

/// Stores short static text chunks on the stack and writes them once in reverse order.
struct ReverseChunkBuffer<const N: usize> {
    chunks: [Option<&'static str>; N],
    len:    usize,
    bytes:  usize,
}

impl<const N: usize> ReverseChunkBuffer<N> {
    /// Creates an empty fixed-size buffer. The size is chosen by the caller for the known maximum output shape.
    #[inline]
    fn new() -> Self {
        Self {
            chunks: [None; N], len: 0, bytes: 0
        }
    }

    /// Pushes a chunk in low-to-high order. `into_string` will write the chunks in the opposite order.
    #[inline]
    fn push(&mut self, chunk: &'static str) {
        debug_assert!(self.len < N);

        self.chunks[self.len] = Some(chunk);
        self.len += 1;
        self.bytes += chunk.len();
    }

    /// Builds the final string from high to low. This avoids repeated `insert_str(0, ...)` byte moves.
    fn into_string(self) -> String {
        let mut s = String::with_capacity(self.bytes);

        for chunk in self.chunks[..self.len].iter().rev() {
            s.push_str(chunk.unwrap());
        }

        s
    }
}

/// Splits an `i128` into a sign flag and an absolute magnitude without overflowing on `i128::MIN`.
#[inline]
pub(crate) fn split_i128_sign(value: i128) -> (bool, u128) {
    if value < 0 {
        // `i128::MIN.abs()` cannot be represented as `i128`, so add one before casting and put the missing one back after the cast.
        (true, -(value + 1) as u128 + 1)
    } else {
        (false, value as u128)
    }
}

/// Adds the Chinese negative sign to the front of an already rendered number.
#[inline]
pub(crate) fn prepend_negative_sign(chinese_variant: ChineseVariant, s: &mut String) {
    s.insert_str(0, ChineseSign::負.to_str(chinese_variant));
}

/// Converts overflow from a positive magnitude into underflow for the negative side of the same range.
#[inline]
pub(crate) fn map_overflow_to_underflow(error: NumberToChineseError) -> NumberToChineseError {
    match error {
        NumberToChineseError::Overflow => NumberToChineseError::Underflow,
        _ => error,
    }
}

/// Splits a finite non-negative `f64` into an integer part and a two-digit rounded fraction.
#[inline]
pub(crate) fn split_positive_f64(value: f64) -> Result<(BigUint, u8), NumberToChineseError> {
    debug_assert!(value.is_finite());
    debug_assert!(value >= 0.0);

    let mut integer = BigUint::from_f64(value.trunc()).ok_or(NumberToChineseError::Overflow)?;
    let fraction = (value.fract() * 100.0).round() as u8;

    // The public behavior keeps two decimal digits, so a rounded fraction of 100 must carry into the integer part.
    let fraction = if fraction >= 100 {
        integer += BigUint::from(1u8);

        0
    } else {
        fraction
    };

    Ok((integer, fraction))
}

/// Handles finite checks, sign handling, and negative overflow mapping for `f64` conversion.
#[inline]
pub(crate) fn signed_f64_to_chinese(
    chinese_variant: ChineseVariant,
    value: f64,
    positive_to_chinese: impl FnOnce(f64) -> Result<String, NumberToChineseError>,
) -> Result<String, NumberToChineseError> {
    if value.is_nan() || value == f64::INFINITY {
        Err(NumberToChineseError::Overflow)
    } else if value == f64::NEG_INFINITY {
        Err(NumberToChineseError::Underflow)
    } else if value < 0.0 {
        let mut s = positive_to_chinese(-value).map_err(map_overflow_to_underflow)?;

        prepend_negative_sign(chinese_variant, &mut s);

        Ok(s)
    } else {
        positive_to_chinese(value)
    }
}

pub(crate) fn unsigned_integer_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: u128,
) -> String {
    debug_assert!(value < 1_0000_0000_0000_0000);

    let mut chunks = ReverseChunkBuffer::<48>::new();

    let mut lower_d = (value % 10) as u8;
    value /= 10;

    if lower_d > 0 {
        chunks.push(
            unsafe { ChineseNumber::from_ordinal_unsafe(lower_d) }
                .to_str(chinese_variant, chinese_case),
        );
    } else if value == 0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    }

    let d = (value % 10) as u8;
    value /= 10;

    if d > 0 {
        chunks.push(ChineseExponent::十.to_str(chinese_variant, chinese_case));

        if value > 0 || dependent || d > 1 {
            chunks.push(
                unsafe { ChineseNumber::from_ordinal_unsafe(d) }
                    .to_str(chinese_variant, chinese_case),
            );
        }
    }

    if value == 0 {
        return chunks.into_string();
    }

    lower_d = d;

    let mut i = ChineseExponent::百.ordinal();

    loop {
        let d = (value % 10) as u8;
        value /= 10;

        if d > 0 {
            if lower_d < 1 && chunks.len > 0 {
                chunks.push(ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            chunks.push(
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            chunks.push(
                unsafe { ChineseNumber::from_ordinal_unsafe(d) }
                    .to_str(chinese_variant, chinese_case),
            );
        }

        if value == 0 {
            break;
        }

        lower_d = d;

        i += 1;
    }

    chunks.into_string()
}

/// Renders an unsigned integer with fixed-size groups and appends output from high to low.
#[inline]
fn grouped_unsigned_integer_to_chinese<const UNIT: u128, const ZERO_THRESHOLD: u128>(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: u128,
    first_exponent: ChineseExponent,
    mut lower_renderer: impl FnMut(ChineseVariant, ChineseCase, bool, u128) -> String,
) -> String {
    debug_assert!(UNIT > 1);
    debug_assert!(ZERO_THRESHOLD < UNIT);

    let mut groups = [0u128; 16];
    let mut group_len = 0usize;

    loop {
        debug_assert!(group_len < groups.len());

        groups[group_len] = value % UNIT;
        group_len += 1;
        value /= UNIT;

        if value == 0 {
            break;
        }
    }

    let Some(highest) = groups[..group_len].iter().rposition(|&group| group > 0) else {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    };

    let mut has_nonzero_below = [false; 16];
    let mut seen_nonzero = false;

    for i in 0..group_len {
        has_nonzero_below[i] = seen_nonzero;
        seen_nonzero |= groups[i] > 0;
    }

    let mut s = String::new();

    for i in (0..=highest).rev() {
        let group = groups[i];

        if group == 0 {
            continue;
        }

        s.push_str(
            lower_renderer(chinese_variant, chinese_case, dependent || i < highest, group).as_str(),
        );

        if i > 0 {
            s.push_str(
                unsafe {
                    ChineseExponent::from_ordinal_unsafe(first_exponent.ordinal() + i as u8 - 1)
                }
                .to_str(chinese_variant, chinese_case),
            );
        }

        // A zero is needed when the next lower group is short or empty but some lower non-zero text still exists.
        if i > 0 && has_nonzero_below[i] && groups[i - 1] < ZERO_THRESHOLD {
            s.push_str(ChineseNumber::零.to_str(chinese_variant, chinese_case));
        }
    }

    s
}

/// Renders a large unsigned integer with the same high-to-low grouping rule as `grouped_unsigned_integer_to_chinese`.
#[inline]
fn grouped_big_unsigned_integer_to_chinese<const UNIT: u128, const ZERO_THRESHOLD: u128>(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: BigUint,
    first_exponent: ChineseExponent,
    mut lower_renderer: impl FnMut(ChineseVariant, ChineseCase, bool, u128) -> String,
) -> String {
    let big_0 = BigUint::zero();
    let big_unit = BigUint::from(UNIT);
    let mut groups = Vec::new();

    loop {
        // Each group is smaller than `UNIT`, so it always fits into `u128` and can reuse the normal lower renderer.
        groups.push((value.clone() % &big_unit).to_u128().unwrap());
        value /= &big_unit;

        if value == big_0 {
            break;
        }
    }

    let Some(highest) = groups.iter().rposition(|&group| group > 0) else {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    };

    let mut has_nonzero_below = Vec::with_capacity(groups.len());
    let mut seen_nonzero = false;

    for &group in groups.iter() {
        has_nonzero_below.push(seen_nonzero);
        seen_nonzero |= group > 0;
    }

    let mut s = String::new();

    for i in (0..=highest).rev() {
        let group = groups[i];

        if group == 0 {
            continue;
        }

        s.push_str(
            lower_renderer(chinese_variant, chinese_case, dependent || i < highest, group).as_str(),
        );

        if i > 0 {
            s.push_str(
                unsafe {
                    ChineseExponent::from_ordinal_unsafe(first_exponent.ordinal() + i as u8 - 1)
                }
                .to_str(chinese_variant, chinese_case),
            );
        }

        // This is the same zero rule as the `u128` helper; only the group storage changes.
        if i > 0 && has_nonzero_below[i] && groups[i - 1] < ZERO_THRESHOLD {
            s.push_str(ChineseNumber::零.to_str(chinese_variant, chinese_case));
        }
    }

    s
}

pub(crate) fn unsigned_integer_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    value: u128,
) -> String {
    if value < 1_0000_0000 {
        let low = value % 1_0000;
        let high = value / 1_0000;

        if high == 0 {
            return unsigned_integer_to_chinese_low(chinese_variant, chinese_case, dependent, low);
        }

        let mut s = unsigned_integer_to_chinese_low(chinese_variant, chinese_case, dependent, high);

        s.push_str(ChineseExponent::萬.to_str(chinese_variant, chinese_case));

        if low > 0 {
            // The lower 4-digit group is short, so the spoken form needs a zero between 萬 and the lower text.
            if low < 1000 {
                s.push_str(ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.push_str(
                unsigned_integer_to_chinese_low(chinese_variant, chinese_case, true, low).as_str(),
            );
        }

        return s;
    }

    grouped_unsigned_integer_to_chinese::<1_0000, 1000>(
        chinese_variant,
        chinese_case,
        dependent,
        value,
        ChineseExponent::萬,
        unsigned_integer_to_chinese_low,
    )
}

pub(crate) fn big_unsigned_integer_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    value: BigUint,
) -> String {
    debug_assert!(value < BigUint::from(10u8).pow(52));

    grouped_big_unsigned_integer_to_chinese::<1_0000, 1000>(
        chinese_variant,
        chinese_case,
        dependent,
        value,
        ChineseExponent::萬,
        unsigned_integer_to_chinese_low,
    )
}

pub(crate) fn unsigned_integer_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    value: u128,
) -> String {
    grouped_unsigned_integer_to_chinese::<1_0000_0000, 1000_0000>(
        chinese_variant,
        chinese_case,
        dependent,
        value,
        ChineseExponent::億,
        unsigned_integer_to_chinese_ten_thousand,
    )
}

pub(crate) fn big_unsigned_integer_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    value: BigUint,
) -> String {
    debug_assert!(value < BigUint::from(10u8).pow(96));

    grouped_big_unsigned_integer_to_chinese::<1_0000_0000, 1000_0000>(
        chinese_variant,
        chinese_case,
        dependent,
        value,
        ChineseExponent::億,
        unsigned_integer_to_chinese_ten_thousand,
    )
}

pub(crate) fn unsigned_integer_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: u128,
) -> String {
    let mut w = 1_0000_0000_0000_0000;

    let mut lower_d = value % w;
    value /= w;

    let mut has_more = value > 0;

    let mut s = if lower_d > 0 {
        unsigned_integer_to_chinese_middle(
            chinese_variant,
            chinese_case,
            dependent || has_more,
            lower_d,
        )
    } else if value == 0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    } else {
        String::new()
    };

    if !has_more {
        return s;
    }

    let mut i = ChineseExponent::兆.ordinal();

    let mut previous_w = w;

    loop {
        let d = value % w;
        value /= w;

        has_more = value > 0;

        if d > 0 {
            if lower_d < previous_w / 10 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
                unsigned_integer_to_chinese_high(
                    chinese_variant,
                    chinese_case,
                    dependent || has_more,
                    d,
                )
                .as_str(),
            );
        }

        if !has_more {
            break;
        }

        lower_d = d;

        i += 1;

        previous_w = w;
        w *= w;
    }

    s
}

pub(crate) fn big_unsigned_integer_to_chinese_high(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: BigUint,
) -> String {
    let big_0 = BigUint::zero();
    let big_10 = BigUint::from(10u8);

    let mut w = BigUint::from(1_0000_0000_0000_0000u64);

    let mut lower_d = value.clone() % &w;
    value /= &w;

    let mut has_more = value > big_0;

    let mut s = if lower_d > big_0 {
        unsigned_integer_to_chinese_middle(
            chinese_variant,
            chinese_case,
            dependent || has_more,
            lower_d.to_u128().unwrap(),
        )
    } else if value == big_0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    } else {
        String::new()
    };

    if !has_more {
        return s;
    }

    let mut i = ChineseExponent::兆.ordinal();

    let mut previous_w = w.clone();

    loop {
        let d = value.clone() % &w;
        value /= &w;

        has_more = value > big_0;

        if d > big_0 {
            if lower_d < previous_w / &big_10 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
                big_unsigned_integer_to_chinese_high(
                    chinese_variant,
                    chinese_case,
                    dependent || has_more,
                    d.clone(),
                )
                .as_str(),
            );
        }

        if !has_more {
            break;
        }

        lower_d = d;

        i += 1;

        previous_w = w.clone();
        w = w.clone() * w;
    }

    s
}

/// Converts a non-zero `BigUint` integer part with the selected Chinese count method.
#[inline]
fn big_integer_to_chinese_by_method(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    method: ChineseCountMethod,
    integer: &BigUint,
) -> Result<String, NumberToChineseError> {
    debug_assert!(!integer.is_zero());

    let big_10 = BigUint::from(10u8);

    match method {
        ChineseCountMethod::Low => {
            let limit = BigUint::from(1_0000_0000_0000_0000u64);

            if integer >= &limit {
                return Err(NumberToChineseError::Overflow);
            }

            Ok(unsigned_integer_to_chinese_low(
                chinese_variant,
                chinese_case,
                false,
                integer.to_u128().ok_or(NumberToChineseError::Overflow)?,
            ))
        },
        ChineseCountMethod::TenThousand => {
            let limit = big_10.pow(52);

            if integer >= &limit {
                return Err(NumberToChineseError::Overflow);
            }

            Ok(big_unsigned_integer_to_chinese_ten_thousand(
                chinese_variant,
                chinese_case,
                false,
                integer.clone(),
            ))
        },
        ChineseCountMethod::Middle => {
            let limit = big_10.pow(96);

            if integer >= &limit {
                return Err(NumberToChineseError::Overflow);
            }

            Ok(big_unsigned_integer_to_chinese_middle(
                chinese_variant,
                chinese_case,
                false,
                integer.clone(),
            ))
        },
        ChineseCountMethod::High => Ok(big_unsigned_integer_to_chinese_high(
            chinese_variant,
            chinese_case,
            false,
            integer.clone(),
        )),
    }
}

/// Appends the rounded money fraction with 角 and 分 units.
#[inline]
fn push_money_fraction(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    s: &mut String,
    fraction: u8,
) {
    debug_assert!(fraction < 100);

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        s.push_str(
            unsafe { ChineseNumber::from_ordinal_unsafe(msd) }
                .to_str(chinese_variant, chinese_case),
        );

        s.push_str(ChineseExponent::角.to_str(chinese_variant, chinese_case));

        if lsd > 0 {
            s.push_str(
                unsafe { ChineseNumber::from_ordinal_unsafe(lsd) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.push_str(ChineseExponent::分.to_str(chinese_variant, chinese_case));
        }
    } else if fraction >= 1 {
        s.push_str(
            unsafe { ChineseNumber::from_ordinal_unsafe(fraction) }
                .to_str(chinese_variant, chinese_case),
        );

        s.push_str(ChineseExponent::分.to_str(chinese_variant, chinese_case));
    }
}

pub(crate) fn positive_float_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    method: ChineseCountMethod,
    value: f64,
) -> Result<String, NumberToChineseError> {
    let (integer, fraction) = split_positive_f64(value)?;

    let mut s = if integer.is_zero() {
        String::new()
    } else {
        big_integer_to_chinese_by_method(chinese_variant, chinese_case, method, &integer)?
    };

    if fraction > 0 {
        push_money_fraction(chinese_variant, chinese_case, &mut s, fraction);
    } else if integer.is_zero() {
        s.push_str(ChineseNumber::零.to_str(chinese_variant, chinese_case));
    }

    Ok(s)
}
