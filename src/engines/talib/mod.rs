use crate::api::results::IndicatorResult;
use crate::Error::CalculationError;
use crate::{Candle, Error};
use ta_lib_sys::RetCode;

pub(crate) mod cdl;
pub(crate) mod ta;


fn map_error(res: RetCode) -> Result<(), Error> {
    match res {
        RetCode::SUCCESS => Ok(()),
        _ => Err(CalculationError(format!("TA-Lib error: {res:?}"))),
    }
}

fn map_output<T: Copy + Default>(out_arr: &[T], out_nb_element: i32, out_beg_idx: i32) -> Vec<T> {
    let mut results: Vec<T> = vec![T::default(); out_arr.len()];

    let calculated_part = &out_arr[0..out_nb_element as usize];

    let start_index = out_beg_idx as usize;

    if out_nb_element > 0 {
        let end_index = start_index + out_nb_element as usize;
        if end_index <= out_arr.len() {
            results[start_index..end_index].copy_from_slice(calculated_part);
        }
    }

    results
}

pub fn map_output_res<T: Copy + Default>(out_arr: &[T], out_nb_element: i32, out_beg_idx: i32) -> IndicatorResult<T> {
    IndicatorResult{
        values: (&out_arr[0..out_nb_element as usize]).to_vec(),
        offset: out_beg_idx as usize,
    }
}

pub(crate) trait IntoRows {
    fn opens(&self) -> Vec<f64>;
    fn closes(&self) -> Vec<f64>;
    fn lows(&self) -> Vec<f64>;
    fn highs(&self) -> Vec<f64>;
}

impl<C: Candle> IntoRows for [C] {
    fn opens(&self) -> Vec<f64> { self.iter().map(|x| x.open().into()).collect() }

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
