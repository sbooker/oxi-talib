# oxi-talib

[Русская версия](README.ru.md)

[![Crates.io](https://img.shields.io/crates/v/oxi-talib.svg)](https://crates.io/crates/oxi-talib)
[![Docs.rs](https://docs.rs/oxi-talib/badge.svg)](https://docs.rs/oxi-talib)
[![CI](https://github.com/sbooker/oxi-talib/actions/workflows/rust.yaml/badge.svg)](https://github.com/sbooker/oxi-talib/actions)

**oxi-talib** is a Rust library for technical analysis and candlestick pattern recognition.

## Key Features

*   **Idiomatic API:** The interface uses standard Rust types (`Vec`, `Result`, `Option`, `NonZeroU16`).
*   **Broad Functionality:** Candlestick patterns and classic technical indicators (SMA, ATR, RSI, ADX, Bollinger Bands, SuperTrend).
*   **Configurability:** Candlestick recognition parameters can be modified.
*   **Safety:** The public API is 100% safe (`safe Rust`). All `unsafe` code required for C library interaction is encapsulated.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oxi-talib = "0.2.0"
```

Or run:
```bash
cargo add oxi-talib
```

### Cargo Features

The crate provides feature flags to pull in only what you need:
* `full` (*default*) — Enables all modules: candlestick patterns (`cdl`) and technical indicators (`ta`).
* `ta` — Only technical indicators (SMA, ATR, RSI, ADX, SuperTrend, Bollinger Bands, etc.).
* `cdl` — Only candlestick pattern recognition.
* `candles` — Core `Candle` trait and `SimpleCandle` struct without external FFI dependencies.

## Example Usage

### 1. Technical Indicators

```rust
use std::num::NonZeroU16;
use oxi_talib::{atr, super_trend, SimpleCandle, Error};

fn main() -> Result<(), Error> {
    let candles = vec![
        SimpleCandle::try_new(100.0, 105.0, 106.0, 98.0)?,
        SimpleCandle::try_new(105.0, 102.0, 107.0, 101.0)?,
        SimpleCandle::try_new(102.0, 103.0, 104.0, 95.0)?,
        SimpleCandle::try_new(103.0, 108.0, 109.0, 102.0)?,
        SimpleCandle::try_new(108.0, 107.0, 110.0, 105.0)?,
    ];

    let period = NonZeroU16::new(3).unwrap();

    // Average True Range (ATR)
    let atr_res = atr(&candles, period)?;
    println!("ATR offset: {}, values: {:?}", atr_res.offset, atr_res.values);

    // SuperTrend
    let st_res = super_trend(&candles, period, 3.0)?;
    for st in &st_res.values {
        println!("SuperTrend: value = {:.2}, trend = {:?}", st.value, st.trend);
    }

    Ok(())
}
```

### 2. Candlestick Patterns

```rust
use oxi_talib::{cdl, Pattern, SimpleCandle, Error};

fn main() -> Result<(), Error> {
    let candles: Vec<SimpleCandle> = vec![
        SimpleCandle::try_new(100.0, 105.0, 106.0, 98.0)?,
        SimpleCandle::try_new(105.0, 102.0, 107.0, 101.0)?,
        SimpleCandle::try_new(102.0, 103.0, 104.0, 95.0)?,
        SimpleCandle::try_new(103.0, 108.0, 109.0, 102.0)?,
    ];

    let signals = cdl().pattern(Pattern::Hammer, &candles)?;

    for (i, signal) in signals.iter().enumerate() {
        if let Some(s) = signal {
            println!(
                "Hammer pattern found at candle #{} with quality {}!",
                i, s.quality.value()
            );
        }
    }
    
    Ok(())
}
```

## Advanced Usage

### Implementing the `Candle` Trait

To work with your own data structures, implement the `Candle` trait:

```rust
use oxi_talib::{Candle, Pattern, cdl};

#[derive(Clone)]
struct MyData {
    open_price: f64,
    high_price: f64,
    low_price: f64,
    close_price: f64,
}

impl Candle for MyData {
    type Price = f64;
    fn open(&self) -> Self::Price { self.open_price }
    fn high(&self) -> Self::Price { self.high_price }
    fn low(&self) -> Self::Price { self.low_price }
    fn close(&self) -> Self::Price { self.close_price }
}
```

### Configuration

Candlestick recognition parameters can be modified via the `configure` function once at application startup:

```rust
use oxi_talib::configure;
use oxi_talib::Settings;

let mut settings = Settings::default();
settings.body_doji_factor = 0.2;

configure(settings).expect("Configuration should not be called more than once");
```

## Roadmap

*   **Stage 1: Extended `TA-Lib` API Coverage**
    *   [ ] Add remaining candlestick patterns (`CDL2CROWS`, `CDLABANDONEDBABY`, `CDL3INSIDE`, `CDL3LINESTRIKE`, etc.).
    *   [ ] Moving averages and trend indicators (EMA, WMA, DEMA, TEMA, MACD, Parabolic SAR, etc.).
    *   [ ] Oscillators and momentum indicators (Stochastic, CCI, Williams %R, ROC, MFI, etc.).
    *   [ ] Volume indicators (OBV, Chaikin A/D, etc.).
    *   [ ] Statistical functions (linear regression, variance, etc.).

*   **Stage 2: Native Rust Implementation**
    *   [ ] Gradually replace C function calls with equivalent Rust implementations to eliminate external C dependencies.

*   **Stage 3: Advanced Analysis**
    *   [ ] Implement a `Quality` (shape correctness) and `Strength` (contextual significance) scoring system.

## License

[MIT License](LICENSE).