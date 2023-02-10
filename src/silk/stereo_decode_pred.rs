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
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
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
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:32"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::{opus_int16, opus_uint8};
    extern "C" {
        #[c2rust::src_loc = "68:26"]
        pub static silk_uniform3_iCDF: [opus_uint8; 3];
        #[c2rust::src_loc = "70:26"]
        pub static silk_uniform5_iCDF: [opus_uint8; 5];
        #[c2rust::src_loc = "89:26"]
        pub static silk_stereo_pred_quant_Q13: [opus_int16; 16];
        #[c2rust::src_loc = "90:26"]
        pub static silk_stereo_pred_joint_iCDF: [opus_uint8; 25];
        #[c2rust::src_loc = "91:26"]
        pub static silk_stereo_only_code_mid_iCDF: [opus_uint8; 2];
    }
}
pub use self::entcode_h::{ec_ctx, ec_dec, ec_window};
use self::entdec_h::ec_dec_icdf;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32, opus_uint8};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::tables_h::{
    silk_stereo_only_code_mid_iCDF, silk_stereo_pred_joint_iCDF, silk_stereo_pred_quant_Q13,
    silk_uniform3_iCDF, silk_uniform5_iCDF,
};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_decode_pred(
    mut psRangeDec: *mut ec_dec,
    mut pred_Q13: *mut opus_int32,
) {
    let mut n: libc::c_int = 0;
    let mut ix: [[libc::c_int; 3]; 2] = [[0; 3]; 2];
    let mut low_Q13: opus_int32 = 0;
    let mut step_Q13: opus_int32 = 0;
    n = ec_dec_icdf(
        psRangeDec,
        silk_stereo_pred_joint_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    ix[0 as libc::c_int as usize][2 as libc::c_int as usize] = n / 5 as libc::c_int;
    ix[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        n - 5 as libc::c_int * ix[0 as libc::c_int as usize][2 as libc::c_int as usize];
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ix[n as usize][0 as libc::c_int as usize] = ec_dec_icdf(
            psRangeDec,
            silk_uniform3_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        ix[n as usize][1 as libc::c_int as usize] = ec_dec_icdf(
            psRangeDec,
            silk_uniform5_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        n += 1;
    }
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ix[n as usize][0 as libc::c_int as usize] +=
            3 as libc::c_int * ix[n as usize][2 as libc::c_int as usize];
        low_Q13 = silk_stereo_pred_quant_Q13[ix[n as usize][0 as libc::c_int as usize] as usize]
            as opus_int32;
        step_Q13 = ((silk_stereo_pred_quant_Q13
            [(ix[n as usize][0 as libc::c_int as usize] + 1 as libc::c_int) as usize]
            as libc::c_int
            - low_Q13) as libc::c_long
            * (0.5f64 / 5 as libc::c_int as libc::c_double
                * ((1 as libc::c_int as opus_int64) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as opus_int32 as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32;
        *pred_Q13.offset(n as isize) = low_Q13
            + step_Q13 as opus_int16 as opus_int32
                * (2 as libc::c_int * ix[n as usize][1 as libc::c_int as usize] + 1 as libc::c_int)
                    as opus_int16 as opus_int32;
        n += 1;
    }
    let ref mut fresh0 = *pred_Q13.offset(0 as libc::c_int as isize);
    *fresh0 -= *pred_Q13.offset(1 as libc::c_int as isize);
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn silk_stereo_decode_mid_only(
    mut psRangeDec: *mut ec_dec,
    mut decode_only_mid: *mut libc::c_int,
) {
    *decode_only_mid = ec_dec_icdf(
        psRangeDec,
        silk_stereo_only_code_mid_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
