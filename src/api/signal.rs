/// Pattern quality score (`u8`).
///
/// The value is guaranteed to be between 1 and 100, inclusive.
/// Construction fails if the input score is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quality(u8);

impl Quality {
    const MAX: u8 = 100;

    pub(crate) fn try_new(score: u8) -> Option<Self> {
        if score > 0 {
            Some(Self(score.min(Self::MAX)))
        } else {
            None
        }
    }

    /// Returns the inner `u8` value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// A signal indicating a detected candlestick pattern.
///
/// Instances of this struct are created by the library and returned
/// from the [`crate::Cdl::pattern`] function. Direct construction by a user is not intended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signal {
    /// The quality score of the signal, from 1 to 100.
    pub quality: Quality,
}

impl Signal {
    pub(crate) fn new(quality: Quality) -> Self {
        Self { quality }
    }

    pub(crate) fn try_from_quality(score: u8) -> Option<Self> {
        let quality = Quality::try_new(score)?;

        Some(Self::new(quality))
    }
}
