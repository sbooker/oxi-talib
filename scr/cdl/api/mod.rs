/// Candle types and traits.
pub mod candles;
/// The main `Cdl` analyzer struct.
pub mod cdl;
/// Error types for the library.
pub mod error;
/// The `Pattern` enum.
pub mod patterns;
/// Configuration `Settings` struct.
pub mod settings;
/// `Signal` and `Quality` structs.
pub mod signal;

pub use crate::cdl::engines::talib::engine::configure;
pub use candles::*;
pub use cdl::*;
pub use error::*;
pub use patterns::*;
pub use settings::*;
pub use signal::*;
