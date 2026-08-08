# oxi-talib

[English version](README.md)

[![Crates.io](https://img.shields.io/crates/v/oxi-talib.svg)](https://crates.io/crates/oxi-talib)
[![Docs.rs](https://docs.rs/oxi-talib/badge.svg)](https://docs.rs/oxi-talib)
[![CI](https://github.com/sbooker/oxi-talib/actions/workflows/rust.yaml/badge.svg)](https://github.com/sbooker/oxi-talib/actions)

**oxi-talib** — библиотека на Rust для технического анализа и распознавания свечных паттернов.

## Ключевые характеристики

*   **API:** Интерфейс использует стандартные типы Rust (`Vec`, `Result`, `Option`, `NonZeroU16`).
*   **Функционал:** Свечные паттерны и классические технические индикаторы (SMA, ATR, RSI, ADX, Bollinger Bands, SuperTrend).
*   **Настраиваемость:** Параметры алгоритмов распознавания можно изменять.
*   **Безопасность:** Публичный API на 100% безопасен (`safe Rust`). Весь `unsafe`-код взаимодействия с C-библиотекой инкапсулирован.

## Установка

Добавьте зависимость в `Cargo.toml`:

```toml
[dependencies]
oxi-talib = "0.2.0"
```

### Cargo Features

Функциональность разделена на фичи, что позволяет подключать только нужные компоненты:
* `full` (*по умолчанию*) — включает всё: свечные паттерны (`cdl`) и технические индикаторы (`ta`).
* `ta` — только модуль индикаторов (SMA, ATR, RSI, ADX, SuperTrend, BBands и др.).
* `cdl` — только распознавание свечных паттернов.
* `candles` — базовый трейт `Candle` и структура `SimpleCandle` без внешних FFI-зависимостей.

## Примеры использования

### 1. Технические индикаторы

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

    // Расчет Average True Range (ATR)
    let atr_res = atr(&candles, period)?;
    println!("ATR offset: {}, значения: {:?}", atr_res.offset, atr_res.values);

    // Расчет SuperTrend
    let st_res = super_trend(&candles, period, 3.0)?;
    for st in &st_res.values {
        println!("SuperTrend: значение = {:.2}, тренд = {:?}", st.value, st.trend);
    }

    Ok(())
}
```

### 2. Свечные паттерны

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
                "Паттерн 'Молот' найден на свече #{} с качеством {}!",
                i, s.quality.value()
            );
        }
    }
    
    Ok(())
}
```

## Продвинутое использование

### Реализация трейта `Candle`

Для работы с собственными структурами данных достаточно реализовать трейт `Candle`:

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

### Конфигурация распознавания свечей

Параметры распознавания свечных моделей можно изменить через функцию `configure` однократно при старте приложения:

```rust
use oxi_talib::configure;
use oxi_talib::Settings;

let mut settings = Settings::default();
settings.body_doji_factor = 0.2;

configure(settings).expect("Конфигурация не должна вызываться повторно");
```

## План развития

*   **Этап 1: Расширение покрытия API `TA-Lib`**
    *   [ ] Добавление оставшихся свечных паттернов (`CDL2CROWS`, `CDLABANDONEDBABY`, `CDL3INSIDE`, `CDL3LINESTRIKE` и др.).
    *   [ ] Скользящие средние и трендовые индикаторы (EMA, WMA, DEMA, TEMA, MACD, Parabolic SAR и др.).
    *   [ ] Осцилляторы и моментум (Stochastic, CCI, Williams %R, ROC, MFI и др.).
    *   [ ] Объемные индикаторы (OBV, Chaikin A/D и др.).
    *   [ ] Статистические функции (линейная регрессия, дисперсия и др.).

*   **Этап 2: Нативная Rust-реализация**
    *   [ ] Постепенная замена вызовов C-функций на нативные реализации на Rust с целью полного устранения C-зависимости.

*   **Этап 3: Продвинутый анализ**
    *   [ ] Реализация расширенной системы оценки `Quality` (форма) и `Strength` (сила сигнала в контексте).

## Лицензия

[MIT License](LICENSE).