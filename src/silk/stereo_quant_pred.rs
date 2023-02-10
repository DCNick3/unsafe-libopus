use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "89:26"]
        pub static silk_stereo_pred_quant_Q13: [i16; 16];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "81:9"]
    pub const STEREO_QUANT_SUB_STEPS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "80:9"]
    pub const STEREO_QUANT_TAB_SIZE: libc::c_int = 16 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
pub use self::define_h::{STEREO_QUANT_SUB_STEPS, STEREO_QUANT_TAB_SIZE};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
use self::tables_h::silk_stereo_pred_quant_Q13;
pub use self::typedef_h::silk_int32_MAX;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_quant_pred(pred_Q13: *mut i32, ix: *mut [i8; 3]) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut low_Q13: i32 = 0;
    let mut step_Q13: i32 = 0;
    let mut lvl_Q13: i32 = 0;
    let mut err_min_Q13: i32 = 0;
    let mut err_Q13: i32 = 0;
    let mut quant_pred_Q13: i32 = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        err_min_Q13 = silk_int32_MAX;
        i = 0 as libc::c_int;
        's_18: while i < STEREO_QUANT_TAB_SIZE - 1 as libc::c_int {
            low_Q13 = silk_stereo_pred_quant_Q13[i as usize] as i32;
            step_Q13 = ((silk_stereo_pred_quant_Q13[(i + 1 as libc::c_int) as usize] as libc::c_int
                - low_Q13) as libc::c_long
                * (0.5f64 / 5 as libc::c_int as libc::c_double
                    * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32 as i16 as i64
                >> 16 as libc::c_int) as i32;
            j = 0 as libc::c_int;
            while j < STEREO_QUANT_SUB_STEPS {
                lvl_Q13 = low_Q13
                    + step_Q13 as i16 as i32
                        * (2 as libc::c_int * j + 1 as libc::c_int) as i16 as i32;
                err_Q13 = if *pred_Q13.offset(n as isize) - lvl_Q13 > 0 as libc::c_int {
                    *pred_Q13.offset(n as isize) - lvl_Q13
                } else {
                    -(*pred_Q13.offset(n as isize) - lvl_Q13)
                };
                if !(err_Q13 < err_min_Q13) {
                    break 's_18;
                }
                err_min_Q13 = err_Q13;
                quant_pred_Q13 = lvl_Q13;
                (*ix.offset(n as isize))[0 as libc::c_int as usize] = i as i8;
                (*ix.offset(n as isize))[1 as libc::c_int as usize] = j as i8;
                j += 1;
            }
            i += 1;
        }
        (*ix.offset(n as isize))[2 as libc::c_int as usize] =
            ((*ix.offset(n as isize))[0 as libc::c_int as usize] as libc::c_int / 3 as libc::c_int)
                as i8;
        let ref mut fresh0 = (*ix.offset(n as isize))[0 as libc::c_int as usize];
        *fresh0 = (*fresh0 as libc::c_int
            - (*ix.offset(n as isize))[2 as libc::c_int as usize] as libc::c_int * 3 as libc::c_int)
            as i8;
        *pred_Q13.offset(n as isize) = quant_pred_Q13;
        n += 1;
    }
    let ref mut fresh1 = *pred_Q13.offset(0 as libc::c_int as isize);
    *fresh1 -= *pred_Q13.offset(1 as libc::c_int as isize);
}
