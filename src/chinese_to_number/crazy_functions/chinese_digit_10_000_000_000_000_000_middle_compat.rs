use crate::chinese_characters::*;

use super::super::{
    chinese_digit_100_000_000_middle_compat, chinese_digit_10_000_ten_thousand_compat,
    chinese_digit_10_compat,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn chinese_digit_10_000_000_000_000_000_middle_compat(
    value: char,
    value2: Option<char>,
    value3: Option<char>,
    value4: Option<char>,
    value5: Option<char>,
    value6: Option<char>,
    value7: Option<char>,
    value8: Option<char>,
    value9: Option<char>,
    value10: Option<char>,
    value11: Option<char>,
    value12: Option<char>,
    value13: Option<char>,
    value14: Option<char>,
    value15: Option<char>,
    value16: Option<char>,
    value17: Option<char>,
    value18: Option<char>,
    value19: Option<char>,
    value20: Option<char>,
    value21: Option<char>,
    value22: Option<char>,
    value23: Option<char>,
    value24: Option<char>,
    value25: Option<char>,
    value26: Option<char>,
    value27: Option<char>,
    value28: Option<char>,
    value29: Option<char>,
    value30: Option<char>,
    value31: Option<char>,
    value32: Option<char>,
    value33: Option<char>,
    value34: Option<char>,
    value35: Option<char>,
    value36: Option<char>,
    value37: Option<char>,
    value38: Option<char>,
    value39: Option<char>,
    value40: Option<char>,
    value41: Option<char>,
    value42: Option<char>,
    value43: Option<char>,
    value44: Option<char>,
    value45: Option<char>,
    value46: Option<char>,
    value47: Option<char>,
) -> Result<u128, usize> {
    match value2 {
        Some(value2) => {
            if CHINESE_NUMBERS_CHARS[15].contains(&value2) {
                let msd = chinese_digit_10_compat(value, None, None)?;

                if msd == 0 {
                    Err(0)
                } else if let Some(value3) = value3 {
                    if CHINESE_NUMBERS_CHARS[0].contains(&value3) {
                        if let Some(value4) = value4 {
                            debug_assert_eq!(None, value35);
                            debug_assert_eq!(None, value36);
                            debug_assert_eq!(None, value37);
                            debug_assert_eq!(None, value38);
                            debug_assert_eq!(None, value39);
                            debug_assert_eq!(None, value40);
                            debug_assert_eq!(None, value41);
                            debug_assert_eq!(None, value42);
                            debug_assert_eq!(None, value43);
                            debug_assert_eq!(None, value44);
                            debug_assert_eq!(None, value45);
                            debug_assert_eq!(None, value46);
                            debug_assert_eq!(None, value47);
                            let rds = chinese_digit_100_000_000_middle_compat(
                                value4, value5, value6, value7, value8, value9, value10, value11,
                                value12, value13, value14, value15, value16, value17, value18,
                                value19, value20, value21, value22, value23, value24, value25,
                                value26, value27, value28, value29, value30, value31, value32,
                                value33, value34,
                            )
                            .map_err(|err| err + 3)?;

                            Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
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
                        debug_assert_eq!(None, value40);
                        debug_assert_eq!(None, value41);
                        debug_assert_eq!(None, value42);
                        debug_assert_eq!(None, value43);
                        debug_assert_eq!(None, value44);
                        debug_assert_eq!(None, value45);
                        debug_assert_eq!(None, value46);
                        debug_assert_eq!(None, value47);
                        let rds = chinese_digit_100_000_000_middle_compat(
                            value3, value4, value5, value6, value7, value8, value9, value10,
                            value11, value12, value13, value14, value15, value16, value17, value18,
                            value19, value20, value21, value22, value23, value24, value25, value26,
                            value27, value28, value29, value30, value31, value32, value33,
                        )
                        .map_err(|err| err + 2)?;

                        Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                    }
                } else {
                    Ok(u128::from(msd) * 10_000_000_000_000_000)
                }
            } else {
                match value3 {
                    Some(value3) => {
                        if CHINESE_NUMBERS_CHARS[15].contains(&value3) {
                            let msd = chinese_digit_10_000_ten_thousand_compat(
                                value,
                                Some(value2),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                            )?;

                            if msd == 0 {
                                Err(0)
                            } else if let Some(value4) = value4 {
                                if CHINESE_NUMBERS_CHARS[0].contains(&value4) {
                                    if let Some(value5) = value5 {
                                        debug_assert_eq!(None, value36);
                                        debug_assert_eq!(None, value37);
                                        debug_assert_eq!(None, value38);
                                        debug_assert_eq!(None, value39);
                                        debug_assert_eq!(None, value40);
                                        debug_assert_eq!(None, value41);
                                        debug_assert_eq!(None, value42);
                                        debug_assert_eq!(None, value43);
                                        debug_assert_eq!(None, value44);
                                        debug_assert_eq!(None, value45);
                                        debug_assert_eq!(None, value46);
                                        debug_assert_eq!(None, value47);
                                        let rds = chinese_digit_100_000_000_middle_compat(
                                            value5, value6, value7, value8, value9, value10,
                                            value11, value12, value13, value14, value15, value16,
                                            value17, value18, value19, value20, value21, value22,
                                            value23, value24, value25, value26, value27, value28,
                                            value29, value30, value31, value32, value33, value34,
                                            value35,
                                        )
                                        .map_err(|err| err + 4)?;

                                        Ok(u128::from(msd) * 10_000_000_000_000_000
                                            + u128::from(rds))
                                    } else {
                                        Err(4)
                                    }
                                } else {
                                    debug_assert_eq!(None, value35);
                                    debug_assert_eq!(None, value36);
                                    debug_assert_eq!(None, value37);
                                    debug_assert_eq!(None, value38);
                                    debug_assert_eq!(None, value39);
                                    debug_assert_eq!(None, value40);
                                    debug_assert_eq!(None, value41);
                                    debug_assert_eq!(None, value42);
                                    debug_assert_eq!(None, value43);
                                    debug_assert_eq!(None, value44);
                                    debug_assert_eq!(None, value45);
                                    debug_assert_eq!(None, value46);
                                    debug_assert_eq!(None, value47);
                                    let rds = chinese_digit_100_000_000_middle_compat(
                                        value4, value5, value6, value7, value8, value9, value10,
                                        value11, value12, value13, value14, value15, value16,
                                        value17, value18, value19, value20, value21, value22,
                                        value23, value24, value25, value26, value27, value28,
                                        value29, value30, value31, value32, value33, value34,
                                    )
                                    .map_err(|err| err + 3)?;

                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                }
                            } else {
                                Ok(u128::from(msd) * 10_000_000_000_000_000)
                            }
                        } else {
                            match value4 {
                                Some(value4) => {
                                    if CHINESE_NUMBERS_CHARS[15].contains(&value4) {
                                        let msd = chinese_digit_10_000_ten_thousand_compat(
                                            value,
                                            Some(value2),
                                            Some(value3),
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                        )?;

                                        if msd == 0 {
                                            Err(0)
                                        } else if let Some(value5) = value5 {
                                            if CHINESE_NUMBERS_CHARS[0].contains(&value5) {
                                                if let Some(value6) = value6 {
                                                    debug_assert_eq!(None, value37);
                                                    debug_assert_eq!(None, value38);
                                                    debug_assert_eq!(None, value39);
                                                    debug_assert_eq!(None, value40);
                                                    debug_assert_eq!(None, value41);
                                                    debug_assert_eq!(None, value42);
                                                    debug_assert_eq!(None, value43);
                                                    debug_assert_eq!(None, value44);
                                                    debug_assert_eq!(None, value45);
                                                    debug_assert_eq!(None, value46);
                                                    debug_assert_eq!(None, value47);
                                                    let rds =
                                                        chinese_digit_100_000_000_middle_compat(
                                                            value6, value7, value8, value9,
                                                            value10, value11, value12, value13,
                                                            value14, value15, value16, value17,
                                                            value18, value19, value20, value21,
                                                            value22, value23, value24, value25,
                                                            value26, value27, value28, value29,
                                                            value30, value31, value32, value33,
                                                            value34, value35, value36,
                                                        )
                                                        .map_err(|err| err + 5)?;

                                                    Ok(u128::from(msd) * 10_000_000_000_000_000
                                                        + u128::from(rds))
                                                } else {
                                                    Err(5)
                                                }
                                            } else {
                                                debug_assert_eq!(None, value36);
                                                debug_assert_eq!(None, value37);
                                                debug_assert_eq!(None, value38);
                                                debug_assert_eq!(None, value39);
                                                debug_assert_eq!(None, value40);
                                                debug_assert_eq!(None, value41);
                                                debug_assert_eq!(None, value42);
                                                debug_assert_eq!(None, value43);
                                                debug_assert_eq!(None, value44);
                                                debug_assert_eq!(None, value45);
                                                debug_assert_eq!(None, value46);
                                                debug_assert_eq!(None, value47);
                                                let rds = chinese_digit_100_000_000_middle_compat(
                                                    value5, value6, value7, value8, value9,
                                                    value10, value11, value12, value13, value14,
                                                    value15, value16, value17, value18, value19,
                                                    value20, value21, value22, value23, value24,
                                                    value25, value26, value27, value28, value29,
                                                    value30, value31, value32, value33, value34,
                                                    value35,
                                                )
                                                .map_err(|err| err + 4)?;

                                                Ok(u128::from(msd) * 10_000_000_000_000_000
                                                    + u128::from(rds))
                                            }
                                        } else {
                                            Ok(u128::from(msd) * 10_000_000_000_000_000)
                                        }
                                    } else {
                                        match value5 {
                                            Some(value5) => {
                                                if CHINESE_NUMBERS_CHARS[15].contains(&value5) {
                                                    let msd =
                                                        chinese_digit_10_000_ten_thousand_compat(
                                                            value,
                                                            Some(value2),
                                                            Some(value3),
                                                            Some(value4),
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                            None,
                                                        )?;

                                                    if msd == 0 {
                                                        Err(0)
                                                    } else if let Some(value6) = value6 {
                                                        if CHINESE_NUMBERS_CHARS[0]
                                                            .contains(&value6)
                                                        {
                                                            if let Some(value7) = value7 {
                                                                debug_assert_eq!(None, value38);
                                                                debug_assert_eq!(None, value39);
                                                                debug_assert_eq!(None, value40);
                                                                debug_assert_eq!(None, value41);
                                                                debug_assert_eq!(None, value42);
                                                                debug_assert_eq!(None, value43);
                                                                debug_assert_eq!(None, value44);
                                                                debug_assert_eq!(None, value45);
                                                                debug_assert_eq!(None, value46);
                                                                debug_assert_eq!(None, value47);
                                                                let rds = chinese_digit_100_000_000_middle_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37).map_err(|err| err + 6)?;

                                                                Ok(u128::from(msd)
                                                                    * 10_000_000_000_000_000
                                                                    + u128::from(rds))
                                                            } else {
                                                                Err(6)
                                                            }
                                                        } else {
                                                            debug_assert_eq!(None, value37);
                                                            debug_assert_eq!(None, value38);
                                                            debug_assert_eq!(None, value39);
                                                            debug_assert_eq!(None, value40);
                                                            debug_assert_eq!(None, value41);
                                                            debug_assert_eq!(None, value42);
                                                            debug_assert_eq!(None, value43);
                                                            debug_assert_eq!(None, value44);
                                                            debug_assert_eq!(None, value45);
                                                            debug_assert_eq!(None, value46);
                                                            debug_assert_eq!(None, value47);
                                                            let rds = chinese_digit_100_000_000_middle_compat(value6, value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36).map_err(|err| err + 5)?;

                                                            Ok(u128::from(msd)
                                                                * 10_000_000_000_000_000
                                                                + u128::from(rds))
                                                        }
                                                    } else {
                                                        Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                    }
                                                } else {
                                                    match value6 {
                                                        Some(value6) => {
                                                            if CHINESE_NUMBERS_CHARS[15]
                                                                .contains(&value6)
                                                            {
                                                                let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None)?;

                                                                if msd == 0 {
                                                                    Err(0)
                                                                } else if let Some(value7) = value7
                                                                {
                                                                    if CHINESE_NUMBERS_CHARS[0]
                                                                        .contains(&value7)
                                                                    {
                                                                        if let Some(value8) = value8
                                                                        {
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value39
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value40
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value41
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value42
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value43
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value44
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value45
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value46
                                                                            );
                                                                            debug_assert_eq!(
                                                                                None,
                                                                                value47
                                                                            );
                                                                            let rds = chinese_digit_100_000_000_middle_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38).map_err(|err| err + 7)?;

                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                        } else {
                                                                            Err(7)
                                                                        }
                                                                    } else {
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value38
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value39
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value40
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value41
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value42
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value43
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value44
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value45
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value46
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value47
                                                                        );
                                                                        let rds = chinese_digit_100_000_000_middle_compat(value7, value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37).map_err(|err| err + 6)?;

                                                                        Ok(u128::from(msd)
                                                                            * 10_000_000_000_000_000
                                                                            + u128::from(rds))
                                                                    }
                                                                } else {
                                                                    Ok(u128::from(msd)
                                                                        * 10_000_000_000_000_000)
                                                                }
                                                            } else {
                                                                match value7 {
                                                                    Some(value7) => {
                                                                        if CHINESE_NUMBERS_CHARS[15]
                                                                            .contains(&value7)
                                                                        {
                                                                            let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None)?;

                                                                            if msd == 0 {
                                                                                Err(0)
                                                                            } else if let Some(
                                                                                value8,
                                                                            ) = value8
                                                                            {
                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value8) {
                                                                                    if let Some(value9) = value9 {
                                                                                        debug_assert_eq!(None, value40);
                                                                                        debug_assert_eq!(None, value41);
                                                                                        debug_assert_eq!(None, value42);
                                                                                        debug_assert_eq!(None, value43);
                                                                                        debug_assert_eq!(None, value44);
                                                                                        debug_assert_eq!(None, value45);
                                                                                        debug_assert_eq!(None, value46);
                                                                                        debug_assert_eq!(None, value47);
                                                                                        let rds = chinese_digit_100_000_000_middle_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39).map_err(|err| err + 8)?;

                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                    } else {
                                                                                        Err(8)
                                                                                    }
                                                                                } else {
                                                                                    debug_assert_eq!(None, value39);
                                                                                    debug_assert_eq!(None, value40);
                                                                                    debug_assert_eq!(None, value41);
                                                                                    debug_assert_eq!(None, value42);
                                                                                    debug_assert_eq!(None, value43);
                                                                                    debug_assert_eq!(None, value44);
                                                                                    debug_assert_eq!(None, value45);
                                                                                    debug_assert_eq!(None, value46);
                                                                                    debug_assert_eq!(None, value47);
                                                                                    let rds = chinese_digit_100_000_000_middle_compat(value8, value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38).map_err(|err| err + 7)?;

                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                }
                                                                            } else {
                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                            }
                                                                        } else {
                                                                            match value8 {
                                                                                Some(value8) => {
                                                                                    if CHINESE_NUMBERS_CHARS[15].contains(&value8) {
                                                                                        let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None)?;

                                                                                        if msd == 0 {
                                                                                            Err(0)
                                                                                        } else if let Some(value9) = value9 {
                                                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value9) {
                                                                                                if let Some(value10) = value10 {
                                                                                                    debug_assert_eq!(None, value41);
                                                                                                    debug_assert_eq!(None, value42);
                                                                                                    debug_assert_eq!(None, value43);
                                                                                                    debug_assert_eq!(None, value44);
                                                                                                    debug_assert_eq!(None, value45);
                                                                                                    debug_assert_eq!(None, value46);
                                                                                                    debug_assert_eq!(None, value47);
                                                                                                    let rds = chinese_digit_100_000_000_middle_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40).map_err(|err| err + 9)?;

                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                } else {
                                                                                                    Err(9)
                                                                                                }
                                                                                            } else {
                                                                                                debug_assert_eq!(None, value40);
                                                                                                debug_assert_eq!(None, value41);
                                                                                                debug_assert_eq!(None, value42);
                                                                                                debug_assert_eq!(None, value43);
                                                                                                debug_assert_eq!(None, value44);
                                                                                                debug_assert_eq!(None, value45);
                                                                                                debug_assert_eq!(None, value46);
                                                                                                debug_assert_eq!(None, value47);
                                                                                                let rds = chinese_digit_100_000_000_middle_compat(value9, value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39).map_err(|err| err + 8)?;

                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                            }
                                                                                        } else {
                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                        }
                                                                                    } else {
                                                                                        match value9 {
                                                                                            Some(value9) => {
                                                                                                if CHINESE_NUMBERS_CHARS[15].contains(&value9) {
                                                                                                    let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), None, None, None, None, None, None, None)?;

                                                                                                    if msd == 0 {
                                                                                                        Err(0)
                                                                                                    } else if let Some(value10) = value10 {
                                                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value10) {
                                                                                                            if let Some(value11) = value11 {
                                                                                                                debug_assert_eq!(None, value42);
                                                                                                                debug_assert_eq!(None, value43);
                                                                                                                debug_assert_eq!(None, value44);
                                                                                                                debug_assert_eq!(None, value45);
                                                                                                                debug_assert_eq!(None, value46);
                                                                                                                debug_assert_eq!(None, value47);
                                                                                                                let rds = chinese_digit_100_000_000_middle_compat(value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41).map_err(|err| err + 10)?;

                                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                            } else {
                                                                                                                Err(10)
                                                                                                            }
                                                                                                        } else {
                                                                                                            debug_assert_eq!(None, value41);
                                                                                                            debug_assert_eq!(None, value42);
                                                                                                            debug_assert_eq!(None, value43);
                                                                                                            debug_assert_eq!(None, value44);
                                                                                                            debug_assert_eq!(None, value45);
                                                                                                            debug_assert_eq!(None, value46);
                                                                                                            debug_assert_eq!(None, value47);
                                                                                                            let rds = chinese_digit_100_000_000_middle_compat(value10, value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40).map_err(|err| err + 9)?;

                                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                        }
                                                                                                    } else {
                                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                    }
                                                                                                } else {
                                                                                                    match value10 {
                                                                                                        Some(value10) => {
                                                                                                            if CHINESE_NUMBERS_CHARS[15].contains(&value10) {
                                                                                                                let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), None, None, None, None, None, None)?;

                                                                                                                if msd == 0 {
                                                                                                                    Err(0)
                                                                                                                } else if let Some(value11) = value11 {
                                                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value11) {
                                                                                                                        if let Some(value12) = value12 {
                                                                                                                            debug_assert_eq!(None, value43);
                                                                                                                            debug_assert_eq!(None, value44);
                                                                                                                            debug_assert_eq!(None, value45);
                                                                                                                            debug_assert_eq!(None, value46);
                                                                                                                            debug_assert_eq!(None, value47);
                                                                                                                            let rds = chinese_digit_100_000_000_middle_compat(value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42).map_err(|err| err + 11)?;

                                                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                        } else {
                                                                                                                            Err(11)
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        debug_assert_eq!(None, value42);
                                                                                                                        debug_assert_eq!(None, value43);
                                                                                                                        debug_assert_eq!(None, value44);
                                                                                                                        debug_assert_eq!(None, value45);
                                                                                                                        debug_assert_eq!(None, value46);
                                                                                                                        debug_assert_eq!(None, value47);
                                                                                                                        let rds = chinese_digit_100_000_000_middle_compat(value11, value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41).map_err(|err| err + 10)?;

                                                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                                }
                                                                                                            } else {
                                                                                                                match value11 {
                                                                                                                    Some(value11) => {
                                                                                                                        if CHINESE_NUMBERS_CHARS[15].contains(&value11) {
                                                                                                                            let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), None, None, None, None, None)?;

                                                                                                                            if msd == 0 {
                                                                                                                                Err(0)
                                                                                                                            } else if let Some(value12) = value12 {
                                                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value12) {
                                                                                                                                    if let Some(value13) = value13 {
                                                                                                                                        debug_assert_eq!(None, value44);
                                                                                                                                        debug_assert_eq!(None, value45);
                                                                                                                                        debug_assert_eq!(None, value46);
                                                                                                                                        debug_assert_eq!(None, value47);
                                                                                                                                        let rds = chinese_digit_100_000_000_middle_compat(value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43).map_err(|err| err + 12)?;

                                                                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                    } else {
                                                                                                                                        Err(12)
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    debug_assert_eq!(None, value43);
                                                                                                                                    debug_assert_eq!(None, value44);
                                                                                                                                    debug_assert_eq!(None, value45);
                                                                                                                                    debug_assert_eq!(None, value46);
                                                                                                                                    debug_assert_eq!(None, value47);
                                                                                                                                    let rds = chinese_digit_100_000_000_middle_compat(value12, value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42).map_err(|err| err + 11)?;

                                                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            match value12 {
                                                                                                                                Some(value12) => {
                                                                                                                                    if CHINESE_NUMBERS_CHARS[15].contains(&value12) {
                                                                                                                                        let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), None, None, None, None)?;

                                                                                                                                        if msd == 0 {
                                                                                                                                            Err(0)
                                                                                                                                        } else if let Some(value13) = value13 {
                                                                                                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value13) {
                                                                                                                                                if let Some(value14) = value14 {
                                                                                                                                                    debug_assert_eq!(None, value45);
                                                                                                                                                    debug_assert_eq!(None, value46);
                                                                                                                                                    debug_assert_eq!(None, value47);
                                                                                                                                                    let rds = chinese_digit_100_000_000_middle_compat(value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44).map_err(|err| err + 13)?;

                                                                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                } else {
                                                                                                                                                    Err(13)
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                debug_assert_eq!(None, value44);
                                                                                                                                                debug_assert_eq!(None, value45);
                                                                                                                                                debug_assert_eq!(None, value46);
                                                                                                                                                debug_assert_eq!(None, value47);
                                                                                                                                                let rds = chinese_digit_100_000_000_middle_compat(value13, value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43).map_err(|err| err + 12)?;

                                                                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        match value13 {
                                                                                                                                            Some(value13) => {
                                                                                                                                                if CHINESE_NUMBERS_CHARS[15].contains(&value13) {
                                                                                                                                                    let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), None, None, None)?;

                                                                                                                                                    if msd == 0 {
                                                                                                                                                        Err(0)
                                                                                                                                                    } else if let Some(value14) = value14 {
                                                                                                                                                        if CHINESE_NUMBERS_CHARS[0].contains(&value14) {
                                                                                                                                                            if let Some(value15) = value15 {
                                                                                                                                                                debug_assert_eq!(None, value46);
                                                                                                                                                                debug_assert_eq!(None, value47);
                                                                                                                                                                let rds = chinese_digit_100_000_000_middle_compat(value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45).map_err(|err| err + 14)?;

                                                                                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                            } else {
                                                                                                                                                                Err(14)
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            debug_assert_eq!(None, value45);
                                                                                                                                                            debug_assert_eq!(None, value46);
                                                                                                                                                            debug_assert_eq!(None, value47);
                                                                                                                                                            let rds = chinese_digit_100_000_000_middle_compat(value14, value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44).map_err(|err| err + 13)?;

                                                                                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    match value14 {
                                                                                                                                                        Some(value14) => {
                                                                                                                                                            if CHINESE_NUMBERS_CHARS[15].contains(&value14) {
                                                                                                                                                                let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), None, None)?;

                                                                                                                                                                if msd == 0 {
                                                                                                                                                                    Err(0)
                                                                                                                                                                } else if let Some(value15) = value15 {
                                                                                                                                                                    if CHINESE_NUMBERS_CHARS[0].contains(&value15) {
                                                                                                                                                                        if let Some(value16) = value16 {
                                                                                                                                                                            debug_assert_eq!(None, value47);
                                                                                                                                                                            let rds = chinese_digit_100_000_000_middle_compat(value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45, value46).map_err(|err| err + 15)?;

                                                                                                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                                        } else {
                                                                                                                                                                            Err(15)
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        debug_assert_eq!(None, value46);
                                                                                                                                                                        debug_assert_eq!(None, value47);
                                                                                                                                                                        let rds = chinese_digit_100_000_000_middle_compat(value15, value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45).map_err(|err| err + 14)?;

                                                                                                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                match value15 {
                                                                                                                                                                    Some(value15) => {
                                                                                                                                                                        if CHINESE_NUMBERS_CHARS[15].contains(&value15) {
                                                                                                                                                                            let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), None)?;

                                                                                                                                                                            if msd == 0 {
                                                                                                                                                                                Err(0)
                                                                                                                                                                            } else if let Some(value16) = value16 {
                                                                                                                                                                                if CHINESE_NUMBERS_CHARS[0].contains(&value16) {
                                                                                                                                                                                    if let Some(value17) = value17 {
                                                                                                                                                                                        let rds = chinese_digit_100_000_000_middle_compat(value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45, value46, value47).map_err(|err| err + 16)?;

                                                                                                                                                                                        Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                                                    } else {
                                                                                                                                                                                        Err(16)
                                                                                                                                                                                    }
                                                                                                                                                                                } else {
                                                                                                                                                                                    debug_assert_eq!(None, value47);
                                                                                                                                                                                    let rds = chinese_digit_100_000_000_middle_compat(value16, value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45, value46).map_err(|err| err + 15)?;

                                                                                                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                                                }
                                                                                                                                                                            } else {
                                                                                                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000)
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            match value16 {
                                                                                                                                                                                Some(value16) => {
                                                                                                                                                                                    if CHINESE_NUMBERS_CHARS[15].contains(&value16) {
                                                                                                                                                                                        let msd = chinese_digit_10_000_ten_thousand_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), Some(value15))?;

                                                                                                                                                                                        if msd == 0 {
                                                                                                                                                                                            Err(0)
                                                                                                                                                                                        } else if let Some(value17) = value17 {
                                                                                                                                                                                            if CHINESE_NUMBERS_CHARS[0].contains(&value17) {
                                                                                                                                                                                                if let Some(value18) = value18 {
                                                                                                                                                                                                    let rds = chinese_digit_100_000_000_middle_compat(value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45, value46, value47, None).map_err(|err| err + 17)?;

                                                                                                                                                                                                    Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    Err(17)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                let rds = chinese_digit_100_000_000_middle_compat(value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31, value32, value33, value34, value35, value36, value37, value38, value39, value40, value41, value42, value43, value44, value45, value46, value47).map_err(|err| err + 16)?;

                                                                                                                                                                                                Ok(u128::from(msd) * 10_000_000_000_000_000 + u128::from(rds))
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            Ok(u128::from(msd) * 10_000_000_000_000_000)
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
                                                                                                                                                                                        debug_assert_eq!(None, value40);
                                                                                                                                                                                        debug_assert_eq!(None, value41);
                                                                                                                                                                                        debug_assert_eq!(None, value42);
                                                                                                                                                                                        debug_assert_eq!(None, value43);
                                                                                                                                                                                        debug_assert_eq!(None, value44);
                                                                                                                                                                                        debug_assert_eq!(None, value45);
                                                                                                                                                                                        debug_assert_eq!(None, value46);
                                                                                                                                                                                        debug_assert_eq!(None, value47);
                                                                                                                                                                                        let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), Some(value15), Some(value16), value17, value18, value19, value20, value21, value22, value23, value24, value25, value26, value27, value28, value29, value30, value31)?;
                                                                                                                                                                                        Ok(u128::from(n))
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
                                                                                                                                                                                    debug_assert_eq!(None, value32);
                                                                                                                                                                                    debug_assert_eq!(None, value33);
                                                                                                                                                                                    debug_assert_eq!(None, value34);
                                                                                                                                                                                    debug_assert_eq!(None, value35);
                                                                                                                                                                                    debug_assert_eq!(None, value36);
                                                                                                                                                                                    debug_assert_eq!(None, value37);
                                                                                                                                                                                    debug_assert_eq!(None, value38);
                                                                                                                                                                                    debug_assert_eq!(None, value39);
                                                                                                                                                                                    debug_assert_eq!(None, value40);
                                                                                                                                                                                    debug_assert_eq!(None, value41);
                                                                                                                                                                                    debug_assert_eq!(None, value42);
                                                                                                                                                                                    debug_assert_eq!(None, value43);
                                                                                                                                                                                    debug_assert_eq!(None, value44);
                                                                                                                                                                                    debug_assert_eq!(None, value45);
                                                                                                                                                                                    debug_assert_eq!(None, value46);
                                                                                                                                                                                    debug_assert_eq!(None, value47);
                                                                                                                                                                                    let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), Some(value15), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                                                                                                    Ok(u128::from(n))
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
                                                                                                                                                                        debug_assert_eq!(None, value32);
                                                                                                                                                                        debug_assert_eq!(None, value33);
                                                                                                                                                                        debug_assert_eq!(None, value34);
                                                                                                                                                                        debug_assert_eq!(None, value35);
                                                                                                                                                                        debug_assert_eq!(None, value36);
                                                                                                                                                                        debug_assert_eq!(None, value37);
                                                                                                                                                                        debug_assert_eq!(None, value38);
                                                                                                                                                                        debug_assert_eq!(None, value39);
                                                                                                                                                                        debug_assert_eq!(None, value40);
                                                                                                                                                                        debug_assert_eq!(None, value41);
                                                                                                                                                                        debug_assert_eq!(None, value42);
                                                                                                                                                                        debug_assert_eq!(None, value43);
                                                                                                                                                                        debug_assert_eq!(None, value44);
                                                                                                                                                                        debug_assert_eq!(None, value45);
                                                                                                                                                                        debug_assert_eq!(None, value46);
                                                                                                                                                                        debug_assert_eq!(None, value47);
                                                                                                                                                                        let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), Some(value14), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                                                                                        Ok(u128::from(n))
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
                                                                                                                                                            debug_assert_eq!(None, value32);
                                                                                                                                                            debug_assert_eq!(None, value33);
                                                                                                                                                            debug_assert_eq!(None, value34);
                                                                                                                                                            debug_assert_eq!(None, value35);
                                                                                                                                                            debug_assert_eq!(None, value36);
                                                                                                                                                            debug_assert_eq!(None, value37);
                                                                                                                                                            debug_assert_eq!(None, value38);
                                                                                                                                                            debug_assert_eq!(None, value39);
                                                                                                                                                            debug_assert_eq!(None, value40);
                                                                                                                                                            debug_assert_eq!(None, value41);
                                                                                                                                                            debug_assert_eq!(None, value42);
                                                                                                                                                            debug_assert_eq!(None, value43);
                                                                                                                                                            debug_assert_eq!(None, value44);
                                                                                                                                                            debug_assert_eq!(None, value45);
                                                                                                                                                            debug_assert_eq!(None, value46);
                                                                                                                                                            debug_assert_eq!(None, value47);
                                                                                                                                                            let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), Some(value13), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                                                                            Ok(u128::from(n))
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
                                                                                                                                                debug_assert_eq!(None, value32);
                                                                                                                                                debug_assert_eq!(None, value33);
                                                                                                                                                debug_assert_eq!(None, value34);
                                                                                                                                                debug_assert_eq!(None, value35);
                                                                                                                                                debug_assert_eq!(None, value36);
                                                                                                                                                debug_assert_eq!(None, value37);
                                                                                                                                                debug_assert_eq!(None, value38);
                                                                                                                                                debug_assert_eq!(None, value39);
                                                                                                                                                debug_assert_eq!(None, value40);
                                                                                                                                                debug_assert_eq!(None, value41);
                                                                                                                                                debug_assert_eq!(None, value42);
                                                                                                                                                debug_assert_eq!(None, value43);
                                                                                                                                                debug_assert_eq!(None, value44);
                                                                                                                                                debug_assert_eq!(None, value45);
                                                                                                                                                debug_assert_eq!(None, value46);
                                                                                                                                                debug_assert_eq!(None, value47);
                                                                                                                                                let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), Some(value12), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                                                                Ok(u128::from(n))
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
                                                                                                                                    debug_assert_eq!(None, value32);
                                                                                                                                    debug_assert_eq!(None, value33);
                                                                                                                                    debug_assert_eq!(None, value34);
                                                                                                                                    debug_assert_eq!(None, value35);
                                                                                                                                    debug_assert_eq!(None, value36);
                                                                                                                                    debug_assert_eq!(None, value37);
                                                                                                                                    debug_assert_eq!(None, value38);
                                                                                                                                    debug_assert_eq!(None, value39);
                                                                                                                                    debug_assert_eq!(None, value40);
                                                                                                                                    debug_assert_eq!(None, value41);
                                                                                                                                    debug_assert_eq!(None, value42);
                                                                                                                                    debug_assert_eq!(None, value43);
                                                                                                                                    debug_assert_eq!(None, value44);
                                                                                                                                    debug_assert_eq!(None, value45);
                                                                                                                                    debug_assert_eq!(None, value46);
                                                                                                                                    debug_assert_eq!(None, value47);
                                                                                                                                    let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), Some(value11), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                                                    Ok(u128::from(n))
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
                                                                                                                        debug_assert_eq!(None, value32);
                                                                                                                        debug_assert_eq!(None, value33);
                                                                                                                        debug_assert_eq!(None, value34);
                                                                                                                        debug_assert_eq!(None, value35);
                                                                                                                        debug_assert_eq!(None, value36);
                                                                                                                        debug_assert_eq!(None, value37);
                                                                                                                        debug_assert_eq!(None, value38);
                                                                                                                        debug_assert_eq!(None, value39);
                                                                                                                        debug_assert_eq!(None, value40);
                                                                                                                        debug_assert_eq!(None, value41);
                                                                                                                        debug_assert_eq!(None, value42);
                                                                                                                        debug_assert_eq!(None, value43);
                                                                                                                        debug_assert_eq!(None, value44);
                                                                                                                        debug_assert_eq!(None, value45);
                                                                                                                        debug_assert_eq!(None, value46);
                                                                                                                        debug_assert_eq!(None, value47);
                                                                                                                        let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), Some(value10), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                                        Ok(u128::from(n))
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
                                                                                                            debug_assert_eq!(None, value32);
                                                                                                            debug_assert_eq!(None, value33);
                                                                                                            debug_assert_eq!(None, value34);
                                                                                                            debug_assert_eq!(None, value35);
                                                                                                            debug_assert_eq!(None, value36);
                                                                                                            debug_assert_eq!(None, value37);
                                                                                                            debug_assert_eq!(None, value38);
                                                                                                            debug_assert_eq!(None, value39);
                                                                                                            debug_assert_eq!(None, value40);
                                                                                                            debug_assert_eq!(None, value41);
                                                                                                            debug_assert_eq!(None, value42);
                                                                                                            debug_assert_eq!(None, value43);
                                                                                                            debug_assert_eq!(None, value44);
                                                                                                            debug_assert_eq!(None, value45);
                                                                                                            debug_assert_eq!(None, value46);
                                                                                                            debug_assert_eq!(None, value47);
                                                                                                            let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), Some(value9), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                            Ok(u128::from(n))
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
                                                                                                debug_assert_eq!(None, value32);
                                                                                                debug_assert_eq!(None, value33);
                                                                                                debug_assert_eq!(None, value34);
                                                                                                debug_assert_eq!(None, value35);
                                                                                                debug_assert_eq!(None, value36);
                                                                                                debug_assert_eq!(None, value37);
                                                                                                debug_assert_eq!(None, value38);
                                                                                                debug_assert_eq!(None, value39);
                                                                                                debug_assert_eq!(None, value40);
                                                                                                debug_assert_eq!(None, value41);
                                                                                                debug_assert_eq!(None, value42);
                                                                                                debug_assert_eq!(None, value43);
                                                                                                debug_assert_eq!(None, value44);
                                                                                                debug_assert_eq!(None, value45);
                                                                                                debug_assert_eq!(None, value46);
                                                                                                debug_assert_eq!(None, value47);
                                                                                                let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), Some(value8), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                                Ok(u128::from(n))
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
                                                                                    debug_assert_eq!(None, value32);
                                                                                    debug_assert_eq!(None, value33);
                                                                                    debug_assert_eq!(None, value34);
                                                                                    debug_assert_eq!(None, value35);
                                                                                    debug_assert_eq!(None, value36);
                                                                                    debug_assert_eq!(None, value37);
                                                                                    debug_assert_eq!(None, value38);
                                                                                    debug_assert_eq!(None, value39);
                                                                                    debug_assert_eq!(None, value40);
                                                                                    debug_assert_eq!(None, value41);
                                                                                    debug_assert_eq!(None, value42);
                                                                                    debug_assert_eq!(None, value43);
                                                                                    debug_assert_eq!(None, value44);
                                                                                    debug_assert_eq!(None, value45);
                                                                                    debug_assert_eq!(None, value46);
                                                                                    debug_assert_eq!(None, value47);
                                                                                    let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), Some(value7), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                                    Ok(u128::from(n))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value8
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value9
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value10
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value11
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value12
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value13
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value14
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value15
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value16
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value17
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value18
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value19
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value20
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value21
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value22
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value23
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value24
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value25
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value26
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value27
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value28
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value29
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value30
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value31
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value32
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value33
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value34
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value35
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value36
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value37
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value38
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value39
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value40
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value41
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value42
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value43
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value44
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value45
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value46
                                                                        );
                                                                        debug_assert_eq!(
                                                                            None,
                                                                            value47
                                                                        );
                                                                        let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), Some(value6), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                                        Ok(u128::from(n))
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
                                                            debug_assert_eq!(None, value40);
                                                            debug_assert_eq!(None, value41);
                                                            debug_assert_eq!(None, value42);
                                                            debug_assert_eq!(None, value43);
                                                            debug_assert_eq!(None, value44);
                                                            debug_assert_eq!(None, value45);
                                                            debug_assert_eq!(None, value46);
                                                            debug_assert_eq!(None, value47);
                                                            let n = chinese_digit_100_000_000_middle_compat(value, Some(value2), Some(value3), Some(value4), Some(value5), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None)?;
                                                            Ok(u128::from(n))
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
                                                debug_assert_eq!(None, value40);
                                                debug_assert_eq!(None, value41);
                                                debug_assert_eq!(None, value42);
                                                debug_assert_eq!(None, value43);
                                                debug_assert_eq!(None, value44);
                                                debug_assert_eq!(None, value45);
                                                debug_assert_eq!(None, value46);
                                                debug_assert_eq!(None, value47);
                                                let n = chinese_digit_100_000_000_middle_compat(
                                                    value,
                                                    Some(value2),
                                                    Some(value3),
                                                    Some(value4),
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                )?;
                                                Ok(u128::from(n))
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
                                    debug_assert_eq!(None, value40);
                                    debug_assert_eq!(None, value41);
                                    debug_assert_eq!(None, value42);
                                    debug_assert_eq!(None, value43);
                                    debug_assert_eq!(None, value44);
                                    debug_assert_eq!(None, value45);
                                    debug_assert_eq!(None, value46);
                                    debug_assert_eq!(None, value47);
                                    let n = chinese_digit_100_000_000_middle_compat(
                                        value,
                                        Some(value2),
                                        Some(value3),
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                        None,
                                    )?;
                                    Ok(u128::from(n))
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
                        debug_assert_eq!(None, value40);
                        debug_assert_eq!(None, value41);
                        debug_assert_eq!(None, value42);
                        debug_assert_eq!(None, value43);
                        debug_assert_eq!(None, value44);
                        debug_assert_eq!(None, value45);
                        debug_assert_eq!(None, value46);
                        debug_assert_eq!(None, value47);
                        let n = chinese_digit_100_000_000_middle_compat(
                            value,
                            Some(value2),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )?;
                        Ok(u128::from(n))
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
            debug_assert_eq!(None, value40);
            debug_assert_eq!(None, value41);
            debug_assert_eq!(None, value42);
            debug_assert_eq!(None, value43);
            debug_assert_eq!(None, value44);
            debug_assert_eq!(None, value45);
            debug_assert_eq!(None, value46);
            debug_assert_eq!(None, value47);
            let n = chinese_digit_10_compat(value, None, None)?;
            Ok(u128::from(n))
        }
    }
}
