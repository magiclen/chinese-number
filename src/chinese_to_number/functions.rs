use alloc::vec::Vec;
use core::cmp::Ordering;

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::float::FloatCore;

use crate::{
    ChineseCountMethod, ChineseExponent, ChineseNumber, ChineseSign, ChineseToNumberError,
};

#[inline]
pub(crate) fn to_chars_vec<S: AsRef<str>>(s: S) -> Vec<char> {
    s.as_ref().chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_exp_base(
    method: ChineseCountMethod,
    exp: ChineseExponent,
) -> Result<u128, ChineseToNumberError> {
    match method {
        ChineseCountMethod::Low => match exp {
            ChineseExponent::個 => Ok(1),
            _ => {
                debug_assert!(exp > ChineseExponent::個);

                Ok(10u128.pow((exp.ordinal() - ChineseExponent::個.ordinal()) as u32))
            },
        },
        ChineseCountMethod::TenThousand => match exp {
            ChineseExponent::個 => Ok(1),
            ChineseExponent::十 => Ok(10),
            ChineseExponent::百 => Ok(100),
            ChineseExponent::千 => Ok(1000),
            _ => {
                debug_assert!(exp > ChineseExponent::千);

                1_0000u128
                    .checked_pow((exp.ordinal() - ChineseExponent::千.ordinal()) as u32)
                    .ok_or(ChineseToNumberError::Overflow)
            },
        },
        ChineseCountMethod::Middle => match exp {
            ChineseExponent::個 => Ok(1),
            ChineseExponent::十 => Ok(10),
            ChineseExponent::百 => Ok(100),
            ChineseExponent::千 => Ok(1000),
            ChineseExponent::萬 => Ok(1_0000),
            _ => {
                debug_assert!(exp > ChineseExponent::萬);

                1_0000_0000u128
                    .checked_pow((exp.ordinal() - ChineseExponent::萬.ordinal()) as u32)
                    .ok_or(ChineseToNumberError::Overflow)
            },
        },
        ChineseCountMethod::High => match exp {
            ChineseExponent::個 => Ok(1),
            ChineseExponent::十 => Ok(10),
            ChineseExponent::百 => Ok(100),
            ChineseExponent::千 => Ok(1000),
            _ => {
                debug_assert!(exp > ChineseExponent::千);

                let mut w = 1_0000u128;

                for _ in 0..exp.ordinal() - ChineseExponent::萬.ordinal() {
                    w = w.checked_pow(2).ok_or(ChineseToNumberError::Overflow)?;
                }

                Ok(w)
            },
        },
    }
}

pub(crate) fn chinese_to_unsigned_integer_unit(
    method: ChineseCountMethod,
    chars: &[char],
    mut pointer: usize,
    level: ChineseExponent,
) -> Result<(u128, Option<(usize, ChineseExponent)>), ChineseToNumberError> {
    debug_assert!(!chars.is_empty() && pointer < chars.len());

    let base = get_exp_base(method, level)?;

    let (n, exp) = match ChineseNumber::from_char(chars[pointer]) {
        Some(n) if n == ChineseNumber::十 => {
            if pointer == 0 {
                return Ok((
                    (n.ordinal() as u128)
                        .checked_mul(base)
                        .ok_or(ChineseToNumberError::Overflow)?,
                    None,
                ));
            }

            (0u128, ChineseExponent::十)
        },
        Some(n) => {
            if pointer == 0 {
                return Ok((
                    (n.ordinal() as u128)
                        .checked_mul(base)
                        .ok_or(ChineseToNumberError::Overflow)?,
                    None,
                ));
            }

            pointer -= 1;

            loop {
                match ChineseExponent::from_char(chars[pointer]) {
                    Some(exp) if exp > ChineseExponent::個 => {
                        if pointer == 0 {
                            if exp == ChineseExponent::十 {
                                return Ok((
                                    ((10 + n.ordinal()) as u128)
                                        .checked_mul(base)
                                        .ok_or(ChineseToNumberError::Overflow)?,
                                    None,
                                ));
                            } else {
                                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                                    char_index: pointer,
                                });
                            }
                        }

                        break (n.ordinal() as u128, exp);
                    },
                    _ => match ChineseNumber::from_char(chars[pointer]) {
                        Some(ChineseNumber::零) => {
                            if pointer == 0 {
                                return Ok((
                                    (n.ordinal() as u128)
                                        .checked_mul(base)
                                        .ok_or(ChineseToNumberError::Overflow)?,
                                    None,
                                ));
                            }

                            pointer -= 1;

                            continue;
                        },
                        _ => {
                            return Err(ChineseToNumberError::ChineseNumberIncorrect {
                                char_index: pointer,
                            })
                        },
                    },
                }
            }
        },
        _ => {
            if pointer == 0 {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: pointer
                });
            }

            match ChineseExponent::from_char(chars[pointer]) {
                Some(exp) if exp < level => (0u128, exp),
                _ => {
                    return Err(ChineseToNumberError::ChineseNumberIncorrect {
                        char_index: pointer,
                    })
                },
            }
        },
    };

    let mut sum = n;
    let mut next = Some((pointer, exp));

    while let Some((pointer, exp)) = next {
        match exp.cmp(&level) {
            Ordering::Greater => {
                break;
            },
            Ordering::Less => {
                let result = chinese_to_unsigned_integer_unit(method, chars, pointer - 1, exp)?;

                sum = sum.checked_add(result.0).ok_or(ChineseToNumberError::Overflow)?;

                next = result.1;
            },
            Ordering::Equal => {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: pointer
                });
            },
        }
    }

    sum = sum.checked_mul(base).ok_or(ChineseToNumberError::Overflow)?;

    Ok((sum, next))
}

pub(crate) fn chinese_to_unsigned_integer(
    method: ChineseCountMethod,
    chars: &[char],
) -> Result<u128, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let mut pointer = length - 1;

    let mut exp = match ChineseExponent::from_char(chars[pointer]) {
        Some(exp) if exp > ChineseExponent::個 => {
            if pointer == 0 {
                if exp == ChineseExponent::十 {
                    return Ok(10);
                } else {
                    return Err(ChineseToNumberError::ChineseNumberIncorrect {
                        char_index: pointer,
                    });
                }
            }

            exp
        },
        _ => {
            pointer += 1;
            ChineseExponent::個
        },
    };

    let mut sum = 0u128;

    loop {
        let result = chinese_to_unsigned_integer_unit(method, chars, pointer - 1, exp)?;

        sum = sum.checked_add(result.0).ok_or(ChineseToNumberError::Overflow)?;

        if let Some((p, e)) = result.1 {
            pointer = p;
            exp = e;

            continue;
        }

        break;
    }

    Ok(sum)
}

pub(crate) fn chinese_to_signed_integer(
    method: ChineseCountMethod,
    chars: &[char],
) -> Result<i128, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let c = chars[0];

    let (sign, offset) = match ChineseSign::from_char(c) {
        Some(sign) => (sign, 1),
        None => (ChineseSign::正, 0),
    };

    let uint = match chinese_to_unsigned_integer(method, &chars[offset..]) {
        Ok(n) => n,
        Err(error) => {
            return match error {
                ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: index,
                } => Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: index + offset,
                }),
                ChineseToNumberError::Overflow if sign == ChineseSign::負 => {
                    Err(ChineseToNumberError::Underflow)
                },
                _ => Err(error),
            };
        },
    };

    match sign {
        ChineseSign::正 => {
            if uint > i128::MAX as u128 {
                return Err(ChineseToNumberError::Overflow);
            }

            Ok(uint as i128)
        },
        ChineseSign::負 => {
            let m = i128::MAX as u128 + 1;

            if uint > m {
                return Err(ChineseToNumberError::Underflow);
            }

            if uint == m {
                Ok(i128::MIN)
            } else {
                Ok(-(uint as i128))
            }
        },
    }
}

// TODO f64

fn get_exp_base_f64(method: ChineseCountMethod, exp: ChineseExponent) -> f64 {
    match method {
        ChineseCountMethod::Low => match exp {
            ChineseExponent::個 => 1f64,
            _ => {
                debug_assert!(exp > ChineseExponent::個);
                10f64.powi((exp.ordinal() - ChineseExponent::個.ordinal()) as i32)
            },
        },
        ChineseCountMethod::TenThousand => match exp {
            ChineseExponent::個 => 1f64,
            ChineseExponent::十 => 10f64,
            ChineseExponent::百 => 100f64,
            ChineseExponent::千 => 1000f64,
            _ => {
                debug_assert!(exp > ChineseExponent::千);

                1_0000f64.powi((exp.ordinal() - ChineseExponent::千.ordinal()) as i32)
            },
        },
        ChineseCountMethod::Middle => match exp {
            ChineseExponent::個 => 1f64,
            ChineseExponent::十 => 10f64,
            ChineseExponent::百 => 100f64,
            ChineseExponent::千 => 1000f64,
            ChineseExponent::萬 => 1_0000f64,
            _ => {
                debug_assert!(exp > ChineseExponent::萬);

                1_0000_0000f64.powi((exp.ordinal() - ChineseExponent::萬.ordinal()) as i32)
            },
        },
        ChineseCountMethod::High => match exp {
            ChineseExponent::個 => 1f64,
            ChineseExponent::十 => 10f64,
            ChineseExponent::百 => 100f64,
            ChineseExponent::千 => 1000f64,
            _ => {
                debug_assert!(exp > ChineseExponent::千);

                let mut w = 1_0000f64;

                for _ in 0..exp.ordinal() - ChineseExponent::萬.ordinal() {
                    w *= w;
                }

                w
            },
        },
    }
}

pub(crate) fn chinese_to_f64_unit(
    method: ChineseCountMethod,
    chars: &[char],
    mut pointer: usize,
    level: ChineseExponent,
) -> Result<(f64, Option<(usize, ChineseExponent)>), ChineseToNumberError> {
    debug_assert!(!chars.is_empty() && pointer < chars.len());

    let base = get_exp_base_f64(method, level);

    let (n, exp) = match ChineseNumber::from_char(chars[pointer]) {
        Some(n) if n == ChineseNumber::十 => {
            if pointer == 0 {
                return Ok(((n.ordinal() as f64) * base, None));
            }

            (0f64, ChineseExponent::十)
        },
        Some(n) => {
            if pointer == 0 {
                return Ok(((n.ordinal() as f64) * base, None));
            }

            pointer -= 1;

            loop {
                match ChineseExponent::from_char(chars[pointer]) {
                    Some(exp) if exp > ChineseExponent::個 => {
                        if pointer == 0 {
                            if exp == ChineseExponent::十 {
                                return Ok((((10 + n.ordinal()) as f64) * base, None));
                            } else {
                                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                                    char_index: pointer,
                                });
                            }
                        }

                        break (n.ordinal() as f64, exp);
                    },
                    _ => match ChineseNumber::from_char(chars[pointer]) {
                        Some(ChineseNumber::零) => {
                            if pointer == 0 {
                                return Ok(((n.ordinal() as f64) * base, None));
                            }

                            pointer -= 1;

                            continue;
                        },
                        _ => {
                            return Err(ChineseToNumberError::ChineseNumberIncorrect {
                                char_index: pointer,
                            })
                        },
                    },
                }
            }
        },
        _ => {
            if pointer == 0 {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: pointer
                });
            }

            match ChineseExponent::from_char(chars[pointer]) {
                Some(exp) if exp < level => (0f64, exp),
                _ => {
                    return Err(ChineseToNumberError::ChineseNumberIncorrect {
                        char_index: pointer,
                    })
                },
            }
        },
    };

    let mut sum = n;
    let mut next = Some((pointer, exp));

    while let Some((pointer, exp)) = next {
        match exp.cmp(&level) {
            Ordering::Greater => {
                break;
            },
            Ordering::Less => {
                let result = chinese_to_f64_unit(method, chars, pointer - 1, exp)?;

                sum += result.0;

                next = result.1;
            },
            Ordering::Equal => {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: pointer
                });
            },
        }
    }

    sum *= base;

    Ok((sum, next))
}

pub(crate) fn chinese_to_unsigned_f64(
    method: ChineseCountMethod,
    chars: &[char],
) -> Result<f64, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let mut pointer = length - 1;

    let mut exp = match ChineseExponent::from_char(chars[pointer]) {
        Some(exp) if exp > ChineseExponent::個 => {
            if pointer == 0 {
                if exp == ChineseExponent::十 {
                    return Ok(10f64);
                } else {
                    return Err(ChineseToNumberError::ChineseNumberIncorrect {
                        char_index: pointer,
                    });
                }
            }

            exp
        },
        _ => {
            pointer += 1;
            ChineseExponent::個
        },
    };

    let mut sum = 0f64;

    loop {
        let result = chinese_to_f64_unit(method, chars, pointer - 1, exp)?;

        sum += result.0;

        if let Some((p, e)) = result.1 {
            pointer = p;
            exp = e;

            continue;
        }

        break;
    }

    Ok(sum)
}

pub(crate) fn chinese_to_f64(
    method: ChineseCountMethod,
    chars: &[char],
) -> Result<f64, ChineseToNumberError> {
    let length = chars.len();

    if length == 0 {
        return Err(ChineseToNumberError::ChineseNumberEmpty);
    }

    let mut end = length - 1;
    let mut fraction = 0.00;

    if let Some(ChineseExponent::分) = ChineseExponent::from_char(chars[end]) {
        if end == 0 {
            return Err(ChineseToNumberError::ChineseNumberIncorrect {
                char_index: end
            });
        }

        end -= 1;

        match ChineseNumber::from_char(chars[end]) {
            Some(n) if n != ChineseNumber::十 => {
                fraction += n.ordinal() as f64 * 0.01;

                if end == 0 {
                    return Ok(fraction);
                }

                end -= 1;
            },
            _ => {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: end
                });
            },
        }
    }

    if let Some(ChineseExponent::角) = ChineseExponent::from_char(chars[end]) {
        if end == 0 {
            return Err(ChineseToNumberError::ChineseNumberIncorrect {
                char_index: end
            });
        }

        end -= 1;

        match ChineseNumber::from_char(chars[end]) {
            Some(n) if n != ChineseNumber::十 => {
                fraction += n.ordinal() as f64 * 0.1;

                if end == 0 {
                    return Ok(fraction);
                }

                end -= 1;
            },
            _ => {
                return Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: end
                });
            },
        }
    }

    let c = chars[0];

    let (sign, offset) = match ChineseSign::from_char(c) {
        Some(sign) => (sign, 1),
        None => (ChineseSign::正, 0),
    };

    let f = match chinese_to_unsigned_f64(method, &chars[offset..=end]) {
        Ok(n) => n + fraction,
        Err(error) => {
            return match error {
                ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: index,
                } => Err(ChineseToNumberError::ChineseNumberIncorrect {
                    char_index: index + offset,
                }),
                ChineseToNumberError::Overflow if sign == ChineseSign::負 => {
                    Err(ChineseToNumberError::Underflow)
                },
                _ => Err(error),
            };
        },
    };

    match sign {
        ChineseSign::正 => Ok(f),
        ChineseSign::負 => Ok(-f),
    }
}
