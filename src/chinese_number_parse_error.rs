#[derive(Debug, Eq, PartialEq)]
pub enum ChineseNumberParseError {
    ChineseNumberEmpty,
    ChineseNumberIncorrect {
        char_index: usize
    },
    Overflow,
    Underflow,
}