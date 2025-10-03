use crate::cdl::engines::internal::CdlApiInternal;
use crate::Pattern;
use crate::Signal;
use crate::{Candle, Error, SimpleCandle};

/// The main struct for candlestick pattern analysis.
///
/// It provides the [`Cdl::pattern()`] method to scan data.
/// An instance is obtained from the top-level [`crate::cdl()`] function.
///
/// # Configuration
///
/// **Important:** The library uses a global state for its settings.
/// If you require custom parameters (e.g., to define the size of a "long" candle),
/// you **must** call the [`crate::cdl::engines::talib::engine::configure`] function
/// **once** at application startup, before calling [`crate::cdl()`] and before
/// spawning any other threads that use this crate.
///
/// If not explicitly configured, balanced default settings built into this crate will be used.
pub struct Cdl {
    internal: &'static (dyn CdlApiInternal + Send + Sync),
}

impl Cdl {
    pub(crate) fn new(internal: &'static (dyn CdlApiInternal + Send + Sync)) -> Self {
        Cdl { internal }
    }

    /// Scans a slice of candles for a specific pattern.
    ///
    /// Returns a `Vec<Option<Signal>>` of the same length as the input `candles` slice.
    /// `Some(Signal)` indicates that a pattern was found at the corresponding candle. `None`
    /// indicates that no pattern was found.
    ///
    /// # Arguments
    ///
    /// * `pattern`: A [`Pattern`] enum variant specifying which pattern to look for.
    /// * `candles`: A slice of items that implement the [`Candle`] trait.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if a calculation error occurs in the underlying engine or if
    /// the input candle data is invalid.
    pub fn pattern<C: Candle>(
        &self,
        pattern: Pattern,
        candles: &[C],
    ) -> Result<Vec<Option<Signal>>, Error> {
        let candles = candles
            .iter()
            .map(|candle| SimpleCandle::try_from_candle(candle.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let signals = self.internal.pattern(pattern, &candles)?;

        Ok(signals)
    }
}
