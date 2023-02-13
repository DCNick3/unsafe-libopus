pub unsafe fn silk_scale_vector_FLP(data1: *mut f32, gain: f32, dataSize: i32) {
    let mut i: i32 = 0;
    let mut dataSize4: i32 = 0;
    dataSize4 = dataSize & 0xfffc as i32;
    i = 0 as i32;
    while i < dataSize4 {
        *data1.offset((i + 0 as i32) as isize) *= gain;
        *data1.offset((i + 1 as i32) as isize) *= gain;
        *data1.offset((i + 2 as i32) as isize) *= gain;
        *data1.offset((i + 3 as i32) as isize) *= gain;
        i += 4 as i32;
    }
    while i < dataSize {
        *data1.offset(i as isize) *= gain;
        i += 1;
    }
}
