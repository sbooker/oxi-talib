pub struct IndicatorResult<T> {
    pub offset: usize,
    pub values: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend { Bullish, Bearish }
pub struct SuperTrendResult {
    pub value: f64,
    pub trend: Trend,
}
