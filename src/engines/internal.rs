use crate::api::results::{IndicatorResult, SuperTrendResult};
use crate::api::{Error, Pattern, Signal, SimpleCandle};
use std::num::NonZeroU16;

pub trait CdlApiInternal {
    fn pattern(
        &self,
        pattern: Pattern,
        candles: &[SimpleCandle],
    ) -> Result<Vec<Option<Signal>>, Error>;
}

pub trait TaApiInternal {
    fn atr(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error>;

    fn super_trend(candles: &[SimpleCandle], period: NonZeroU16, multiplier: NonZeroU16) -> Result<IndicatorResult<SuperTrendResult>, Error>;
}
