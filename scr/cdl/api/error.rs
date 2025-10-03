use crate::cdl::api::error::Error::*;
use std::fmt::Formatter;

/// An enum representing all possible errors in the library.
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying calculation engine.
    CalculationError(String),
    /// An attempt was made to configure the library more than once.
    AlreadyConfigured,
    /// Provided candle data was invalid (e.g., `high < low`).
    InvalidCandle(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculationError(r) => write!(f, "Calculation error: {r}"),
            AlreadyConfigured => write!(f, "Already Configured"),
            InvalidCandle(r) => write!(f, "Invalid Candle: {r}"),
        }
    }
}

impl std::error::Error for Error {}
