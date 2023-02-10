use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int32_t, __int64_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int32_t, int64_t};
}
pub use self::types_h::{__int32_t, __int64_t};
pub use self::stdint_intn_h::{int32_t, int64_t};
pub use self::opus_types_h::{opus_int32, opus_int64};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_bwexpander_32(
    mut ar: *mut opus_int32,
    d: libc::c_int,
    mut chirp_Q16: opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut chirp_minus_one_Q16: opus_int32 = chirp_Q16 - 65536 as libc::c_int;
    i = 0 as libc::c_int;
    while i < d - 1 as libc::c_int {
        *ar
            .offset(
                i as isize,
            ) = (chirp_Q16 as opus_int64 * *ar.offset(i as isize) as libc::c_long
            >> 16 as libc::c_int) as opus_int32;
        chirp_Q16
            += if 16 as libc::c_int == 1 as libc::c_int {
                (chirp_Q16 * chirp_minus_one_Q16 >> 1 as libc::c_int)
                    + (chirp_Q16 * chirp_minus_one_Q16 & 1 as libc::c_int)
            } else {
                (chirp_Q16 * chirp_minus_one_Q16 >> 16 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int >> 1 as libc::c_int
            };
        i += 1;
    }
    *ar
        .offset(
            (d - 1 as libc::c_int) as isize,
        ) = (chirp_Q16 as opus_int64
        * *ar.offset((d - 1 as libc::c_int) as isize) as libc::c_long
        >> 16 as libc::c_int) as opus_int32;
}
