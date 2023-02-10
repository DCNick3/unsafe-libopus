use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    use super::types_h::__int8_t;
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
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::int8_t;
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
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:32"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    use super::opus_types_h::opus_uint8;
    extern "C" {
        #[c2rust::src_loc = "68:26"]
        pub static silk_uniform3_iCDF: [opus_uint8; 3];
        #[c2rust::src_loc = "70:26"]
        pub static silk_uniform5_iCDF: [opus_uint8; 5];
        #[c2rust::src_loc = "90:26"]
        pub static silk_stereo_pred_joint_iCDF: [opus_uint8; 25];
        #[c2rust::src_loc = "91:26"]
        pub static silk_stereo_only_code_mid_iCDF: [opus_uint8; 2];
    }
}
pub use self::entcode_h::{ec_ctx, ec_enc, ec_window};
use self::entenc_h::ec_enc_icdf;
pub use self::opus_types_h::{opus_int8, opus_uint32, opus_uint8};
pub use self::stdint_intn_h::int8_t;
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::tables_h::{
    silk_stereo_only_code_mid_iCDF, silk_stereo_pred_joint_iCDF, silk_uniform3_iCDF,
    silk_uniform5_iCDF,
};
pub use self::types_h::{__int8_t, __uint32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_encode_pred(
    mut psRangeEnc: *mut ec_enc,
    mut ix: *mut [opus_int8; 3],
) {
    let mut n: libc::c_int = 0;
    n = 5 as libc::c_int
        * (*ix.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] as libc::c_int
        + (*ix.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] as libc::c_int;
    ec_enc_icdf(
        psRangeEnc,
        n,
        silk_stereo_pred_joint_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ec_enc_icdf(
            psRangeEnc,
            (*ix.offset(n as isize))[0 as libc::c_int as usize] as libc::c_int,
            silk_uniform3_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        ec_enc_icdf(
            psRangeEnc,
            (*ix.offset(n as isize))[1 as libc::c_int as usize] as libc::c_int,
            silk_uniform5_iCDF.as_ptr(),
            8 as libc::c_int as libc::c_uint,
        );
        n += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn silk_stereo_encode_mid_only(
    mut psRangeEnc: *mut ec_enc,
    mut mid_only_flag: opus_int8,
) {
    ec_enc_icdf(
        psRangeEnc,
        mid_only_flag as libc::c_int,
        silk_stereo_only_code_mid_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
