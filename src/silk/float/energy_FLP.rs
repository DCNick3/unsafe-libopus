pub unsafe fn silk_energy_FLP(data: *const f32, dataSize: i32) -> f64 {
    let mut i: i32 = 0;
    let mut result: f64 = 0.;
    result = 0.0f64;
    i = 0 as i32;
    while i < dataSize - 3 as i32 {
        result += *data.offset((i + 0 as i32) as isize) as f64
            * *data.offset((i + 0 as i32) as isize) as f64
            + *data.offset((i + 1 as i32) as isize) as f64
                * *data.offset((i + 1 as i32) as isize) as f64
            + *data.offset((i + 2 as i32) as isize) as f64
                * *data.offset((i + 2 as i32) as isize) as f64
            + *data.offset((i + 3 as i32) as isize) as f64
                * *data.offset((i + 3 as i32) as isize) as f64;
        i += 4 as i32;
    }
    while i < dataSize {
        result += *data.offset(i as isize) as f64 * *data.offset(i as isize) as f64;
        i += 1;
    }
    return result;
}
