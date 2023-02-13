#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_bwexpander(ar: *mut i16, d: libc::c_int, mut chirp_Q16: i32) {
    let mut i: libc::c_int = 0;
    let chirp_minus_one_Q16: i32 = chirp_Q16 - 65536 as libc::c_int;
    i = 0 as libc::c_int;
    while i < d - 1 as libc::c_int {
        *ar.offset(i as isize) = (if 16 as libc::c_int == 1 as libc::c_int {
            (chirp_Q16 * *ar.offset(i as isize) as libc::c_int >> 1 as libc::c_int)
                + (chirp_Q16 * *ar.offset(i as isize) as libc::c_int & 1 as libc::c_int)
        } else {
            (chirp_Q16 * *ar.offset(i as isize) as libc::c_int
                >> 16 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as i16;
        chirp_Q16 += if 16 as libc::c_int == 1 as libc::c_int {
            (chirp_Q16 * chirp_minus_one_Q16 >> 1 as libc::c_int)
                + (chirp_Q16 * chirp_minus_one_Q16 & 1 as libc::c_int)
        } else {
            (chirp_Q16 * chirp_minus_one_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        i += 1;
    }
    *ar.offset((d - 1 as libc::c_int) as isize) = (if 16 as libc::c_int == 1 as libc::c_int {
        (chirp_Q16 * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_int >> 1 as libc::c_int)
            + (chirp_Q16 * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_int
                & 1 as libc::c_int)
    } else {
        (chirp_Q16 * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_int
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    }) as i16;
}
