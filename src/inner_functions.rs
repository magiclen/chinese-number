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
            let lsd = chinese_digit_1(value2).map_err(|err| 1u8)?;

            Ok(10 + lsd)
        } else {
            Ok(10)
        }
    } else {
        let msd = chinese_digit_1(value)?;

        if msd == 0 {
            if let Some(_) = value2 {
                Err(1)
            } else if let Some(_) = value3 {
                Err(2)
            } else {
                Ok(0)
            }
        } else {
            if let Some(value2) = value2 {
                if !CHINESE_NUMBERS_CHARS[10].contains(&value2) {
                    Err(1)
                } else {
                    if let Some(value3) = value3 {
                        let lsd = chinese_digit_1(value3).map_err(|err| 2u8)?;

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
                        let lsd = chinese_digit_1(value4).map_err(|err| 3u8)?;

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
    match chinese_digit_10(value, value2, value3) {
        Ok(number) => Ok(number as u16),
        Err(_) => match value2 {
            Some(value2) => match chinese_digit_100(value, value2, value3, value4, value5) {
                Ok(number) => Ok(number),
                Err(err) => Err(err)
            }
            None => chinese_digit_1(value).map(|v| v as u16)
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
                                let rds = chinese_digit_10(value4, Some(value5), value6).map_err(|err| 3u8)?;

                                Ok(msd as u16 * 1000 + rds as u16)
                            }
                        } else {
                            if let Some(_) = value6 {
                                Err(5)
                            } else if let Some(_) = value7 {
                                Err(6)
                            } else {
                                let rds = chinese_digit_1(value4).map_err(|err| 3u8)?;

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

