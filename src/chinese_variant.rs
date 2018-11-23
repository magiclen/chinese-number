/// 繁體中文或簡體中文。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChineseVariant {
    /// 繁體中文。
    Traditional,
    /// 簡體中文。
    Simple,
}