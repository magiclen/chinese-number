use alloc::string::{String, ToString};

use num_bigint::BigUint;

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use num_traits::float::FloatCore;

use num_traits::{FromPrimitive, ToPrimitive, Zero};

use crate::{ChineseCase, ChineseCountMethod, ChineseExponent, ChineseNumber, ChineseVariant};

pub(crate) fn unsigned_integer_to_chinese_low(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: u128,
) -> String {
    debug_assert!(value < 1_0000_0000_0000_0000);

    let mut s = String::new();

    let mut lower_d = (value % 10) as u8;
    value /= 10;

    if lower_d > 0 {
        s.push_str(
            unsafe { ChineseNumber::from_ordinal_unsafe(lower_d) }
                .to_str(chinese_variant, chinese_case),
        );
    } else if value == 0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    }

    let d = (value % 10) as u8;
    value /= 10;

    if d > 0 {
        s.insert_str(0, ChineseExponent::十.to_str(chinese_variant, chinese_case));

        if value > 0 || dependent || d > 1 {
            s.insert_str(
                0,
                unsafe { ChineseNumber::from_ordinal_unsafe(d) }
                    .to_str(chinese_variant, chinese_case),
            );
        }
    }

    if value == 0 {
        return s;
    }

    lower_d = d;

    let mut i = ChineseExponent::百.ordinal();

    loop {
        let d = (value % 10) as u8;
        value /= 10;

        if d > 0 {
            if lower_d < 1 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
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

    s
}

pub(crate) fn unsigned_integer_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: u128,
) -> String {
    let mut lower_d = value % 1_0000;
    value /= 1_0000;

    let mut has_more = value > 0;

    let mut s = if lower_d > 0 {
        unsigned_integer_to_chinese_low(
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

    let mut i = ChineseExponent::萬.ordinal();

    loop {
        let d = value % 1_0000;
        value /= 1_0000;

        has_more = value > 0;

        if d > 0 {
            if lower_d < 1000 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
                unsigned_integer_to_chinese_low(
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
    }

    s
}

pub(crate) fn big_unsigned_integer_to_chinese_ten_thousand(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: BigUint,
) -> String {
    debug_assert!(value < BigUint::from(10u8).pow(52));

    let big_0 = BigUint::zero();
    let big_1_0000 = BigUint::from(1_0000u16);

    let mut lower_d = (value.clone() % &big_1_0000).to_u128().unwrap();
    value /= &big_1_0000;

    let mut has_more = value > big_0;

    let mut s = if lower_d > 0 {
        unsigned_integer_to_chinese_low(
            chinese_variant,
            chinese_case,
            dependent || has_more,
            lower_d,
        )
    } else if value == big_0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    } else {
        String::new()
    };

    if !has_more {
        return s;
    }

    let mut i = ChineseExponent::萬.ordinal();

    loop {
        let d = (value.clone() % &big_1_0000).to_u128().unwrap();
        value /= &big_1_0000;

        has_more = value > big_0;

        if d > 0 {
            if lower_d < 1000 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
                unsigned_integer_to_chinese_low(
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
    }

    s
}

pub(crate) fn unsigned_integer_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: u128,
) -> String {
    let mut lower_d = value % 1_0000_0000;
    value /= 1_0000_0000;

    let mut has_more = value > 0;

    let mut s = if lower_d > 0 {
        unsigned_integer_to_chinese_ten_thousand(
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

    let mut i = ChineseExponent::億.ordinal();

    loop {
        let d = value % 1_0000_0000;
        value /= 1_0000_0000;

        has_more = value > 0;

        if d > 0 {
            if lower_d < 1000_0000 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
                unsigned_integer_to_chinese_ten_thousand(
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
    }

    s
}

pub(crate) fn big_unsigned_integer_to_chinese_middle(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    dependent: bool,
    mut value: BigUint,
) -> String {
    debug_assert!(value < BigUint::from(10u8).pow(96));

    let big_0 = BigUint::zero();
    let big_1_0000_0000 = BigUint::from(1_0000_0000u32);

    let mut lower_d = (value.clone() % &big_1_0000_0000).to_u128().unwrap();
    value /= &big_1_0000_0000;

    let mut has_more = value > big_0;

    let mut s = if lower_d > 0 {
        unsigned_integer_to_chinese_ten_thousand(
            chinese_variant,
            chinese_case,
            dependent || has_more,
            lower_d,
        )
    } else if value == big_0 {
        return ChineseNumber::零.to_str(chinese_variant, chinese_case).to_string();
    } else {
        String::new()
    };

    if !has_more {
        return s;
    }

    let mut i = ChineseExponent::億.ordinal();

    loop {
        let d = (value.clone() % &big_1_0000_0000).to_u128().unwrap();
        value /= &big_1_0000_0000;

        has_more = value > big_0;

        if d > 0 {
            if lower_d < 1000_0000 && !s.is_empty() {
                s.insert_str(0, ChineseNumber::零.to_str(chinese_variant, chinese_case));
            }

            s.insert_str(
                0,
                unsafe { ChineseExponent::from_ordinal_unsafe(i) }
                    .to_str(chinese_variant, chinese_case),
            );

            s.insert_str(
                0,
                unsigned_integer_to_chinese_ten_thousand(
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
    }

    s
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

pub(crate) fn positive_float_to_chinese(
    chinese_variant: ChineseVariant,
    chinese_case: ChineseCase,
    method: ChineseCountMethod,
    value: f64,
) -> String {
    let (integer, fraction) = {
        let integer = BigUint::from_f64(value.trunc()).unwrap();
        let fraction = ((value.fract() * 100.0).round() % 100f64) as u8;

        (integer, fraction)
    };

    let big_0 = BigUint::zero();

    let mut s = if integer > big_0 {
        match method {
            ChineseCountMethod::Low => {
                unsigned_integer_to_chinese_low(
                    chinese_variant,
                    chinese_case,
                    false,
                    integer.to_u128().unwrap(),
                )
            }
            ChineseCountMethod::TenThousand => {
                big_unsigned_integer_to_chinese_ten_thousand(
                    chinese_variant,
                    chinese_case,
                    false,
                    integer.clone(),
                )
            }
            ChineseCountMethod::Middle => {
                big_unsigned_integer_to_chinese_middle(
                    chinese_variant,
                    chinese_case,
                    false,
                    integer.clone(),
                )
            }
            ChineseCountMethod::High => {
                big_unsigned_integer_to_chinese_high(
                    chinese_variant,
                    chinese_case,
                    false,
                    integer.clone(),
                )
            }
        }
    } else {
        String::new()
    };

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
    } else if integer == big_0 {
        s.push_str(ChineseNumber::零.to_str(chinese_variant, chinese_case));
    }

    s
}
