//! Technical analysis indicators.

use crate::api::results::{BBandsResult, IndicatorResult, SuperTrendResult};
use crate::engines::internal::TaApiInternal;
use crate::engines::talib::ta::engine::TaLibEngine;
use crate::engines::talib::IntoRows;
use crate::{Candle, Error, SimpleCandle};
use std::num::NonZeroU16;

/// Calculates the Simple Moving Average (SMA) of candle closing prices.
pub fn sma<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::sma(SimpleCandle::try_map_candles(candles)?.closes().as_slice(), period)
}

/// Calculates the True Range (TRANGE) for a series of candles.
pub fn trange<C: Candle>(candles: &[C]) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::trange(SimpleCandle::try_map_candles(candles)?.as_slice())
}

/// Calculates the Standard Deviation (STDDEV) of candle closing prices.
pub fn stddev<C: Candle>(candles: &[C], period: NonZeroU16, number_of_deviations: f64) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::stddev(SimpleCandle::try_map_candles(candles)?.as_slice(), period, number_of_deviations)
}

/// Calculates the Average True Range (ATR).
pub fn atr<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::atr(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

/// Calculates the Average Directional Movement Index (ADX).
pub fn adx<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::adx(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

/// Calculates the Relative Strength Index (RSI).
pub fn rsi<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::rsi(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

/// Calculates the SuperTrend indicator.
pub fn super_trend<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error> {
    TaLibEngine::super_trend(SimpleCandle::try_map_candles(candles)?.as_slice(), period, multiplier)
}

/// Calculates Bollinger Bands (BBANDS) on candle closing prices.
pub fn bbands<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<BBandsResult>, Error> {
    TaLibEngine::bbands(SimpleCandle::try_map_candles(candles)?.as_slice(), period, multiplier)
}

/// Mathematical and statistical calculations on numeric series.
pub mod math {
    use crate::api::results::IndicatorResult;
    use crate::engines::internal::TaApiInternal;
    use crate::engines::talib::ta::engine::TaLibEngine;
    use crate::Error;
    use std::num::NonZeroU16;

    /// Calculates the Simple Moving Average (SMA) on a slice of numbers.
    pub fn sma(data: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        TaLibEngine::sma(data, period)
    }

    /// Calculates Pearson's Correlation Coefficient (CORREL) between two data series.
    pub fn correl(data1: &[f64], data2: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        TaLibEngine::correl(data1, data2, period)
    }
}