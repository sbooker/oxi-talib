#[cfg(any(feature = "cdl", feature = "ta"))]
use crate::api::Error;

#[cfg(feature = "cdl")]
use crate::api::{Pattern, Signal};

#[cfg(any(feature = "cdl", feature = "ta"))]
use crate::api::SimpleCandle;

#[cfg(feature = "ta")]
use crate::api::results::{IndicatorResult, SuperTrendResult};

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
    fn atr(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn adx(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;

    fn rsi(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;
    fn super_trend(candles: &[SimpleCandle], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error>;
}
