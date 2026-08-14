use std::num::NonZeroU16;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// An error from the underlying calculation engine.
    #[error("Calculation error: {0}")]
    CalculationError(String),
    /// An attempt was made to configure the library more than once.
    #[error("Already Configured")]
    AlreadyConfigured,
    /// Provided candle data was invalid (e.g., `high < low`).
    #[error("Invalid Candle: {0}")]
    InvalidCandle(String),
    /// Argument Out of range
    #[error("Argument out of range:  {0}")]
    ArgumentOutOfRange(String),
    /// Insufficient input data
    #[error("Insufficient input data: input size {0}, period: {1}")]
    InsufficientInputData(usize, NonZeroU16),
}
