use crate::api::{Error, Pattern, Signal, SimpleCandle, Candle};
use std::num::NonZeroU16;

pub trait CdlApiInternal {
    fn pattern(
        &self,
        pattern: Pattern,
        candles: &[SimpleCandle],
    ) -> Result<Vec<Option<Signal>>, Error>;
}

pub trait TaApiInternal {
    fn atr<C: Candle>(candles: &[C], period: NonZeroU16) -> Result<Vec<f64>, Error>;
}
