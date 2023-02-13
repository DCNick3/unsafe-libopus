pub unsafe fn silk_bwexpander_32(ar: *mut i32, d: i32, mut chirp_Q16: i32) {
    let mut i: i32 = 0;
    let chirp_minus_one_Q16: i32 = chirp_Q16 - 65536 as i32;
    i = 0 as i32;
    while i < d - 1 as i32 {
        *ar.offset(i as isize) =
            (chirp_Q16 as i64 * *ar.offset(i as isize) as i64 >> 16 as i32) as i32;
        chirp_Q16 += if 16 as i32 == 1 as i32 {
            (chirp_Q16 * chirp_minus_one_Q16 >> 1 as i32)
                + (chirp_Q16 * chirp_minus_one_Q16 & 1 as i32)
        } else {
            (chirp_Q16 * chirp_minus_one_Q16 >> 16 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
        };
        i += 1;
    }
    *ar.offset((d - 1 as i32) as isize) =
        (chirp_Q16 as i64 * *ar.offset((d - 1 as i32) as isize) as i64 >> 16 as i32) as i32;
}
