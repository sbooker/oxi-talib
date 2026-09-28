use crate::api::results::{BBandsResult, IndicatorResult, SuperTrendResult};
use crate::engines::internal::TaApiInternal;
use crate::engines::talib::ta::engine::TaLibEngine;
use crate::{Candle, Error, SimpleCandle};
use std::num::NonZeroU16;

pub fn sma(data: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::sma(data, period)
}
pub fn trange<C: Candle>(candles: &[C]) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::trange(SimpleCandle::try_map_candles(candles)?.as_slice())
}

pub fn stddev<C: Candle>(candles: &[C], period: NonZeroU16, number_of_deviations: f64) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::stddev(SimpleCandle::try_map_candles(candles)?.as_slice(), period, number_of_deviations)
}

pub fn atr<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::atr(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

pub fn adx<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::adx(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

pub fn rsi<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::rsi(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

pub fn super_trend<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error> {
    TaLibEngine::super_trend(SimpleCandle::try_map_candles(candles)?.as_slice(), period, multiplier)
}

pub fn bbands<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<BBandsResult>, Error> {
    TaLibEngine::bbands(SimpleCandle::try_map_candles(candles)?.as_slice(), period, multiplier)
}