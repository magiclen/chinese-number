/// 根據 **五經算術** 將大的單位分為上數、中數、下數三種類型。
pub enum ChineseBigNumberCountMethod {
    /// 下數者，十十變之。若言十萬曰億，十億曰兆，十兆曰京也。
    Low,
    /// 中數者，萬萬變之。若言萬萬曰億，萬億曰兆，萬兆曰京也。
    Middle,
    // TODO: 上數(High)
}