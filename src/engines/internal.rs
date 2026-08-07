#[cfg(any(feature = "cdl", feature = "ta"))]
use crate::api::Error;

#[cfg(feature = "cdl")]
use crate::api::{Pattern, Signal};

#[cfg(any(feature = "cdl"))]
use crate::api::SimpleCandle;

#[cfg(feature = "ta")]
use crate::api::results::BBandsResult;
#[cfg(feature = "ta")]
use crate::api::results::{IndicatorResult, SuperTrendResult};
use crate::Candle;
#[cfg(feature = "ta")]
use std::num::NonZeroU16;

#[cfg(feature = "cdl")]
pub trait CdlApiInternal {
    fn pattern(
        &self,
        pattern: Pattern,
        candles: &[SimpleCandle],
    ) -> Result<Vec<Option<Signal>>, Error>;
}

#[cfg(feature = "ta")]
pub trait TaApiInternal {
    fn trange<C: Candle>(candles: &[C]) -> Result<IndicatorResult<f64>, Error>;
    fn stddev<C: Candle>(candles: &[C], period: NonZeroU16, number_of_deviations: f64) -> Result<IndicatorResult<f64>, Error>;
    fn correl(data1: &[f64], data2: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn sma(data: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn atr<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn adx<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn rsi<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn super_trend<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error>;
    fn bbands<C: Candle>(candles: &[C], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<BBandsResult>, Error>;
}
