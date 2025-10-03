# oxi-talib

[English version](README.md)

[![Crates.io](https://img.shields.io/crates/v/oxi-talib.svg)](https://crates.io/crates/oxi-talib)
[![Docs.rs](https://docs.rs/oxi-talib/badge.svg)](https://docs.rs/oxi-talib)
[![CI](https://github.com/sbooker/oxi-talib/actions/workflows/rust.yaml/badge.svg)](https://github.com/sbooker/oxi-talib/actions)

**oxi-talib** — библиотека на Rust для распознавания свечных паттернов.

## Ключевые характеристики

*   **API:** Интерфейс использует стандартные типы Rust (`Vec`, `Result`, `Option`).
*   **Настраиваемость:** Параметры алгоритмов распознавания можно изменять.
*   **Безопасность:** Публичный API на 100% безопасен. Весь `unsafe`-код, необходимый для взаимодействия с C-библиотекой, инкапсулирован.

## Установка

### 1. Системные зависимости

**Debian / Ubuntu:**
```bash
sudo apt-get update && sudo apt-get install build-essential libclang-dev
```

**Другие системы:**
Установите аналогичный пакет (например, `base-devel` в Arch Linux, "Build Tools for Visual Studio" в Windows, или Xcode Command Line Tools в macOS).

### 2. Зависимость в `Cargo.toml`

```toml
[dependencies]
oxi-talib = "0.1.0"
```

## Пример использования

```rust
use oxi_talib::{cdl, Pattern, SimpleCandle, Error};

fn main() -> Result<(), Error> {
    // 1. Данные для анализа.
    let candles: Vec<SimpleCandle> = vec![
        SimpleCandle::try_new(100.0, 105.0, 106.0, 98.0)?,
        SimpleCandle::try_new(105.0, 102.0, 107.0, 101.0)?,
        // Свеча, соответствующая паттерну "Молот"
        SimpleCandle::try_new(102.0, 103.0, 104.0, 95.0)?,
        SimpleCandle::try_new(103.0, 108.0, 109.0, 102.0)?,
    ];

    // 2. Выполнение анализа для паттерна "Молот".
    let signals = cdl().pattern(Pattern::Hammer, &candles)?;

    // 3. Обработка результатов.
    assert_eq!(signals.len(), candles.len());

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

Для работы с собственными структурами данных необходимо реализовать трейт `Candle`.

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

### Конфигурация

Параметры распознавания можно изменить через функцию `configure`. Ее следует вызывать **один раз** при старте приложения в однопоточном контексте.

```rust
use oxi_talib::cdl::engines::talib::engine::configure;
use oxi_talib::Settings;

// Выполняется один раз в функции main.
let mut settings = Settings::default();
settings.body_doji_factor = 0.2; // Пример изменения параметра

configure(settings).expect("Конфигурация не должна вызываться повторно");

// Все последующие вызовы `cdl()` будут использовать эти настройки.
```

## План развития

*   **Этап 1: Покрытие API `TA-Lib`**
    *   [ ] Добавление всех оставшихся свечных паттернов (`CDL`).
    *   [ ] Реализация модуля для технических индикаторов (SMA, EMA, RSI, MACD и т.д.).

*   **Этап 2: Нативная Rust-реализация**
    *   [ ] Постепенная замена вызовов C-функций на эквивалентные реализации на Rust. Цель — устранить зависимость от системной C-библиотеки.

*   **Этап 3: Продвинутый анализ**
    *   [ ] Реализация системы оценки `Quality` (качество формы) и `Strength` (сила сигнала в контексте) после перехода на нативную реализацию.

## Лицензия

[MIT License](LICENSE).