/// Candle types and traits.
pub mod candles;
/// The main `Cdl` analyzer struct.
#[cfg(feature = "cdl")]
pub mod cdl;
/// Error types for the library.
pub mod error;
/// The `Pattern` enum.
#[cfg(feature = "cdl")]
pub mod patterns;
/// Configuration `Settings` struct.
#[cfg(feature = "cdl")]
pub mod settings;
/// `Signal` and `Quality` structs.
#[cfg(feature = "cdl")]
pub mod signal;
#[cfg(feature = "ta")]
pub mod ta;
#[cfg(feature = "ta")]
pub mod results;

#[cfg(feature = "cdl")]
pub use crate::engines::talib::cdl::engine::configure;
pub use candles::*;
pub use error::*;

#[cfg(feature = "cdl")]
pub use cdl::*;
#[cfg(feature = "cdl")]
pub use patterns::*;
#[cfg(feature = "cdl")]
pub use settings::*;
#[cfg(feature = "cdl")]
pub use signal::*;

#[cfg(feature = "ta")]
pub use ta::*;
