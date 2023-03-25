#![cfg(feature = "chinese-to-number")]

use chinese_number::{ChineseCountMethod, ChineseToNumber, ChineseToNumberError};

macro_rules! test_group {
    ($method:expr) => {
        macro_rules! test {
            ($expect: expr,$value: expr) => {
                assert_eq!($expect, $value.to_number($method).unwrap());
            };
        }

        macro_rules! test_err {
            ($expect: expr,$value: expr) => {
                test_err!(i8, $expect, $value);
                test_err!(u8, $expect, $value);
                test_err!(i16, $expect, $value);
                test_err!(u16, $expect, $value);
                test_err!(i32, $expect, $value);
                test_err!(u32, $expect, $value);
                test_err!(i64, $expect, $value);
                test_err!(u64, $expect, $value);
                test_err!(i128, $expect, $value);
                test_err!(u128, $expect, $value);
                test_err!(isize, $expect, $value);
                test_err!(usize, $expect, $value);
                test_err!(f32, $expect, $value);
                test_err!(f64, $expect, $value);
            };
            ($typ: ty,$expect: expr,$value: expr) => {
                assert_eq!(Err::<$typ, _>($expect), $value.to_number($method));
            };
        }
    };
}

#[test]
fn to_number_low() {
    test_group!(ChineseCountMethod::Low);

    test!(i8::MAX, "壹佰貳拾柒");
    test!(i8::MIN, "負壹佰貳拾捌");
    test!(u8::MAX, "貳佰伍拾伍");
    test!(i16::MAX, "參萬貳仟柒佰陸拾柒");
    test!(i16::MIN, "負參萬貳仟柒佰陸拾捌");
    test!(u16::MAX, "陸萬伍仟伍佰參拾伍");
    test!(i32::MAX, "貳秭壹垓肆京柒兆肆億捌萬參仟陸佰肆拾柒");
    test!(i32::MIN, "負貳秭壹垓肆京柒兆肆億捌萬參仟陸佰肆拾捌");
    test!(u32::MAX, "肆秭貳垓玖京肆兆玖億陸萬柒仟貳佰玖拾伍");
    test!(9999999999999999i64, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    test!(-9999999999999999i64, "負玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    test!(9999999999999999u64, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    test!(9999999999999999i128, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    test!(
        -9999999999999999i128,
        "負玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖"
    );
    test!(9999999999999999u128, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");

    test!(123.46f32, "壹佰貳拾參肆角陸分");
    test!(-123.46f32, "負壹佰貳拾參肆角陸分");
    test!(123.46f64, "壹佰貳拾參肆角陸分");
    test!(-123.46f64, "負壹佰貳拾參肆角陸分");

    test_err!(ChineseToNumberError::ChineseNumberEmpty, "");

    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 0
        },
        "a"
    );
    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 1
        },
        "壹佰貳拾佰"
    );

    test_err!(i8, ChineseToNumberError::Overflow, "壹佰貳拾捌");
    test_err!(i8, ChineseToNumberError::Underflow, "負壹佰貳拾玖");
}

#[test]
fn to_number_ten_thousand() {
    test_group!(ChineseCountMethod::TenThousand);

    test!(i8::MAX, "壹佰貳拾柒");
    test!(i8::MIN, "負壹佰貳拾捌");
    test!(u8::MAX, "貳佰伍拾伍");
    test!(i16::MAX, "參萬貳仟柒佰陸拾柒");
    test!(i16::MIN, "負參萬貳仟柒佰陸拾捌");
    test!(u16::MAX, "陸萬伍仟伍佰參拾伍");
    test!(i32::MAX, "貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾柒");
    test!(i32::MIN, "負貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾捌");
    test!(u32::MAX, "肆拾貳億玖仟肆佰玖拾陸萬柒仟貳佰玖拾伍");
    test!(i64::MAX, "玖佰貳拾貳京參仟參佰柒拾貳兆零參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零柒");
    test!(i64::MIN, "負玖佰貳拾貳京參仟參佰柒拾貳兆零參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零捌");
    test!(u64::MAX, "壹仟捌佰肆拾肆京陸仟柒佰肆拾肆兆零柒佰參拾柒億零玖佰伍拾伍萬壹仟陸佰壹拾伍");
    test!(i128::MAX, "壹佰柒拾澗壹仟肆佰壹拾壹溝捌仟參佰肆拾陸穰零肆佰陸拾玖秭貳仟參佰壹拾柒垓參仟壹佰陸拾捌京柒仟參佰零參兆柒仟壹佰伍拾捌億捌仟肆佰壹拾萬伍仟柒佰貳拾柒");
    test!(i128::MIN, "負壹佰柒拾澗壹仟肆佰壹拾壹溝捌仟參佰肆拾陸穰零肆佰陸拾玖秭貳仟參佰壹拾柒垓參仟壹佰陸拾捌京柒仟參佰零參兆柒仟壹佰伍拾捌億捌仟肆佰壹拾萬伍仟柒佰貳拾捌");
    test!(u128::MAX, "參佰肆拾澗貳仟捌佰貳拾參溝陸仟陸佰玖拾貳穰零玖佰參拾捌秭肆仟陸佰參拾肆垓陸仟參佰參拾柒京肆仟陸佰零柒兆肆仟參佰壹拾柒億陸仟捌佰貳拾壹萬壹仟肆佰伍拾伍");

    test!(123.46f32, "壹佰貳拾參肆角陸分");
    test!(-123.46f32, "負壹佰貳拾參肆角陸分");
    test!(123.46f64, "壹佰貳拾參肆角陸分");
    test!(-123.46f64, "負壹佰貳拾參肆角陸分");

    test_err!(ChineseToNumberError::ChineseNumberEmpty, "");

    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 0
        },
        "a"
    );
    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 1
        },
        "壹佰貳拾佰"
    );

    test_err!(i8, ChineseToNumberError::Overflow, "壹佰貳拾捌");
    test_err!(i8, ChineseToNumberError::Underflow, "負壹佰貳拾玖");
}

#[test]
fn to_number_middle() {
    test_group!(ChineseCountMethod::Middle);

    test!(i8::MAX, "壹佰貳拾柒");
    test!(i8::MIN, "負壹佰貳拾捌");
    test!(u8::MAX, "貳佰伍拾伍");
    test!(i16::MAX, "參萬貳仟柒佰陸拾柒");
    test!(i16::MIN, "負參萬貳仟柒佰陸拾捌");
    test!(u16::MAX, "陸萬伍仟伍佰參拾伍");
    test!(i32::MAX, "貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾柒");
    test!(i32::MIN, "負貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾捌");
    test!(u32::MAX, "肆拾貳億玖仟肆佰玖拾陸萬柒仟貳佰玖拾伍");
    test!(i64::MAX, "玖佰貳拾貳兆參仟參佰柒拾貳萬零參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零柒");
    test!(i64::MIN, "負玖佰貳拾貳兆參仟參佰柒拾貳萬零參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零捌");
    test!(u64::MAX, "壹仟捌佰肆拾肆兆陸仟柒佰肆拾肆萬零柒佰參拾柒億零玖佰伍拾伍萬壹仟陸佰壹拾伍");
    test!(i128::MAX, "壹佰柒拾萬壹仟肆佰壹拾壹垓捌仟參佰肆拾陸萬零肆佰陸拾玖京貳仟參佰壹拾柒萬參仟壹佰陸拾捌兆柒仟參佰零參萬柒仟壹佰伍拾捌億捌仟肆佰壹拾萬伍仟柒佰貳拾柒");
    test!(i128::MIN, "負壹佰柒拾萬壹仟肆佰壹拾壹垓捌仟參佰肆拾陸萬零肆佰陸拾玖京貳仟參佰壹拾柒萬參仟壹佰陸拾捌兆柒仟參佰零參萬柒仟壹佰伍拾捌億捌仟肆佰壹拾萬伍仟柒佰貳拾捌");
    test!(u128::MAX, "參佰肆拾萬貳仟捌佰貳拾參垓陸仟陸佰玖拾貳萬零玖佰參拾捌京肆仟陸佰參拾肆萬陸仟參佰參拾柒兆肆仟陸佰零柒萬肆仟參佰壹拾柒億陸仟捌佰貳拾壹萬壹仟肆佰伍拾伍");

    test!(123.46f32, "壹佰貳拾參肆角陸分");
    test!(-123.46f32, "負壹佰貳拾參肆角陸分");
    test!(123.46f64, "壹佰貳拾參肆角陸分");
    test!(-123.46f64, "負壹佰貳拾參肆角陸分");

    test!(f32::MAX, "參佰肆拾萬貳仟捌佰貳拾參垓肆仟陸佰陸拾參萬捌仟伍佰貳拾捌京捌仟伍佰玖拾捌萬壹仟壹佰柒拾兆肆仟壹佰捌拾參萬肆仟捌佰肆拾伍億壹仟陸佰玖拾貳萬伍仟肆佰肆拾");
    test!(f32::MIN, "負參佰肆拾萬貳仟捌佰貳拾參垓肆仟陸佰陸拾參萬捌仟伍佰貳拾捌京捌仟伍佰玖拾捌萬壹仟壹佰柒拾兆肆仟壹佰捌拾參萬肆仟捌佰肆拾伍億壹仟陸佰玖拾貳萬伍仟肆佰肆拾");

    // test!(1e96f64 - 1e81, "玖仟玖佰玖拾玖萬玖仟玖佰玖拾玖極玖仟玖佰玖拾玖萬玖仟玖佰玖拾壹載零壹佰貳拾玖萬貳仟捌佰伍拾捌正玖仟參佰玖拾捌萬壹仟肆佰陸拾貳澗零壹佰零柒萬壹仟壹佰陸拾柒溝伍仟玖佰貳拾玖萬肆仟貳佰陸拾柒穰壹仟貳佰壹拾萬壹仟柒佰伍拾捌秭柒仟壹佰柒拾捌萬伍仟肆佰玖拾陸垓捌仟肆佰肆拾捌萬柒仟柒佰捌拾柒京捌仟陸佰貳拾柒萬柒仟柒佰陸拾貳兆貳仟伍佰陸拾萬壹仟零壹拾億零伍佰萬零柒佰零肆");
    // test!(-1e96f64 + 1e81, "負玖仟玖佰玖拾玖萬玖仟玖佰玖拾玖極玖仟玖佰玖拾玖萬玖仟玖佰玖拾壹載零壹佰貳拾玖萬貳仟捌佰伍拾捌正玖仟參佰玖拾捌萬壹仟肆佰陸拾貳澗零壹佰零柒萬壹仟壹佰陸拾柒溝伍仟玖佰貳拾玖萬肆仟貳佰陸拾柒穰壹仟貳佰壹拾萬壹仟柒佰伍拾捌秭柒仟壹佰柒拾捌萬伍仟肆佰玖拾陸垓捌仟肆佰肆拾捌萬柒仟柒佰捌拾柒京捌仟陸佰貳拾柒萬柒仟柒佰陸拾貳兆貳仟伍佰陸拾萬壹仟零壹拾億零伍佰萬零柒佰零肆");

    test_err!(ChineseToNumberError::ChineseNumberEmpty, "");

    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 0
        },
        "a"
    );
    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 1
        },
        "壹佰貳拾佰"
    );

    test_err!(i8, ChineseToNumberError::Overflow, "壹佰貳拾捌");
    test_err!(i8, ChineseToNumberError::Underflow, "負壹佰貳拾玖");
}

#[test]
fn to_number_high() {
    test_group!(ChineseCountMethod::High);

    test!(i8::MAX, "壹佰貳拾柒");
    test!(i8::MIN, "負壹佰貳拾捌");
    test!(u8::MAX, "貳佰伍拾伍");
    test!(i16::MAX, "參萬貳仟柒佰陸拾柒");
    test!(i16::MIN, "負參萬貳仟柒佰陸拾捌");
    test!(u16::MAX, "陸萬伍仟伍佰參拾伍");
    test!(i32::MAX, "貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾柒");
    test!(i32::MIN, "負貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾捌");
    test!(u32::MAX, "肆拾貳億玖仟肆佰玖拾陸萬柒仟貳佰玖拾伍");
    test!(i64::MAX, "玖佰貳拾貳兆參仟參佰柒拾貳萬零參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零柒");
    test!(i64::MIN, "負玖佰貳拾貳兆參仟參佰柒拾貳萬零參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零捌");
    test!(u64::MAX, "壹仟捌佰肆拾肆兆陸仟柒佰肆拾肆萬零柒佰參拾柒億零玖佰伍拾伍萬壹仟陸佰壹拾伍");
    test!(i128::MAX, "壹佰柒拾萬壹仟肆佰壹拾壹京捌仟參佰肆拾陸萬零肆佰陸拾玖億貳仟參佰壹拾柒萬參仟壹佰陸拾捌兆柒仟參佰零參萬柒仟壹佰伍拾捌億捌仟肆佰壹拾萬伍仟柒佰貳拾柒");
    test!(i128::MIN, "負壹佰柒拾萬壹仟肆佰壹拾壹京捌仟參佰肆拾陸萬零肆佰陸拾玖億貳仟參佰壹拾柒萬參仟壹佰陸拾捌兆柒仟參佰零參萬柒仟壹佰伍拾捌億捌仟肆佰壹拾萬伍仟柒佰貳拾捌");
    test!(u128::MAX, "參佰肆拾萬貳仟捌佰貳拾參京陸仟陸佰玖拾貳萬零玖佰參拾捌億肆仟陸佰參拾肆萬陸仟參佰參拾柒兆肆仟陸佰零柒萬肆仟參佰壹拾柒億陸仟捌佰貳拾壹萬壹仟肆佰伍拾伍");

    test!(123.46f32, "壹佰貳拾參肆角陸分");
    test!(-123.46f32, "負壹佰貳拾參肆角陸分");
    test!(123.46f64, "壹佰貳拾參肆角陸分");
    test!(-123.46f64, "負壹佰貳拾參肆角陸分");

    test!(f32::MAX, "參佰肆拾萬貳仟捌佰貳拾參京肆仟陸佰陸拾參萬捌仟伍佰貳拾捌億捌仟伍佰玖拾捌萬壹仟壹佰柒拾兆肆仟壹佰捌拾參萬肆仟捌佰肆拾伍億壹仟陸佰玖拾貳萬伍仟肆佰肆拾");
    test!(f32::MIN, "負參佰肆拾萬貳仟捌佰貳拾參京肆仟陸佰陸拾參萬捌仟伍佰貳拾捌億捌仟伍佰玖拾捌萬壹仟壹佰柒拾兆肆仟壹佰捌拾參萬肆仟捌佰肆拾伍億壹仟陸佰玖拾貳萬伍仟肆佰肆拾");

    // test!(f64::MAX, "壹萬柒仟玖佰柒拾陸兆玖仟參佰壹拾參萬肆仟捌佰陸拾貳億參仟壹佰萬京穰");
    // test!(f64::MIN, "負壹萬柒仟玖佰柒拾陸兆玖仟參佰壹拾參萬肆仟捌佰陸拾貳億參仟壹佰萬京穰");

    test_err!(ChineseToNumberError::ChineseNumberEmpty, "");

    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 0
        },
        "a"
    );
    test_err!(
        ChineseToNumberError::ChineseNumberIncorrect {
            char_index: 1
        },
        "壹佰貳拾佰"
    );

    test_err!(i8, ChineseToNumberError::Overflow, "壹佰貳拾捌");
    test_err!(i8, ChineseToNumberError::Underflow, "負壹佰貳拾玖");
}
