use crate::api::results::{IndicatorResult, SuperTrendResult};
use crate::engines::internal::TaApiInternal;
use crate::engines::talib::ta::engine::TaLibEngine;
use crate::{Candle, Error, SimpleCandle};
use std::num::NonZeroU16;

pub fn atr<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::atr(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

pub fn adx<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
    TaLibEngine::atr(SimpleCandle::try_map_candles(candles)?.as_slice(), period)
}

pub fn super_trend<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error> {
    TaLibEngine::super_trend(SimpleCandle::try_map_candles(candles)?.as_slice(), period, multiplier)
}