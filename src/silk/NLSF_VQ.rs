use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __int32_t, __uint32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::opus_types_h::{opus_uint8, opus_int16, opus_int32, opus_uint32};
use self::arch_h::celt_fatal;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_NLSF_VQ(
    mut err_Q24: *mut opus_int32,
    mut in_Q15: *const opus_int16,
    mut pCB_Q8: *const opus_uint8,
    mut pWght_Q9: *const opus_int16,
    K: libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut diff_Q15: opus_int32 = 0;
    let mut diffw_Q24: opus_int32 = 0;
    let mut sum_error_Q24: opus_int32 = 0;
    let mut pred_Q24: opus_int32 = 0;
    let mut w_Q9_ptr: *const opus_int16 = 0 as *const opus_int16;
    let mut cb_Q8_ptr: *const opus_uint8 = 0 as *const opus_uint8;
    if !(LPC_order & 1 as libc::c_int == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ( LPC_order & 1 ) == 0\0" as *const u8
                as *const libc::c_char,
            b"silk/NLSF_VQ.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
        );
    }
    cb_Q8_ptr = pCB_Q8;
    w_Q9_ptr = pWght_Q9;
    i = 0 as libc::c_int;
    while i < K {
        sum_error_Q24 = 0 as libc::c_int;
        pred_Q24 = 0 as libc::c_int;
        m = LPC_order - 2 as libc::c_int;
        while m >= 0 as libc::c_int {
            diff_Q15 = *in_Q15.offset((m + 1 as libc::c_int) as isize) as libc::c_int
                - ((*cb_Q8_ptr.offset((m + 1 as libc::c_int) as isize) as opus_int32
                    as opus_uint32) << 7 as libc::c_int) as opus_int32;
            diffw_Q24 = diff_Q15 as opus_int16 as opus_int32
                * *w_Q9_ptr.offset((m + 1 as libc::c_int) as isize) as opus_int32;
            sum_error_Q24 = sum_error_Q24
                + (if diffw_Q24 - (pred_Q24 >> 1 as libc::c_int) > 0 as libc::c_int {
                    diffw_Q24 - (pred_Q24 >> 1 as libc::c_int)
                } else {
                    -(diffw_Q24 - (pred_Q24 >> 1 as libc::c_int))
                });
            pred_Q24 = diffw_Q24;
            diff_Q15 = *in_Q15.offset(m as isize) as libc::c_int
                - ((*cb_Q8_ptr.offset(m as isize) as opus_int32 as opus_uint32)
                    << 7 as libc::c_int) as opus_int32;
            diffw_Q24 = diff_Q15 as opus_int16 as opus_int32
                * *w_Q9_ptr.offset(m as isize) as opus_int32;
            sum_error_Q24 = sum_error_Q24
                + (if diffw_Q24 - (pred_Q24 >> 1 as libc::c_int) > 0 as libc::c_int {
                    diffw_Q24 - (pred_Q24 >> 1 as libc::c_int)
                } else {
                    -(diffw_Q24 - (pred_Q24 >> 1 as libc::c_int))
                });
            pred_Q24 = diffw_Q24;
            m -= 2 as libc::c_int;
        }
        *err_Q24.offset(i as isize) = sum_error_Q24;
        cb_Q8_ptr = cb_Q8_ptr.offset(LPC_order as isize);
        w_Q9_ptr = w_Q9_ptr.offset(LPC_order as isize);
        i += 1;
    }
}
