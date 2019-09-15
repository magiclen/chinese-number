/// 大寫或小寫數字。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChineseNumberCase {
    /// 大寫數字。
    Upper,
    /// 小寫數字。
    Lower,
}
