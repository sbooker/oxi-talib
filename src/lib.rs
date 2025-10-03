#![warn(missing_docs)]
//! A library for candlestick pattern recognition in Rust.
//!
//! This crate provides an idiomatic and safe API for the technical analysis
//! of candlestick patterns.
//!
//! # Usage
//!
//! The main entry point is the [`cdl()`] function, which returns an analyzer instance.
//! All analysis is performed through the [`Cdl::pattern()`] method.
//!
//! You can use the built-in [`SimpleCandle`] struct for analysis or implement
//! the [`Candle`] trait for your own data types.
//!
//! # Example
//!
//! ```no_run
//! use oxi_talib::{cdl, Pattern, SimpleCandle};
//!
//! // Use the built-in SimpleCandle struct
//! let candles: Vec<SimpleCandle> = vec![
//!     SimpleCandle::try_new(100.0, 102.0, 103.0, 99.0).unwrap(),
//!     // ... other candles
//! ];
//!
//! // 0. Configure the library
//! //    (optional but required for custom settings)
//! let settings = oxi_talib::cdl::api::settings::Settings::default();
//! oxi_talib::configure(settings).unwrap();
//!
//! // 1. Get the analyzer
//! let analyzer = cdl();
//!
//! // 2. Run the analysis for the "Hammer" pattern
//! let signals = analyzer.pattern(Pattern::Hammer, &candles).unwrap();
//!
//! // 3. Process the results
//! for (i, signal) in signals.iter().enumerate() {
//!     if signal.is_some() {
//!         println!("Hammer pattern found at candle #{}!", i);
//!     }
//! }
//! ```
//!
//! # Configuration
//!
//! The library uses internal global parameters for its recognition algorithms.
//! See the documentation for [`cdl::engines::talib::engine::configure`] for details on how to set them.
//! This step is optional but required for custom settings. Configuration
//! must be performed once at startup, in a single-threaded context.
/// Candlestick pattern recognition API.
pub mod cdl;

pub use cdl::api::*;

use crate::cdl::engines::talib::engine::instance;

/// Returns an analyzer for candlestick pattern recognition.
///
/// This function provides a `Cdl` instance, which is the main interface
/// for all pattern analysis functions.
///
/// The underlying engine is initialized as a thread-safe singleton. The first call
/// to this function initializes the engine, while subsequent calls return a reference to the
/// same instance.
///
/// Custom engine settings can be applied via the
/// [`cdl::engines::talib::engine::configure`] function before the first call to `cdl()`.
/// If it is not called, balanced default settings built into this crate will be used.
pub fn cdl() -> Cdl {
    Cdl::new(instance())
}
