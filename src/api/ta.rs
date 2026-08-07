use crate::engines::talib::ta::engine::TaLibEngine;
use crate::engines::internal::TaApiInternal;
use crate::{Candle, Error};
use std::num::NonZeroU16;

pub fn atr<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<Vec<f64>, Error> {
    TaLibEngine::atr(candles, period)
}