//! Result types returned by technical indicator calculations.

/// Result of an indicator calculation containing calculated values and lookback offset.
#[derive(Debug, Clone)]
pub struct IndicatorResult<T> {
    /// The index offset in the input slice where valid calculated values begin.
    pub offset: usize,
    /// The calculated indicator values.
    pub values: Vec<T>,
}

/// Market trend direction identified by trend indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    /// Upward (bullish) trend.
    Bullish,
    /// Downward (bearish) trend.
    Bearish,
}

/// Result of the SuperTrend indicator calculation for a candle.
#[derive(Debug, Clone, Copy)]
pub struct SuperTrendResult {
    /// The SuperTrend threshold value.
    pub value: f64,
    /// The detected trend direction.
    pub trend: Trend,
}

/// Result of the Bollinger Bands calculation for a candle.
#[derive(Debug, Clone, Copy, Default)]
pub struct BBandsResult {
    /// Upper band value.
    pub upper: f64,
    /// Middle band value (moving average).
    pub middle: f64,
    /// Lower band value.
    pub lower: f64,
}