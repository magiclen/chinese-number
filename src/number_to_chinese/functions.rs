use alloc::string::String;

use crate::chinese_characters::*;
use crate::{ChineseNumberCase, ChineseVariant};

#[inline]
pub(crate) fn get_chinese_number_table(
    variant: ChineseVariant,
    case: ChineseNumberCase,
) -> ChineseNumberTable {
    match variant {
        ChineseVariant::Simple => {
            match case {
                ChineseNumberCase::Lower => &CHINESE_NUMBERS_SIMPLE_LOWER,
                ChineseNumberCase::Upper => &CHINESE_NUMBERS_SIMPLE_UPPER,
            }
        }
        ChineseVariant::Traditional => {
            match case {
                ChineseNumberCase::Lower => &CHINESE_NUMBERS_TRADITIONAL_LOWER,
                ChineseNumberCase::Upper => &CHINESE_NUMBERS_TRADITIONAL_UPPER,
            }
        }
    }
}

#[inline]
pub(crate) fn get_chinese_negative_str(variant: ChineseVariant) -> &'static str {
    match variant {
        ChineseVariant::Simple => CHINESE_NEGATIVE_SIMPLE,
        ChineseVariant::Traditional => CHINESE_NEGATIVE_TRADITIONAL,
    }
}

#[inline]
pub(crate) fn digit_1(chinese_number_table: ChineseNumberTable, value: usize, buffer: &mut String) {
    debug_assert!(value < 10);

    buffer.push_str(chinese_number_table[value]);
}

#[inline]
pub(crate) fn digit_10(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!((10..100).contains(&value));

    let msd = value / 10;
    let lsd = value % 10;

    if msd != 1 || dependent || case == ChineseNumberCase::Upper {
        //  20 ->     二十
        //  10 ->       十
        // 110 -> 一百一十
        //  10 ->     壹拾
        digit_1(chinese_number_table, msd, buffer);
    }

    buffer.push_str(chinese_number_table[10]);

    if lsd > 0 {
        digit_1(chinese_number_table, lsd, buffer);
    }
}

#[inline]
pub(crate) fn digit_100(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!((100..1000).contains(&value));

    let msd = value / 100;
    let rds = value % 100;

    digit_1(chinese_number_table, msd, buffer);

    buffer.push_str(chinese_number_table[11]);

    if rds >= 10 {
        digit_10(chinese_number_table, case, true, rds, buffer);
    } else if rds >= 1 {
        buffer.push_str(chinese_number_table[0]);

        digit_1(chinese_number_table, rds, buffer);
    }
}

#[inline]
pub(crate) fn digit_100_compat(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!(value < 1000);

    if value >= 100 {
        digit_100(chinese_number_table, case, value, buffer);
    } else if value >= 10 {
        digit_10(chinese_number_table, case, dependent, value, buffer);
    } else if !dependent || value >= 1 {
        digit_1(chinese_number_table, value, buffer);
    }
}

#[inline]
pub(crate) fn digit_1000(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!((1000..10000).contains(&value));

    let msd = value / 1000;
    let rds = value % 1000;

    digit_1(chinese_number_table, msd, buffer);

    buffer.push_str(chinese_number_table[12]);

    if rds > 0 {
        if rds < 100 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_100_compat(chinese_number_table, case, true, rds, buffer);
    }
}

#[inline]
pub(crate) fn digit_1000_compat(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!(value < 10000);

    if value >= 1000 {
        digit_1000(chinese_number_table, case, value, buffer);
    } else {
        digit_100_compat(chinese_number_table, case, dependent, value, buffer);
    }
}

pub(crate) fn digit_10_000(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!((10_000..100_000_000).contains(&value));

    let msds = value / 10_000;
    let rds = value % 10_000;

    digit_1000_compat(chinese_number_table, case, dependent, msds, buffer);

    buffer.push_str(chinese_number_table[13]);

    if rds > 0 {
        if rds < 1000 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_1000_compat(chinese_number_table, case, true, rds, buffer);
    }
}

#[inline]
pub(crate) fn digit_10_000_compat(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: usize,
    buffer: &mut String,
) {
    debug_assert!(value < 100_000_000);

    if value >= 10_000 {
        digit_10_000(chinese_number_table, case, dependent, value, buffer);
    } else {
        digit_1000_compat(chinese_number_table, case, dependent, value, buffer);
    }
}

#[inline]
pub(crate) fn digit_100_000_000_compat(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: u64,
    buffer: &mut String,
) {
    debug_assert!(value < 10_000_000_000_000_000);

    if value < 100_000_000 {
        digit_10_000_compat(chinese_number_table, case, dependent, value as usize, buffer);
    } else {
        let msds = value / 100_000_000;
        let rds = value % 100_000_000;

        digit_10_000_compat(chinese_number_table, case, dependent, msds as usize, buffer);

        buffer.push_str(chinese_number_table[14]);

        if rds > 0 {
            if rds < 10_000_000 {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_10_000_compat(chinese_number_table, case, true, rds as usize, buffer);
        }
    }
}

#[inline]
pub(crate) fn digit_10_000_000_000_000_000_compat(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: u128,
    buffer: &mut String,
) {
    debug_assert!(value < 100_000_000_000_000_000_000_000_000_000_000);

    if value < 10_000_000_000_000_000 {
        digit_100_000_000_compat(chinese_number_table, case, dependent, value as u64, buffer);
    } else {
        let msds = value / 10_000_000_000_000_000;
        let rds = value % 10_000_000_000_000_000;

        digit_100_000_000_compat(chinese_number_table, case, dependent, msds as u64, buffer);

        buffer.push_str(chinese_number_table[15]);

        if rds > 0 {
            if rds < 1_000_000_000_000_000 {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_100_000_000_compat(chinese_number_table, case, true, rds as u64, buffer);
        }
    }
}

#[inline]
pub(crate) fn digit_100_000_000_000_000_000_000_000_000_000_000_compat(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    dependent: bool,
    value: u128,
    buffer: &mut String,
) {
    if value < 100_000_000_000_000_000_000_000_000_000_000 {
        digit_10_000_000_000_000_000_compat(chinese_number_table, case, dependent, value, buffer);
    } else {
        let msds = value / 100_000_000_000_000_000_000_000_000_000_000;
        let rds = value % 100_000_000_000_000_000_000_000_000_000_000;

        digit_10_000_000_000_000_000_compat(chinese_number_table, case, dependent, msds, buffer);

        buffer.push_str(chinese_number_table[16]);

        if rds > 0 {
            if rds < 100_000_000_000_000_000_000_000_000_000_000 {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_10_000_000_000_000_000_compat(chinese_number_table, case, true, rds, buffer);
        }
    }
}

pub(crate) fn digit_compat_low_u32(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    mut value: u32,
    buffer: &mut String,
) {
    if value == 0 {
        buffer.push_str(chinese_number_table[0]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=9).rev().map(|i| (i, 10u32.pow(i))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_1(chinese_number_table, msd as usize, buffer);

            buffer.push_str(chinese_number_table[(9 + i) as usize]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10_000 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_10_000_compat(chinese_number_table, case, zero_initial, value as usize, buffer);
    }
}

#[inline]
pub(crate) fn digit_compat_ten_thousand_u32(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: u32,
    buffer: &mut String,
) {
    if value >= 100_000_000 {
        let msds = value / 100_000_000;
        let rds = value % 100_000_000;

        digit_1000_compat(chinese_number_table, case, false, msds as usize, buffer);

        buffer.push_str(chinese_number_table[14]);

        if rds > 0 {
            if rds < 1000 {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_10_000_compat(chinese_number_table, case, true, rds as usize, buffer);
        }
    } else {
        digit_10_000_compat(chinese_number_table, case, false, value as usize, buffer);
    }
}

pub(crate) fn digit_compat_low_u64(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    mut value: u64,
    buffer: &mut String,
) {
    debug_assert!(value < 10_000_000_000_000_000); // support to "極"

    if value == 0 {
        buffer.push_str(chinese_number_table[0]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=15).rev().map(|i| (i, 10u64.pow(i))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_1(chinese_number_table, msd as usize, buffer);

            buffer.push_str(chinese_number_table[(9 + i) as usize]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10_000 {
            buffer.push_str(chinese_number_table[0]);
        }
        digit_10_000_compat(chinese_number_table, case, zero_initial, value as usize, buffer);
    }
}

pub(crate) fn digit_compat_ten_thousand_u64(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    mut value: u64,
    buffer: &mut String,
) {
    if value == 0 {
        buffer.push_str(chinese_number_table[0]);

        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=7).rev().map(|i| (i, 10u64.pow((i - 3) * 4))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 1000) {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_1000_compat(chinese_number_table, case, zero_initial, msd as usize, buffer);

            buffer.push_str(chinese_number_table[(9 + i) as usize]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10_000_000 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_10_000_compat(chinese_number_table, case, zero_initial, value as usize, buffer);
    }
}

pub(crate) fn digit_compat_middle_u64(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    mut value: u64,
    buffer: &mut String,
) {
    if value == 0 {
        buffer.push_str(chinese_number_table[0]);

        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=6).rev().map(|i| (i, 10u64.pow((i - 4) * 8))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 10_000_000) {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_10_000_compat(chinese_number_table, case, zero_initial, msd as usize, buffer);

            buffer.push_str(chinese_number_table[(9 + i) as usize]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10_000_000 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_10_000_compat(chinese_number_table, case, zero_initial, value as usize, buffer);
    }
}

#[inline]
pub(crate) fn digit_compat_high_u64(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: u64,
    buffer: &mut String,
) {
    digit_100_000_000_000_000_000_000_000_000_000_000_compat(
        chinese_number_table,
        case,
        false,
        value as u128,
        buffer,
    );
}

pub(crate) fn digit_compat_ten_thousand_u128(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    mut value: u128,
    buffer: &mut String,
) {
    if value == 0 {
        buffer.push_str(chinese_number_table[0]);

        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=12).rev().map(|i| (i, 10u128.pow((i - 3) * 4))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 1000) {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_1000_compat(chinese_number_table, case, zero_initial, msd as usize, buffer);

            buffer.push_str(chinese_number_table[(9 + i) as usize]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10_000_000 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_10_000_compat(chinese_number_table, case, zero_initial, value as usize, buffer);
    }
}

pub(crate) fn digit_compat_middle_u128(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    mut value: u128,
    buffer: &mut String,
) {
    if value == 0 {
        buffer.push_str(chinese_number_table[0]);

        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=8).rev().map(|i| (i, 10u128.pow((i - 4) * 8))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 10_000_000) {
                buffer.push_str(chinese_number_table[0]);
            }

            digit_10_000_compat(chinese_number_table, case, zero_initial, msd as usize, buffer);

            buffer.push_str(chinese_number_table[(9 + i) as usize]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10_000_000 {
            buffer.push_str(chinese_number_table[0]);
        }

        digit_10_000_compat(chinese_number_table, case, zero_initial, value as usize, buffer);
    }
}

#[inline]
pub(crate) fn digit_compat_high_u128(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: u128,
    buffer: &mut String,
) {
    digit_100_000_000_000_000_000_000_000_000_000_000_compat(
        chinese_number_table,
        case,
        false,
        value,
        buffer,
    );
}

#[inline]
pub(crate) fn fraction_compat_low(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: f64,
    buffer: &mut String,
) {
    let integer = value as u64;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_low_u64(chinese_number_table, case, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_table, msd, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[0]);

        if lsd > 0 {
            digit_1(chinese_number_table, lsd, buffer);

            buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_table, fraction, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
    } else if integer == 0 {
        buffer.push_str(chinese_number_table[0]);
    }
}

#[inline]
pub(crate) fn fraction_compat_ten_thousand(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: f64,
    buffer: &mut String,
) {
    let integer = value as u128;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_ten_thousand_u128(chinese_number_table, case, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_table, msd, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[0]);

        if lsd > 0 {
            digit_1(chinese_number_table, lsd, buffer);

            buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_table, fraction, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
    } else if integer == 0 {
        buffer.push_str(chinese_number_table[0]);
    }
}

#[inline]
pub(crate) fn fraction_compat_middle(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: f64,
    buffer: &mut String,
) {
    let integer = value as u128;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_middle_u128(chinese_number_table, case, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_table, msd, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[0]);

        if lsd > 0 {
            digit_1(chinese_number_table, lsd, buffer);

            buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_table, fraction, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
    } else if integer == 0 {
        buffer.push_str(chinese_number_table[0]);
    }
}

#[inline]
pub(crate) fn fraction_compat_high(
    chinese_number_table: ChineseNumberTable,
    case: ChineseNumberCase,
    value: f64,
    buffer: &mut String,
) {
    let integer = value as u128;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_high_u128(chinese_number_table, case, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_table, msd, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[0]);

        if lsd > 0 {
            digit_1(chinese_number_table, lsd, buffer);

            buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_table, fraction, buffer);

        buffer.push_str(CHINESE_NUMBERS_FRACTION[1]);
    } else if integer == 0 {
        buffer.push_str(chinese_number_table[0]);
    }
}
