use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:34"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
#[c2rust::src_loc = "37:25"]
static mut sigm_LUT_slope_Q10: [opus_int32; 6] = [
    237 as libc::c_int,
    153 as libc::c_int,
    73 as libc::c_int,
    30 as libc::c_int,
    12 as libc::c_int,
    7 as libc::c_int,
];
#[c2rust::src_loc = "41:25"]
static mut sigm_LUT_pos_Q15: [opus_int32; 6] = [
    16384 as libc::c_int,
    23955 as libc::c_int,
    28861 as libc::c_int,
    31213 as libc::c_int,
    32178 as libc::c_int,
    32548 as libc::c_int,
];
#[c2rust::src_loc = "45:25"]
static mut sigm_LUT_neg_Q15: [opus_int32; 6] = [
    16384 as libc::c_int,
    8812 as libc::c_int,
    3906 as libc::c_int,
    1554 as libc::c_int,
    589 as libc::c_int,
    219 as libc::c_int,
];
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn silk_sigm_Q15(mut in_Q5: libc::c_int) -> libc::c_int {
    let mut ind: libc::c_int = 0;
    if in_Q5 < 0 as libc::c_int {
        in_Q5 = -in_Q5;
        if in_Q5 >= 6 as libc::c_int * 32 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            ind = in_Q5 >> 5 as libc::c_int;
            return sigm_LUT_neg_Q15[ind as usize]
                - sigm_LUT_slope_Q10[ind as usize] as opus_int16 as opus_int32
                    * (in_Q5 & 0x1f as libc::c_int) as opus_int16 as opus_int32;
        }
    } else if in_Q5 >= 6 as libc::c_int * 32 as libc::c_int {
        return 32767 as libc::c_int;
    } else {
        ind = in_Q5 >> 5 as libc::c_int;
        return sigm_LUT_pos_Q15[ind as usize]
            + sigm_LUT_slope_Q10[ind as usize] as opus_int16 as opus_int32
                * (in_Q5 & 0x1f as libc::c_int) as opus_int16 as opus_int32;
    };
}
