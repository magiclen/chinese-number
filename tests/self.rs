#![cfg(all(feature = "chinese-to-number", feature = "number-to-chinese"))]

use core::{
    fmt::Display,
    ops::{AddAssign, RangeInclusive, SubAssign},
};

use chinese_number::{
    ChineseCase, ChineseCountMethod, ChineseToNumber, ChineseVariant, NumberToChinese,
};
use num_traits::{CheckedAdd, CheckedMul};

fn ranger<
    T: PartialOrd + Copy + AddAssign + SubAssign + CheckedAdd + CheckedMul + TryFrom<usize> + Display,
>(
    range: RangeInclusive<T>,
    f: impl Fn(T),
) {
    let e = *range.end();

    let mut s = *range.start();

    let one = match T::try_from(1) {
        Ok(n) => n,
        Err(_) => unimplemented!(),
    };
    let mut c = one;
    let mut cc = one;

    while s <= e {
        f(s);

        s = match s.checked_add(&cc) {
            Some(n) => n,
            None => break,
        };

        c += one;

        cc = match cc.checked_mul(&c) {
            Some(n) => n,
            None => break,
        };

        cc -= one;
    }
}

#[test]
fn test_u8() {
    for method in ChineseCountMethod::variants() {
        ranger(u8::MIN..=u8::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(u8::MIN..=u8::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_i8() {
    for method in ChineseCountMethod::variants() {
        ranger(i8::MIN..=i8::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(i8::MIN..=i8::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_u16() {
    for method in ChineseCountMethod::variants() {
        ranger(u16::MIN..=u16::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(u16::MIN..=u16::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_i16() {
    for method in ChineseCountMethod::variants() {
        ranger(i16::MIN..=i16::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(i16::MIN..=i16::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_u32() {
    for method in ChineseCountMethod::variants() {
        ranger(u32::MIN..=u32::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(u32::MIN..=u32::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_i32() {
    for method in ChineseCountMethod::variants() {
        ranger(i32::MIN..=i32::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(i32::MIN..=i32::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_u64() {
    ranger(u64::MIN..=9999_9999_9999_9999, |i| {
        assert_eq!(
            i,
            i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::Low)
                .unwrap()
                .to_number(ChineseCountMethod::Low)
                .unwrap()
        );
    });

    for method in ChineseCountMethod::variants().iter().copied().skip(1) {
        ranger(u64::MIN..=u64::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(u64::MIN..=u64::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_i64() {
    ranger(-9999_9999_9999_9999i64..=9999_9999_9999_9999, |i| {
        assert_eq!(
            i,
            i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::Low)
                .unwrap()
                .to_number(ChineseCountMethod::Low)
                .unwrap()
        );
    });

    for method in ChineseCountMethod::variants().iter().copied().skip(1) {
        ranger(i64::MIN..=i64::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(i64::MIN..=i64::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_u128() {
    ranger(u128::MIN..=9999_9999_9999_9999, |i| {
        assert_eq!(
            i,
            i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::Low)
                .unwrap()
                .to_number(ChineseCountMethod::Low)
                .unwrap()
        );
    });

    for method in ChineseCountMethod::variants().iter().copied().skip(1) {
        ranger(u128::MIN..=u128::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(u128::MIN..=u128::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}

#[test]
fn test_i128() {
    ranger(-9999_9999_9999_9999i128..=9999_9999_9999_9999, |i| {
        assert_eq!(
            i,
            i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, ChineseCountMethod::Low)
                .unwrap()
                .to_number(ChineseCountMethod::Low)
                .unwrap()
        );
    });

    for method in ChineseCountMethod::variants().iter().copied().skip(1) {
        ranger(i128::MIN..=i128::MAX, |i| {
            assert_eq!(
                i,
                i.to_chinese(ChineseVariant::Traditional, ChineseCase::Lower, method)
                    .unwrap()
                    .to_number(method)
                    .unwrap()
            );
        });
    }

    ranger(i128::MIN..=i128::MAX, |i| {
        assert_eq!(
            i,
            i.to_chinese_naive(ChineseVariant::Traditional, ChineseCase::Lower)
                .to_number_naive()
                .unwrap()
        );
    });
}
