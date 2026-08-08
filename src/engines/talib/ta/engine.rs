use crate::api::results::{IndicatorResult, SuperTrendResult};
use crate::engines::internal::TaApiInternal;
use crate::engines::talib::ta::map_output_res;
use crate::engines::talib::ta::super_trend::SuperTrend;
use crate::engines::talib::{map_error, IntoRows};
use crate::{Error, SimpleCandle};
use std::num::NonZeroU16;
use ta_lib_sys::ATR;

pub(crate) struct TaLibEngine {}

#[allow(non_snake_case, clippy::too_many_arguments)]
impl TaApiInternal for TaLibEngine {
    fn atr(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; candles.len()];

        unsafe {
            map_error(
                ATR(
                    0,
                    (candles.len() - 1) as i32,
                    candles.highs().as_ptr(),
                    candles.lows().as_ptr(),
                    candles.closes().as_ptr(),
                    period.get() as i32,
                    &mut out_beg_idx as *mut i32,
                    &mut out_nb_element as *mut i32,
                    out_arr.as_mut_ptr(),
                )
            )?
        }

        Ok(map_output_res(&out_arr, out_nb_element, out_beg_idx))
    }

    fn super_trend(candles: &[SimpleCandle], period: NonZeroU16, multiplier: NonZeroU16) -> Result<IndicatorResult<SuperTrendResult>, Error> {
        SuperTrend::calc(candles, period, multiplier.get() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod atr {
        use super::*;
        use float_eq::assert_float_eq;

        #[test]
        fn sber_20260103_20260109() {
            let candles: Vec<SimpleCandle> = vec![
                (274.0, 275.9, 271.8,  275.29),
                (275.8, 276.4, 273.22, 274.83),
                (275.2, 275.5, 273.1,  274.60),
                (275.0, 276.6, 274.6,  276.44),
                (276.5, 278.4, 274.0,  274.50),
            ]
                .into_iter()
                .map(|(open, high, low, close)| SimpleCandle::try_new(open, close, high, low).unwrap())
                .collect();

            let expected_val = vec![2.5266666666666664, 3.1511111111111107];

            let res = TaLibEngine::atr(candles.as_slice(), 3.try_into().unwrap()).unwrap();

            assert_eq!(res.offset, 3);
            assert_float_eq!(res.values, expected_val, abs_all <= 1e-10);
        }
    }
}