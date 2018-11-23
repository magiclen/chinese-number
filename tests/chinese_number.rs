extern crate chinese_number;

use chinese_number::{ChineseNumber, ChineseVariant};

macro_rules! batch_test {
    ( $value:expr, $m: ident, $exp:expr ) => {
        assert_eq!($exp, $value.$m(ChineseVariant::Traditional));
    }
}

#[test]
fn to_uppercase_middle() {
    macro_rules! to_uppercase_middle_batch_test {
        ( $value:expr, $exp:expr ) => {
            batch_test!($value, to_uppercase_middle, $exp);
        }
    }
    to_uppercase_middle_batch_test!(i8::max_value(), "壹佰貳拾柒");
    to_uppercase_middle_batch_test!(i8::min_value(), "負壹佰貳拾捌");
    to_uppercase_middle_batch_test!(u8::max_value(), "貳佰伍拾伍");
    to_uppercase_middle_batch_test!(i16::max_value(), "參萬貳仟柒佰陸拾柒");
    to_uppercase_middle_batch_test!(i16::min_value(), "負參萬貳仟柒佰陸拾捌");
    to_uppercase_middle_batch_test!(u16::max_value(), "陸萬伍仟伍佰參拾伍");
    to_uppercase_middle_batch_test!(i32::max_value(), "貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾柒");
    to_uppercase_middle_batch_test!(i32::min_value(), "負貳拾壹億肆仟柒佰肆拾捌萬參仟陸佰肆拾捌");
    to_uppercase_middle_batch_test!(u32::max_value(), "肆拾貳億玖仟肆佰玖拾陸萬柒仟貳佰玖拾伍");
    to_uppercase_middle_batch_test!(i64::max_value(), "玖佰貳拾貳京參仟參佰柒拾貳兆參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零柒");
    to_uppercase_middle_batch_test!(i64::min_value(), "負玖佰貳拾貳京參仟參佰柒拾貳兆參佰陸拾捌億伍仟肆佰柒拾柒萬伍仟捌佰零捌");
    to_uppercase_middle_batch_test!(u64::max_value(), "壹仟捌佰肆拾肆京陸仟柒佰肆拾肆兆柒佰參拾柒億玖佰伍拾伍萬壹仟陸佰壹拾伍");
    to_uppercase_middle_batch_test!(i128::max_value(), "壹佰柒拾澗壹仟肆佰壹拾壹溝捌仟參佰肆拾陸穰肆佰陸拾玖秭貳仟參佰壹拾柒垓參仟壹佰陸拾捌京柒仟參佰零參兆柒仟壹佰伍拾捌億捌仟肆佰壹拾萬零伍仟柒佰貳拾柒");
    to_uppercase_middle_batch_test!(i128::min_value(), "負壹佰柒拾澗壹仟肆佰壹拾壹溝捌仟參佰肆拾陸穰肆佰陸拾玖秭貳仟參佰壹拾柒垓參仟壹佰陸拾捌京柒仟參佰零參兆柒仟壹佰伍拾捌億捌仟肆佰壹拾萬零伍仟柒佰貳拾捌");
    to_uppercase_middle_batch_test!(u128::max_value(), "參佰肆拾澗貳仟捌佰貳拾參溝陸仟陸佰玖拾貳穰玖佰參拾捌秭肆仟陸佰參拾肆垓陸仟參佰參拾柒京肆仟陸佰零柒兆肆仟參佰壹拾柒億陸仟捌佰貳拾壹萬壹仟肆佰伍拾伍");
}

#[test]
fn to_uppercase_low() {
    macro_rules! to_uppercase_low_batch_test {
        ( $value:expr, $exp:expr ) => {
            batch_test!($value, to_uppercase_low, $exp);
        }
    }
    to_uppercase_low_batch_test!(i8::max_value(), "壹佰貳拾柒");
    to_uppercase_low_batch_test!(i8::min_value(), "負壹佰貳拾捌");
    to_uppercase_low_batch_test!(u8::max_value(), "貳佰伍拾伍");
    to_uppercase_low_batch_test!(i16::max_value(), "參萬貳仟柒佰陸拾柒");
    to_uppercase_low_batch_test!(i16::min_value(), "負參萬貳仟柒佰陸拾捌");
    to_uppercase_low_batch_test!(u16::max_value(), "陸萬伍仟伍佰參拾伍");
    to_uppercase_low_batch_test!(i32::max_value(), "貳秭壹垓肆京柒兆肆億捌萬參仟陸佰肆拾柒");
    to_uppercase_low_batch_test!(i32::min_value(), "負貳秭壹垓肆京柒兆肆億捌萬參仟陸佰肆拾捌");
    to_uppercase_low_batch_test!(u32::max_value(), "肆秭貳垓玖京肆兆玖億陸萬柒仟貳佰玖拾伍");
    to_uppercase_low_batch_test!(9999999999999999i64, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    to_uppercase_low_batch_test!(-9999999999999999i64, "負玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    to_uppercase_low_batch_test!(9999999999999999u64, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    to_uppercase_low_batch_test!(9999999999999999i128, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    to_uppercase_low_batch_test!(-9999999999999999i128, "負玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
    to_uppercase_low_batch_test!(9999999999999999u128, "玖極玖載玖正玖澗玖溝玖穰玖秭玖垓玖京玖兆玖億玖萬玖仟玖佰玖拾玖");
}

#[test]
fn to_lowercase_middle() {
    macro_rules! to_lowercase_middle_batch_test {
        ( $value:expr, $exp:expr ) => {
            batch_test!($value, to_lowercase_middle, $exp);
        }
    }
    to_lowercase_middle_batch_test!(i8::max_value(), "一百二十七");
    to_lowercase_middle_batch_test!(i8::min_value(), "負一百二十八");
    to_lowercase_middle_batch_test!(u8::max_value(), "二百五十五");
    to_lowercase_middle_batch_test!(i16::max_value(), "三萬二千七百六十七");
    to_lowercase_middle_batch_test!(i16::min_value(), "負三萬二千七百六十八");
    to_lowercase_middle_batch_test!(u16::max_value(), "六萬五千五百三十五");
    to_lowercase_middle_batch_test!(i32::max_value(), "二十一億四千七百四十八萬三千六百四十七");
    to_lowercase_middle_batch_test!(i32::min_value(), "負二十一億四千七百四十八萬三千六百四十八");
    to_lowercase_middle_batch_test!(u32::max_value(), "四十二億九千四百九十六萬七千二百九十五");
    to_lowercase_middle_batch_test!(i64::max_value(), "九百二十二京三千三百七十二兆三百六十八億五千四百七十七萬五千八百零七");
    to_lowercase_middle_batch_test!(i64::min_value(), "負九百二十二京三千三百七十二兆三百六十八億五千四百七十七萬五千八百零八");
    to_lowercase_middle_batch_test!(u64::max_value(), "一千八百四十四京六千七百四十四兆七百三十七億九百五十五萬一千六百一十五");
    to_lowercase_middle_batch_test!(i128::max_value(), "一百七十澗一千四百一十一溝八千三百四十六穰四百六十九秭二千三百一十七垓三千一百六十八京七千三百零三兆七千一百五十八億八千四百一十萬零五千七百二十七");
    to_lowercase_middle_batch_test!(i128::min_value(), "負一百七十澗一千四百一十一溝八千三百四十六穰四百六十九秭二千三百一十七垓三千一百六十八京七千三百零三兆七千一百五十八億八千四百一十萬零五千七百二十八");
    to_lowercase_middle_batch_test!(u128::max_value(), "三百四十澗二千八百二十三溝六千六百九十二穰九百三十八秭四千六百三十四垓六千三百三十七京四千六百零七兆四千三百一十七億六千八百二十一萬一千四百五十五");
}

#[test]
fn to_lowercase_low() {
    macro_rules! to_lowercase_low_batch_test {
        ( $value:expr, $exp:expr ) => {
            batch_test!($value, to_lowercase_low, $exp);
        }
    }
    to_lowercase_low_batch_test!(i8::max_value(), "一百二十七");
    to_lowercase_low_batch_test!(i8::min_value(), "負一百二十八");
    to_lowercase_low_batch_test!(u8::max_value(), "二百五十五");
    to_lowercase_low_batch_test!(i16::max_value(), "三萬二千七百六十七");
    to_lowercase_low_batch_test!(i16::min_value(), "負三萬二千七百六十八");
    to_lowercase_low_batch_test!(u16::max_value(), "六萬五千五百三十五");
    to_lowercase_low_batch_test!(i32::max_value(), "二秭一垓四京七兆四億八萬三千六百四十七");
    to_lowercase_low_batch_test!(i32::min_value(), "負二秭一垓四京七兆四億八萬三千六百四十八");
    to_lowercase_low_batch_test!(u32::max_value(), "四秭二垓九京四兆九億六萬七千二百九十五");
    to_lowercase_low_batch_test!(9999999999999999i64, "九極九載九正九澗九溝九穰九秭九垓九京九兆九億九萬九千九百九十九");
    to_lowercase_low_batch_test!(-9999999999999999i64, "負九極九載九正九澗九溝九穰九秭九垓九京九兆九億九萬九千九百九十九");
    to_lowercase_low_batch_test!(9999999999999999u64, "九極九載九正九澗九溝九穰九秭九垓九京九兆九億九萬九千九百九十九");
    to_lowercase_low_batch_test!(9999999999999999i128, "九極九載九正九澗九溝九穰九秭九垓九京九兆九億九萬九千九百九十九");
    to_lowercase_low_batch_test!(-9999999999999999i128, "負九極九載九正九澗九溝九穰九秭九垓九京九兆九億九萬九千九百九十九");
    to_lowercase_low_batch_test!(9999999999999999u128, "九極九載九正九澗九溝九穰九秭九垓九京九兆九億九萬九千九百九十九");
}