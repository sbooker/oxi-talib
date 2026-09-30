use crate::api::results::{BBandsResult, IndicatorResult, SuperTrendResult};
use crate::engines::internal::TaApiInternal;
use crate::engines::talib::ta::map_output_res;
use crate::engines::talib::ta::super_trend::SuperTrend;
use crate::engines::talib::{map_error, IntoRows};
use crate::{Error, SimpleCandle};
use std::num::NonZeroU16;
use ta_lib_sys::{MAType, ADX, ATR, BBANDS, CORREL, RSI, SMA, STDDEV, TRANGE};

pub(crate) struct TaLibEngine {}

#[allow(non_snake_case, clippy::too_many_arguments)]
impl TaApiInternal for TaLibEngine {
    fn correl(data1: &[f64], data2: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        if data1.len() != data2.len() {
            return Err(Error::InputDataMismatchLength(data1.len(), data2.len()));
        }
        if data1.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(data1.len(), period))
        }
        if data2.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(data2.len(), period))
        }


        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; data1.len()];

        unsafe {
            map_error(
                CORREL(
                    0,
                    (data1.len() - 1) as i32,
                    data1.as_ptr(),
                    data2.as_ptr(),
                    period.get() as i32,
                    &mut out_beg_idx as *mut i32,
                    &mut out_nb_element as *mut i32,
                    out_arr.as_mut_ptr(),
                )
            )?
        }

        Ok(map_output_res(&out_arr, out_nb_element, out_beg_idx))
    }

    fn sma(data: &[f64], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        if data.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(data.len(), period))
        }

        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; data.len()];

        unsafe {
            map_error(
                SMA(
                    0,
                    (data.len() - 1) as i32,
                    data.as_ptr(),
                    period.get() as i32,
                    &mut out_beg_idx as *mut i32,
                    &mut out_nb_element as *mut i32,
                    out_arr.as_mut_ptr(),
                )
            )?
        }

        Ok(map_output_res(&out_arr, out_nb_element, out_beg_idx))
    }

    fn trange(candles: &[SimpleCandle]) -> Result<IndicatorResult<f64>, Error> {
        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; candles.len()];

        unsafe {
            map_error(
                TRANGE(
                    0,
                    (candles.len() - 1) as i32,
                    candles.highs().as_ptr(),
                    candles.lows().as_ptr(),
                    candles.closes().as_ptr(),
                    &mut out_beg_idx as *mut i32,
                    &mut out_nb_element as *mut i32,
                    out_arr.as_mut_ptr(),
                )
            )?
        }

        Ok(map_output_res(&out_arr, out_nb_element, out_beg_idx))
    }

    fn stddev(candles: &[SimpleCandle], period: NonZeroU16, number_of_deviations: f64) -> Result<IndicatorResult<f64>, Error> {
        if candles.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(candles.len(), period))
        }

        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; candles.len()];

        unsafe {
            map_error(
                STDDEV(
                    0,
                    (candles.len() - 1) as i32,
                    candles.closes().as_ptr(),
                    period.get() as i32,
                    number_of_deviations,
                    &mut out_beg_idx as *mut i32,
                    &mut out_nb_element as *mut i32,
                    out_arr.as_mut_ptr(),
                )
            )?
        }

        Ok(map_output_res(&out_arr, out_nb_element, out_beg_idx))
    }
    fn atr(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        if candles.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(candles.len(), period))
        }

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

    fn adx(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        if candles.len() < (period.get() * 2) as usize {
            return Err(Error::InsufficientInputData(candles.len(), period))
        }
        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; candles.len()];

        unsafe {
            map_error(
                ADX(
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

    fn rsi(candles: &[SimpleCandle], period: NonZeroU16) -> Result<IndicatorResult<f64>, Error> {
        if candles.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(candles.len(), period))
        }
        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_arr: Vec<f64> = vec![0f64; candles.len()];

        unsafe {
            map_error(
                RSI(
                    0,
                    (candles.len() - 1) as i32,
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

    fn super_trend(candles: &[SimpleCandle], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<SuperTrendResult>, Error> {
        SuperTrend::calc(candles, period, multiplier)
    }

    fn bbands(candles: &[SimpleCandle], period: NonZeroU16, multiplier: f64) -> Result<IndicatorResult<BBandsResult>, Error> {
        if candles.len() < period.get() as usize {
            return Err(Error::InsufficientInputData(candles.len(), period))
        }

        let mut out_beg_idx: i32 = 0;
        let mut out_nb_element: i32 = 0;
        let mut out_upper_band: Vec<f64> = vec![0f64; candles.len()];
        let mut out_middle_band: Vec<f64> = vec![0f64; candles.len()];
        let mut out_lower_band: Vec<f64> = vec![0f64; candles.len()];

        unsafe {
            map_error(
                BBANDS(
                    0,
                    (candles.len() - 1) as i32,
                    candles.closes().as_ptr(),
                    period.get() as i32,
                    multiplier,
                    multiplier,
                    MAType::MAType_SMA,
                    &mut out_beg_idx,
                    &mut out_nb_element as *mut i32,
                    out_upper_band.as_mut_ptr(),
                    out_middle_band.as_mut_ptr(),
                    out_lower_band.as_mut_ptr(),
                )
            )?
        }

        let mut bband_results = Vec::new();
        for i in 0..candles.len() {
            bband_results.push(BBandsResult{
                upper: out_upper_band[i],
                middle: out_middle_band[i],
                lower: out_lower_band[i],
            })
        }

        Ok(map_output_res(bband_results.as_slice(), out_nb_element, out_beg_idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

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

    mod adx {
        use super::*;
        use float_eq::assert_float_eq;

        #[test]
        fn sber_20260103_20260109() {
            let candles: Vec<SimpleCandle> = vec![
                (280.21,280.35,278.0,279.55),
                (279.55,281.41,275.7,277.4),
                (277.3,283.89,275.34,282.88),
                (283.89,288.92,282.09,287.15),
                (287.66,288.9,285.08,285.81),
                (287.61,294.55,287.61,289.94),
            ]
                .into_iter()
                .map(|(open, high, low, close)| SimpleCandle::try_new(open, close, high, low).unwrap())
                .collect();

            let expected_val = vec![70.23412384168718];

            let res = TaLibEngine::adx(candles.as_slice(), 3.try_into().unwrap()).unwrap();

            assert_eq!(res.offset, 5);
            assert_float_eq!(res.values, expected_val, abs_all <= 1e-10);
        }
    }

    mod bbands {
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

            // Период = 3, Отклонение = 2.0
            let res = TaLibEngine::bbands(candles.as_slice(), 3.try_into().unwrap(), 2.0).unwrap();

            assert_eq!(res.offset, 2);

            let uppers: Vec<f64> = res.values.iter().map(|v| v.upper).collect();
            let middles: Vec<f64> = res.values.iter().map(|v| v.middle).collect();
            let lowers: Vec<f64> = res.values.iter().map(|v| v.lower).collect();

            let expected_middles = vec![
                274.90666666666664,
                275.29,
                275.18,
                274.31333333333333,
                270.8333333333333
            ];

            let expected_uppers = vec![
                275.4803874659721,
                276.9271519986448,
                276.9637787605716,
                277.9481770549429,
                277.9669782502755
            ];

            let expected_lowers = vec![
                274.33294586736113,
                273.6528480013552,
                273.3962212394284,
                270.6784896117237,
                263.6996884163911
            ];

            assert_float_eq!(middles, expected_middles, abs_all <= 1e-7);
            assert_float_eq!(uppers, expected_uppers, abs_all <= 1e-7);
            assert_float_eq!(lowers, expected_lowers, abs_all <= 1e-7);
        }
    }

    fn sample_candles() -> Vec<SimpleCandle> {
        vec![
            (10.0, 15.0, 8.0,  12.0),
            (12.0, 14.0, 10.0, 11.0),
            (11.0, 16.0, 9.0,  14.0),
            (14.0, 14.0, 12.0, 13.0),
        ]
            .into_iter()
            .map(|(open, high, low, close)| SimpleCandle::try_new(open, close, high, low).unwrap())
            .collect()
    }

    #[test]
    fn test_trange() {
        let candles = sample_candles();
        let res = TaLibEngine::trange(&candles).unwrap();

        assert_eq!(res.offset, 1);
        assert_float_eq!(res.values, vec![4.0, 7.0, 2.0], abs_all <= 1e-6);
    }

    #[test]
    fn test_sma() {
        // Тестируем абстрактный массив f64
        let data = vec![4.0, 7.0, 2.0];
        let res = TaLibEngine::sma(&data, 2.try_into().unwrap()).unwrap();

        assert_eq!(res.offset, 1);
        assert_float_eq!(res.values, vec![5.5, 4.5], abs_all <= 1e-6);
    }

    #[test]
    fn test_stddev() {
        let candles = sample_candles();
        // stddev(12, 11)=0.5 | stddev(11, 14)=1.5 | stddev(14, 13)=0.5
        let res = TaLibEngine::stddev(&candles, 2.try_into().unwrap(), 1.0).unwrap();

        assert_eq!(res.offset, 1);
        assert_float_eq!(res.values, vec![0.5, 1.5, 0.5], abs_all <= 1e-6);
    }
}