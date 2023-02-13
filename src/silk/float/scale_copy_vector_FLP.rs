pub unsafe fn silk_scale_copy_vector_FLP(
    data_out: *mut f32,
    data_in: *const f32,
    gain: f32,
    dataSize: i32,
) {
    let mut i: i32 = 0;
    let mut dataSize4: i32 = 0;
    dataSize4 = dataSize & 0xfffc as i32;
    i = 0 as i32;
    while i < dataSize4 {
        *data_out.offset((i + 0 as i32) as isize) = gain * *data_in.offset((i + 0 as i32) as isize);
        *data_out.offset((i + 1 as i32) as isize) = gain * *data_in.offset((i + 1 as i32) as isize);
        *data_out.offset((i + 2 as i32) as isize) = gain * *data_in.offset((i + 2 as i32) as isize);
        *data_out.offset((i + 3 as i32) as isize) = gain * *data_in.offset((i + 3 as i32) as isize);
        i += 4 as i32;
    }
    while i < dataSize {
        *data_out.offset(i as isize) = gain * *data_in.offset(i as isize);
        i += 1;
    }
}
