pub(crate) static CHINESE_NUMBERS_TRADITIONAL_LOWER: [&str; 25] = [
    "零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "百", "千", "萬", "億", "兆",
    "京", "垓", "秭", "穰", "溝", "澗", "正", "載", "極",
];

pub(crate) static CHINESE_NUMBERS_TRADITIONAL_UPPER: [&str; 25] = [
    "零", "壹", "貳", "參", "肆", "伍", "陸", "柒", "捌", "玖", "拾", "佰", "仟", "萬", "億", "兆",
    "京", "垓", "秭", "穰", "溝", "澗", "正", "載", "極",
];

pub(crate) static CHINESE_NUMBERS_SIMPLE_LOWER: [&str; 25] = [
    "零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "百", "千", "万", "亿", "兆",
    "京", "垓", "秭", "穰", "沟", "涧", "正", "载", "极",
];

pub(crate) static CHINESE_NUMBERS_SIMPLE_UPPER: [&str; 25] = [
    "零", "壹", "贰", "参", "肆", "伍", "陆", "柒", "捌", "玖", "拾", "佰", "仟", "万", "亿", "兆",
    "京", "垓", "秭", "穰", "沟", "涧", "正", "载", "极",
];

pub(crate) type ChineseNumberTable = &'static [&'static str; 25];

pub(crate) static CHINESE_NUMBERS_FRACTION: [&str; 2] = ["角", "分"];

pub(crate) const CHINESE_NEGATIVE_TRADITIONAL: &str = "負";
pub(crate) const CHINESE_NEGATIVE_SIMPLE: &str = "负";

// TODO

pub(crate) static CHINESE_NUMBERS_CHARS: [[char; 4]; 25] = [
    ['零', '零', '零', '零'],
    ['壹', '一', '壹', '一'],
    ['貳', '二', '贰', '二'],
    ['參', '三', '参', '三'],
    ['肆', '四', '肆', '四'],
    ['伍', '五', '伍', '五'],
    ['陸', '六', '陆', '六'],
    ['柒', '七', '柒', '七'],
    ['捌', '八', '捌', '八'],
    ['玖', '九', '玖', '九'],
    ['拾', '十', '拾', '十'],
    ['佰', '百', '佰', '百'],
    ['仟', '千', '仟', '千'],
    ['萬', '萬', '万', '万'],
    ['億', '億', '亿', '亿'],
    ['兆', '兆', '兆', '兆'],
    ['京', '京', '京', '京'],
    ['垓', '垓', '垓', '垓'],
    ['秭', '秭', '秭', '秭'],
    ['穰', '穰', '穰', '穰'],
    ['溝', '溝', '沟', '沟'],
    ['澗', '澗', '涧', '涧'],
    ['正', '正', '正', '正'],
    ['載', '載', '载', '载'],
    ['極', '極', '极', '极'],
];

pub(crate) const ONE_TENTH_CHAR: char = '角';
pub(crate) const ONE_HUNDRED_PERCENT: char = '分';

pub(crate) static CHINESE_NEGATIVE_SIGN_CHARS: [char; 2] = ['負', '负'];
