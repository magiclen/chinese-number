use ::{constants::*, ChineseNumberCase, ChineseVariant};

pub(crate) fn get_chinese_number_index(variant: ChineseVariant, case: ChineseNumberCase) -> usize {
    (variant as usize * 2) + case as usize
}

pub(crate) fn digit_1(chinese_number_index: usize, value: usize, buffer: &mut String) {
    debug_assert!(value < 10);

    buffer.push_str(CHINESE_NUMBERS[value][chinese_number_index]);
}

pub(crate) fn digit_10(chinese_number_index: usize, value: usize, buffer: &mut String, dependent: bool) {
    debug_assert!(value < 100 && value >= 10);

    let msd = value / 10;
    let lsd = value % 10;

    if msd != 1 || dependent || chinese_number_index % 2 == 0 {
        digit_1(chinese_number_index, msd, buffer);
    }

    buffer.push_str(CHINESE_NUMBERS[10][chinese_number_index]);

    if lsd > 0 {
        digit_1(chinese_number_index, lsd, buffer);
    }
}

pub(crate) fn digit_100(chinese_number_index: usize, value: usize, buffer: &mut String) {
    debug_assert!(value < 1000 && value >= 100);

    let msd = value / 100;
    let rds = value % 100;

    digit_1(chinese_number_index, msd, buffer);

    buffer.push_str(CHINESE_NUMBERS[11][chinese_number_index]);

    if rds >= 10 {
        digit_10(chinese_number_index, rds, buffer, true);
    } else if rds >= 1 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        digit_1(chinese_number_index, rds, buffer);
    }
}

pub(crate) fn digit_1000(chinese_number_index: usize, value: usize, buffer: &mut String) {
    debug_assert!(value < 10000 && value >= 1000);

    let msd = value / 1000;
    let rds = value % 1000;

    digit_1(chinese_number_index, msd, buffer);

    buffer.push_str(CHINESE_NUMBERS[12][chinese_number_index]);

    if rds >= 100 {
        digit_100(chinese_number_index, rds, buffer);
    } else if rds >= 10 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        digit_10(chinese_number_index, rds, buffer, true);
    } else if rds >= 1 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        digit_1(chinese_number_index, rds, buffer);
    }
}

pub(crate) fn digit_1000_compat(chinese_number_index: usize, value: usize, buffer: &mut String, dependent: bool) {
    debug_assert!(value < 10000);

    if value >= 1000 {
        digit_1000(chinese_number_index, value, buffer);
    } else if value >= 100 {
        digit_100(chinese_number_index, value, buffer);
    } else if value >= 10 {
        digit_10(chinese_number_index, value, buffer, dependent);
    } else if !dependent || value >= 1 {
        digit_1(chinese_number_index, value, buffer);
    }
}

pub(crate) fn digit_10000(chinese_number_index: usize, value: usize, buffer: &mut String, dependent: bool) {
    debug_assert!(value < 100000000 && value >= 10000);

    let msds = value / 10000;
    let rds = value % 10000;

    digit_1000_compat(chinese_number_index, msds, buffer, dependent);

    buffer.push_str(CHINESE_NUMBERS[13][chinese_number_index]);

    if rds > 0 && rds < 1000 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
    }

    digit_1000_compat(chinese_number_index, rds, buffer, true);
}

pub(crate) fn digit_10000_compat(chinese_number_index: usize, value: usize, buffer: &mut String, dependent: bool) {
    debug_assert!(value < 100000000);

    if value >= 10000 {
        digit_10000(chinese_number_index, value, buffer, dependent);
    } else {
        digit_1000_compat(chinese_number_index, value, buffer, dependent);
    }
}

pub(crate) fn digit_100000000_compat(chinese_number_index: usize, value: u64, buffer: &mut String, dependent: bool) {
    debug_assert!(value < 10000000000000000);

    if value < 100000000 {
        digit_10000_compat(chinese_number_index, value as usize, buffer, dependent);
    } else {
        let msds = value / 100000000;
        let rds = value % 100000000;

        digit_10000_compat(chinese_number_index, msds as usize, buffer, dependent);

        buffer.push_str(CHINESE_NUMBERS[14][chinese_number_index]);

        if rds > 0 && rds < 10000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }

        digit_10000_compat(chinese_number_index, rds as usize, buffer, true);
    }
}

pub(crate) fn digit_10000000000000000_compat(chinese_number_index: usize, value: u128, buffer: &mut String, dependent: bool) {
    debug_assert!(value < 100000000000000000000000000000000);

    if value < 10000000000000000 {
        digit_100000000_compat(chinese_number_index, value as u64, buffer, dependent);
    } else {
        let msds = value / 10000000000000000;
        let rds = value % 10000000000000000;

        digit_100000000_compat(chinese_number_index, msds as u64, buffer, dependent);

        buffer.push_str(CHINESE_NUMBERS[15][chinese_number_index]);

        if rds > 0 && rds < 1000000000000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }

        digit_100000000_compat(chinese_number_index, rds as u64, buffer, true);
    }
}

pub(crate) fn digit_100000000000000000000000000000000_compat(chinese_number_index: usize, value: u128, buffer: &mut String, dependent: bool) {
    if value < 100000000000000000000000000000000 {
        digit_10000000000000000_compat(chinese_number_index, value, buffer, dependent);
    } else {
        let msds = value / 100000000000000000000000000000000;
        let rds = value % 100000000000000000000000000000000;

        digit_10000000000000000_compat(chinese_number_index, msds, buffer, dependent);

        buffer.push_str(CHINESE_NUMBERS[16][chinese_number_index]);

        if rds > 0 && rds < 100000000000000000000000000000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }

        digit_10000000000000000_compat(chinese_number_index, rds, buffer, true);
    }
}

pub(crate) fn fraction_compat_low(chinese_number_index: usize, value: f64, buffer: &mut String) {
    let integer = value as u64;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_low_u64(chinese_number_index, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_index, msd, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[0][chinese_number_index]);

        if lsd > 0 {
            digit_1(chinese_number_index, lsd, buffer);
            buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_index, fraction, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
    } else if integer == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
    }
}

pub(crate) fn fraction_compat_ten_thousand(chinese_number_index: usize, value: f64, buffer: &mut String) {
    let integer = value as u128;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_ten_thousand_u128(chinese_number_index, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_index, msd, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[0][chinese_number_index]);

        if lsd > 0 {
            digit_1(chinese_number_index, lsd, buffer);
            buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_index, fraction, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
    } else if integer == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
    }
}

pub(crate) fn fraction_compat_middle(chinese_number_index: usize, value: f64, buffer: &mut String) {
    let integer = value as u128;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_middle_u128(chinese_number_index, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_index, msd, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[0][chinese_number_index]);

        if lsd > 0 {
            digit_1(chinese_number_index, lsd, buffer);
            buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_index, fraction, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
    } else if integer == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
    }
}

pub(crate) fn fraction_compat_high(chinese_number_index: usize, value: f64, buffer: &mut String) {
    let integer = value as u128;
    let fraction = (value * 100.0) as usize % 100;

    if integer > 0 {
        digit_compat_high_u128(chinese_number_index, integer, buffer);
    }

    if fraction >= 10 {
        let msd = fraction / 10;
        let lsd = fraction % 10;

        digit_1(chinese_number_index, msd, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[0][chinese_number_index]);

        if lsd > 0 {
            digit_1(chinese_number_index, lsd, buffer);
            buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
        }
    } else if fraction >= 1 {
        digit_1(chinese_number_index, fraction, buffer);
        buffer.push_str(CHINESE_NUMBERS_FRACTION[1][chinese_number_index]);
    } else if integer == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
    }
}

pub(crate) fn digit_compat_low_u32(chinese_number_index: usize, mut value: u32, buffer: &mut String) {
    if value == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=9).rev().map(|i| (i, 10u32.pow(i))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero {
                buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
            }

            digit_1(chinese_number_index, msd as usize, buffer);

            buffer.push_str(CHINESE_NUMBERS[(9 + i) as usize][chinese_number_index]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }
        digit_10000_compat(chinese_number_index, value as usize, buffer, zero_initial);
    }
}

pub(crate) fn digit_compat_low_u64(chinese_number_index: usize, mut value: u64, buffer: &mut String) {
    assert!(value < 10000000000000000); // support to "極"

    if value == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=15).rev().map(|i| (i, 10u64.pow(i))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero {
                buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
            }

            digit_1(chinese_number_index, msd as usize, buffer);

            buffer.push_str(CHINESE_NUMBERS[(9 + i) as usize][chinese_number_index]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }
        digit_10000_compat(chinese_number_index, value as usize, buffer, zero_initial);
    }
}

pub(crate) fn digit_compat_ten_thousand_u32(chinese_number_index: usize, value: u32, buffer: &mut String) {
    if value >= 100000000 {
        let msds = value / 100000000;
        let rds = value % 100000000;

        digit_1000_compat(chinese_number_index, msds as usize, buffer, false);

        buffer.push_str(CHINESE_NUMBERS[14][chinese_number_index]);

        if rds > 0 && rds < 1000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }

        digit_10000_compat(chinese_number_index, rds as usize, buffer, true);
    } else {
        digit_10000_compat(chinese_number_index, value as usize, buffer, false);
    }
}

pub(crate) fn digit_compat_ten_thousand_u64(chinese_number_index: usize, mut value: u64, buffer: &mut String) {
    if value == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=7).rev().map(|i| (i, 10u64.pow((i - 3) * 4))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 1000) {
                buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
            }

            digit_1000_compat(chinese_number_index, msd as usize, buffer, zero_initial);

            buffer.push_str(CHINESE_NUMBERS[(9 + i) as usize][chinese_number_index]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }
        digit_10000_compat(chinese_number_index, value as usize, buffer, zero_initial);
    }
}

pub(crate) fn digit_compat_ten_thousand_u128(chinese_number_index: usize, mut value: u128, buffer: &mut String) {
    if value == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=12).rev().map(|i| (i, 10u128.pow((i - 3) * 4))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 1000) {
                buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
            }

            digit_1000_compat(chinese_number_index, msd as usize, buffer, zero_initial);

            buffer.push_str(CHINESE_NUMBERS[(9 + i) as usize][chinese_number_index]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }
        digit_10000_compat(chinese_number_index, value as usize, buffer, zero_initial);
    }
}

pub(crate) fn digit_compat_middle_u64(chinese_number_index: usize, mut value: u64, buffer: &mut String) {
    if value == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=6).rev().map(|i| (i, 10u64.pow((i - 4) * 8))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 10000000) {
                buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
            }

            digit_10000_compat(chinese_number_index, msd as usize, buffer, zero_initial);

            buffer.push_str(CHINESE_NUMBERS[(9 + i) as usize][chinese_number_index]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }
        digit_10000_compat(chinese_number_index, value as usize, buffer, zero_initial);
    }
}

pub(crate) fn digit_compat_middle_u128(chinese_number_index: usize, mut value: u128, buffer: &mut String) {
    if value == 0 {
        buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        return;
    }

    let mut zero_initial = false;
    let mut zero = false;

    for (i, v) in (5..=8).rev().map(|i| (i, 10u128.pow((i - 4) * 8))) {
        if value >= v {
            let msd = value / v;
            value %= v;

            if zero || (zero_initial && msd < 10000000) {
                buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
            }

            digit_10000_compat(chinese_number_index, msd as usize, buffer, zero_initial);

            buffer.push_str(CHINESE_NUMBERS[(9 + i) as usize][chinese_number_index]);

            zero = false;

            zero_initial = true;
        } else if zero_initial {
            zero = true;
        }
    }

    if value > 0 {
        if zero_initial && value < 10000000 {
            buffer.push_str(CHINESE_NUMBERS[0][chinese_number_index]);
        }
        digit_10000_compat(chinese_number_index, value as usize, buffer, zero_initial);
    }
}

pub(crate) fn digit_compat_high_u128(chinese_number_index: usize, value: u128, buffer: &mut String) {
    digit_100000000000000000000000000000000_compat(chinese_number_index, value, buffer, false);
}

pub(crate) fn chinese_digit_1(value: char) -> Result<u8, usize> {
    for i in 0..=9 {
        if CHINESE_NUMBERS_CHARS[i].contains(&value) {
            return Ok(i as u8);
        }
    }

    Err(0)
}

pub(crate) fn chinese_digit_10(value: char, value2: Option<char>, value3: Option<char>) -> Result<u8, usize> {
    if CHINESE_NUMBERS_CHARS[10].contains(&value) {
        if let Some(_) = value3 {
            Err(2)
        } else if let Some(value2) = value2 {
            let lsd = chinese_digit_1(value2).map_err(|_| 1u8)?;

            Ok(10 + lsd)
        } else {
            Ok(10)
        }
    } else {
        let msd = chinese_digit_1(value)?;

        if msd == 0 {
            if let Some(_) = value2 {
                Err(1)
            } else {
                debug_assert_eq!(None, value3);
                Ok(0)
            }
        } else {
            if let Some(value2) = value2 {
                if !CHINESE_NUMBERS_CHARS[10].contains(&value2) {
                    Err(1)
                } else {
                    if let Some(value3) = value3 {
                        let lsd = chinese_digit_1(value3).map_err(|_| 2u8)?;

                        if lsd == 0 {
                            Err(2)
                        } else {
                            Ok(msd * 10 + lsd)
                        }
                    } else {
                        Ok(msd * 10)
                    }
                }
            } else {
                Err(1)
            }
        }
    }
}

pub(crate) fn chinese_digit_10_compat(value: char, value2: Option<char>, value3: Option<char>) -> Result<u8, usize> {
    match chinese_digit_10(value, value2, value3) {
        Ok(number) => Ok(number),
        Err(err) => match chinese_digit_1(value) {
            Ok(number) => {
                if let Some(_) = value2 {
                    Err(1)
                } else {
                    debug_assert_eq!(None, value3);
                    Ok(number)
                }
            }
            Err(_) => Err(err)
        }
    }
}

pub(crate) fn chinese_digit_100(value: char, value2: char, value3: Option<char>, value4: Option<char>, value5: Option<char>) -> Result<u16, usize> {
    let msd = chinese_digit_1(value)?;

    if msd == 0 {
        Err(0)
    } else {
        if !CHINESE_NUMBERS_CHARS[11].contains(&value2) {
            Err(1)
        } else {
            if let Some(value3) = value3 {
                if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                    if let Some(value4) = value4 {
                        let lsd = chinese_digit_1(value4).map_err(|_| 3u8)?;

                        if lsd == 0 {
                            Err(3)
                        } else {
                            if let Some(_) = value5 {
                                Err(4)
                            } else {
                                Ok(msd as u16 * 100 + lsd as u16)
                            }
                        }
                    } else {
                        Err(3)
                    }
                } else {
                    let rds = chinese_digit_10(value3, value4, value5).map_err(|err| err + 2)?;

                    Ok(msd as u16 * 100 + rds as u16)
                }
            } else {
                Ok(msd as u16 * 100)
            }
        }
    }
}

pub(crate) fn chinese_digit_100_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>) -> Result<u16, usize> {
    match value2 {
        Some(value2) => match chinese_digit_100(value, value2, value3, value4, value5) {
            Ok(number) => Ok(number),
            Err(err) => match chinese_digit_10_compat(value, Some(value2), value3) {
                Ok(number) => {
                    if let Some(_) = value4 {
                        Err(3)
                    } else {
                        debug_assert_eq!(None, value5);
                        Ok(number as u16)
                    }
                }
                Err(_) => Err(err)
            }
        }
        None => match chinese_digit_10_compat(value, None, None) {
            Ok(number) => {
                debug_assert_eq!(None, value3);
                debug_assert_eq!(None, value4);
                debug_assert_eq!(None, value5);
                Ok(number as u16)
            }
            Err(err) => Err(err)
        }
    }
}

pub(crate) fn chinese_digit_1000(value: char, value2: char, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>) -> Result<u16, usize> {
    let msd = chinese_digit_1(value)?;

    if msd == 0 {
        Err(0)
    } else {
        if !CHINESE_NUMBERS_CHARS[12].contains(&value2) {
            Err(1)
        } else {
            if let Some(value3) = value3 {
                if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                    if let Some(value4) = value4 {
                        if let Some(value5) = value5 {
                            if let Some(_) = value7 {
                                Err(6)
                            } else {
                                let rds = chinese_digit_10(value4, Some(value5), value6).map_err(|_| 3u8)?;

                                Ok(msd as u16 * 1000 + rds as u16)
                            }
                        } else {
                            if let Some(_) = value6 {
                                Err(5)
                            } else {
                                debug_assert_eq!(None, value7);

                                let rds = chinese_digit_1(value4).map_err(|_| 3u8)?;

                                Ok(msd as u16 * 1000 + rds as u16)
                            }
                        }
                    } else {
                        Err(3)
                    }
                } else {
                    if let Some(value4) = value4 {
                        let rds = chinese_digit_100(value3, value4, value5, value6, value7).map_err(|err| err + 2)?;

                        Ok(msd as u16 * 1000 + rds as u16)
                    } else {
                        Err(3)
                    }
                }
            } else {
                Ok(msd as u16 * 1000)
            }
        }
    }
}

pub(crate) fn chinese_digit_1000_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>) -> Result<u16, usize> {
    match value2 {
        Some(value2) => match chinese_digit_1000(value, value2, value3, value4, value5, value6, value7) {
            Ok(number) => Ok(number),
            Err(err) => match chinese_digit_100_compat(value, Some(value2), value3, value4, value5) {
                Ok(number) => {
                    if let Some(_) = value6 {
                        Err(5)
                    } else {
                        debug_assert_eq!(None, value7);
                        Ok(number as u16)
                    }
                }
                Err(_) => Err(err)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u16)
        }
    }
}

pub(crate) fn chinese_digit_10000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>) -> Result<u32, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[13].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value9);
                                let rds = chinese_digit_100_compat(value4, value5, value6, value7, value8).map_err(|err| err + 3)?;
                                Ok(msd as u32 * 10000 + rds as u32)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_1000_compat(value3, value4, value5, value6, value7, value8, value9).map_err(|err| err + 2)?;
                            Ok(msd as u32 * 10000 + rds as u32)
                        }
                    } else {
                        Ok(msd as u32 * 10000)
                    }
                }
            } else {
                debug_assert_eq!(None, value8);
                debug_assert_eq!(None, value9);
                let rds = chinese_digit_1000_compat(value, Some(value2), value3, value4, value5, value6, value7)?;
                Ok(rds as u32)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u32)
        }
    }
}

pub(crate) fn chinese_digit_100000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>) -> Result<u32, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[14].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value11);
                                let rds = chinese_digit_1000_compat(value4, value5, value6, value7, value8, value9, value10).map_err(|err| err + 3)?;
                                Ok(msd as u32 * 100000 + rds as u32)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_10000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11).map_err(|err| err + 2)?;
                            Ok(msd as u32 * 100000 + rds as u32)
                        }
                    } else {
                        Ok(msd as u32 * 100000)
                    }
                }
            } else {
                debug_assert_eq!(None, value10);
                debug_assert_eq!(None, value11);
                let rds = chinese_digit_10000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9)?;
                Ok(rds as u32)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u32)
        }
    }
}

pub(crate) fn chinese_digit_1000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>) -> Result<u32, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[15].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value13);
                                let rds = chinese_digit_10000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12).map_err(|err| err + 3)?;
                                Ok(msd as u32 * 1000000 + rds as u32)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_100000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13).map_err(|err| err + 2)?;
                            Ok(msd as u32 * 1000000 + rds as u32)
                        }
                    } else {
                        Ok(msd as u32 * 1000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value12);
                debug_assert_eq!(None, value13);
                let rds = chinese_digit_100000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11)?;
                Ok(rds as u32)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u32)
        }
    }
}

pub(crate) fn chinese_digit_10000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>) -> Result<u32, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[16].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value15);
                                let rds = chinese_digit_100000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14).map_err(|err| err + 3)?;
                                Ok(msd as u32 * 10000000 + rds as u32)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_1000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15).map_err(|err| err + 2)?;
                            Ok(msd as u32 * 10000000 + rds as u32)
                        }
                    } else {
                        Ok(msd as u32 * 10000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value14);
                debug_assert_eq!(None, value15);
                let rds = chinese_digit_1000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13)?;
                Ok(rds as u32)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u32)
        }
    }
}

pub(crate) fn chinese_digit_100000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>) -> Result<u32, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[17].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value17);
                                let rds = chinese_digit_1000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16).map_err(|err| err + 3)?;
                                Ok(msd as u32 * 100000000 + rds as u32)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_10000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17).map_err(|err| err + 2)?;
                            Ok(msd as u32 * 100000000 + rds as u32)
                        }
                    } else {
                        Ok(msd as u32 * 100000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value16);
                debug_assert_eq!(None, value17);
                let rds = chinese_digit_10000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15)?;
                Ok(rds as u32)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u32)
        }
    }
}

pub(crate) fn chinese_digit_1000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[18].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value19);
                                let rds = chinese_digit_10000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 1000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_100000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 1000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 1000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value18);
                debug_assert_eq!(None, value19);
                let rds = chinese_digit_100000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_10000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[19].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value21);
                                let rds = chinese_digit_100000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 10000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_1000000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 10000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 10000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value20);
                debug_assert_eq!(None, value21);
                let rds = chinese_digit_1000000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_100000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[20].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value23);
                                let rds = chinese_digit_1000000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 100000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_10000000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 100000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 100000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value22);
                debug_assert_eq!(None, value23);
                let rds = chinese_digit_10000000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_1000000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[21].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value25);
                                let rds = chinese_digit_10000000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 1000000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_100000000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 1000000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 1000000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value24);
                debug_assert_eq!(None, value25);
                let rds = chinese_digit_100000000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_10000000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>, value26: Option<char>, value27: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[22].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value27);
                                let rds = chinese_digit_100000000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 10000000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_1000000000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 10000000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 10000000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value26);
                debug_assert_eq!(None, value27);
                let rds = chinese_digit_1000000000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            debug_assert_eq!(None, value26);
            debug_assert_eq!(None, value27);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_100000000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>, value26: Option<char>, value27: Option<char>, value28: Option<char>, value29: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[23].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value29);
                                let rds = chinese_digit_1000000000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 100000000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_10000000000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 100000000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 100000000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value28);
                debug_assert_eq!(None, value29);
                let rds = chinese_digit_10000000000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            debug_assert_eq!(None, value26);
            debug_assert_eq!(None, value27);
            debug_assert_eq!(None, value28);
            debug_assert_eq!(None, value29);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_1000000000000000_low_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>, value26: Option<char>, value27: Option<char>, value28: Option<char>, value29: Option<char>, value30: Option<char>, value31: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[24].contains(&value2) {
                let msd = chinese_digit_1(value)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value31);
                                let rds = chinese_digit_10000000000000_low_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30).map_err(|err| err + 3)?;
                                Ok(msd as u64 * 1000000000000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            let rds = chinese_digit_100000000000000_low_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31).map_err(|err| err + 2)?;
                            Ok(msd as u64 * 1000000000000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 1000000000000000)
                    }
                }
            } else {
                debug_assert_eq!(None, value30);
                debug_assert_eq!(None, value31);
                let rds = chinese_digit_100000000000000_low_compat(value, Some(value2), value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29)?;
                Ok(rds as u64)
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            debug_assert_eq!(None, value26);
            debug_assert_eq!(None, value27);
            debug_assert_eq!(None, value28);
            debug_assert_eq!(None, value29);
            debug_assert_eq!(None, value30);
            debug_assert_eq!(None, value31);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_10000_ten_thousand_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>) -> Result<u32, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[13].contains(&value2) {
                let msd = chinese_digit_10_compat(value, None, None)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value9);
                                debug_assert_eq!(None, value10);
                                debug_assert_eq!(None, value11);
                                debug_assert_eq!(None, value12);
                                debug_assert_eq!(None, value13);
                                debug_assert_eq!(None, value14);
                                debug_assert_eq!(None, value15);
                                let rds = chinese_digit_100_compat(value4, value5, value6, value7, value8).map_err(|err| err + 3)?;
                                Ok(msd as u32 * 10000 + rds as u32)
                            } else {
                                Err(3)
                            }
                        } else {
                            debug_assert_eq!(None, value10);
                            debug_assert_eq!(None, value11);
                            debug_assert_eq!(None, value12);
                            debug_assert_eq!(None, value13);
                            debug_assert_eq!(None, value14);
                            debug_assert_eq!(None, value15);
                            let rds = chinese_digit_1000_compat(value3, value4, value5, value6, value7, value8, value9).map_err(|err| err + 2)?;
                            Ok(msd as u32 * 10000 + rds as u32)
                        }
                    } else {
                        Ok(msd as u32 * 10000)
                    }
                }
            } else {
                match value3 {
                    Some(value3) => {
                        if CHINESE_NUMBERS_CHARS[13].contains(&value3) {
                            let msd = chinese_digit_1000_compat(value, Some(value2), None, None, None, None, None)?;

                            if msd == 0 {
                                Err(0)
                            } else {
                                if let Some(value4) = value4 {
                                    if CHINESE_NUMBERS_CHARS[0].contains(&value4) {
                                        if let Some(value5) = value5 {
                                            debug_assert_eq!(None, value10);
                                            debug_assert_eq!(None, value11);
                                            debug_assert_eq!(None, value12);
                                            debug_assert_eq!(None, value13);
                                            debug_assert_eq!(None, value14);
                                            debug_assert_eq!(None, value15);
                                            let rds = chinese_digit_100_compat(value5, value6, value7, value8, value9).map_err(|err| err + 4)?;
                                            Ok(msd as u32 * 10000 + rds as u32)
                                        } else {
                                            Err(4)
                                        }
                                    } else {
                                        debug_assert_eq!(None, value11);
                                        debug_assert_eq!(None, value12);
                                        debug_assert_eq!(None, value13);
                                        debug_assert_eq!(None, value14);
                                        debug_assert_eq!(None, value15);
                                        let rds = chinese_digit_1000_compat(value4, value5, value6, value7, value8, value9, value10).map_err(|err| err + 3)?;
                                        Ok(msd as u32 * 10000 + rds as u32)
                                    }
                                } else {
                                    Ok(msd as u32 * 10000)
                                }
                            }
                        } else {
                            match value4 {
                                Some(value4) => {
                                    if CHINESE_NUMBERS_CHARS[13].contains(&value4) {
                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), None, None, None, None)?;

                                        if msd == 0 {
                                            Err(0)
                                        } else {
                                            if let Some(value5) = value5 {
                                                if CHINESE_NUMBERS_CHARS[0].contains(&value5) {
                                                    if let Some(value6) = value6 {
                                                        debug_assert_eq!(None, value11);
                                                        debug_assert_eq!(None, value12);
                                                        debug_assert_eq!(None, value13);
                                                        debug_assert_eq!(None, value14);
                                                        debug_assert_eq!(None, value15);
                                                        let rds = chinese_digit_100_compat(value6, value7, value8, value9, value10).map_err(|err| err + 5)?;
                                                        Ok(msd as u32 * 10000 + rds as u32)
                                                    } else {
                                                        Err(5)
                                                    }
                                                } else {
                                                    debug_assert_eq!(None, value12);
                                                    debug_assert_eq!(None, value13);
                                                    debug_assert_eq!(None, value14);
                                                    debug_assert_eq!(None, value15);
                                                    let rds = chinese_digit_1000_compat(value5, value6, value7, value8, value9, value10, value11).map_err(|err| err + 4)?;
                                                    Ok(msd as u32 * 10000 + rds as u32)
                                                }
                                            } else {
                                                Ok(msd as u32 * 10000)
                                            }
                                        }
                                    } else {
                                        match value5 {
                                            Some(value5) => {
                                                if CHINESE_NUMBERS_CHARS[13].contains(&value5) {
                                                    let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), None, None, None)?;

                                                    if msd == 0 {
                                                        Err(0)
                                                    } else {
                                                        if let Some(value6) = value6 {
                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value6) {
                                                                if let Some(value7) = value7 {
                                                                    debug_assert_eq!(None, value12);
                                                                    debug_assert_eq!(None, value13);
                                                                    debug_assert_eq!(None, value14);
                                                                    debug_assert_eq!(None, value15);
                                                                    let rds = chinese_digit_100_compat(value7, value8, value9, value10, value11).map_err(|err| err + 6)?;
                                                                    Ok(msd as u32 * 10000 + rds as u32)
                                                                } else {
                                                                    Err(6)
                                                                }
                                                            } else {
                                                                debug_assert_eq!(None, value13);
                                                                debug_assert_eq!(None, value14);
                                                                debug_assert_eq!(None, value15);
                                                                let rds = chinese_digit_1000_compat(value6, value7, value8, value9, value10, value11, value12).map_err(|err| err + 5)?;
                                                                Ok(msd as u32 * 10000 + rds as u32)
                                                            }
                                                        } else {
                                                            Ok(msd as u32 * 10000)
                                                        }
                                                    }
                                                } else {
                                                    match value6 {
                                                        Some(value6) => {
                                                            if CHINESE_NUMBERS_CHARS[13].contains(&value6) {
                                                                let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None)?;

                                                                if msd == 0 {
                                                                    Err(0)
                                                                } else {
                                                                    if let Some(value7) = value7 {
                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value7) {
                                                                            if let Some(value8) = value8 {
                                                                                debug_assert_eq!(None, value13);
                                                                                debug_assert_eq!(None, value14);
                                                                                debug_assert_eq!(None, value15);
                                                                                let rds = chinese_digit_100_compat(value8, value9, value10, value11, value12).map_err(|err| err + 7)?;
                                                                                Ok(msd as u32 * 10000 + rds as u32)
                                                                            } else {
                                                                                Err(7)
                                                                            }
                                                                        } else {
                                                                            debug_assert_eq!(None, value14);
                                                                            debug_assert_eq!(None, value15);
                                                                            let rds = chinese_digit_1000_compat(value7, value8, value9, value10, value11, value12, value13).map_err(|err| err + 6)?;
                                                                            Ok(msd as u32 * 10000 + rds as u32)
                                                                        }
                                                                    } else {
                                                                        Ok(msd as u32 * 10000)
                                                                    }
                                                                }
                                                            } else {
                                                                match value7 {
                                                                    Some(value7) => {
                                                                        if CHINESE_NUMBERS_CHARS[13].contains(&value7) {
                                                                            let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None)?;

                                                                            if msd == 0 {
                                                                                Err(0)
                                                                            } else {
                                                                                if let Some(value8) = value8 {
                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value8) {
                                                                                        if let Some(value9) = value9 {
                                                                                            debug_assert_eq!(None, value14);
                                                                                            debug_assert_eq!(None, value15);
                                                                                            let rds = chinese_digit_100_compat(value9, value10, value11, value12, value13).map_err(|err| err + 8)?;
                                                                                            Ok(msd as u32 * 10000 + rds as u32)
                                                                                        } else {
                                                                                            Err(8)
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value15);
                                                                                        let rds = chinese_digit_1000_compat(value8, value9, value10, value11, value12, value13, value14).map_err(|err| err + 7)?;
                                                                                        Ok(msd as u32 * 10000 + rds as u32)
                                                                                    }
                                                                                } else {
                                                                                    Ok(msd as u32 * 10000)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            match value8 {
                                                                                Some(value8) => {
                                                                                    if CHINESE_NUMBERS_CHARS[13].contains(&value8) {
                                                                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7))?;

                                                                                        if msd == 0 {
                                                                                            Err(0)
                                                                                        } else {
                                                                                            if let Some(value9) = value9 {
                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value9) {
                                                                                                    if let Some(value10) = value10 {
                                                                                                        debug_assert_eq!(None, value15);
                                                                                                        let rds = chinese_digit_100_compat(value10, value11, value12, value13, value14).map_err(|err| err + 9)?;
                                                                                                        Ok(msd as u32 * 10000 + rds as u32)
                                                                                                    } else {
                                                                                                        Err(9)
                                                                                                    }
                                                                                                } else {
                                                                                                    let rds = chinese_digit_1000_compat(value9, value10, value11, value12, value13, value14, value15).map_err(|err| err + 8)?;
                                                                                                    Ok(msd as u32 * 10000 + rds as u32)
                                                                                                }
                                                                                            } else {
                                                                                                Ok(msd as u32 * 10000)
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        Err(7)
                                                                                    }
                                                                                }
                                                                                None => {
                                                                                    debug_assert_eq!(None, value9);
                                                                                    debug_assert_eq!(None, value10);
                                                                                    debug_assert_eq!(None, value11);
                                                                                    debug_assert_eq!(None, value12);
                                                                                    debug_assert_eq!(None, value13);
                                                                                    debug_assert_eq!(None, value14);
                                                                                    debug_assert_eq!(None, value15);
                                                                                    let n = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7))?;
                                                                                    Ok(n as u32)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug_assert_eq!(None, value8);
                                                                        debug_assert_eq!(None, value9);
                                                                        debug_assert_eq!(None, value10);
                                                                        debug_assert_eq!(None, value11);
                                                                        debug_assert_eq!(None, value12);
                                                                        debug_assert_eq!(None, value13);
                                                                        debug_assert_eq!(None, value14);
                                                                        debug_assert_eq!(None, value15);
                                                                        let n = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None)?;
                                                                        Ok(n as u32)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            debug_assert_eq!(None, value7);
                                                            debug_assert_eq!(None, value8);
                                                            debug_assert_eq!(None, value9);
                                                            debug_assert_eq!(None, value10);
                                                            debug_assert_eq!(None, value11);
                                                            debug_assert_eq!(None, value12);
                                                            debug_assert_eq!(None, value13);
                                                            debug_assert_eq!(None, value14);
                                                            debug_assert_eq!(None, value15);
                                                            let n = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None)?;
                                                            Ok(n as u32)
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                debug_assert_eq!(None, value6);
                                                debug_assert_eq!(None, value7);
                                                debug_assert_eq!(None, value8);
                                                debug_assert_eq!(None, value9);
                                                debug_assert_eq!(None, value10);
                                                debug_assert_eq!(None, value11);
                                                debug_assert_eq!(None, value12);
                                                debug_assert_eq!(None, value13);
                                                debug_assert_eq!(None, value14);
                                                debug_assert_eq!(None, value15);
                                                let n = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), None, None, None)?;
                                                Ok(n as u32)
                                            }
                                        }
                                    }
                                }
                                None => {
                                    debug_assert_eq!(None, value5);
                                    debug_assert_eq!(None, value6);
                                    debug_assert_eq!(None, value7);
                                    debug_assert_eq!(None, value8);
                                    debug_assert_eq!(None, value9);
                                    debug_assert_eq!(None, value10);
                                    debug_assert_eq!(None, value11);
                                    debug_assert_eq!(None, value12);
                                    debug_assert_eq!(None, value13);
                                    debug_assert_eq!(None, value14);
                                    debug_assert_eq!(None, value15);
                                    let n = chinese_digit_1000_compat(value, Some(value2), Some(value3), None, None, None, None)?;
                                    Ok(n as u32)
                                }
                            }
                        }
                    }
                    None => {
                        debug_assert_eq!(None, value4);
                        debug_assert_eq!(None, value5);
                        debug_assert_eq!(None, value6);
                        debug_assert_eq!(None, value7);
                        debug_assert_eq!(None, value8);
                        debug_assert_eq!(None, value9);
                        debug_assert_eq!(None, value10);
                        debug_assert_eq!(None, value11);
                        debug_assert_eq!(None, value12);
                        debug_assert_eq!(None, value13);
                        debug_assert_eq!(None, value14);
                        debug_assert_eq!(None, value15);
                        let n = chinese_digit_1000_compat(value, Some(value2), None, None, None, None, None)?;
                        Ok(n as u32)
                    }
                }
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u32)
        }
    }
}

pub(crate) fn chinese_digit_100000000_ten_thousand_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[14].contains(&value2) {
                let msd = chinese_digit_10_compat(value, None, None)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value19);
                                debug_assert_eq!(None, value20);
                                debug_assert_eq!(None, value21);
                                debug_assert_eq!(None, value22);
                                debug_assert_eq!(None, value23);
                                let rds = chinese_digit_10000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18).map_err(|err| err + 3)?;

                                Ok(msd as u64 * 100000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            debug_assert_eq!(None, value18);
                            debug_assert_eq!(None, value19);
                            debug_assert_eq!(None, value20);
                            debug_assert_eq!(None, value21);
                            debug_assert_eq!(None, value22);
                            debug_assert_eq!(None, value23);
                            let rds = chinese_digit_10000_ten_thousand_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17).map_err(|err| err + 2)?;

                            Ok(msd as u64 * 100000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 100000000)
                    }
                }
            } else {
                match value3 {
                    Some(value3) => {
                        if CHINESE_NUMBERS_CHARS[14].contains(&value3) {
                            let msd = chinese_digit_1000_compat(value, Some(value2), None, None, None, None, None)?;

                            if msd == 0 {
                                Err(0)
                            } else {
                                if let Some(value4) = value4 {
                                    if CHINESE_NUMBERS_CHARS[0].contains(&value4) {
                                        if let Some(value5) = value5 {
                                            debug_assert_eq!(None, value20);
                                            debug_assert_eq!(None, value21);
                                            debug_assert_eq!(None, value22);
                                            debug_assert_eq!(None, value23);
                                            let rds = chinese_digit_10000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19).map_err(|err| err + 4)?;

                                            Ok(msd as u64 * 100000000 + rds as u64)
                                        } else {
                                            Err(4)
                                        }
                                    } else {
                                        debug_assert_eq!(None, value19);
                                        debug_assert_eq!(None, value20);
                                        debug_assert_eq!(None, value21);
                                        debug_assert_eq!(None, value22);
                                        debug_assert_eq!(None, value23);
                                        let rds = chinese_digit_10000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18).map_err(|err| err + 3)?;

                                        Ok(msd as u64 * 100000000 + rds as u64)
                                    }
                                } else {
                                    Ok(msd as u64 * 100000000)
                                }
                            }
                        } else {
                            match value4 {
                                Some(value4) => {
                                    if CHINESE_NUMBERS_CHARS[14].contains(&value4) {
                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), None, None, None, None)?;

                                        if msd == 0 {
                                            Err(0)
                                        } else {
                                            if let Some(value5) = value5 {
                                                if CHINESE_NUMBERS_CHARS[0].contains(&value5) {
                                                    if let Some(value6) = value6 {
                                                        debug_assert_eq!(None, value21);
                                                        debug_assert_eq!(None, value22);
                                                        debug_assert_eq!(None, value23);
                                                        let rds = chinese_digit_10000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20).map_err(|err| err + 5)?;

                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                    } else {
                                                        Err(5)
                                                    }
                                                } else {
                                                    debug_assert_eq!(None, value20);
                                                    debug_assert_eq!(None, value21);
                                                    debug_assert_eq!(None, value22);
                                                    debug_assert_eq!(None, value23);
                                                    let rds = chinese_digit_10000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19).map_err(|err| err + 4)?;

                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                }
                                            } else {
                                                Ok(msd as u64 * 100000000)
                                            }
                                        }
                                    } else {
                                        match value5 {
                                            Some(value5) => {
                                                if CHINESE_NUMBERS_CHARS[14].contains(&value5) {
                                                    let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), None, None, None)?;

                                                    if msd == 0 {
                                                        Err(0)
                                                    } else {
                                                        if let Some(value6) = value6 {
                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value6) {
                                                                if let Some(value7) = value7 {
                                                                    debug_assert_eq!(None, value22);
                                                                    debug_assert_eq!(None, value23);
                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21).map_err(|err| err + 6)?;

                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                } else {
                                                                    Err(6)
                                                                }
                                                            } else {
                                                                debug_assert_eq!(None, value21);
                                                                debug_assert_eq!(None, value22);
                                                                debug_assert_eq!(None, value23);
                                                                let rds = chinese_digit_10000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20).map_err(|err| err + 5)?;

                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                            }
                                                        } else {
                                                            Ok(msd as u64 * 100000000)
                                                        }
                                                    }
                                                } else {
                                                    match value6 {
                                                        Some(value6) => {
                                                            if CHINESE_NUMBERS_CHARS[14].contains(&value6) {
                                                                let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None)?;

                                                                if msd == 0 {
                                                                    Err(0)
                                                                } else {
                                                                    if let Some(value7) = value7 {
                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value7) {
                                                                            if let Some(value8) = value8 {
                                                                                debug_assert_eq!(None, value23);
                                                                                let rds = chinese_digit_10000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22).map_err(|err| err + 7)?;

                                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                                            } else {
                                                                                Err(7)
                                                                            }
                                                                        } else {
                                                                            debug_assert_eq!(None, value22);
                                                                            debug_assert_eq!(None, value23);
                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21).map_err(|err| err + 6)?;

                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                        }
                                                                    } else {
                                                                        Ok(msd as u64 * 100000000)
                                                                    }
                                                                }
                                                            } else {
                                                                match value7 {
                                                                    Some(value7) => {
                                                                        if CHINESE_NUMBERS_CHARS[14].contains(&value7) {
                                                                            let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None)?;

                                                                            if msd == 0 {
                                                                                Err(0)
                                                                            } else {
                                                                                if let Some(value8) = value8 {
                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value8) {
                                                                                        if let Some(value9) = value9 {
                                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23).map_err(|err| err + 8)?;

                                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                                        } else {
                                                                                            Err(8)
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value23);
                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22).map_err(|err| err + 7)?;

                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                    }
                                                                                } else {
                                                                                    Ok(msd as u64 * 100000000)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            match value8 {
                                                                                Some(value8) => {
                                                                                    if CHINESE_NUMBERS_CHARS[14].contains(&value8) {
                                                                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7))?;

                                                                                        if msd == 0 {
                                                                                            Err(0)
                                                                                        } else {
                                                                                            if let Some(value9) = value9 {
                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value9) {
                                                                                                    if let Some(value10) = value10 {
                                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, None).map_err(|err| err + 9)?;

                                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                    } else {
                                                                                                        Err(9)
                                                                                                    }
                                                                                                } else {
                                                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23).map_err(|err| err + 8)?;

                                                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                }
                                                                                            } else {
                                                                                                Ok(msd as u64 * 100000000)
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value16);
                                                                                        debug_assert_eq!(None, value17);
                                                                                        debug_assert_eq!(None, value18);
                                                                                        debug_assert_eq!(None, value19);
                                                                                        debug_assert_eq!(None, value20);
                                                                                        debug_assert_eq!(None, value21);
                                                                                        debug_assert_eq!(None, value22);
                                                                                        debug_assert_eq!(None, value23);
                                                                                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), value9, value10, value11, value12, value13, value14, value15)?;
                                                                                        Ok(n as u64)
                                                                                    }
                                                                                }
                                                                                None => {
                                                                                    debug_assert_eq!(None, value9);
                                                                                    debug_assert_eq!(None, value10);
                                                                                    debug_assert_eq!(None, value11);
                                                                                    debug_assert_eq!(None, value12);
                                                                                    debug_assert_eq!(None, value13);
                                                                                    debug_assert_eq!(None, value14);
                                                                                    debug_assert_eq!(None, value15);
                                                                                    debug_assert_eq!(None, value16);
                                                                                    debug_assert_eq!(None, value17);
                                                                                    debug_assert_eq!(None, value18);
                                                                                    debug_assert_eq!(None, value19);
                                                                                    debug_assert_eq!(None, value20);
                                                                                    debug_assert_eq!(None, value21);
                                                                                    debug_assert_eq!(None, value22);
                                                                                    debug_assert_eq!(None, value23);
                                                                                    let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None)?;
                                                                                    Ok(n as u64)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug_assert_eq!(None, value8);
                                                                        debug_assert_eq!(None, value9);
                                                                        debug_assert_eq!(None, value10);
                                                                        debug_assert_eq!(None, value11);
                                                                        debug_assert_eq!(None, value12);
                                                                        debug_assert_eq!(None, value13);
                                                                        debug_assert_eq!(None, value14);
                                                                        debug_assert_eq!(None, value15);
                                                                        debug_assert_eq!(None, value16);
                                                                        debug_assert_eq!(None, value17);
                                                                        debug_assert_eq!(None, value18);
                                                                        debug_assert_eq!(None, value19);
                                                                        debug_assert_eq!(None, value20);
                                                                        debug_assert_eq!(None, value21);
                                                                        debug_assert_eq!(None, value22);
                                                                        debug_assert_eq!(None, value23);
                                                                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None)?;
                                                                        Ok(n as u64)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            debug_assert_eq!(None, value7);
                                                            debug_assert_eq!(None, value8);
                                                            debug_assert_eq!(None, value9);
                                                            debug_assert_eq!(None, value10);
                                                            debug_assert_eq!(None, value11);
                                                            debug_assert_eq!(None, value12);
                                                            debug_assert_eq!(None, value13);
                                                            debug_assert_eq!(None, value14);
                                                            debug_assert_eq!(None, value15);
                                                            debug_assert_eq!(None, value16);
                                                            debug_assert_eq!(None, value17);
                                                            debug_assert_eq!(None, value18);
                                                            debug_assert_eq!(None, value19);
                                                            debug_assert_eq!(None, value20);
                                                            debug_assert_eq!(None, value21);
                                                            debug_assert_eq!(None, value22);
                                                            debug_assert_eq!(None, value23);
                                                            let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None)?;
                                                            Ok(n as u64)
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                debug_assert_eq!(None, value6);
                                                debug_assert_eq!(None, value7);
                                                debug_assert_eq!(None, value8);
                                                debug_assert_eq!(None, value9);
                                                debug_assert_eq!(None, value10);
                                                debug_assert_eq!(None, value11);
                                                debug_assert_eq!(None, value12);
                                                debug_assert_eq!(None, value13);
                                                debug_assert_eq!(None, value14);
                                                debug_assert_eq!(None, value15);
                                                debug_assert_eq!(None, value16);
                                                debug_assert_eq!(None, value17);
                                                debug_assert_eq!(None, value18);
                                                debug_assert_eq!(None, value19);
                                                debug_assert_eq!(None, value20);
                                                debug_assert_eq!(None, value21);
                                                debug_assert_eq!(None, value22);
                                                debug_assert_eq!(None, value23);
                                                let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), None, None, None, None, None, None, None, None, None, None, None)?;
                                                Ok(n as u64)
                                            }
                                        }
                                    }
                                }
                                None => {
                                    debug_assert_eq!(None, value5);
                                    debug_assert_eq!(None, value6);
                                    debug_assert_eq!(None, value7);
                                    debug_assert_eq!(None, value8);
                                    debug_assert_eq!(None, value9);
                                    debug_assert_eq!(None, value10);
                                    debug_assert_eq!(None, value11);
                                    debug_assert_eq!(None, value12);
                                    debug_assert_eq!(None, value13);
                                    debug_assert_eq!(None, value14);
                                    debug_assert_eq!(None, value15);
                                    debug_assert_eq!(None, value16);
                                    debug_assert_eq!(None, value17);
                                    debug_assert_eq!(None, value18);
                                    debug_assert_eq!(None, value19);
                                    debug_assert_eq!(None, value20);
                                    debug_assert_eq!(None, value21);
                                    debug_assert_eq!(None, value22);
                                    debug_assert_eq!(None, value23);
                                    let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), None, None, None, None, None, None, None, None, None, None, None, None)?;
                                    Ok(n as u64)
                                }
                            }
                        }
                    }
                    None => {
                        debug_assert_eq!(None, value4);
                        debug_assert_eq!(None, value5);
                        debug_assert_eq!(None, value6);
                        debug_assert_eq!(None, value7);
                        debug_assert_eq!(None, value8);
                        debug_assert_eq!(None, value9);
                        debug_assert_eq!(None, value10);
                        debug_assert_eq!(None, value11);
                        debug_assert_eq!(None, value12);
                        debug_assert_eq!(None, value13);
                        debug_assert_eq!(None, value14);
                        debug_assert_eq!(None, value15);
                        debug_assert_eq!(None, value16);
                        debug_assert_eq!(None, value17);
                        debug_assert_eq!(None, value18);
                        debug_assert_eq!(None, value19);
                        debug_assert_eq!(None, value20);
                        debug_assert_eq!(None, value21);
                        debug_assert_eq!(None, value22);
                        debug_assert_eq!(None, value23);
                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                        Ok(n as u64)
                    }
                }
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_1000000000000_ten_thousand_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>, value26: Option<char>, value27: Option<char>, value28: Option<char>, value29: Option<char>, value30: Option<char>, value31: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[15].contains(&value2) {
                let msd = chinese_digit_10_compat(value, None, None)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value27);
                                debug_assert_eq!(None, value28);
                                debug_assert_eq!(None, value29);
                                debug_assert_eq!(None, value30);
                                debug_assert_eq!(None, value31);
                                let rds = chinese_digit_100000000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26).map_err(|err| err + 3)?;

                                Ok(msd as u64 * 1000000000000 + rds)
                            } else {
                                Err(3)
                            }
                        } else {
                            debug_assert_eq!(None, value26);
                            debug_assert_eq!(None, value27);
                            debug_assert_eq!(None, value28);
                            debug_assert_eq!(None, value29);
                            debug_assert_eq!(None, value30);
                            debug_assert_eq!(None, value31);
                            let rds = chinese_digit_100000000_ten_thousand_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25).map_err(|err| err + 2)?;

                            Ok(msd as u64 * 1000000000000 + rds)
                        }
                    } else {
                        Ok(msd as u64 * 1000000000000)
                    }
                }
            } else {
                match value3 {
                    Some(value3) => {
                        if CHINESE_NUMBERS_CHARS[15].contains(&value3) {
                            let msd = chinese_digit_1000_compat(value, Some(value2), None, None, None, None, None)?;

                            if msd == 0 {
                                Err(0)
                            } else {
                                if let Some(value4) = value4 {
                                    if CHINESE_NUMBERS_CHARS[0].contains(&value4) {
                                        if let Some(value5) = value5 {
                                            debug_assert_eq!(None, value28);
                                            debug_assert_eq!(None, value29);
                                            debug_assert_eq!(None, value30);
                                            debug_assert_eq!(None, value31);
                                            let rds = chinese_digit_100000000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27).map_err(|err| err + 4)?;

                                            Ok(msd as u64 * 1000000000000 + rds)
                                        } else {
                                            Err(4)
                                        }
                                    } else {
                                        debug_assert_eq!(None, value27);
                                        debug_assert_eq!(None, value28);
                                        debug_assert_eq!(None, value29);
                                        debug_assert_eq!(None, value30);
                                        debug_assert_eq!(None, value31);
                                        let rds = chinese_digit_100000000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26).map_err(|err| err + 3)?;

                                        Ok(msd as u64 * 1000000000000 + rds)
                                    }
                                } else {
                                    Ok(msd as u64 * 1000000000000)
                                }
                            }
                        } else {
                            match value4 {
                                Some(value4) => {
                                    if CHINESE_NUMBERS_CHARS[15].contains(&value4) {
                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), None, None, None, None)?;

                                        if msd == 0 {
                                            Err(0)
                                        } else {
                                            if let Some(value5) = value5 {
                                                if CHINESE_NUMBERS_CHARS[0].contains(&value5) {
                                                    if let Some(value6) = value6 {
                                                        debug_assert_eq!(None, value29);
                                                        debug_assert_eq!(None, value30);
                                                        debug_assert_eq!(None, value31);
                                                        let rds = chinese_digit_100000000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28).map_err(|err| err + 5)?;

                                                        Ok(msd as u64 * 1000000000000 + rds)
                                                    } else {
                                                        Err(5)
                                                    }
                                                } else {
                                                    debug_assert_eq!(None, value28);
                                                    debug_assert_eq!(None, value29);
                                                    debug_assert_eq!(None, value30);
                                                    debug_assert_eq!(None, value31);
                                                    let rds = chinese_digit_100000000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27).map_err(|err| err + 4)?;

                                                    Ok(msd as u64 * 1000000000000 + rds)
                                                }
                                            } else {
                                                Ok(msd as u64 * 1000000000000)
                                            }
                                        }
                                    } else {
                                        match value5 {
                                            Some(value5) => {
                                                if CHINESE_NUMBERS_CHARS[15].contains(&value5) {
                                                    let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), None, None, None)?;

                                                    if msd == 0 {
                                                        Err(0)
                                                    } else {
                                                        if let Some(value6) = value6 {
                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value6) {
                                                                if let Some(value7) = value7 {
                                                                    debug_assert_eq!(None, value30);
                                                                    debug_assert_eq!(None, value31);
                                                                    let rds = chinese_digit_100000000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29).map_err(|err| err + 6)?;

                                                                    Ok(msd as u64 * 1000000000000 + rds)
                                                                } else {
                                                                    Err(6)
                                                                }
                                                            } else {
                                                                debug_assert_eq!(None, value29);
                                                                debug_assert_eq!(None, value30);
                                                                debug_assert_eq!(None, value31);
                                                                let rds = chinese_digit_100000000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28).map_err(|err| err + 5)?;

                                                                Ok(msd as u64 * 1000000000000 + rds)
                                                            }
                                                        } else {
                                                            Ok(msd as u64 * 1000000000000)
                                                        }
                                                    }
                                                } else {
                                                    match value6 {
                                                        Some(value6) => {
                                                            if CHINESE_NUMBERS_CHARS[15].contains(&value6) {
                                                                let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None)?;

                                                                if msd == 0 {
                                                                    Err(0)
                                                                } else {
                                                                    if let Some(value7) = value7 {
                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value7) {
                                                                            if let Some(value8) = value8 {
                                                                                debug_assert_eq!(None, value31);
                                                                                let rds = chinese_digit_100000000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30).map_err(|err| err + 7)?;

                                                                                Ok(msd as u64 * 1000000000000 + rds)
                                                                            } else {
                                                                                Err(7)
                                                                            }
                                                                        } else {
                                                                            debug_assert_eq!(None, value30);
                                                                            debug_assert_eq!(None, value31);
                                                                            let rds = chinese_digit_100000000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29).map_err(|err| err + 6)?;

                                                                            Ok(msd as u64 * 1000000000000 + rds)
                                                                        }
                                                                    } else {
                                                                        Ok(msd as u64 * 1000000000000)
                                                                    }
                                                                }
                                                            } else {
                                                                match value7 {
                                                                    Some(value7) => {
                                                                        if CHINESE_NUMBERS_CHARS[15].contains(&value7) {
                                                                            let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None)?;

                                                                            if msd == 0 {
                                                                                Err(0)
                                                                            } else {
                                                                                if let Some(value8) = value8 {
                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value8) {
                                                                                        if let Some(value9) = value9 {
                                                                                            let rds = chinese_digit_100000000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31).map_err(|err| err + 8)?;

                                                                                            Ok(msd as u64 * 1000000000000 + rds)
                                                                                        } else {
                                                                                            Err(8)
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value31);
                                                                                        let rds = chinese_digit_100000000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30).map_err(|err| err + 7)?;

                                                                                        Ok(msd as u64 * 1000000000000 + rds)
                                                                                    }
                                                                                } else {
                                                                                    Ok(msd as u64 * 1000000000000)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            match value8 {
                                                                                Some(value8) => {
                                                                                    if CHINESE_NUMBERS_CHARS[15].contains(&value8) {
                                                                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7))?;

                                                                                        if msd == 0 {
                                                                                            Err(0)
                                                                                        } else {
                                                                                            if let Some(value9) = value9 {
                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value9) {
                                                                                                    if let Some(value10) = value10 {
                                                                                                        let rds = chinese_digit_100000000_ten_thousand_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, None).map_err(|err| err + 9)?;

                                                                                                        Ok(msd as u64 * 1000000000000 + rds)
                                                                                                    } else {
                                                                                                        Err(9)
                                                                                                    }
                                                                                                } else {
                                                                                                    let rds = chinese_digit_100000000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31).map_err(|err| err + 8)?;

                                                                                                    Ok(msd as u64 * 1000000000000 + rds)
                                                                                                }
                                                                                            } else {
                                                                                                Ok(msd as u64 * 1000000000000)
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value16);
                                                                                        debug_assert_eq!(None, value17);
                                                                                        debug_assert_eq!(None, value18);
                                                                                        debug_assert_eq!(None, value19);
                                                                                        debug_assert_eq!(None, value20);
                                                                                        debug_assert_eq!(None, value21);
                                                                                        debug_assert_eq!(None, value22);
                                                                                        debug_assert_eq!(None, value23);
                                                                                        debug_assert_eq!(None, value24);
                                                                                        debug_assert_eq!(None, value25);
                                                                                        debug_assert_eq!(None, value26);
                                                                                        debug_assert_eq!(None, value27);
                                                                                        debug_assert_eq!(None, value28);
                                                                                        debug_assert_eq!(None, value29);
                                                                                        debug_assert_eq!(None, value30);
                                                                                        debug_assert_eq!(None, value31);
                                                                                        let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23)?;
                                                                                        Ok(n)
                                                                                    }
                                                                                }
                                                                                None => {
                                                                                    debug_assert_eq!(None, value9);
                                                                                    debug_assert_eq!(None, value10);
                                                                                    debug_assert_eq!(None, value11);
                                                                                    debug_assert_eq!(None, value12);
                                                                                    debug_assert_eq!(None, value13);
                                                                                    debug_assert_eq!(None, value14);
                                                                                    debug_assert_eq!(None, value15);
                                                                                    debug_assert_eq!(None, value16);
                                                                                    debug_assert_eq!(None, value17);
                                                                                    debug_assert_eq!(None, value18);
                                                                                    debug_assert_eq!(None, value19);
                                                                                    debug_assert_eq!(None, value20);
                                                                                    debug_assert_eq!(None, value21);
                                                                                    debug_assert_eq!(None, value22);
                                                                                    debug_assert_eq!(None, value23);
                                                                                    debug_assert_eq!(None, value24);
                                                                                    debug_assert_eq!(None, value25);
                                                                                    debug_assert_eq!(None, value26);
                                                                                    debug_assert_eq!(None, value27);
                                                                                    debug_assert_eq!(None, value28);
                                                                                    debug_assert_eq!(None, value29);
                                                                                    debug_assert_eq!(None, value30);
                                                                                    debug_assert_eq!(None, value31);
                                                                                    let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                    Ok(n)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug_assert_eq!(None, value8);
                                                                        debug_assert_eq!(None, value9);
                                                                        debug_assert_eq!(None, value10);
                                                                        debug_assert_eq!(None, value11);
                                                                        debug_assert_eq!(None, value12);
                                                                        debug_assert_eq!(None, value13);
                                                                        debug_assert_eq!(None, value14);
                                                                        debug_assert_eq!(None, value15);
                                                                        debug_assert_eq!(None, value16);
                                                                        debug_assert_eq!(None, value17);
                                                                        debug_assert_eq!(None, value18);
                                                                        debug_assert_eq!(None, value19);
                                                                        debug_assert_eq!(None, value20);
                                                                        debug_assert_eq!(None, value21);
                                                                        debug_assert_eq!(None, value22);
                                                                        debug_assert_eq!(None, value23);
                                                                        debug_assert_eq!(None, value24);
                                                                        debug_assert_eq!(None, value25);
                                                                        debug_assert_eq!(None, value26);
                                                                        debug_assert_eq!(None, value27);
                                                                        debug_assert_eq!(None, value28);
                                                                        debug_assert_eq!(None, value29);
                                                                        debug_assert_eq!(None, value30);
                                                                        debug_assert_eq!(None, value31);
                                                                        let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                        Ok(n)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            debug_assert_eq!(None, value7);
                                                            debug_assert_eq!(None, value8);
                                                            debug_assert_eq!(None, value9);
                                                            debug_assert_eq!(None, value10);
                                                            debug_assert_eq!(None, value11);
                                                            debug_assert_eq!(None, value12);
                                                            debug_assert_eq!(None, value13);
                                                            debug_assert_eq!(None, value14);
                                                            debug_assert_eq!(None, value15);
                                                            debug_assert_eq!(None, value16);
                                                            debug_assert_eq!(None, value17);
                                                            debug_assert_eq!(None, value18);
                                                            debug_assert_eq!(None, value19);
                                                            debug_assert_eq!(None, value20);
                                                            debug_assert_eq!(None, value21);
                                                            debug_assert_eq!(None, value22);
                                                            debug_assert_eq!(None, value23);
                                                            debug_assert_eq!(None, value24);
                                                            debug_assert_eq!(None, value25);
                                                            debug_assert_eq!(None, value26);
                                                            debug_assert_eq!(None, value27);
                                                            debug_assert_eq!(None, value28);
                                                            debug_assert_eq!(None, value29);
                                                            debug_assert_eq!(None, value30);
                                                            debug_assert_eq!(None, value31);
                                                            let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                            Ok(n)
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                debug_assert_eq!(None, value6);
                                                debug_assert_eq!(None, value7);
                                                debug_assert_eq!(None, value8);
                                                debug_assert_eq!(None, value9);
                                                debug_assert_eq!(None, value10);
                                                debug_assert_eq!(None, value11);
                                                debug_assert_eq!(None, value12);
                                                debug_assert_eq!(None, value13);
                                                debug_assert_eq!(None, value14);
                                                debug_assert_eq!(None, value15);
                                                debug_assert_eq!(None, value16);
                                                debug_assert_eq!(None, value17);
                                                debug_assert_eq!(None, value18);
                                                debug_assert_eq!(None, value19);
                                                debug_assert_eq!(None, value20);
                                                debug_assert_eq!(None, value21);
                                                debug_assert_eq!(None, value22);
                                                debug_assert_eq!(None, value23);
                                                debug_assert_eq!(None, value24);
                                                debug_assert_eq!(None, value25);
                                                debug_assert_eq!(None, value26);
                                                debug_assert_eq!(None, value27);
                                                debug_assert_eq!(None, value28);
                                                debug_assert_eq!(None, value29);
                                                debug_assert_eq!(None, value30);
                                                debug_assert_eq!(None, value31);
                                                let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                Ok(n)
                                            }
                                        }
                                    }
                                }
                                None => {
                                    debug_assert_eq!(None, value5);
                                    debug_assert_eq!(None, value6);
                                    debug_assert_eq!(None, value7);
                                    debug_assert_eq!(None, value8);
                                    debug_assert_eq!(None, value9);
                                    debug_assert_eq!(None, value10);
                                    debug_assert_eq!(None, value11);
                                    debug_assert_eq!(None, value12);
                                    debug_assert_eq!(None, value13);
                                    debug_assert_eq!(None, value14);
                                    debug_assert_eq!(None, value15);
                                    debug_assert_eq!(None, value16);
                                    debug_assert_eq!(None, value17);
                                    debug_assert_eq!(None, value18);
                                    debug_assert_eq!(None, value19);
                                    debug_assert_eq!(None, value20);
                                    debug_assert_eq!(None, value21);
                                    debug_assert_eq!(None, value22);
                                    debug_assert_eq!(None, value23);
                                    debug_assert_eq!(None, value24);
                                    debug_assert_eq!(None, value25);
                                    debug_assert_eq!(None, value26);
                                    debug_assert_eq!(None, value27);
                                    debug_assert_eq!(None, value28);
                                    debug_assert_eq!(None, value29);
                                    debug_assert_eq!(None, value30);
                                    debug_assert_eq!(None, value31);
                                    let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), Some(value3), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                    Ok(n)
                                }
                            }
                        }
                    }
                    None => {
                        debug_assert_eq!(None, value4);
                        debug_assert_eq!(None, value5);
                        debug_assert_eq!(None, value6);
                        debug_assert_eq!(None, value7);
                        debug_assert_eq!(None, value8);
                        debug_assert_eq!(None, value9);
                        debug_assert_eq!(None, value10);
                        debug_assert_eq!(None, value11);
                        debug_assert_eq!(None, value12);
                        debug_assert_eq!(None, value13);
                        debug_assert_eq!(None, value14);
                        debug_assert_eq!(None, value15);
                        debug_assert_eq!(None, value16);
                        debug_assert_eq!(None, value17);
                        debug_assert_eq!(None, value18);
                        debug_assert_eq!(None, value19);
                        debug_assert_eq!(None, value20);
                        debug_assert_eq!(None, value21);
                        debug_assert_eq!(None, value22);
                        debug_assert_eq!(None, value23);
                        debug_assert_eq!(None, value24);
                        debug_assert_eq!(None, value25);
                        debug_assert_eq!(None, value26);
                        debug_assert_eq!(None, value27);
                        debug_assert_eq!(None, value28);
                        debug_assert_eq!(None, value29);
                        debug_assert_eq!(None, value30);
                        debug_assert_eq!(None, value31);
                        let n = chinese_digit_100000000_ten_thousand_compat(value, Some(value2), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                        Ok(n)
                    }
                }
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            debug_assert_eq!(None, value26);
            debug_assert_eq!(None, value27);
            debug_assert_eq!(None, value28);
            debug_assert_eq!(None, value29);
            debug_assert_eq!(None, value30);
            debug_assert_eq!(None, value31);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

pub(crate) fn chinese_digit_10000000000000000_ten_thousand_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>, value26: Option<char>, value27: Option<char>, value28: Option<char>, value29: Option<char>, value30: Option<char>, value31: Option<char>, value32: Option<char>, value33: Option<char>, value34: Option<char>, value35: Option<char>, value36: Option<char>, value37: Option<char>, value38: Option<char>, value39: Option<char>) -> Result<u128, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[16].contains(&value2) {
                let msd = chinese_digit_10_compat(value, None, None)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value35);
                                debug_assert_eq!(None, value36);
                                debug_assert_eq!(None, value37);
                                debug_assert_eq!(None, value38);
                                debug_assert_eq!(None, value39);
                                let rds = chinese_digit_1000000000000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34).map_err(|err| err + 3)?;

                                Ok(msd as u128 * 10000000000000000 + rds as u128)
                            } else {
                                Err(3)
                            }
                        } else {
                            debug_assert_eq!(None, value34);
                            debug_assert_eq!(None, value35);
                            debug_assert_eq!(None, value36);
                            debug_assert_eq!(None, value37);
                            debug_assert_eq!(None, value38);
                            debug_assert_eq!(None, value39);
                            let rds = chinese_digit_1000000000000_ten_thousand_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33).map_err(|err| err + 2)?;

                            Ok(msd as u128 * 10000000000000000 + rds as u128)
                        }
                    } else {
                        Ok(msd as u128 * 10000000000000000)
                    }
                }
            } else {
                match value3 {
                    Some(value3) => {
                        if CHINESE_NUMBERS_CHARS[16].contains(&value3) {
                            let msd = chinese_digit_1000_compat(value, Some(value2), None, None, None, None, None)?;

                            if msd == 0 {
                                Err(0)
                            } else {
                                if let Some(value4) = value4 {
                                    if CHINESE_NUMBERS_CHARS[0].contains(&value4) {
                                        if let Some(value5) = value5 {
                                            debug_assert_eq!(None, value36);
                                            debug_assert_eq!(None, value37);
                                            debug_assert_eq!(None, value38);
                                            debug_assert_eq!(None, value39);
                                            let rds = chinese_digit_1000000000000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35).map_err(|err| err + 4)?;

                                            Ok(msd as u128 * 10000000000000000 + rds as u128)
                                        } else {
                                            Err(4)
                                        }
                                    } else {
                                        debug_assert_eq!(None, value35);
                                        debug_assert_eq!(None, value36);
                                        debug_assert_eq!(None, value37);
                                        debug_assert_eq!(None, value38);
                                        debug_assert_eq!(None, value39);
                                        let rds = chinese_digit_1000000000000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34).map_err(|err| err + 3)?;

                                        Ok(msd as u128 * 10000000000000000 + rds as u128)
                                    }
                                } else {
                                    Ok(msd as u128 * 10000000000000000)
                                }
                            }
                        } else {
                            match value4 {
                                Some(value4) => {
                                    if CHINESE_NUMBERS_CHARS[16].contains(&value4) {
                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), None, None, None, None)?;

                                        if msd == 0 {
                                            Err(0)
                                        } else {
                                            if let Some(value5) = value5 {
                                                if CHINESE_NUMBERS_CHARS[0].contains(&value5) {
                                                    if let Some(value6) = value6 {
                                                        debug_assert_eq!(None, value37);
                                                        debug_assert_eq!(None, value38);
                                                        debug_assert_eq!(None, value39);
                                                        let rds = chinese_digit_1000000000000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36).map_err(|err| err + 5)?;

                                                        Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                    } else {
                                                        Err(5)
                                                    }
                                                } else {
                                                    debug_assert_eq!(None, value36);
                                                    debug_assert_eq!(None, value37);
                                                    debug_assert_eq!(None, value38);
                                                    debug_assert_eq!(None, value39);
                                                    let rds = chinese_digit_1000000000000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35).map_err(|err| err + 4)?;

                                                    Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                }
                                            } else {
                                                Ok(msd as u128 * 10000000000000000)
                                            }
                                        }
                                    } else {
                                        match value5 {
                                            Some(value5) => {
                                                if CHINESE_NUMBERS_CHARS[16].contains(&value5) {
                                                    let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), None, None, None)?;

                                                    if msd == 0 {
                                                        Err(0)
                                                    } else {
                                                        if let Some(value6) = value6 {
                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value6) {
                                                                if let Some(value7) = value7 {
                                                                    debug_assert_eq!(None, value38);
                                                                    debug_assert_eq!(None, value39);
                                                                    let rds = chinese_digit_1000000000000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37).map_err(|err| err + 6)?;

                                                                    Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                } else {
                                                                    Err(6)
                                                                }
                                                            } else {
                                                                debug_assert_eq!(None, value37);
                                                                debug_assert_eq!(None, value38);
                                                                debug_assert_eq!(None, value39);
                                                                let rds = chinese_digit_1000000000000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36).map_err(|err| err + 5)?;

                                                                Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                            }
                                                        } else {
                                                            Ok(msd as u128 * 10000000000000000)
                                                        }
                                                    }
                                                } else {
                                                    match value6 {
                                                        Some(value6) => {
                                                            if CHINESE_NUMBERS_CHARS[16].contains(&value6) {
                                                                let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None)?;

                                                                if msd == 0 {
                                                                    Err(0)
                                                                } else {
                                                                    if let Some(value7) = value7 {
                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value7) {
                                                                            if let Some(value8) = value8 {
                                                                                debug_assert_eq!(None, value39);
                                                                                let rds = chinese_digit_1000000000000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38).map_err(|err| err + 7)?;

                                                                                Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                            } else {
                                                                                Err(7)
                                                                            }
                                                                        } else {
                                                                            debug_assert_eq!(None, value38);
                                                                            debug_assert_eq!(None, value39);
                                                                            let rds = chinese_digit_1000000000000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37).map_err(|err| err + 6)?;

                                                                            Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                        }
                                                                    } else {
                                                                        Ok(msd as u128 * 1000000000000)
                                                                    }
                                                                }
                                                            } else {
                                                                match value7 {
                                                                    Some(value7) => {
                                                                        if CHINESE_NUMBERS_CHARS[16].contains(&value7) {
                                                                            let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None)?;

                                                                            if msd == 0 {
                                                                                Err(0)
                                                                            } else {
                                                                                if let Some(value8) = value8 {
                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value8) {
                                                                                        if let Some(value9) = value9 {
                                                                                            let rds = chinese_digit_1000000000000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39).map_err(|err| err + 8)?;

                                                                                            Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                                        } else {
                                                                                            Err(8)
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value39);
                                                                                        let rds = chinese_digit_1000000000000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38).map_err(|err| err + 7)?;

                                                                                        Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                                    }
                                                                                } else {
                                                                                    Ok(msd as u128 * 10000000000000000)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            match value8 {
                                                                                Some(value8) => {
                                                                                    if CHINESE_NUMBERS_CHARS[16].contains(&value8) {
                                                                                        let msd = chinese_digit_1000_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7))?;

                                                                                        if msd == 0 {
                                                                                            Err(0)
                                                                                        } else {
                                                                                            if let Some(value9) = value9 {
                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value9) {
                                                                                                    if let Some(value10) = value10 {
                                                                                                        let rds = chinese_digit_1000000000000_ten_thousand_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, None).map_err(|err| err + 9)?;

                                                                                                        Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                                                    } else {
                                                                                                        Err(9)
                                                                                                    }
                                                                                                } else {
                                                                                                    let rds = chinese_digit_1000000000000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39).map_err(|err| err + 8)?;

                                                                                                    Ok(msd as u128 * 10000000000000000 + rds as u128)
                                                                                                }
                                                                                            } else {
                                                                                                Ok(msd as u128 * 10000000000000000)
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value32);
                                                                                        debug_assert_eq!(None, value33);
                                                                                        debug_assert_eq!(None, value34);
                                                                                        debug_assert_eq!(None, value35);
                                                                                        debug_assert_eq!(None, value36);
                                                                                        debug_assert_eq!(None, value37);
                                                                                        debug_assert_eq!(None, value38);
                                                                                        debug_assert_eq!(None, value39);
                                                                                        let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31)?;
                                                                                        Ok(n as u128)
                                                                                    }
                                                                                }
                                                                                None => {
                                                                                    debug_assert_eq!(None, value9);
                                                                                    debug_assert_eq!(None, value10);
                                                                                    debug_assert_eq!(None, value11);
                                                                                    debug_assert_eq!(None, value12);
                                                                                    debug_assert_eq!(None, value13);
                                                                                    debug_assert_eq!(None, value14);
                                                                                    debug_assert_eq!(None, value15);
                                                                                    debug_assert_eq!(None, value16);
                                                                                    debug_assert_eq!(None, value17);
                                                                                    debug_assert_eq!(None, value18);
                                                                                    debug_assert_eq!(None, value19);
                                                                                    debug_assert_eq!(None, value20);
                                                                                    debug_assert_eq!(None, value21);
                                                                                    debug_assert_eq!(None, value22);
                                                                                    debug_assert_eq!(None, value23);
                                                                                    debug_assert_eq!(None, value24);
                                                                                    debug_assert_eq!(None, value25);
                                                                                    debug_assert_eq!(None, value26);
                                                                                    debug_assert_eq!(None, value27);
                                                                                    debug_assert_eq!(None, value28);
                                                                                    debug_assert_eq!(None, value29);
                                                                                    debug_assert_eq!(None, value30);
                                                                                    debug_assert_eq!(None, value31);
                                                                                    debug_assert_eq!(None, value32);
                                                                                    debug_assert_eq!(None, value33);
                                                                                    debug_assert_eq!(None, value34);
                                                                                    debug_assert_eq!(None, value35);
                                                                                    debug_assert_eq!(None, value36);
                                                                                    debug_assert_eq!(None, value37);
                                                                                    debug_assert_eq!(None, value38);
                                                                                    debug_assert_eq!(None, value39);
                                                                                    let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                    Ok(n as u128)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug_assert_eq!(None, value8);
                                                                        debug_assert_eq!(None, value9);
                                                                        debug_assert_eq!(None, value10);
                                                                        debug_assert_eq!(None, value11);
                                                                        debug_assert_eq!(None, value12);
                                                                        debug_assert_eq!(None, value13);
                                                                        debug_assert_eq!(None, value14);
                                                                        debug_assert_eq!(None, value15);
                                                                        debug_assert_eq!(None, value16);
                                                                        debug_assert_eq!(None, value17);
                                                                        debug_assert_eq!(None, value18);
                                                                        debug_assert_eq!(None, value19);
                                                                        debug_assert_eq!(None, value20);
                                                                        debug_assert_eq!(None, value21);
                                                                        debug_assert_eq!(None, value22);
                                                                        debug_assert_eq!(None, value23);
                                                                        debug_assert_eq!(None, value24);
                                                                        debug_assert_eq!(None, value25);
                                                                        debug_assert_eq!(None, value26);
                                                                        debug_assert_eq!(None, value27);
                                                                        debug_assert_eq!(None, value28);
                                                                        debug_assert_eq!(None, value29);
                                                                        debug_assert_eq!(None, value30);
                                                                        debug_assert_eq!(None, value31);
                                                                        debug_assert_eq!(None, value32);
                                                                        debug_assert_eq!(None, value33);
                                                                        debug_assert_eq!(None, value34);
                                                                        debug_assert_eq!(None, value35);
                                                                        debug_assert_eq!(None, value36);
                                                                        debug_assert_eq!(None, value37);
                                                                        debug_assert_eq!(None, value38);
                                                                        debug_assert_eq!(None, value39);
                                                                        let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                        Ok(n as u128)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            debug_assert_eq!(None, value7);
                                                            debug_assert_eq!(None, value8);
                                                            debug_assert_eq!(None, value9);
                                                            debug_assert_eq!(None, value10);
                                                            debug_assert_eq!(None, value11);
                                                            debug_assert_eq!(None, value12);
                                                            debug_assert_eq!(None, value13);
                                                            debug_assert_eq!(None, value14);
                                                            debug_assert_eq!(None, value15);
                                                            debug_assert_eq!(None, value16);
                                                            debug_assert_eq!(None, value17);
                                                            debug_assert_eq!(None, value18);
                                                            debug_assert_eq!(None, value19);
                                                            debug_assert_eq!(None, value20);
                                                            debug_assert_eq!(None, value21);
                                                            debug_assert_eq!(None, value22);
                                                            debug_assert_eq!(None, value23);
                                                            debug_assert_eq!(None, value24);
                                                            debug_assert_eq!(None, value25);
                                                            debug_assert_eq!(None, value26);
                                                            debug_assert_eq!(None, value27);
                                                            debug_assert_eq!(None, value28);
                                                            debug_assert_eq!(None, value29);
                                                            debug_assert_eq!(None, value30);
                                                            debug_assert_eq!(None, value31);
                                                            debug_assert_eq!(None, value32);
                                                            debug_assert_eq!(None, value33);
                                                            debug_assert_eq!(None, value34);
                                                            debug_assert_eq!(None, value35);
                                                            debug_assert_eq!(None, value36);
                                                            debug_assert_eq!(None, value37);
                                                            debug_assert_eq!(None, value38);
                                                            debug_assert_eq!(None, value39);
                                                            let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                            Ok(n as u128)
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                debug_assert_eq!(None, value6);
                                                debug_assert_eq!(None, value7);
                                                debug_assert_eq!(None, value8);
                                                debug_assert_eq!(None, value9);
                                                debug_assert_eq!(None, value10);
                                                debug_assert_eq!(None, value11);
                                                debug_assert_eq!(None, value12);
                                                debug_assert_eq!(None, value13);
                                                debug_assert_eq!(None, value14);
                                                debug_assert_eq!(None, value15);
                                                debug_assert_eq!(None, value16);
                                                debug_assert_eq!(None, value17);
                                                debug_assert_eq!(None, value18);
                                                debug_assert_eq!(None, value19);
                                                debug_assert_eq!(None, value20);
                                                debug_assert_eq!(None, value21);
                                                debug_assert_eq!(None, value22);
                                                debug_assert_eq!(None, value23);
                                                debug_assert_eq!(None, value24);
                                                debug_assert_eq!(None, value25);
                                                debug_assert_eq!(None, value26);
                                                debug_assert_eq!(None, value27);
                                                debug_assert_eq!(None, value28);
                                                debug_assert_eq!(None, value29);
                                                debug_assert_eq!(None, value30);
                                                debug_assert_eq!(None, value31);
                                                debug_assert_eq!(None, value32);
                                                debug_assert_eq!(None, value33);
                                                debug_assert_eq!(None, value34);
                                                debug_assert_eq!(None, value35);
                                                debug_assert_eq!(None, value36);
                                                debug_assert_eq!(None, value37);
                                                debug_assert_eq!(None, value38);
                                                debug_assert_eq!(None, value39);
                                                let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                Ok(n as u128)
                                            }
                                        }
                                    }
                                }
                                None => {
                                    debug_assert_eq!(None, value5);
                                    debug_assert_eq!(None, value6);
                                    debug_assert_eq!(None, value7);
                                    debug_assert_eq!(None, value8);
                                    debug_assert_eq!(None, value9);
                                    debug_assert_eq!(None, value10);
                                    debug_assert_eq!(None, value11);
                                    debug_assert_eq!(None, value12);
                                    debug_assert_eq!(None, value13);
                                    debug_assert_eq!(None, value14);
                                    debug_assert_eq!(None, value15);
                                    debug_assert_eq!(None, value16);
                                    debug_assert_eq!(None, value17);
                                    debug_assert_eq!(None, value18);
                                    debug_assert_eq!(None, value19);
                                    debug_assert_eq!(None, value20);
                                    debug_assert_eq!(None, value21);
                                    debug_assert_eq!(None, value22);
                                    debug_assert_eq!(None, value23);
                                    debug_assert_eq!(None, value24);
                                    debug_assert_eq!(None, value25);
                                    debug_assert_eq!(None, value26);
                                    debug_assert_eq!(None, value27);
                                    debug_assert_eq!(None, value28);
                                    debug_assert_eq!(None, value29);
                                    debug_assert_eq!(None, value30);
                                    debug_assert_eq!(None, value31);
                                    debug_assert_eq!(None, value32);
                                    debug_assert_eq!(None, value33);
                                    debug_assert_eq!(None, value34);
                                    debug_assert_eq!(None, value35);
                                    debug_assert_eq!(None, value36);
                                    debug_assert_eq!(None, value37);
                                    debug_assert_eq!(None, value38);
                                    debug_assert_eq!(None, value39);
                                    let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), Some(value3), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                    Ok(n as u128)
                                }
                            }
                        }
                    }
                    None => {
                        debug_assert_eq!(None, value4);
                        debug_assert_eq!(None, value5);
                        debug_assert_eq!(None, value6);
                        debug_assert_eq!(None, value7);
                        debug_assert_eq!(None, value8);
                        debug_assert_eq!(None, value9);
                        debug_assert_eq!(None, value10);
                        debug_assert_eq!(None, value11);
                        debug_assert_eq!(None, value12);
                        debug_assert_eq!(None, value13);
                        debug_assert_eq!(None, value14);
                        debug_assert_eq!(None, value15);
                        debug_assert_eq!(None, value16);
                        debug_assert_eq!(None, value17);
                        debug_assert_eq!(None, value18);
                        debug_assert_eq!(None, value19);
                        debug_assert_eq!(None, value20);
                        debug_assert_eq!(None, value21);
                        debug_assert_eq!(None, value22);
                        debug_assert_eq!(None, value23);
                        debug_assert_eq!(None, value24);
                        debug_assert_eq!(None, value25);
                        debug_assert_eq!(None, value26);
                        debug_assert_eq!(None, value27);
                        debug_assert_eq!(None, value28);
                        debug_assert_eq!(None, value29);
                        debug_assert_eq!(None, value30);
                        debug_assert_eq!(None, value31);
                        debug_assert_eq!(None, value32);
                        debug_assert_eq!(None, value33);
                        debug_assert_eq!(None, value34);
                        debug_assert_eq!(None, value35);
                        debug_assert_eq!(None, value36);
                        debug_assert_eq!(None, value37);
                        debug_assert_eq!(None, value38);
                        debug_assert_eq!(None, value39);
                        let n = chinese_digit_1000000000000_ten_thousand_compat(value, Some(value2), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                        Ok(n as u128)
                    }
                }
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            debug_assert_eq!(None, value26);
            debug_assert_eq!(None, value27);
            debug_assert_eq!(None, value28);
            debug_assert_eq!(None, value29);
            debug_assert_eq!(None, value30);
            debug_assert_eq!(None, value31);
            debug_assert_eq!(None, value32);
            debug_assert_eq!(None, value33);
            debug_assert_eq!(None, value34);
            debug_assert_eq!(None, value35);
            debug_assert_eq!(None, value36);
            debug_assert_eq!(None, value37);
            debug_assert_eq!(None, value38);
            debug_assert_eq!(None, value39);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u128)
        }
    }
}

pub(crate) fn chinese_digit_100000000_middle_compat(value: char, value2: Option<char>, value3: Option<char>, value4: Option<char>, value5: Option<char>, value6: Option<char>, value7: Option<char>, value8: Option<char>, value9: Option<char>, value10: Option<char>, value11: Option<char>, value12: Option<char>, value13: Option<char>, value14: Option<char>, value15: Option<char>, value16: Option<char>, value17: Option<char>, value18: Option<char>, value19: Option<char>, value20: Option<char>, value21: Option<char>, value22: Option<char>, value23: Option<char>, value24: Option<char>, value25: Option<char>, value26: Option<char>, value27: Option<char>, value28: Option<char>, value29: Option<char>, value30: Option<char>, value31: Option<char>) -> Result<u64, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[14].contains(&value2) {
                let msd = chinese_digit_10_compat(value, None, None)?;

                if msd == 0 {
                    Err(0)
                } else {
                    if let Some(value3) = value3 {
                        if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                            if let Some(value4) = value4 {
                                debug_assert_eq!(None, value19);
                                debug_assert_eq!(None, value20);
                                debug_assert_eq!(None, value21);
                                debug_assert_eq!(None, value22);
                                debug_assert_eq!(None, value23);
                                debug_assert_eq!(None, value24);
                                debug_assert_eq!(None, value25);
                                debug_assert_eq!(None, value26);
                                debug_assert_eq!(None, value27);
                                debug_assert_eq!(None, value28);
                                debug_assert_eq!(None, value29);
                                debug_assert_eq!(None, value30);
                                debug_assert_eq!(None, value31);
                                let rds = chinese_digit_10000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18).map_err(|err| err + 3)?;

                                Ok(msd as u64 * 100000000 + rds as u64)
                            } else {
                                Err(3)
                            }
                        } else {
                            debug_assert_eq!(None, value18);
                            debug_assert_eq!(None, value19);
                            debug_assert_eq!(None, value20);
                            debug_assert_eq!(None, value21);
                            debug_assert_eq!(None, value22);
                            debug_assert_eq!(None, value23);
                            debug_assert_eq!(None, value24);
                            debug_assert_eq!(None, value25);
                            debug_assert_eq!(None, value26);
                            debug_assert_eq!(None, value27);
                            debug_assert_eq!(None, value28);
                            debug_assert_eq!(None, value29);
                            debug_assert_eq!(None, value30);
                            debug_assert_eq!(None, value31);
                            let rds = chinese_digit_10000_ten_thousand_compat(value3, value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17).map_err(|err| err + 2)?;

                            Ok(msd as u64 * 100000000 + rds as u64)
                        }
                    } else {
                        Ok(msd as u64 * 100000000)
                    }
                }
            } else {
                match value3 {
                    Some(value3) => {
                        if CHINESE_NUMBERS_CHARS[14].contains(&value3) {
                            let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), None, None, None, None, None, None, None, None, None, None, None, None, None)?;

                            if msd == 0 {
                                Err(0)
                            } else {
                                if let Some(value4) = value4 {
                                    if CHINESE_NUMBERS_CHARS[0].contains(&value4) {
                                        if let Some(value5) = value5 {
                                            debug_assert_eq!(None, value20);
                                            debug_assert_eq!(None, value21);
                                            debug_assert_eq!(None, value22);
                                            debug_assert_eq!(None, value23);
                                            debug_assert_eq!(None, value24);
                                            debug_assert_eq!(None, value25);
                                            debug_assert_eq!(None, value26);
                                            debug_assert_eq!(None, value27);
                                            debug_assert_eq!(None, value28);
                                            debug_assert_eq!(None, value29);
                                            debug_assert_eq!(None, value30);
                                            debug_assert_eq!(None, value31);
                                            let rds = chinese_digit_10000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19).map_err(|err| err + 4)?;

                                            Ok(msd as u64 * 100000000 + rds as u64)
                                        } else {
                                            Err(4)
                                        }
                                    } else {
                                        debug_assert_eq!(None, value19);
                                        debug_assert_eq!(None, value20);
                                        debug_assert_eq!(None, value21);
                                        debug_assert_eq!(None, value22);
                                        debug_assert_eq!(None, value23);
                                        debug_assert_eq!(None, value24);
                                        debug_assert_eq!(None, value25);
                                        debug_assert_eq!(None, value26);
                                        debug_assert_eq!(None, value27);
                                        debug_assert_eq!(None, value28);
                                        debug_assert_eq!(None, value29);
                                        debug_assert_eq!(None, value30);
                                        debug_assert_eq!(None, value31);
                                        let rds = chinese_digit_10000_ten_thousand_compat(value4, value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18).map_err(|err| err + 3)?;

                                        Ok(msd as u64 * 100000000 + rds as u64)
                                    }
                                } else {
                                    Ok(msd as u64 * 100000000)
                                }
                            }
                        } else {
                            match value4 {
                                Some(value4) => {
                                    if CHINESE_NUMBERS_CHARS[14].contains(&value4) {
                                        let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), None, None, None, None, None, None, None, None, None, None, None, None)?;

                                        if msd == 0 {
                                            Err(0)
                                        } else {
                                            if let Some(value5) = value5 {
                                                if CHINESE_NUMBERS_CHARS[0].contains(&value5) {
                                                    if let Some(value6) = value6 {
                                                        debug_assert_eq!(None, value21);
                                                        debug_assert_eq!(None, value22);
                                                        debug_assert_eq!(None, value23);
                                                        debug_assert_eq!(None, value24);
                                                        debug_assert_eq!(None, value25);
                                                        debug_assert_eq!(None, value26);
                                                        debug_assert_eq!(None, value27);
                                                        debug_assert_eq!(None, value28);
                                                        debug_assert_eq!(None, value29);
                                                        debug_assert_eq!(None, value30);
                                                        debug_assert_eq!(None, value31);
                                                        let rds = chinese_digit_10000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20).map_err(|err| err + 5)?;

                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                    } else {
                                                        Err(5)
                                                    }
                                                } else {
                                                    debug_assert_eq!(None, value20);
                                                    debug_assert_eq!(None, value21);
                                                    debug_assert_eq!(None, value22);
                                                    debug_assert_eq!(None, value23);
                                                    debug_assert_eq!(None, value24);
                                                    debug_assert_eq!(None, value25);
                                                    debug_assert_eq!(None, value26);
                                                    debug_assert_eq!(None, value27);
                                                    debug_assert_eq!(None, value28);
                                                    debug_assert_eq!(None, value29);
                                                    debug_assert_eq!(None, value30);
                                                    debug_assert_eq!(None, value31);
                                                    let rds = chinese_digit_10000_ten_thousand_compat(value5, value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19).map_err(|err| err + 4)?;

                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                }
                                            } else {
                                                Ok(msd as u64 * 100000000)
                                            }
                                        }
                                    } else {
                                        match value5 {
                                            Some(value5) => {
                                                if CHINESE_NUMBERS_CHARS[14].contains(&value5) {
                                                    let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), None, None, None, None, None, None, None, None, None, None, None)?;

                                                    if msd == 0 {
                                                        Err(0)
                                                    } else {
                                                        if let Some(value6) = value6 {
                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value6) {
                                                                if let Some(value7) = value7 {
                                                                    debug_assert_eq!(None, value22);
                                                                    debug_assert_eq!(None, value23);
                                                                    debug_assert_eq!(None, value24);
                                                                    debug_assert_eq!(None, value25);
                                                                    debug_assert_eq!(None, value26);
                                                                    debug_assert_eq!(None, value27);
                                                                    debug_assert_eq!(None, value28);
                                                                    debug_assert_eq!(None, value29);
                                                                    debug_assert_eq!(None, value30);
                                                                    debug_assert_eq!(None, value31);
                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21).map_err(|err| err + 6)?;

                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                } else {
                                                                    Err(6)
                                                                }
                                                            } else {
                                                                debug_assert_eq!(None, value21);
                                                                debug_assert_eq!(None, value22);
                                                                debug_assert_eq!(None, value23);
                                                                debug_assert_eq!(None, value24);
                                                                debug_assert_eq!(None, value25);
                                                                debug_assert_eq!(None, value26);
                                                                debug_assert_eq!(None, value27);
                                                                debug_assert_eq!(None, value28);
                                                                debug_assert_eq!(None, value29);
                                                                debug_assert_eq!(None, value30);
                                                                debug_assert_eq!(None, value31);
                                                                let rds = chinese_digit_10000_ten_thousand_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20).map_err(|err| err + 5)?;

                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                            }
                                                        } else {
                                                            Ok(msd as u64 * 100000000)
                                                        }
                                                    }
                                                } else {
                                                    match value6 {
                                                        Some(value6) => {
                                                            if CHINESE_NUMBERS_CHARS[14].contains(&value6) {
                                                                let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None)?;

                                                                if msd == 0 {
                                                                    Err(0)
                                                                } else {
                                                                    if let Some(value7) = value7 {
                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value7) {
                                                                            if let Some(value8) = value8 {
                                                                                debug_assert_eq!(None, value23);
                                                                                debug_assert_eq!(None, value24);
                                                                                debug_assert_eq!(None, value25);
                                                                                debug_assert_eq!(None, value26);
                                                                                debug_assert_eq!(None, value27);
                                                                                debug_assert_eq!(None, value28);
                                                                                debug_assert_eq!(None, value29);
                                                                                debug_assert_eq!(None, value30);
                                                                                debug_assert_eq!(None, value31);
                                                                                let rds = chinese_digit_10000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22).map_err(|err| err + 7)?;

                                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                                            } else {
                                                                                Err(7)
                                                                            }
                                                                        } else {
                                                                            debug_assert_eq!(None, value22);
                                                                            debug_assert_eq!(None, value23);
                                                                            debug_assert_eq!(None, value24);
                                                                            debug_assert_eq!(None, value25);
                                                                            debug_assert_eq!(None, value26);
                                                                            debug_assert_eq!(None, value27);
                                                                            debug_assert_eq!(None, value28);
                                                                            debug_assert_eq!(None, value29);
                                                                            debug_assert_eq!(None, value30);
                                                                            debug_assert_eq!(None, value31);
                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21).map_err(|err| err + 6)?;

                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                        }
                                                                    } else {
                                                                        Ok(msd as u64 * 100000000)
                                                                    }
                                                                }
                                                            } else {
                                                                match value7 {
                                                                    Some(value7) => {
                                                                        if CHINESE_NUMBERS_CHARS[14].contains(&value7) {
                                                                            let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None)?;

                                                                            if msd == 0 {
                                                                                Err(0)
                                                                            } else {
                                                                                if let Some(value8) = value8 {
                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value8) {
                                                                                        if let Some(value9) = value9 {
                                                                                            debug_assert_eq!(None, value24);
                                                                                            debug_assert_eq!(None, value25);
                                                                                            debug_assert_eq!(None, value26);
                                                                                            debug_assert_eq!(None, value27);
                                                                                            debug_assert_eq!(None, value28);
                                                                                            debug_assert_eq!(None, value29);
                                                                                            debug_assert_eq!(None, value30);
                                                                                            debug_assert_eq!(None, value31);
                                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23).map_err(|err| err + 8)?;

                                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                                        } else {
                                                                                            Err(8)
                                                                                        }
                                                                                    } else {
                                                                                        debug_assert_eq!(None, value23);
                                                                                        debug_assert_eq!(None, value24);
                                                                                        debug_assert_eq!(None, value25);
                                                                                        debug_assert_eq!(None, value26);
                                                                                        debug_assert_eq!(None, value27);
                                                                                        debug_assert_eq!(None, value28);
                                                                                        debug_assert_eq!(None, value29);
                                                                                        debug_assert_eq!(None, value30);
                                                                                        debug_assert_eq!(None, value31);
                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22).map_err(|err| err + 7)?;

                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                    }
                                                                                } else {
                                                                                    Ok(msd as u64 * 100000000)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            match value8 {
                                                                                Some(value8) => {
                                                                                    if CHINESE_NUMBERS_CHARS[14].contains(&value8) {
                                                                                        let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None)?;

                                                                                        if msd == 0 {
                                                                                            Err(0)
                                                                                        } else {
                                                                                            if let Some(value9) = value9 {
                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value9) {
                                                                                                    if let Some(value10) = value10 {
                                                                                                        debug_assert_eq!(None, value25);
                                                                                                        debug_assert_eq!(None, value26);
                                                                                                        debug_assert_eq!(None, value27);
                                                                                                        debug_assert_eq!(None, value28);
                                                                                                        debug_assert_eq!(None, value29);
                                                                                                        debug_assert_eq!(None, value30);
                                                                                                        debug_assert_eq!(None, value31);
                                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24).map_err(|err| err + 9)?;

                                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                    } else {
                                                                                                        Err(9)
                                                                                                    }
                                                                                                } else {
                                                                                                    debug_assert_eq!(None, value24);
                                                                                                    debug_assert_eq!(None, value25);
                                                                                                    debug_assert_eq!(None, value26);
                                                                                                    debug_assert_eq!(None, value27);
                                                                                                    debug_assert_eq!(None, value28);
                                                                                                    debug_assert_eq!(None, value29);
                                                                                                    debug_assert_eq!(None, value30);
                                                                                                    debug_assert_eq!(None, value31);
                                                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23).map_err(|err| err + 8)?;

                                                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                }
                                                                                            } else {
                                                                                                Ok(msd as u64 * 100000000)
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        match value9 {
                                                                                            Some(value9) => {
                                                                                                if CHINESE_NUMBERS_CHARS[14].contains(&value9) {
                                                                                                    let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), None, None, None, None, None, None, None)?;

                                                                                                    if msd == 0 {
                                                                                                        Err(0)
                                                                                                    } else {
                                                                                                        if let Some(value10) = value10 {
                                                                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value10) {
                                                                                                                if let Some(value11) = value11 {
                                                                                                                    debug_assert_eq!(None, value26);
                                                                                                                    debug_assert_eq!(None, value27);
                                                                                                                    debug_assert_eq!(None, value28);
                                                                                                                    debug_assert_eq!(None, value29);
                                                                                                                    debug_assert_eq!(None, value30);
                                                                                                                    debug_assert_eq!(None, value31);
                                                                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25).map_err(|err| err + 10)?;

                                                                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                } else {
                                                                                                                    Err(10)
                                                                                                                }
                                                                                                            } else {
                                                                                                                debug_assert_eq!(None, value25);
                                                                                                                debug_assert_eq!(None, value26);
                                                                                                                debug_assert_eq!(None, value27);
                                                                                                                debug_assert_eq!(None, value28);
                                                                                                                debug_assert_eq!(None, value29);
                                                                                                                debug_assert_eq!(None, value30);
                                                                                                                debug_assert_eq!(None, value31);
                                                                                                                let rds = chinese_digit_10000_ten_thousand_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24).map_err(|err| err + 9)?;

                                                                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                            }
                                                                                                        } else {
                                                                                                            Ok(msd as u64 * 100000000)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    match value10 {
                                                                                                        Some(value10) => {
                                                                                                            if CHINESE_NUMBERS_CHARS[14].contains(&value10) {
                                                                                                                let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), None, None, None, None, None, None)?;

                                                                                                                if msd == 0 {
                                                                                                                    Err(0)
                                                                                                                } else {
                                                                                                                    if let Some(value11) = value11 {
                                                                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value11) {
                                                                                                                            if let Some(value12) = value12 {
                                                                                                                                debug_assert_eq!(None, value27);
                                                                                                                                debug_assert_eq!(None, value28);
                                                                                                                                debug_assert_eq!(None, value29);
                                                                                                                                debug_assert_eq!(None, value30);
                                                                                                                                debug_assert_eq!(None, value31);
                                                                                                                                let rds = chinese_digit_10000_ten_thousand_compat(value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26).map_err(|err| err + 11)?;

                                                                                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                            } else {
                                                                                                                                Err(11)
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            debug_assert_eq!(None, value26);
                                                                                                                            debug_assert_eq!(None, value27);
                                                                                                                            debug_assert_eq!(None, value28);
                                                                                                                            debug_assert_eq!(None, value29);
                                                                                                                            debug_assert_eq!(None, value30);
                                                                                                                            debug_assert_eq!(None, value31);
                                                                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25).map_err(|err| err + 10)?;

                                                                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        Ok(msd as u64 * 100000000)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                match value11 {
                                                                                                                    Some(value11) => {
                                                                                                                        if CHINESE_NUMBERS_CHARS[14].contains(&value11) {
                                                                                                                            let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), None, None, None, None, None)?;

                                                                                                                            if msd == 0 {
                                                                                                                                Err(0)
                                                                                                                            } else {
                                                                                                                                if let Some(value12) = value12 {
                                                                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value12) {
                                                                                                                                        if let Some(value13) = value13 {
                                                                                                                                            debug_assert_eq!(None, value28);
                                                                                                                                            debug_assert_eq!(None, value29);
                                                                                                                                            debug_assert_eq!(None, value30);
                                                                                                                                            debug_assert_eq!(None, value31);
                                                                                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27).map_err(|err| err + 12)?;

                                                                                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                        } else {
                                                                                                                                            Err(12)
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        debug_assert_eq!(None, value27);
                                                                                                                                        debug_assert_eq!(None, value28);
                                                                                                                                        debug_assert_eq!(None, value29);
                                                                                                                                        debug_assert_eq!(None, value30);
                                                                                                                                        debug_assert_eq!(None, value31);
                                                                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26).map_err(|err| err + 11)?;

                                                                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    Ok(msd as u64 * 100000000)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            match value12 {
                                                                                                                                Some(value12) => {
                                                                                                                                    if CHINESE_NUMBERS_CHARS[14].contains(&value12) {
                                                                                                                                        let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), None, None, None, None)?;

                                                                                                                                        if msd == 0 {
                                                                                                                                            Err(0)
                                                                                                                                        } else {
                                                                                                                                            if let Some(value13) = value13 {
                                                                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value13) {
                                                                                                                                                    if let Some(value14) = value14 {
                                                                                                                                                        debug_assert_eq!(None, value29);
                                                                                                                                                        debug_assert_eq!(None, value30);
                                                                                                                                                        debug_assert_eq!(None, value31);
                                                                                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28).map_err(|err| err + 13)?;

                                                                                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                    } else {
                                                                                                                                                        Err(13)
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    debug_assert_eq!(None, value28);
                                                                                                                                                    debug_assert_eq!(None, value29);
                                                                                                                                                    debug_assert_eq!(None, value30);
                                                                                                                                                    debug_assert_eq!(None, value31);
                                                                                                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27).map_err(|err| err + 12)?;

                                                                                                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                Ok(msd as u64 * 100000000)
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        match value13 {
                                                                                                                                            Some(value13) => {
                                                                                                                                                if CHINESE_NUMBERS_CHARS[14].contains(&value13) {
                                                                                                                                                    let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), None, None, None)?;

                                                                                                                                                    if msd == 0 {
                                                                                                                                                        Err(0)
                                                                                                                                                    } else {
                                                                                                                                                        if let Some(value14) = value14 {
                                                                                                                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value14) {
                                                                                                                                                                if let Some(value15) = value15 {
                                                                                                                                                                    debug_assert_eq!(None, value30);
                                                                                                                                                                    debug_assert_eq!(None, value31);
                                                                                                                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29).map_err(|err| err + 14)?;

                                                                                                                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                } else {
                                                                                                                                                                    Err(14)
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                debug_assert_eq!(None, value29);
                                                                                                                                                                debug_assert_eq!(None, value30);
                                                                                                                                                                debug_assert_eq!(None, value31);
                                                                                                                                                                let rds = chinese_digit_10000_ten_thousand_compat(value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28).map_err(|err| err + 13)?;

                                                                                                                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            Ok(msd as u64 * 100000000)
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    match value14 {
                                                                                                                                                        Some(value14) => {
                                                                                                                                                            if CHINESE_NUMBERS_CHARS[14].contains(&value14) {
                                                                                                                                                                let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), None, None)?;

                                                                                                                                                                if msd == 0 {
                                                                                                                                                                    Err(0)
                                                                                                                                                                } else {
                                                                                                                                                                    if let Some(value15) = value15 {
                                                                                                                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value15) {
                                                                                                                                                                            if let Some(value16) = value16 {
                                                                                                                                                                                debug_assert_eq!(None, value31);
                                                                                                                                                                                let rds = chinese_digit_10000_ten_thousand_compat(value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30).map_err(|err| err + 15)?;

                                                                                                                                                                                Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                            } else {
                                                                                                                                                                                Err(15)
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            debug_assert_eq!(None, value30);
                                                                                                                                                                            debug_assert_eq!(None, value31);
                                                                                                                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29).map_err(|err| err + 14)?;

                                                                                                                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        Ok(msd as u64 * 100000000)
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                match value15 {
                                                                                                                                                                    Some(value15) => {
                                                                                                                                                                        if CHINESE_NUMBERS_CHARS[14].contains(&value15) {
                                                                                                                                                                            let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), None)?;

                                                                                                                                                                            if msd == 0 {
                                                                                                                                                                                Err(0)
                                                                                                                                                                            } else {
                                                                                                                                                                                if let Some(value16) = value16 {
                                                                                                                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value16) {
                                                                                                                                                                                        if let Some(value17) = value17 {
                                                                                                                                                                                            let rds = chinese_digit_10000_ten_thousand_compat(value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31).map_err(|err| err + 16)?;

                                                                                                                                                                                            Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            Err(16)
                                                                                                                                                                                        }
                                                                                                                                                                                    } else {
                                                                                                                                                                                        debug_assert_eq!(None, value31);
                                                                                                                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30).map_err(|err| err + 15)?;

                                                                                                                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                                    }
                                                                                                                                                                                } else {
                                                                                                                                                                                    Ok(msd as u64 * 100000000)
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            match value16 {
                                                                                                                                                                                Some(value16) => {
                                                                                                                                                                                    if CHINESE_NUMBERS_CHARS[14].contains(&value16) {
                                                                                                                                                                                        let msd = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), Some(value15))?;

                                                                                                                                                                                        if msd == 0 {
                                                                                                                                                                                            Err(0)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if let Some(value17) = value17 {
                                                                                                                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value17) {
                                                                                                                                                                                                    if let Some(value18) = value18 {
                                                                                                                                                                                                        let rds = chinese_digit_10000_ten_thousand_compat(value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, None).map_err(|err| err + 17)?;

                                                                                                                                                                                                        Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                                                    } else {
                                                                                                                                                                                                        Err(17)
                                                                                                                                                                                                    }
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    let rds = chinese_digit_10000_ten_thousand_compat(value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31).map_err(|err| err + 16)?;

                                                                                                                                                                                                    Ok(msd as u64 * 100000000 + rds as u64)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                Ok(msd as u64 * 100000000)
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    } else {
                                                                                                                                                                                        Err(15)
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                                None => {
                                                                                                                                                                                    debug_assert_eq!(None, value17);
                                                                                                                                                                                    debug_assert_eq!(None, value18);
                                                                                                                                                                                    debug_assert_eq!(None, value19);
                                                                                                                                                                                    debug_assert_eq!(None, value20);
                                                                                                                                                                                    debug_assert_eq!(None, value21);
                                                                                                                                                                                    debug_assert_eq!(None, value22);
                                                                                                                                                                                    debug_assert_eq!(None, value23);
                                                                                                                                                                                    debug_assert_eq!(None, value24);
                                                                                                                                                                                    debug_assert_eq!(None, value25);
                                                                                                                                                                                    debug_assert_eq!(None, value26);
                                                                                                                                                                                    debug_assert_eq!(None, value27);
                                                                                                                                                                                    debug_assert_eq!(None, value28);
                                                                                                                                                                                    debug_assert_eq!(None, value29);
                                                                                                                                                                                    debug_assert_eq!(None, value30);
                                                                                                                                                                                    debug_assert_eq!(None, value31);
                                                                                                                                                                                    let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), Some(value15))?;
                                                                                                                                                                                    Ok(n as u64)
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                    None => {
                                                                                                                                                                        debug_assert_eq!(None, value16);
                                                                                                                                                                        debug_assert_eq!(None, value17);
                                                                                                                                                                        debug_assert_eq!(None, value18);
                                                                                                                                                                        debug_assert_eq!(None, value19);
                                                                                                                                                                        debug_assert_eq!(None, value20);
                                                                                                                                                                        debug_assert_eq!(None, value21);
                                                                                                                                                                        debug_assert_eq!(None, value22);
                                                                                                                                                                        debug_assert_eq!(None, value23);
                                                                                                                                                                        debug_assert_eq!(None, value24);
                                                                                                                                                                        debug_assert_eq!(None, value25);
                                                                                                                                                                        debug_assert_eq!(None, value26);
                                                                                                                                                                        debug_assert_eq!(None, value27);
                                                                                                                                                                        debug_assert_eq!(None, value28);
                                                                                                                                                                        debug_assert_eq!(None, value29);
                                                                                                                                                                        debug_assert_eq!(None, value30);
                                                                                                                                                                        debug_assert_eq!(None, value31);
                                                                                                                                                                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), None)?;
                                                                                                                                                                        Ok(n as u64)
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        None => {
                                                                                                                                                            debug_assert_eq!(None, value15);
                                                                                                                                                            debug_assert_eq!(None, value16);
                                                                                                                                                            debug_assert_eq!(None, value17);
                                                                                                                                                            debug_assert_eq!(None, value18);
                                                                                                                                                            debug_assert_eq!(None, value19);
                                                                                                                                                            debug_assert_eq!(None, value20);
                                                                                                                                                            debug_assert_eq!(None, value21);
                                                                                                                                                            debug_assert_eq!(None, value22);
                                                                                                                                                            debug_assert_eq!(None, value23);
                                                                                                                                                            debug_assert_eq!(None, value24);
                                                                                                                                                            debug_assert_eq!(None, value25);
                                                                                                                                                            debug_assert_eq!(None, value26);
                                                                                                                                                            debug_assert_eq!(None, value27);
                                                                                                                                                            debug_assert_eq!(None, value28);
                                                                                                                                                            debug_assert_eq!(None, value29);
                                                                                                                                                            debug_assert_eq!(None, value30);
                                                                                                                                                            debug_assert_eq!(None, value31);
                                                                                                                                                            let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), None, None)?;
                                                                                                                                                            Ok(n as u64)
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            None => {
                                                                                                                                                debug_assert_eq!(None, value14);
                                                                                                                                                debug_assert_eq!(None, value15);
                                                                                                                                                debug_assert_eq!(None, value16);
                                                                                                                                                debug_assert_eq!(None, value17);
                                                                                                                                                debug_assert_eq!(None, value18);
                                                                                                                                                debug_assert_eq!(None, value19);
                                                                                                                                                debug_assert_eq!(None, value20);
                                                                                                                                                debug_assert_eq!(None, value21);
                                                                                                                                                debug_assert_eq!(None, value22);
                                                                                                                                                debug_assert_eq!(None, value23);
                                                                                                                                                debug_assert_eq!(None, value24);
                                                                                                                                                debug_assert_eq!(None, value25);
                                                                                                                                                debug_assert_eq!(None, value26);
                                                                                                                                                debug_assert_eq!(None, value27);
                                                                                                                                                debug_assert_eq!(None, value28);
                                                                                                                                                debug_assert_eq!(None, value29);
                                                                                                                                                debug_assert_eq!(None, value30);
                                                                                                                                                debug_assert_eq!(None, value31);
                                                                                                                                                let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), None, None, None)?;
                                                                                                                                                Ok(n as u64)
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                None => {
                                                                                                                                    debug_assert_eq!(None, value13);
                                                                                                                                    debug_assert_eq!(None, value14);
                                                                                                                                    debug_assert_eq!(None, value15);
                                                                                                                                    debug_assert_eq!(None, value16);
                                                                                                                                    debug_assert_eq!(None, value17);
                                                                                                                                    debug_assert_eq!(None, value18);
                                                                                                                                    debug_assert_eq!(None, value19);
                                                                                                                                    debug_assert_eq!(None, value20);
                                                                                                                                    debug_assert_eq!(None, value21);
                                                                                                                                    debug_assert_eq!(None, value22);
                                                                                                                                    debug_assert_eq!(None, value23);
                                                                                                                                    debug_assert_eq!(None, value24);
                                                                                                                                    debug_assert_eq!(None, value25);
                                                                                                                                    debug_assert_eq!(None, value26);
                                                                                                                                    debug_assert_eq!(None, value27);
                                                                                                                                    debug_assert_eq!(None, value28);
                                                                                                                                    debug_assert_eq!(None, value29);
                                                                                                                                    debug_assert_eq!(None, value30);
                                                                                                                                    debug_assert_eq!(None, value31);
                                                                                                                                    let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), None, None, None, None)?;
                                                                                                                                    Ok(n as u64)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    None => {
                                                                                                                        debug_assert_eq!(None, value12);
                                                                                                                        debug_assert_eq!(None, value13);
                                                                                                                        debug_assert_eq!(None, value14);
                                                                                                                        debug_assert_eq!(None, value15);
                                                                                                                        debug_assert_eq!(None, value16);
                                                                                                                        debug_assert_eq!(None, value17);
                                                                                                                        debug_assert_eq!(None, value18);
                                                                                                                        debug_assert_eq!(None, value19);
                                                                                                                        debug_assert_eq!(None, value20);
                                                                                                                        debug_assert_eq!(None, value21);
                                                                                                                        debug_assert_eq!(None, value22);
                                                                                                                        debug_assert_eq!(None, value23);
                                                                                                                        debug_assert_eq!(None, value24);
                                                                                                                        debug_assert_eq!(None, value25);
                                                                                                                        debug_assert_eq!(None, value26);
                                                                                                                        debug_assert_eq!(None, value27);
                                                                                                                        debug_assert_eq!(None, value28);
                                                                                                                        debug_assert_eq!(None, value29);
                                                                                                                        debug_assert_eq!(None, value30);
                                                                                                                        debug_assert_eq!(None, value31);
                                                                                                                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), None, None, None, None, None)?;
                                                                                                                        Ok(n as u64)
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        None => {
                                                                                                            debug_assert_eq!(None, value11);
                                                                                                            debug_assert_eq!(None, value12);
                                                                                                            debug_assert_eq!(None, value13);
                                                                                                            debug_assert_eq!(None, value14);
                                                                                                            debug_assert_eq!(None, value15);
                                                                                                            debug_assert_eq!(None, value16);
                                                                                                            debug_assert_eq!(None, value17);
                                                                                                            debug_assert_eq!(None, value18);
                                                                                                            debug_assert_eq!(None, value19);
                                                                                                            debug_assert_eq!(None, value20);
                                                                                                            debug_assert_eq!(None, value21);
                                                                                                            debug_assert_eq!(None, value22);
                                                                                                            debug_assert_eq!(None, value23);
                                                                                                            debug_assert_eq!(None, value24);
                                                                                                            debug_assert_eq!(None, value25);
                                                                                                            debug_assert_eq!(None, value26);
                                                                                                            debug_assert_eq!(None, value27);
                                                                                                            debug_assert_eq!(None, value28);
                                                                                                            debug_assert_eq!(None, value29);
                                                                                                            debug_assert_eq!(None, value30);
                                                                                                            debug_assert_eq!(None, value31);
                                                                                                            let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), None, None, None, None, None, None)?;
                                                                                                            Ok(n as u64)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            None => {
                                                                                                debug_assert_eq!(None, value10);
                                                                                                debug_assert_eq!(None, value11);
                                                                                                debug_assert_eq!(None, value12);
                                                                                                debug_assert_eq!(None, value13);
                                                                                                debug_assert_eq!(None, value14);
                                                                                                debug_assert_eq!(None, value15);
                                                                                                debug_assert_eq!(None, value16);
                                                                                                debug_assert_eq!(None, value17);
                                                                                                debug_assert_eq!(None, value18);
                                                                                                debug_assert_eq!(None, value19);
                                                                                                debug_assert_eq!(None, value20);
                                                                                                debug_assert_eq!(None, value21);
                                                                                                debug_assert_eq!(None, value22);
                                                                                                debug_assert_eq!(None, value23);
                                                                                                debug_assert_eq!(None, value24);
                                                                                                debug_assert_eq!(None, value25);
                                                                                                debug_assert_eq!(None, value26);
                                                                                                debug_assert_eq!(None, value27);
                                                                                                debug_assert_eq!(None, value28);
                                                                                                debug_assert_eq!(None, value29);
                                                                                                debug_assert_eq!(None, value30);
                                                                                                debug_assert_eq!(None, value31);
                                                                                                let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), None, None, None, None, None, None, None)?;
                                                                                                Ok(n as u64)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                None => {
                                                                                    debug_assert_eq!(None, value9);
                                                                                    debug_assert_eq!(None, value10);
                                                                                    debug_assert_eq!(None, value11);
                                                                                    debug_assert_eq!(None, value12);
                                                                                    debug_assert_eq!(None, value13);
                                                                                    debug_assert_eq!(None, value14);
                                                                                    debug_assert_eq!(None, value15);
                                                                                    debug_assert_eq!(None, value16);
                                                                                    debug_assert_eq!(None, value17);
                                                                                    debug_assert_eq!(None, value18);
                                                                                    debug_assert_eq!(None, value19);
                                                                                    debug_assert_eq!(None, value20);
                                                                                    debug_assert_eq!(None, value21);
                                                                                    debug_assert_eq!(None, value22);
                                                                                    debug_assert_eq!(None, value23);
                                                                                    debug_assert_eq!(None, value24);
                                                                                    debug_assert_eq!(None, value25);
                                                                                    debug_assert_eq!(None, value26);
                                                                                    debug_assert_eq!(None, value27);
                                                                                    debug_assert_eq!(None, value28);
                                                                                    debug_assert_eq!(None, value29);
                                                                                    debug_assert_eq!(None, value30);
                                                                                    debug_assert_eq!(None, value31);
                                                                                    let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None)?;
                                                                                    Ok(n as u64)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug_assert_eq!(None, value8);
                                                                        debug_assert_eq!(None, value9);
                                                                        debug_assert_eq!(None, value10);
                                                                        debug_assert_eq!(None, value11);
                                                                        debug_assert_eq!(None, value12);
                                                                        debug_assert_eq!(None, value13);
                                                                        debug_assert_eq!(None, value14);
                                                                        debug_assert_eq!(None, value15);
                                                                        debug_assert_eq!(None, value16);
                                                                        debug_assert_eq!(None, value17);
                                                                        debug_assert_eq!(None, value18);
                                                                        debug_assert_eq!(None, value19);
                                                                        debug_assert_eq!(None, value20);
                                                                        debug_assert_eq!(None, value21);
                                                                        debug_assert_eq!(None, value22);
                                                                        debug_assert_eq!(None, value23);
                                                                        debug_assert_eq!(None, value24);
                                                                        debug_assert_eq!(None, value25);
                                                                        debug_assert_eq!(None, value26);
                                                                        debug_assert_eq!(None, value27);
                                                                        debug_assert_eq!(None, value28);
                                                                        debug_assert_eq!(None, value29);
                                                                        debug_assert_eq!(None, value30);
                                                                        debug_assert_eq!(None, value31);
                                                                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None)?;
                                                                        Ok(n as u64)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            debug_assert_eq!(None, value7);
                                                            debug_assert_eq!(None, value8);
                                                            debug_assert_eq!(None, value9);
                                                            debug_assert_eq!(None, value10);
                                                            debug_assert_eq!(None, value11);
                                                            debug_assert_eq!(None, value12);
                                                            debug_assert_eq!(None, value13);
                                                            debug_assert_eq!(None, value14);
                                                            debug_assert_eq!(None, value15);
                                                            debug_assert_eq!(None, value16);
                                                            debug_assert_eq!(None, value17);
                                                            debug_assert_eq!(None, value18);
                                                            debug_assert_eq!(None, value19);
                                                            debug_assert_eq!(None, value20);
                                                            debug_assert_eq!(None, value21);
                                                            debug_assert_eq!(None, value22);
                                                            debug_assert_eq!(None, value23);
                                                            debug_assert_eq!(None, value24);
                                                            debug_assert_eq!(None, value25);
                                                            debug_assert_eq!(None, value26);
                                                            debug_assert_eq!(None, value27);
                                                            debug_assert_eq!(None, value28);
                                                            debug_assert_eq!(None, value29);
                                                            debug_assert_eq!(None, value30);
                                                            debug_assert_eq!(None, value31);
                                                            let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None)?;
                                                            Ok(n as u64)
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                debug_assert_eq!(None, value6);
                                                debug_assert_eq!(None, value7);
                                                debug_assert_eq!(None, value8);
                                                debug_assert_eq!(None, value9);
                                                debug_assert_eq!(None, value10);
                                                debug_assert_eq!(None, value11);
                                                debug_assert_eq!(None, value12);
                                                debug_assert_eq!(None, value13);
                                                debug_assert_eq!(None, value14);
                                                debug_assert_eq!(None, value15);
                                                debug_assert_eq!(None, value16);
                                                debug_assert_eq!(None, value17);
                                                debug_assert_eq!(None, value18);
                                                debug_assert_eq!(None, value19);
                                                debug_assert_eq!(None, value20);
                                                debug_assert_eq!(None, value21);
                                                debug_assert_eq!(None, value22);
                                                debug_assert_eq!(None, value23);
                                                debug_assert_eq!(None, value24);
                                                debug_assert_eq!(None, value25);
                                                debug_assert_eq!(None, value26);
                                                debug_assert_eq!(None, value27);
                                                debug_assert_eq!(None, value28);
                                                debug_assert_eq!(None, value29);
                                                debug_assert_eq!(None, value30);
                                                debug_assert_eq!(None, value31);
                                                let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), None, None, None, None, None, None, None, None, None, None, None)?;
                                                Ok(n as u64)
                                            }
                                        }
                                    }
                                }
                                None => {
                                    debug_assert_eq!(None, value5);
                                    debug_assert_eq!(None, value6);
                                    debug_assert_eq!(None, value7);
                                    debug_assert_eq!(None, value8);
                                    debug_assert_eq!(None, value9);
                                    debug_assert_eq!(None, value10);
                                    debug_assert_eq!(None, value11);
                                    debug_assert_eq!(None, value12);
                                    debug_assert_eq!(None, value13);
                                    debug_assert_eq!(None, value14);
                                    debug_assert_eq!(None, value15);
                                    debug_assert_eq!(None, value16);
                                    debug_assert_eq!(None, value17);
                                    debug_assert_eq!(None, value18);
                                    debug_assert_eq!(None, value19);
                                    debug_assert_eq!(None, value20);
                                    debug_assert_eq!(None, value21);
                                    debug_assert_eq!(None, value22);
                                    debug_assert_eq!(None, value23);
                                    debug_assert_eq!(None, value24);
                                    debug_assert_eq!(None, value25);
                                    debug_assert_eq!(None, value26);
                                    debug_assert_eq!(None, value27);
                                    debug_assert_eq!(None, value28);
                                    debug_assert_eq!(None, value29);
                                    debug_assert_eq!(None, value30);
                                    debug_assert_eq!(None, value31);
                                    let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), Some(value3), None, None, None, None, None, None, None, None, None, None, None, None)?;
                                    Ok(n as u64)
                                }
                            }
                        }
                    }
                    None => {
                        debug_assert_eq!(None, value4);
                        debug_assert_eq!(None, value5);
                        debug_assert_eq!(None, value6);
                        debug_assert_eq!(None, value7);
                        debug_assert_eq!(None, value8);
                        debug_assert_eq!(None, value9);
                        debug_assert_eq!(None, value10);
                        debug_assert_eq!(None, value11);
                        debug_assert_eq!(None, value12);
                        debug_assert_eq!(None, value13);
                        debug_assert_eq!(None, value14);
                        debug_assert_eq!(None, value15);
                        debug_assert_eq!(None, value16);
                        debug_assert_eq!(None, value17);
                        debug_assert_eq!(None, value18);
                        debug_assert_eq!(None, value19);
                        debug_assert_eq!(None, value20);
                        debug_assert_eq!(None, value21);
                        debug_assert_eq!(None, value22);
                        debug_assert_eq!(None, value23);
                        debug_assert_eq!(None, value24);
                        debug_assert_eq!(None, value25);
                        debug_assert_eq!(None, value26);
                        debug_assert_eq!(None, value27);
                        debug_assert_eq!(None, value28);
                        debug_assert_eq!(None, value29);
                        debug_assert_eq!(None, value30);
                        debug_assert_eq!(None, value31);
                        let n = chinese_digit_10000_ten_thousand_compat(value, Some(value2), None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                        Ok(n as u64)
                    }
                }
            }
        }
        None => {
            debug_assert_eq!(None, value3);
            debug_assert_eq!(None, value4);
            debug_assert_eq!(None, value5);
            debug_assert_eq!(None, value6);
            debug_assert_eq!(None, value7);
            debug_assert_eq!(None, value8);
            debug_assert_eq!(None, value9);
            debug_assert_eq!(None, value10);
            debug_assert_eq!(None, value11);
            debug_assert_eq!(None, value12);
            debug_assert_eq!(None, value13);
            debug_assert_eq!(None, value14);
            debug_assert_eq!(None, value15);
            debug_assert_eq!(None, value16);
            debug_assert_eq!(None, value17);
            debug_assert_eq!(None, value18);
            debug_assert_eq!(None, value19);
            debug_assert_eq!(None, value20);
            debug_assert_eq!(None, value21);
            debug_assert_eq!(None, value22);
            debug_assert_eq!(None, value23);
            debug_assert_eq!(None, value24);
            debug_assert_eq!(None, value25);
            debug_assert_eq!(None, value26);
            debug_assert_eq!(None, value27);
            debug_assert_eq!(None, value28);
            debug_assert_eq!(None, value29);
            debug_assert_eq!(None, value30);
            debug_assert_eq!(None, value31);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(n as u64)
        }
    }
}

mod crazy_functions;

pub(crate) use self::crazy_functions::*;