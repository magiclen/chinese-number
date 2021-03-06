/// 根據 **五經算術** 將大的單位分為上數、中數、下數三種類型，再加上現代使用的萬進。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChineseNumberCountMethod {
    /// 下數者，十十變之。若言十萬曰億，十億曰兆，十兆曰京也。
    Low,
    /// 萬進者，一萬變之。若言萬萬曰億，萬億曰兆，萬兆曰京也。
    TenThousand,
    /// 中數者，萬萬變之。若言萬萬曰億，萬萬億曰兆，萬萬兆曰京也。
    Middle,
    /// 上數者，數窮則變。若言萬萬曰億，億億曰兆、兆兆曰京也。
    High,
}
