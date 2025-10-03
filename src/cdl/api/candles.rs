use crate::cdl::api::error::Error;
use crate::cdl::api::error::Error::*;

/// A trait for types that represent a candlestick.
///
/// Implement this trait for your own data structures to make them compatible
/// with the analysis functions.
pub trait Candle: Clone {
    /// The numeric type for price values. Must be convertible to `f64`.
    type Price: Into<f64> + Copy;

    /// The opening price.
    fn open(&self) -> Self::Price;
    /// The closing price.
    fn close(&self) -> Self::Price;
    /// The highest price.
    fn high(&self) -> Self::Price;
    /// The lowest price.
    fn low(&self) -> Self::Price;
}

/// A basic, validated implementation of the [`Candle`] trait.
#[derive(Clone)]
pub struct SimpleCandle {
    open: f64,
    close: f64,
    high: f64,
    low: f64,
}

impl SimpleCandle {
    /// Creates a `SimpleCandle` if the price values are valid.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidCandle`] if `high < low` or if `open` or `close`
    /// are outside the `[low, high]` range.
    pub fn try_new(open: f64, close: f64, high: f64, low: f64) -> Result<Self, Error> {
        if high < low {
            return Err(InvalidCandle(
                "a high price must be greater or equal than a low price".into(),
            ));
        }
        if close < low || close > high {
            return Err(InvalidCandle(
                "a close price must be between a low and a high price".into(),
            ));
        }
        if open < low || open > high {
            return Err(InvalidCandle(
                "an open price must be between a low and a high price".into(),
            ));
        }

        Ok(Self {
            open,
            close,
            high,
            low,
        })
    }

    /// Converts a type implementing [`Candle`] into a `SimpleCandle`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidCandle`] if the source candle's prices are not valid.
    pub fn try_from_candle<C: Candle>(candle: C) -> Result<Self, Error> {
        Self::try_new(
            candle.open().into(),
            candle.close().into(),
            candle.high().into(),
            candle.low().into(),
        )
    }
}

impl Candle for SimpleCandle {
    type Price = f64;

    fn open(&self) -> Self::Price {
        self.open
    }

    fn close(&self) -> Self::Price {
        self.close
    }

    fn high(&self) -> Self::Price {
        self.high
    }

    fn low(&self) -> Self::Price {
        self.low
    }
}
