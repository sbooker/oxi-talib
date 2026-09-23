#[derive(Debug, Clone)]
pub struct IndicatorResult<T> {
    pub offset: usize,
    pub values: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend { Bullish, Bearish }
#[derive(Debug, Clone, Copy)]
pub struct SuperTrendResult {
    pub value: f64,
    pub trend: Trend,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BBandsResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}