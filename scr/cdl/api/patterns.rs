/// An enumeration of all supported candlestick patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pattern {
    // --- Single directional patterns ---
    /// Hammer
    Hammer,
    /// Inverted Hammer
    InvertedHammer,
    /// Three White Soldiers
    ThreeWhiteSoldiers,
    /// Morning Star
    MorningStar,
    /// Piercing Line
    PiercingLine,
    /// DragonFly Doji
    DragonFly,

    // --- Bearish Reversal ---
    /// Hanging Man
    HangingMan,
    /// Shooting Star
    ShootingStar,
    /// Three Black Crows
    ThreeBlackCrows,
    /// Evening Star
    EveningStar,
    /// Dark Cloud Cover
    DarkCloudCover,
    /// Gravestone Doji
    Gravestone,

    // --- Indecision patterns ---
    /// Doji
    Doji,
    /// Spinning Top
    SpinningTop,

    // --- Split double directional patterns ---
    /// Bullish Engulfing
    BullishEngulfing,
    /// Bullish Harami
    BullishHarami,
    /// Bullish Harami Cross
    BullishHaramiCross,
    /// Bullish Marubozu
    BullishMarubozu,
    /// Bullish Long Line
    BullishLongLine,
    /// Bullish Short Line
    BullishShortLine,
    /// Bullish Kicking
    BullishKicking,

    // --- Bearish Patterns ---
    /// Bearish Engulfing
    BearishEngulfing,
    /// Bearish Harami
    BearishHarami,
    /// Bearish Harami Cross
    BearishHaramiCross,
    /// Bearish Marubozu
    BearishMarubozu,
    /// Bearish Long Line
    BearishLongLine,
    /// Bearish Short Line
    BearishShortLine,
    /// Bearish Kicking
    BearishKicking,
}
