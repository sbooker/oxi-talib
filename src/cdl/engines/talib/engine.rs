use crate::cdl::engines::internal::CdlApiInternal;
use crate::cdl::engines::talib::functions::TaCdlFnPtr;
use crate::Error::{AlreadyConfigured, CalculationError};
use crate::{Candle, Error, Pattern, Settings, Signal, SimpleCandle};
use std::sync::OnceLock;
use ta_lib_sys::{SetCandleSettings, RetCode};
use crate::cdl::engines::talib::functions::{PIERCING_PENETRATION, STAR_PENETRATION};
use ta_lib_sys::CandleSettingType::*;
use ta_lib_sys::RangeType::*;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// Configures the global settings for the TA-Lib engine.
///
/// This function should be called **once** at application startup (e.g., in `main`),
/// **before** any other threads are spawned and before the first call to `oxi_talib::cdl()`.
///
/// If this function is not called, balanced default settings will be used.
///
/// # Examples
///
/// ```ignore
/// // In main.rs
/// fn main() {
///     let my_settings = oxi_talib::cdl::api::settings::Settings::default();
///     oxi_talib::cdl::engines::talib::engine::configure(my_settings)
///         .expect("Settings should be configured successfully");
///
///     // Now it's safe to use the analyzer in a multi-threaded environment
///     // let analyzer = oxi_talib::cdl();
///     // ...
/// }
/// ```
///
/// # Errors
///
/// Returns an error if `configure` is called more than once.
///
/// # Safety
///
/// This function is **not thread-safe**. It modifies global static variables.
/// It should only be called from the main thread during application initialization,
/// before any analysis functions are called from other threads. Calling this function
/// concurrently with any other function from this library is undefined behavior.
#[allow(dead_code)] // Allow because this is part of the public API
pub fn configure(settings: Settings) -> Result<(), Error> {
    SETTINGS.set(settings).map_err(|_| AlreadyConfigured)
}

pub(crate) fn instance() -> &'static impl CdlApiInternal {
    static ENGINE: OnceLock<TaLibEngine> = OnceLock::new();

    ENGINE.get_or_init(|| TaLibEngine::new(SETTINGS.get().cloned().unwrap_or_default()))
}

struct TaLibEngine {}
impl TaLibEngine {
    fn new(settings: Settings) -> Self {
        Self::apply_settings(&settings);
        Self {}
    }

    fn apply_settings(settings: &Settings) {
        unsafe {
            SetCandleSettings(
                BodyLong,
                RangeType_RealBody,
                settings.period,
                settings.body_long_factor,
            );
            SetCandleSettings(
                BodyVeryLong,
                RangeType_RealBody,
                settings.period,
                settings.body_very_long_factor,
            );
            SetCandleSettings(
                BodyShort,
                RangeType_RealBody,
                settings.period,
                settings.body_short_factor,
            );
            SetCandleSettings(
                BodyDoji,
                RangeType_HighLow,
                settings.period,
                settings.body_doji_factor,
            );

            // --- Тени ---
            SetCandleSettings(
                ShadowLong,
                RangeType_RealBody,
                settings.period,
                settings.shadow_long_factor,
            );
            SetCandleSettings(
                ShadowVeryLong,
                RangeType_RealBody,
                settings.period,
                settings.shadow_very_long_factor,
            );
            SetCandleSettings(
                ShadowShort,
                RangeType_HighLow,
                settings.period,
                settings.shadow_short_factor,
            );
            SetCandleSettings(
                ShadowVeryShort,
                RangeType_HighLow,
                settings.period,
                settings.shadow_very_short_factor,
            );

            // --- Сравнение цен ---
            SetCandleSettings(
                Near,
                RangeType_RealBody,
                settings.period,
                settings.near_factor,
            );
            SetCandleSettings(
                Far,
                RangeType_RealBody,
                settings.period,
                settings.far_factor,
            );
            SetCandleSettings(
                Equal,
                RangeType_RealBody,
                settings.period,
                settings.equal_factor,
            );

            STAR_PENETRATION = settings.star_penetration_factor;
            PIERCING_PENETRATION = settings.piercing_penetration_factor;
        }
    }
}

impl CdlApiInternal for TaLibEngine {
    fn pattern(
        &self,
        pattern: Pattern,
        candles: &[SimpleCandle],
    ) -> Result<Vec<Option<Signal>>, Error> {
        Self::unsafe_call(candles, pattern.ta_lib_function())
    }
}

trait IntoRows {
    fn opens(&self) -> Vec<f64>;
    fn closes(&self) -> Vec<f64>;
    fn lows(&self) -> Vec<f64>;
    fn highs(&self) -> Vec<f64>;
}

impl<C: Candle> IntoRows for [C] {
    fn opens(&self) -> Vec<f64> {
        self.iter().map(|x| x.open().into()).collect()
    }

    fn closes(&self) -> Vec<f64> {
        self.iter().map(|x| x.close().into()).collect()
    }

    fn lows(&self) -> Vec<f64> {
        self.iter().map(|x| x.low().into()).collect()
    }

    fn highs(&self) -> Vec<f64> {
        self.iter().map(|x| x.high().into()).collect()
    }
}

impl TaLibEngine {
    fn unsafe_call<C: Candle>(
        candles: &[C],
        cdl_fn_ptr: TaCdlFnPtr,
    ) -> Result<Vec<Option<Signal>>, Error> {
        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<i32> = vec![0; candles.len()];

        unsafe {
            Self::map_error(cdl_fn_ptr(
                0,
                (candles.len() - 1) as i32,
                candles.opens().as_ptr(),
                candles.highs().as_ptr(),
                candles.lows().as_ptr(),
                candles.closes().as_ptr(),
                &mut out_beg_idx as *mut i32,
                &mut out_nb_element as *mut i32,
                out_arr.as_mut_ptr(),
            ))?;
        }

        Self::map_ok(out_beg_idx, out_nb_element, out_arr)
    }

    fn map_error(res: RetCode) -> Result<(), Error> {
        match res {
            RetCode::SUCCESS => Ok(()),
            _ => Err(CalculationError(format!("TA-Lib error: {res:?}"))),
        }
    }

    fn map_ok(
        out_beg_idx: i32,
        out_nb_element: i32,
        out_arr: Vec<i32>,
    ) -> Result<Vec<Option<Signal>>, Error> {
        let mut results: Vec<i32> = vec![0; out_arr.len()];

        let calculated_part = &out_arr[0..out_nb_element as usize];

        let start_index = out_beg_idx as usize;

        if out_nb_element > 0 {
            let end_index = start_index + out_nb_element as usize;
            if end_index <= out_arr.len() {
                results[start_index..end_index].copy_from_slice(calculated_part);
            }
        }

        Ok(results
            .iter()
            .map(|&x| Signal::try_from_quality(x as u8))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern::*;
    use super::*;

    #[test]
    fn test_cdl_doji_t() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (3268.8, 3301.2, 3264.2, 3278.8),
            (3281.0, 3284.8, 3255.6, 3268.2),
            (3270.8, 3279.0, 3235.0, 3244.4),
            (3239.8, 3257.4, 3208.2, 3217.4),
            (3218.6, 3222.0, 3215.2, 3216.0),
            (3215.6, 3215.6, 3201.8, 3211.6),
            (3215.2, 3232.8, 3187.6, 3188.0),
            (3189.0, 3216.8, 3151.0, 3157.0),
            (3158.0, 3192.0, 3155.4, 3182.4),
            (3184.8, 3213.8, 3176.0, 3199.8),
            (3202.0, 3206.8, 3115.2, 3119.4),
            (3124.8, 3128.6, 3110.2, 3114.0),
            (3116.6, 3120.8, 3077.4, 3093.0),
            (3079.0, 3243.6, 3067.0, 3243.0),
            (3244.0, 3335.0, 3228.2, 3316.8),
            (3316.6, 3364.0, 3296.2, 3351.0),
            (3335.0, 3349.4, 3257.0, 3281.6),
            (3283.0, 3354.6, 3280.6, 3341.0),
            (3341.8, 3367.0, 3337.0, 3351.8),
            (3358.4, 3358.4, 3348.4, 3352.6), // Doji
            (3357.2, 3378.8, 3351.2, 3358.0), // Doji
        ];

        let expected: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 100,
        ];

        helper(data, expected, |candles| {
            crate::cdl().pattern(Doji, candles)
        })
    }

    #[test]
    fn test_cdl_doji_x5() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (2871.0, 2967.0, 2850.0, 2885.5),
            (2896.5, 2906.5, 2805.5, 2816.0),
            (2819.0, 2846.5, 2815.0, 2820.0),
            (2825.5, 2830.0, 2789.5, 2811.0),
            (2800.0, 2867.0, 2755.0, 2860.0),
            (2865.0, 2922.0, 2847.5, 2909.5),
            (2910.5, 2940.0, 2905.5, 2922.0),
            (2931.0, 2984.5, 2925.5, 2932.0),
            (2934.0, 3000.0, 2934.0, 2994.0),
            (3029.0, 3075.0, 3025.5, 3069.0),
            (3077.5, 3083.0, 3069.0, 3079.5), // Doji
            (3070.0, 3130.0, 3031.5, 3075.0), // Doji
        ];

        let expected: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 100];

        helper(data, expected, |candles| {
            crate::cdl().pattern(Doji, candles)
        })
    }

    #[test]
    fn test_dragonfly_flot() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (77.98, 78.5, 76.26, 76.4),
            (76.16, 76.75, 75.77, 75.91),
            (76.14, 76.14, 75.41, 75.79),
            (75.5, 81.0, 74.17, 80.55),
            (80.3, 82.25, 79.84, 80.75),
            (80.75, 81.47, 80.13, 80.41),
            (80.41, 81.12, 78.38, 79.07),
            (78.83, 80.81, 78.04, 80.77),
            (80.99, 81.55, 80.66, 80.66),
            (80.68, 81.27, 80.55, 80.98),
            (80.88, 81.48, 79.61, 80.51),
            (80.53, 84.1, 80.11, 83.62),
            (83.6, 85.35, 83.5, 84.77),
            (84.77, 85.3, 83.4, 84.93),
            (85.33, 87.95, 81.8, 82.5),
            (82.72, 83.0, 82.31, 82.5),
            (82.49, 82.5, 82.05, 82.2),
            (82.2, 82.2, 80.11, 80.65),
            (80.31, 81.24, 79.55, 79.7),
            (79.69, 79.7, 78.11, 79.69), // Dragonfly
            (79.63, 81.3, 79.03, 80.87),
            (80.87, 81.78, 79.99, 80.43),
        ];

        let expected: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0,
        ];

        helper(data, expected, |candles| {
            crate::cdl().pattern(DragonFly, candles)
        })
    }

    #[test]
    fn test_bearish_engulfing_engulfing_ugld_20250910_20250929() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (0.6144, 0.624, 0.6018, 0.6169), // 2025-09-10
            (0.6198, 0.6383, 0.6094, 0.633),
            (0.6335, 0.6478, 0.616, 0.6277),
            (0.6275, 0.6446, 0.618, 0.6428), // 2025-09-13 Bullish Engulfing
            (0.6429, 0.6445, 0.635, 0.6421),
            (0.6431, 0.6621, 0.6056, 0.6344),
            (0.6345, 0.649, 0.6255, 0.6429),
            (0.6434, 0.6469, 0.62, 0.6302), // 2025-09-17 Bearish Engulfing
            (0.6254, 0.6359, 0.62, 0.6212),
            (0.6211, 0.63, 0.6204, 0.6241),
            (0.6242, 0.641, 0.6242, 0.6371),
            (0.6375, 0.6391, 0.6158, 0.6228), // 2025-09-23 Bearish Engulfing
            (0.6245, 0.6277, 0.6152, 0.6226),
            (0.623, 0.6282, 0.6201, 0.6205),
            (0.6205, 0.6228, 0.6012, 0.608),
            (0.608, 0.614, 0.6058, 0.6077),
            (0.608, 0.609, 0.6064, 0.608),
            (0.6084, 0.6112, 0.585, 0.5939), // 2025-09-29 Bearish Engulfing
        ];

        let expected_bearish: Vec<u8> =
            vec![0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 100, 0, 0, 0, 0, 0, 100];
        let expected_bullish: Vec<u8> =
            vec![0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper(data.clone(), expected_bearish, |candles| {
            crate::cdl().pattern(BearishEngulfing, candles)
        });
        helper(data.clone(), expected_bullish, |candles| {
            crate::cdl().pattern(BullishEngulfing, candles)
        });
    }

    #[test]
    fn test_hammer_enpg_2025_07_29_2025_08_12() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (419.5, 440.6, 418.35, 432.4),
            (433.85, 433.85, 418.05, 418.65),
            (418.3, 429.7, 416.45, 428.4),
            (430.2, 449.0, 423.8, 446.0),
            (447.4, 448.85, 435.1, 441.1),
            (441.3, 453.9, 437.15, 451.05),
            (453.65, 478.0, 442.15, 469.5),
            (469.0, 481.15, 462.6, 473.1),
            (473.5, 496.0, 468.65, 495.0),
            (496.1, 516.95, 496.1, 507.1),
            (508.0, 508.0, 492.5, 501.65), // hammer 75
        ];

        let expected: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper(data, expected, |candles| {
            crate::cdl().pattern(Hammer, candles)
        })
    }

    #[test]
    fn test_hammer_spbe_2025_07_29_2025_08_12() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (195.0, 196.7, 192.0, 193.4),
            (193.4, 193.7, 189.3, 191.7),
            (191.9, 195.4, 191.1, 193.8),
            (196.8, 202.0, 194.2, 198.8),
            (203.0, 203.0, 196.0, 200.8),
            (199.8, 208.6, 198.0, 208.0),
            (208.6, 209.8, 203.6, 209.4),
            (209.4, 230.1, 205.2, 230.1),
            (230.1, 266.3, 229.5, 254.0),
            (254.9, 285.9, 251.0, 284.0),
            (286.0, 292.3, 286.0, 292.3),
            (292.3, 292.3, 292.3, 292.3),
            (299.2, 313.0, 276.9, 284.5),
            (285.1, 285.7, 271.9, 281.9), //hammer 95
        ];

        let expected: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        helper(data, expected, |candles| {
            crate::cdl().pattern(Hammer, candles)
        })
    }

    #[test]
    fn test_hammer_irao_20250915_20251002() {
        let data: Vec<(f64, f64, f64, f64)> = vec![
            (3.1315, 3.135, 3.094, 3.124),
            (3.124, 3.146, 3.0925, 3.1075),
            (3.1075, 3.13, 3.097, 3.1235),
            (3.1235, 3.128, 3.0735, 3.0825),
            (3.09, 3.097, 3.066, 3.0715),
            (3.0815, 3.105, 3.058, 3.0895),
            (3.09, 3.1085, 3.04, 3.0455), // 2025-09-23 BearishEngulfing
            (3.0505, 3.0805, 3.0225, 3.0785),
            (3.08, 3.0965, 3.0655, 3.0705),
            (3.0765, 3.0845, 3.03, 3.081),
            (3.081, 3.0865, 3.072, 3.083),
            (3.083, 3.0955, 3.081, 3.0855),
            (3.0905, 3.0985, 3.011, 3.018), // 2025-09-29 BearishEngulfing
            (3.013, 3.041, 2.992, 3.0255),
            (3.0255, 3.037, 3.003, 3.005),
            (3.0185, 3.0185, 2.981, 3.006), // 2025-10-02 Hammer, BullishHarami
        ];

        let hammer_expected: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100];
        let bearish_engulfing_expected: Vec<u8> =
            vec![0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 100, 0, 0, 0];
        let bullish_harami_expected: Vec<u8> =
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100];

        helper(data.clone(), hammer_expected, |candles| {
            crate::cdl().pattern(Hammer, candles)
        });
        helper(data.clone(), bearish_engulfing_expected, |candles| {
            crate::cdl().pattern(BearishEngulfing, candles)
        });
        helper(data.clone(), bullish_harami_expected, |candles| {
            crate::cdl().pattern(BullishHarami, candles)
        });
    }

    fn helper<F>(data: Vec<(f64, f64, f64, f64)>, expected: Vec<u8>, indicator_fn: F)
    where
        F: Fn(&[SimpleCandle]) -> Result<Vec<Option<Signal>>, Error>,
    {
        let candles: Vec<SimpleCandle> = data
            .into_iter()
            .map(|(open, high, low, close)| SimpleCandle::try_new(open, close, high, low).unwrap())
            .collect();
        let expected_signals: Vec<Option<Signal>> = expected
            .into_iter()
            .map(|x| Signal::try_from_quality(x))
            .collect();
        let result = indicator_fn(candles.as_slice())
            .expect("Функция индикатора должна вернуть Ok в тестах");

        assert_eq!(result, expected_signals);
    }
}
