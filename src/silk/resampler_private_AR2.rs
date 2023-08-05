pub unsafe fn silk_resampler_private_AR2(
    S: *mut i32,
    out_Q8: *mut i32,
    in_0: *const i16,
    A_Q14: *const i16,
    len: i32,
) {
    let mut k: i32 = 0;
    let mut out32: i32 = 0;
    k = 0;
    while k < len {
        out32 = *S.offset(0 as isize) + ((*in_0.offset(k as isize) as i32 as u32) << 8) as i32;
        *out_Q8.offset(k as isize) = out32;
        out32 = ((out32 as u32) << 2) as i32;
        *S.offset(0 as isize) = (*S.offset(1 as isize) as i64
            + (out32 as i64 * *A_Q14.offset(0 as isize) as i64 >> 16))
            as i32;
        *S.offset(1 as isize) = (out32 as i64 * *A_Q14.offset(1 as isize) as i64 >> 16) as i32;
        k += 1;
    }
}
