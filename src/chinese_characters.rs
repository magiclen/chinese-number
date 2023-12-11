use enum_ordinalize::Ordinalize;

#[cfg(feature = "number-to-chinese")]
use crate::{ChineseCase, ChineseVariant};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[ordinalize(impl_trait = false)]
#[ordinalize(ordinal(pub(crate) fn ordinal))]
#[ordinalize(from_ordinal_unsafe(pub(crate) fn from_ordinal_unsafe))]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum ChineseNumber {
    零,
    一,
    二,
    三,
    四,
    五,
    六,
    七,
    八,
    九,
    十,
}

impl ChineseNumber {
    #[cfg(feature = "number-to-chinese")]
    #[inline]
    pub(crate) const fn to_str(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> &'static str {
        match self {
            Self::零 => "零",
            Self::一 => match chinese_case {
                ChineseCase::Upper => "壹",
                ChineseCase::Lower => "一",
            },
            Self::二 => match chinese_case {
                ChineseCase::Upper => match chinese_variant {
                    ChineseVariant::Traditional => "貳",
                    ChineseVariant::Simple => "贰",
                },
                ChineseCase::Lower => "二",
            },
            Self::三 => match chinese_case {
                ChineseCase::Upper => match chinese_variant {
                    ChineseVariant::Traditional => "參",
                    ChineseVariant::Simple => "叁",
                },
                ChineseCase::Lower => "三",
            },
            Self::四 => match chinese_case {
                ChineseCase::Upper => "肆",
                ChineseCase::Lower => "四",
            },
            Self::五 => match chinese_case {
                ChineseCase::Upper => "伍",
                ChineseCase::Lower => "五",
            },
            Self::六 => match chinese_case {
                ChineseCase::Upper => match chinese_variant {
                    ChineseVariant::Traditional => "陸",
                    ChineseVariant::Simple => "陆",
                },
                ChineseCase::Lower => "六",
            },
            Self::七 => match chinese_case {
                ChineseCase::Upper => "柒",
                ChineseCase::Lower => "七",
            },
            Self::八 => match chinese_case {
                ChineseCase::Upper => "捌",
                ChineseCase::Lower => "八",
            },
            Self::九 => match chinese_case {
                ChineseCase::Upper => "玖",
                ChineseCase::Lower => "九",
            },
            Self::十 => match chinese_case {
                ChineseCase::Upper => "拾",
                ChineseCase::Lower => "十",
            },
        }
    }

    #[cfg(feature = "chinese-to-number")]
    #[inline]
    pub(crate) const fn from_char(character: char) -> Option<Self> {
        match character {
            '零' | '0' | '〇' => Some(Self::零),
            '一' | '壹' | '1' => Some(Self::一),
            '二' | '貳' | '贰' | '貮' | '兩' | '两' | '2' => Some(Self::二),
            '三' | '參' | '叁' | '叄' | '参' | '3' => Some(Self::三),
            '四' | '肆' | '4' => Some(Self::四),
            '五' | '伍' | '5' => Some(Self::五),
            '六' | '陸' | '陆' | '6' => Some(Self::六),
            '七' | '柒' | '7' => Some(Self::七),
            '八' | '捌' | '8' => Some(Self::八),
            '九' | '玖' | '9' => Some(Self::九),
            '十' | '拾' => Some(Self::十),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Ordinalize)]
#[ordinalize(impl_trait = false)]
#[ordinalize(ordinal(pub(crate) fn ordinal))]
#[ordinalize(from_ordinal_unsafe(pub(crate) fn from_ordinal_unsafe))]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum ChineseExponent {
    分,
    角,
    個,
    十,
    百,
    千,
    萬,
    億,
    兆,
    京,
    垓,
    秭,
    穰,
    溝,
    澗,
    正,
    載,
    極,
}

impl ChineseExponent {
    #[cfg(feature = "number-to-chinese")]
    #[inline]
    pub(crate) const fn to_str(
        self,
        chinese_variant: ChineseVariant,
        chinese_case: ChineseCase,
    ) -> &'static str {
        match self {
            Self::分 => "分",
            Self::角 => "角",
            Self::個 => match chinese_variant {
                ChineseVariant::Traditional => "個",
                ChineseVariant::Simple => "个",
            },
            Self::十 => match chinese_case {
                ChineseCase::Upper => "拾",
                ChineseCase::Lower => "十",
            },
            Self::百 => match chinese_case {
                ChineseCase::Upper => "佰",
                ChineseCase::Lower => "百",
            },
            Self::千 => match chinese_case {
                ChineseCase::Upper => "仟",
                ChineseCase::Lower => "千",
            },
            Self::萬 => match chinese_variant {
                ChineseVariant::Traditional => "萬",
                ChineseVariant::Simple => "万",
            },
            Self::億 => match chinese_variant {
                ChineseVariant::Traditional => "億",
                ChineseVariant::Simple => "亿",
            },
            Self::兆 => "兆",
            Self::京 => "京",
            Self::垓 => "垓",
            Self::秭 => "秭",
            Self::穰 => "穰",
            Self::溝 => match chinese_variant {
                ChineseVariant::Traditional => "溝",
                ChineseVariant::Simple => "沟",
            },
            Self::澗 => match chinese_variant {
                ChineseVariant::Traditional => "澗",
                ChineseVariant::Simple => "涧",
            },
            Self::正 => "正",
            Self::載 => match chinese_variant {
                ChineseVariant::Traditional => "載",
                ChineseVariant::Simple => "载",
            },
            Self::極 => match chinese_variant {
                ChineseVariant::Traditional => "極",
                ChineseVariant::Simple => "极",
            },
        }
    }

    #[cfg(feature = "chinese-to-number")]
    #[inline]
    pub(crate) const fn from_char(character: char) -> Option<Self> {
        match character {
            '分' => Some(Self::分),
            '角' => Some(Self::角),
            '個' | '个' => Some(Self::個),
            '十' | '拾' => Some(Self::十),
            '百' | '佰' => Some(Self::百),
            '千' | '仟' => Some(Self::千),
            '萬' | '万' => Some(Self::萬),
            '億' | '亿' => Some(Self::億),
            '兆' => Some(Self::兆),
            '京' => Some(Self::京),
            '垓' => Some(Self::垓),
            '秭' => Some(Self::秭),
            '穰' => Some(Self::穰),
            '溝' | '沟' => Some(Self::溝),
            '澗' | '涧' => Some(Self::澗),
            '正' => Some(Self::正),
            '載' | '载' => Some(Self::載),
            '極' | '极' => Some(Self::極),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum ChineseSign {
    正,
    負,
}

impl ChineseSign {
    #[cfg(feature = "number-to-chinese")]
    #[inline]
    pub(crate) const fn to_str(self, chinese_variant: ChineseVariant) -> &'static str {
        match self {
            Self::正 => "正",
            Self::負 => match chinese_variant {
                ChineseVariant::Traditional => "負",
                ChineseVariant::Simple => "负",
            },
        }
    }

    #[cfg(feature = "chinese-to-number")]
    #[inline]
    pub(crate) const fn from_char(character: char) -> Option<Self> {
        match character {
            '正' => Some(Self::正),
            '負' | '负' => Some(Self::負),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct ChinesePoint;

impl ChinesePoint {
    #[cfg(feature = "number-to-chinese")]
    #[inline]
    pub(crate) const fn to_str(chinese_variant: ChineseVariant) -> &'static str {
        match chinese_variant {
            ChineseVariant::Traditional => "點",
            ChineseVariant::Simple => "点",
        }
    }

    #[cfg(feature = "chinese-to-number")]
    #[inline]
    pub(crate) const fn from_char(character: char) -> Option<Self> {
        match character {
            '點' | '点' => Some(ChinesePoint),
            _ => None,
        }
    }
}
