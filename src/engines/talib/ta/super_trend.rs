use crate::api::results::{IndicatorResult, SuperTrendResult, Trend};
use crate::engines::internal::TaApiInternal;
use crate::engines::talib::ta::engine::TaLibEngine;
use crate::{Candle, Error, SimpleCandle};
use std::num::NonZeroU16;

pub(crate) struct SuperTrend {}

impl SuperTrend {
    pub(crate) fn calc(candles: &[SimpleCandle], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error> {
        let atr_indicator = TaLibEngine::atr(candles, period)?;
        let offset = atr_indicator.offset;
        let mut prev_close: f64 = candles[offset - 1].close();

        let mut values = Vec::with_capacity(atr_indicator.values.len());
        let candle = &candles[offset];
        let (mut prev_upper, mut prev_lower, hl2) = Self::upper_lower(candle, atr_indicator.values[0], multiplier);
        let mut prev_trend = if candle.close() > hl2 { Trend::Bullish } else { Trend::Bearish };

        values.push(SuperTrendResult {
            value: if prev_trend == Trend::Bullish { prev_lower } else { prev_upper },
            trend: prev_trend,
        });

        let iter = atr_indicator.values.iter()
            .zip(&candles[offset..]) // Берем срез свечей сразу с нужного места
            .skip(1);

        for (&atr, candle) in iter {
            let (basic_upper, basic_lower, _) = SuperTrend::upper_lower(candle, atr, multiplier);

            let final_upper = if basic_upper < prev_upper || prev_close > prev_upper {
                basic_upper
            } else {
                prev_upper
            };

            let final_lower = if basic_lower > prev_lower || prev_close < prev_lower {
                basic_lower
            } else {
                prev_lower
            };

            let trend = match prev_trend {
                Trend::Bullish => if candle.close() < final_lower { Trend::Bearish } else { Trend::Bullish },
                Trend::Bearish => if candle.close() > final_upper { Trend::Bullish } else { Trend::Bearish },
            };

            let value = match trend {
                Trend::Bullish => final_lower,
                Trend::Bearish => final_upper,
            };

            values.push(SuperTrendResult { value, trend });

            // Обновляем состояние для следующего шага
            prev_upper = final_upper;
            prev_lower = final_lower;
            prev_trend = trend;
            prev_close = candle.close();
        }

        Ok(IndicatorResult{ offset, values })
    }

    fn upper_lower(candle: &SimpleCandle, atr: f64, multiplier: f64) -> (f64, f64, f64) {
        let hl2 = (candle.high() + candle.low()) / 2.0;
        let diapason = multiplier * atr;

        (hl2 + diapason, hl2 - diapason, hl2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn sber_20260103_20260111() {
        let candles: Vec<SimpleCandle> = vec![
            (274.0, 275.9, 271.8,  275.29),
            (275.8, 276.4, 273.22, 274.83),
            (275.2, 275.5, 273.1,  274.60),
            (275.0, 276.6, 274.6,  276.44),
            (276.5, 278.4, 274.0,  274.50),
            (274.5, 275.0, 271.0,  272.00),
            (272.0, 272.5, 265.0,  266.00),
        ]
            .into_iter()
            .map(|(open, high, low, close)| SimpleCandle::try_new(open, close, high, low).unwrap())
            .collect();

        let expected_values = vec![268.02, 268.02, 268.02, 283.11814814814816];
        let expected_trends = vec![Trend::Bullish, Trend::Bullish, Trend::Bullish, Trend::Bearish];

        let res = TaLibEngine::super_trend(candles.as_slice(), 3.try_into().unwrap(), 3.try_into().unwrap()).unwrap();

        let values = res.values.iter().map(|v| v.value).collect::<Vec<_>>();
        let trends = res.values.iter().map(|v| v.trend).collect::<Vec<_>>();

        assert_eq!(res.offset, 3);
        assert_eq!(trends, expected_trends);
        assert_float_eq!(values, expected_values, abs_all <= 1e-10);
    }
}