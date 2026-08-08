use crate::api::results::IndicatorResult;

pub(crate) mod engine;
mod super_trend;

pub(crate) fn map_output_res<T: Copy + Default>(out_arr: &[T], out_nb_element: i32, out_beg_idx: i32) -> IndicatorResult<T> {
    IndicatorResult{
        values: (&out_arr[0..out_nb_element as usize]).to_vec(),
        offset: out_beg_idx as usize,
    }
}
