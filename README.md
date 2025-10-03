# oxi-talib

[Русская версия](README.ru.md)

[![Crates.io](https://img.shields.io/crates/v/oxi-talib.svg)](https://crates.io/crates/oxi-talib)
[![Docs.rs](https://docs.rs/oxi-talib/badge.svg)](https://docs.rs/oxi-talib)
[![CI](https://github.com/sbooker/oxi-talib/actions/workflows/rust.yaml/badge.svg)](https://github.com/sbooker/oxi-talib/actions)

**oxi-talib** is a Rust library for candlestick pattern recognition.

## Key Features

*   **API:** The interface uses standard Rust types (`Vec`, `Result`, `Option`).
*   **Configurability:** The parameters of the recognition algorithms can be modified.
*   **Safety:** The public API is 100% safe. All `unsafe` code required for C library interaction is encapsulated.

## Installation

### 1. Dependency: TA-Lib

The library depends on the C library `TA-Lib`, which must be installed on the system.

**Linux / macOS**
Build from source:
```bash
wget http://prdownloads.sourceforge.net/ta-lib/ta-lib-0.4.0-src.tar.gz
tar -xzf ta-lib-0.4.0-src.tar.gz
cd ta-lib/
./configure --prefix=/usr
make
sudo make install
```

**Windows**
It is recommended to use [WSL (Windows Subsystem for Linux)](https://learn.microsoft.com/en-us/windows/wsl/install) and follow the Linux instructions.

[Details on installation for different systems...](https://github.com/TA-Lib/ta-lib-python/blob/master/docs/install.md)

### 2. Dependency in `Cargo.toml`

```toml
[dependencies]
oxi-talib = "0.1.0"
```

## Example Usage

```rust
use oxi_talib::{cdl, Pattern, SimpleCandle, Error};

fn main() -> Result<(), Error> {
    // 1. Data for analysis.
    let candles: Vec<SimpleCandle> = vec![
        SimpleCandle::try_new(100.0, 105.0, 106.0, 98.0)?,
        SimpleCandle::try_new(105.0, 102.0, 107.0, 101.0)?,
        // A candle corresponding to the "Hammer" pattern
        SimpleCandle::try_new(102.0, 103.0, 104.0, 95.0)?,
        SimpleCandle::try_new(103.0, 108.0, 109.0, 102.0)?,
    ];

    // 2. Perform the analysis for the "Hammer" pattern.
    let signals = cdl().pattern(Pattern::Hammer, &candles)?;

    // 3. Process the results.
    assert_eq!(signals.len(), candles.len());

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

To work with your own data structures, you need to implement the `Candle` trait.

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

let my_data: Vec<MyData> = // ...

let signals = cdl().pattern(Pattern::Doji, &my_data);
```

### Configuration

Recognition parameters can be changed via the `configure` function. It should be called **once** at application startup in a single-threaded context.

```rust
use oxi_talib::cdl::engines::talib::engine::configure;
use oxi_talib::Settings;

// Executed once in the main function.
let mut settings = Settings::default();
settings.body_doji_factor = 0.2; // Example of changing a parameter

configure(settings).expect("Configuration should not be called more than once");

// All subsequent calls to `cdl()` will use these settings.
```

## Roadmap

*   **Stage 1: `TA-Lib` API Coverage**
    *   [ ] Add all remaining candlestick patterns (`CDL`).
    *   [ ] Implement a module for technical indicators (SMA, EMA, RSI, MACD, etc.).

*   **Stage 2: Native Rust Implementation**
    *   [ ] Gradually replace C function calls with equivalent Rust implementations. The goal is to eliminate the dependency on the system's C library.

*   **Stage 3: Advanced Analysis**
    *   [ ] Implement a `Quality` (shape correctness) and `Strength` (contextual significance) scoring system after transitioning to a native implementation.

## License

[MIT License](LICENSE).