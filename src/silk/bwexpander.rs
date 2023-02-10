use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_bwexpander(ar: *mut i16, d: libc::c_int, mut chirp_Q16: i32) {
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
