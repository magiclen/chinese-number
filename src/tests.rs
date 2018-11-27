use super::*;

#[test]
fn test_digit_1() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    for i in 0..=9 {
        digit_1(chinese_number_index, i, &mut s);
    }

    assert_eq!("零壹貳參肆伍陸柒捌玖", s);
}

#[test]
fn test_digit_10_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    digit_10(chinese_number_index, 10, &mut s, false);
    digit_10(chinese_number_index, 11, &mut s, false);
    digit_10(chinese_number_index, 12, &mut s, false);
    digit_10(chinese_number_index, 30, &mut s, false);
    digit_10(chinese_number_index, 95, &mut s, false);

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10(chinese_number_index, 10, &mut s, false);
    digit_10(chinese_number_index, 11, &mut s, false);
    digit_10(chinese_number_index, 12, &mut s, false);
    digit_10(chinese_number_index, 30, &mut s, false);
    digit_10(chinese_number_index, 95, &mut s, false);

    assert_eq!("壹拾壹拾壹壹拾貳參拾玖拾伍十十一十二三十九十五", s);
}

#[test]
fn test_digit_10_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    digit_10(chinese_number_index, 10, &mut s, true);
    digit_10(chinese_number_index, 11, &mut s, true);
    digit_10(chinese_number_index, 12, &mut s, true);
    digit_10(chinese_number_index, 30, &mut s, true);
    digit_10(chinese_number_index, 95, &mut s, true);

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10(chinese_number_index, 10, &mut s, true);
    digit_10(chinese_number_index, 11, &mut s, true);
    digit_10(chinese_number_index, 12, &mut s, true);
    digit_10(chinese_number_index, 30, &mut s, true);
    digit_10(chinese_number_index, 95, &mut s, true);

    assert_eq!("壹拾壹拾壹壹拾貳參拾玖拾伍一十一十一一十二三十九十五", s);
}

#[test]
fn test_digit_100() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    digit_100(chinese_number_index, 100, &mut s);
    digit_100(chinese_number_index, 101, &mut s);
    digit_100(chinese_number_index, 102, &mut s);
    digit_100(chinese_number_index, 110, &mut s);
    digit_100(chinese_number_index, 111, &mut s);
    digit_100(chinese_number_index, 200, &mut s);
    digit_100(chinese_number_index, 950, &mut s);
    digit_100(chinese_number_index, 999, &mut s);

    assert_eq!("壹佰壹佰零壹壹佰零貳壹佰壹拾壹佰壹拾壹貳佰玖佰伍拾玖佰玖拾玖", s);
}

#[test]
fn test_digit_1000() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    digit_1000(chinese_number_index, 1000, &mut s);
    digit_1000(chinese_number_index, 1001, &mut s);
    digit_1000(chinese_number_index, 1010, &mut s);
    digit_1000(chinese_number_index, 1011, &mut s);
    digit_1000(chinese_number_index, 1100, &mut s);
    digit_1000(chinese_number_index, 1101, &mut s);
    digit_1000(chinese_number_index, 1110, &mut s);
    digit_1000(chinese_number_index, 1111, &mut s);
    digit_1000(chinese_number_index, 9999, &mut s);

    assert_eq!("壹仟壹仟零壹壹仟零壹拾壹仟零壹拾壹壹仟壹佰壹仟壹佰零壹壹仟壹佰壹拾壹仟壹佰壹拾壹玖仟玖佰玖拾玖", s);
}

#[test]
fn test_digit_10000_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    digit_10000(chinese_number_index, 10000, &mut s, false);
    digit_10000(chinese_number_index, 10001, &mut s, false);
    digit_10000(chinese_number_index, 100001, &mut s, false);
    digit_10000(chinese_number_index, 110010, &mut s, false);
    digit_10000(chinese_number_index, 1001000, &mut s, false);
    digit_10000(chinese_number_index, 1100101, &mut s, false);
    digit_10000(chinese_number_index, 99999999, &mut s, false);

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10000(chinese_number_index, 10000, &mut s, false);
    digit_10000(chinese_number_index, 10001, &mut s, false);
    digit_10000(chinese_number_index, 100001, &mut s, false);
    digit_10000(chinese_number_index, 110010, &mut s, false);
    digit_10000(chinese_number_index, 1001000, &mut s, false);
    digit_10000(chinese_number_index, 1100101, &mut s, false);
    digit_10000(chinese_number_index, 99999999, &mut s, false);

    assert_eq!("壹萬壹萬零壹壹拾萬零壹壹拾壹萬零壹拾壹佰萬壹仟壹佰壹拾萬零壹佰零壹玖仟玖佰玖拾玖萬玖仟玖佰玖拾玖一萬一萬零一十萬零一十一萬零一十一百萬一千一百一十萬零一百零一九千九百九十九萬九千九百九十九", s);
}

#[test]
fn test_digit_10000_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Upper);

    digit_10000(chinese_number_index, 10000, &mut s, true);
    digit_10000(chinese_number_index, 10001, &mut s, true);
    digit_10000(chinese_number_index, 100001, &mut s, true);
    digit_10000(chinese_number_index, 110010, &mut s, true);
    digit_10000(chinese_number_index, 1001000, &mut s, true);
    digit_10000(chinese_number_index, 1100101, &mut s, true);
    digit_10000(chinese_number_index, 99999999, &mut s, true);

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10000(chinese_number_index, 10000, &mut s, true);
    digit_10000(chinese_number_index, 10001, &mut s, true);
    digit_10000(chinese_number_index, 100001, &mut s, true);
    digit_10000(chinese_number_index, 110010, &mut s, true);
    digit_10000(chinese_number_index, 1001000, &mut s, true);
    digit_10000(chinese_number_index, 1100101, &mut s, true);
    digit_10000(chinese_number_index, 99999999, &mut s, true);

    assert_eq!("壹萬壹萬零壹壹拾萬零壹壹拾壹萬零壹拾壹佰萬壹仟壹佰壹拾萬零壹佰零壹玖仟玖佰玖拾玖萬玖仟玖佰玖拾玖一萬一萬零一一十萬零一一十一萬零一十一百萬一千一百一十萬零一百零一九千九百九十九萬九千九百九十九", s);
}

#[test]
fn test_digit_1000_compat_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_1000_compat(chinese_number_index, 0, &mut s, false);
    digit_1000_compat(chinese_number_index, 1, &mut s, false);
    digit_1000_compat(chinese_number_index, 10, &mut s, false);
    digit_1000_compat(chinese_number_index, 22, &mut s, false);
    digit_1000_compat(chinese_number_index, 333, &mut s, false);
    digit_1000_compat(chinese_number_index, 4444, &mut s, false);
    digit_1000_compat(chinese_number_index, 9090, &mut s, false);

    assert_eq!("零一十二十二三百三十三四千四百四十四九千零九十", s);
}

#[test]
fn test_digit_1000_compat_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_1000_compat(chinese_number_index, 0, &mut s, true);
    digit_1000_compat(chinese_number_index, 1, &mut s, true);
    digit_1000_compat(chinese_number_index, 10, &mut s, true);
    digit_1000_compat(chinese_number_index, 22, &mut s, true);
    digit_1000_compat(chinese_number_index, 333, &mut s, true);
    digit_1000_compat(chinese_number_index, 4444, &mut s, true);
    digit_1000_compat(chinese_number_index, 9090, &mut s, true);

    assert_eq!("一一十二十二三百三十三四千四百四十四九千零九十", s);
}

#[test]
fn test_digit_10000_compat_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10000_compat(chinese_number_index, 0, &mut s, false);
    digit_10000_compat(chinese_number_index, 1, &mut s, false);
    digit_10000_compat(chinese_number_index, 10, &mut s, false);
    digit_10000_compat(chinese_number_index, 22, &mut s, false);
    digit_10000_compat(chinese_number_index, 333, &mut s, false);
    digit_10000_compat(chinese_number_index, 4444, &mut s, false);
    digit_10000_compat(chinese_number_index, 55555, &mut s, false);
    digit_10000_compat(chinese_number_index, 100000, &mut s, false);
    digit_10000_compat(chinese_number_index, 666066, &mut s, false);
    digit_10000_compat(chinese_number_index, 990909, &mut s, false);

    assert_eq!("零一十二十二三百三十三四千四百四十四五萬五千五百五十五十萬六十六萬六千零六十六九十九萬零九百零九", s);
}

#[test]
fn test_digit_10000_compat_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10000_compat(chinese_number_index, 0, &mut s, true);
    digit_10000_compat(chinese_number_index, 1, &mut s, true);
    digit_10000_compat(chinese_number_index, 10, &mut s, true);
    digit_10000_compat(chinese_number_index, 22, &mut s, true);
    digit_10000_compat(chinese_number_index, 333, &mut s, true);
    digit_10000_compat(chinese_number_index, 4444, &mut s, true);
    digit_10000_compat(chinese_number_index, 55555, &mut s, true);
    digit_10000_compat(chinese_number_index, 100000, &mut s, true);
    digit_10000_compat(chinese_number_index, 666066, &mut s, true);
    digit_10000_compat(chinese_number_index, 990909, &mut s, true);

    assert_eq!("一一十二十二三百三十三四千四百四十四五萬五千五百五十五一十萬六十六萬六千零六十六九十九萬零九百零九", s);
}

#[test]
fn test_digit_100000000_compat_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_100000000_compat(chinese_number_index, 0, &mut s, false);
    digit_100000000_compat(chinese_number_index, 1, &mut s, false);
    digit_100000000_compat(chinese_number_index, 10, &mut s, false);
    digit_100000000_compat(chinese_number_index, 2222, &mut s, false);
    digit_100000000_compat(chinese_number_index, 333333, &mut s, false);
    digit_100000000_compat(chinese_number_index, 44444444, &mut s, false);
    digit_100000000_compat(chinese_number_index, 5555555555, &mut s, false);
    digit_100000000_compat(chinese_number_index, 1000000000000000, &mut s, false);
    digit_100000000_compat(chinese_number_index, 9990099009900909, &mut s, false);

    assert_eq!("零一十二千二百二十二三十三萬三千三百三十三四千四百四十四萬四千四百四十四五十五億五千五百五十五萬五千五百五十五一千萬億九千九百九十萬零九百九十億零九百九十萬零九百零九", s);
}

#[test]
fn test_digit_100000000_compat_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_100000000_compat(chinese_number_index, 0, &mut s, true);
    digit_100000000_compat(chinese_number_index, 1, &mut s, true);
    digit_100000000_compat(chinese_number_index, 10, &mut s, true);
    digit_100000000_compat(chinese_number_index, 2222, &mut s, true);
    digit_100000000_compat(chinese_number_index, 333333, &mut s, true);
    digit_100000000_compat(chinese_number_index, 44444444, &mut s, true);
    digit_100000000_compat(chinese_number_index, 5555555555, &mut s, true);
    digit_100000000_compat(chinese_number_index, 1000000000000000, &mut s, true);
    digit_100000000_compat(chinese_number_index, 9990099009900909, &mut s, true);

    assert_eq!("一一十二千二百二十二三十三萬三千三百三十三四千四百四十四萬四千四百四十四五十五億五千五百五十五萬五千五百五十五一千萬億九千九百九十萬零九百九十億零九百九十萬零九百零九", s);
}

#[test]
fn test_digit_10000000000000000_compat_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10000000000000000_compat(chinese_number_index, 0, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 1, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 10, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 222222, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 3333333333, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 44444444444444, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 555555555555555555, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 1000000000000000000000000000, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 9999900009999000099990000909, &mut s, false);
    digit_10000000000000000_compat(chinese_number_index, 10000000000000000000000000000000, &mut s, false);

    assert_eq!("零一十二十二萬二千二百二十二三十三億三千三百三十三萬三千三百三十三四十四萬四千四百四十四億四千四百四十四萬四千四百四十四五十五兆五千五百五十五萬五千五百五十五億五千五百五十五萬五千五百五十五一千億兆九千九百九十九億九千萬零九百九十九兆九千萬零九百九十九億九千萬零九百零九一千萬億兆", s);
}

#[test]
fn test_digit_10000000000000000_compat_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_10000000000000000_compat(chinese_number_index, 0, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 1, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 10, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 222222, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 3333333333, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 44444444444444, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 555555555555555555, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 1000000000000000000000000000, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 9999900009999000099990000909, &mut s, true);
    digit_10000000000000000_compat(chinese_number_index, 10000000000000000000000000000000, &mut s, true);

    assert_eq!("一一十二十二萬二千二百二十二三十三億三千三百三十三萬三千三百三十三四十四萬四千四百四十四億四千四百四十四萬四千四百四十四五十五兆五千五百五十五萬五千五百五十五億五千五百五十五萬五千五百五十五一千億兆九千九百九十九億九千萬零九百九十九兆九千萬零九百九十九億九千萬零九百零九一千萬億兆", s);
}

#[test]
fn test_digit_100000000000000000000000000000000_compat_independently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_100000000000000000000000000000000_compat(chinese_number_index, 0, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 1, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 10, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 22222222, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 33333333333333, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 44444444444444444444, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 55555555555555555555555555, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 100000000000000000000000000000000000000, &mut s, false);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 333333000000333333000000333333000000303, &mut s, false);

    assert_eq!("零一十二千二百二十二萬二千二百二十二三十三萬三千三百三十三億三千三百三十三萬三千三百三十三四千四百四十四兆四千四百四十四萬四千四百四十四億四千四百四十四萬四千四百四十四五十五億五千五百五十五萬五千五百五十五兆五千五百五十五萬五千五百五十五億五千五百五十五萬五千五百五十五一百萬京三百三十三萬三千三百三十京零三百三十三億三千三百三十萬兆零三百三十三萬三千三百三十億零三百零三", s);
}

#[test]
fn test_digit_100000000000000000000000000000000_compat_dependently() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_100000000000000000000000000000000_compat(chinese_number_index, 0, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 1, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 10, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 22222222, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 33333333333333, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 44444444444444444444, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 55555555555555555555555555, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 100000000000000000000000000000000000000, &mut s, true);
    digit_100000000000000000000000000000000_compat(chinese_number_index, 333333000000333333000000333333000000303, &mut s, true);

    assert_eq!("一一十二千二百二十二萬二千二百二十二三十三萬三千三百三十三億三千三百三十三萬三千三百三十三四千四百四十四兆四千四百四十四萬四千四百四十四億四千四百四十四萬四千四百四十四五十五億五千五百五十五萬五千五百五十五兆五千五百五十五萬五千五百五十五億五千五百五十五萬五千五百五十五一百萬京三百三十三萬三千三百三十京零三百三十三億三千三百三十萬兆零三百三十三萬三千三百三十億零三百零三", s);
}

#[test]
fn test_digit_compat_low_u32() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_low_u32(chinese_number_index, 0, &mut s);
    digit_compat_low_u32(chinese_number_index, 1, &mut s);
    digit_compat_low_u32(chinese_number_index, 10, &mut s);
    digit_compat_low_u32(chinese_number_index, 99999, &mut s);
    digit_compat_low_u32(chinese_number_index, 100000, &mut s);
    digit_compat_low_u32(chinese_number_index, u32::max_value(), &mut s);

    assert_eq!("零一十九萬九千九百九十九一億四秭二垓九京四兆九億六萬七千二百九十五", s);
}

#[test]
fn test_digit_compat_low_u64() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_low_u64(chinese_number_index, 0, &mut s);
    digit_compat_low_u64(chinese_number_index, 1, &mut s);
    digit_compat_low_u64(chinese_number_index, 10, &mut s);
    digit_compat_low_u64(chinese_number_index, 99999, &mut s);
    digit_compat_low_u64(chinese_number_index, 100000, &mut s);
    digit_compat_low_u64(chinese_number_index, 100001, &mut s);
    digit_compat_low_u64(chinese_number_index, 1000000000000000, &mut s);
    digit_compat_low_u64(chinese_number_index, 1001000000000101, &mut s);

    assert_eq!("零一十九萬九千九百九十九一億一億零一一極一極零一澗零一百零一", s);
}

#[test]
fn test_digit_compat_ten_thousand_u32() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_ten_thousand_u32(chinese_number_index, 0, &mut s);
    digit_compat_ten_thousand_u32(chinese_number_index, 1, &mut s);
    digit_compat_ten_thousand_u32(chinese_number_index, 10, &mut s);
    digit_compat_ten_thousand_u32(chinese_number_index, 99999, &mut s);
    digit_compat_ten_thousand_u32(chinese_number_index, 100000, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 1000000001, &mut s);
    digit_compat_ten_thousand_u32(chinese_number_index, u32::max_value(), &mut s);

    assert_eq!("零一十九萬九千九百九十九十萬十億零一四十二億九千四百九十六萬七千二百九十五", s);
}

#[test]
fn test_digit_compat_ten_thousand_u64() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_ten_thousand_u64(chinese_number_index, 0, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 1, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 10, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 99999, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 100000, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 1000000001, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 1000000000000000, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, 1001000000000101, &mut s);
    digit_compat_ten_thousand_u64(chinese_number_index, u64::max_value(), &mut s);

    assert_eq!("零一十九萬九千九百九十九十萬十億零一一千兆一千零一兆零一百零一一千八百四十四京六千七百四十四兆零七百三十七億零九百五十五萬一千六百一十五", s);
}

#[test]
fn test_digit_compat_ten_thousand_u128() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_ten_thousand_u128(chinese_number_index, 0, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 1, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 10, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 99999, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 100000, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 1000000001, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 1000000000000000, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, 1001000000000101, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, u64::max_value() as u128, &mut s);
    digit_compat_ten_thousand_u128(chinese_number_index, u128::max_value(), &mut s);

    assert_eq!("零一十九萬九千九百九十九十萬十億零一一千兆一千零一兆零一百零一一千八百四十四京六千七百四十四兆零七百三十七億零九百五十五萬一千六百一十五三百四十澗二千八百二十三溝六千六百九十二穰零九百三十八秭四千六百三十四垓六千三百三十七京四千六百零七兆四千三百一十七億六千八百二十一萬一千四百五十五", s);
}

#[test]
fn test_digit_compat_middle_u64() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_middle_u64(chinese_number_index, 0, &mut s);
    digit_compat_middle_u64(chinese_number_index, 1, &mut s);
    digit_compat_middle_u64(chinese_number_index, 10, &mut s);
    digit_compat_middle_u64(chinese_number_index, 99999, &mut s);
    digit_compat_middle_u64(chinese_number_index, 100000, &mut s);
    digit_compat_middle_u64(chinese_number_index, 1000000001, &mut s);
    digit_compat_middle_u64(chinese_number_index, 1000000000000000, &mut s);
    digit_compat_middle_u64(chinese_number_index, 1001000000000101, &mut s);
    digit_compat_middle_u64(chinese_number_index, u64::max_value(), &mut s);

    assert_eq!("零一十九萬九千九百九十九十萬十億零一一千萬億一千零一萬億零一百零一一千八百四十四兆六千七百四十四萬零七百三十七億零九百五十五萬一千六百一十五", s);
}

#[test]
fn test_digit_compat_middle_u128() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    digit_compat_middle_u128(chinese_number_index, 0, &mut s);
    digit_compat_middle_u128(chinese_number_index, 1, &mut s);
    digit_compat_middle_u128(chinese_number_index, 10, &mut s);
    digit_compat_middle_u128(chinese_number_index, 99999, &mut s);
    digit_compat_middle_u128(chinese_number_index, 100000, &mut s);
    digit_compat_middle_u128(chinese_number_index, 1000000001, &mut s);
    digit_compat_middle_u128(chinese_number_index, 1000000000000000, &mut s);
    digit_compat_middle_u128(chinese_number_index, 1001000000000101, &mut s);
    digit_compat_middle_u128(chinese_number_index, u64::max_value() as u128, &mut s);
    digit_compat_middle_u128(chinese_number_index, u128::max_value(), &mut s);

    assert_eq!("零一十九萬九千九百九十九十萬十億零一一千萬億一千零一萬億零一百零一一千八百四十四兆六千七百四十四萬零七百三十七億零九百五十五萬一千六百一十五三百四十萬二千八百二十三垓六千六百九十二萬零九百三十八京四千六百三十四萬六千三百三十七兆四千六百零七萬四千三百一十七億六千八百二十一萬一千四百五十五", s);
}

#[test]
fn test_fraction_compat_low() {
    let mut s = String::new();

    let chinese_number_index = get_chinese_number_index(ChineseVariant::Traditional, ChineseNumberCase::Lower);

    fraction_compat_low(chinese_number_index, 0f64, &mut s);
    fraction_compat_low(chinese_number_index, 0.01f64, &mut s);
    fraction_compat_low(chinese_number_index, 0.1f64, &mut s);
    fraction_compat_low(chinese_number_index, 0.55f64, &mut s);

    assert_eq!("零一分一角五角五分", s);
}

#[test]
fn test_chinese_digit_1() {
    assert_eq!(0, chinese_digit_1('零').unwrap());
    assert_eq!(1, chinese_digit_1('壹').unwrap());
    assert_eq!(2, chinese_digit_1('贰').unwrap());
    assert_eq!(3, chinese_digit_1('參').unwrap());
    assert_eq!(4, chinese_digit_1('四').unwrap());
    assert_eq!(5, chinese_digit_1('五').unwrap());
    assert_eq!(6, chinese_digit_1('陸').unwrap());
    assert_eq!(7, chinese_digit_1('七').unwrap());
    assert_eq!(8, chinese_digit_1('八').unwrap());
    assert_eq!(9, chinese_digit_1('玖').unwrap());
}

#[test]
fn test_chinese_digit_10() {
    assert_eq!(10, chinese_digit_10('十', None, None).unwrap());
    assert_eq!(10, chinese_digit_10('壹', Some('十'), None).unwrap());
    assert_eq!(25, chinese_digit_10('贰', Some('拾'), Some('五')).unwrap());
}

#[test]
fn test_chinese_digit_100() {
    assert_eq!(100, chinese_digit_100('壹', '百', None, None, None).unwrap());
    assert_eq!(204, chinese_digit_100('二', '百', Some('零'), Some('四'), None).unwrap());
    assert_eq!(380, chinese_digit_100('三', '百', Some('八'), Some('十'), None).unwrap());
    assert_eq!(999, chinese_digit_100('九', '佰', Some('玖'), Some('拾'), Some('玖')).unwrap());
}

#[test]
fn test_chinese_digit_1000() {
    assert_eq!(1000, chinese_digit_1000('壹', '千', None, None, None, None, None).unwrap());
    assert_eq!(1001, chinese_digit_1000('一', '仟', Some('零'), Some('壹'), None, None, None).unwrap());
    assert_eq!(1010, chinese_digit_1000('一', '仟', Some('零'), Some('壹'), Some('拾'), None, None).unwrap());
    assert_eq!(1011, chinese_digit_1000('一', '仟', Some('零'), Some('壹'), Some('拾'), Some('一'), None).unwrap());
    assert_eq!(2300, chinese_digit_1000('二', '千', Some('三'), Some('百'), None, None, None).unwrap());
    assert_eq!(2301, chinese_digit_1000('二', '千', Some('三'), Some('百'), Some('零'), Some('一'), None).unwrap());
    assert_eq!(2320, chinese_digit_1000('二', '千', Some('三'), Some('百'), Some('二'), Some('十'), None).unwrap());
    assert_eq!(9999, chinese_digit_1000('九', '千', Some('玖'), Some('佰'), Some('玖'), Some('十'), Some('玖')).unwrap());
}