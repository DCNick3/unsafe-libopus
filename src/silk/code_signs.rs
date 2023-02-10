use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
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
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: u32,
        pub end_offs: u32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: u32,
        pub rng: u32,
        pub val: u32,
        pub ext: u32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
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
    extern "C" {
        #[c2rust::src_loc = "66:26"]
        pub static silk_sign_iCDF: [u8; 42];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:32"]
pub mod define_h {
    #[c2rust::src_loc = "168:9"]
    pub const SHELL_CODEC_FRAME_LENGTH: libc::c_int = 16 as libc::c_int;
}
pub use self::define_h::SHELL_CODEC_FRAME_LENGTH;
pub use self::entcode_h::{ec_ctx, ec_dec, ec_enc, ec_window};
use self::entdec_h::ec_dec_icdf;
use self::entenc_h::ec_enc_icdf;
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::tables_h::silk_sign_iCDF;
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint32_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "41:1"]
pub unsafe extern "C" fn silk_encode_signs(
    psRangeEnc: *mut ec_enc,
    pulses: *const i8,
    mut length: libc::c_int,
    signalType: libc::c_int,
    quantOffsetType: libc::c_int,
    sum_pulses: *const libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut icdf: [u8; 2] = [0; 2];
    let mut q_ptr: *const i8 = 0 as *const i8;
    let mut icdf_ptr: *const u8 = 0 as *const u8;
    icdf[1 as libc::c_int as usize] = 0 as libc::c_int as u8;
    q_ptr = pulses;
    i = 7 as libc::c_int as i16 as i32
        * (quantOffsetType + ((signalType as u32) << 1 as libc::c_int) as i32) as i16 as i32;
    icdf_ptr = &*silk_sign_iCDF.as_ptr().offset(i as isize) as *const u8;
    length = length + 16 as libc::c_int / 2 as libc::c_int >> 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        p = *sum_pulses.offset(i as isize);
        if p > 0 as libc::c_int {
            icdf[0 as libc::c_int as usize] = *icdf_ptr.offset(
                (if (p & 0x1f as libc::c_int) < 6 as libc::c_int {
                    p & 0x1f as libc::c_int
                } else {
                    6 as libc::c_int
                }) as isize,
            );
            j = 0 as libc::c_int;
            while j < SHELL_CODEC_FRAME_LENGTH {
                if *q_ptr.offset(j as isize) as libc::c_int != 0 as libc::c_int {
                    ec_enc_icdf(
                        psRangeEnc,
                        (*q_ptr.offset(j as isize) as libc::c_int >> 15 as libc::c_int)
                            + 1 as libc::c_int,
                        icdf.as_mut_ptr(),
                        8 as libc::c_int as libc::c_uint,
                    );
                }
                j += 1;
            }
        }
        q_ptr = q_ptr.offset(SHELL_CODEC_FRAME_LENGTH as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn silk_decode_signs(
    psRangeDec: *mut ec_dec,
    pulses: *mut i16,
    mut length: libc::c_int,
    signalType: libc::c_int,
    quantOffsetType: libc::c_int,
    sum_pulses: *const libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut icdf: [u8; 2] = [0; 2];
    let mut q_ptr: *mut i16 = 0 as *mut i16;
    let mut icdf_ptr: *const u8 = 0 as *const u8;
    icdf[1 as libc::c_int as usize] = 0 as libc::c_int as u8;
    q_ptr = pulses;
    i = 7 as libc::c_int as i16 as i32
        * (quantOffsetType + ((signalType as u32) << 1 as libc::c_int) as i32) as i16 as i32;
    icdf_ptr = &*silk_sign_iCDF.as_ptr().offset(i as isize) as *const u8;
    length = length + 16 as libc::c_int / 2 as libc::c_int >> 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        p = *sum_pulses.offset(i as isize);
        if p > 0 as libc::c_int {
            icdf[0 as libc::c_int as usize] = *icdf_ptr.offset(
                (if (p & 0x1f as libc::c_int) < 6 as libc::c_int {
                    p & 0x1f as libc::c_int
                } else {
                    6 as libc::c_int
                }) as isize,
            );
            j = 0 as libc::c_int;
            while j < SHELL_CODEC_FRAME_LENGTH {
                if *q_ptr.offset(j as isize) as libc::c_int > 0 as libc::c_int {
                    let ref mut fresh0 = *q_ptr.offset(j as isize);
                    *fresh0 = (*fresh0 as libc::c_int
                        * (((ec_dec_icdf(
                            psRangeDec,
                            icdf.as_mut_ptr(),
                            8 as libc::c_int as libc::c_uint,
                        ) as u32)
                            << 1 as libc::c_int) as i32
                            - 1 as libc::c_int)) as i16;
                }
                j += 1;
            }
        }
        q_ptr = q_ptr.offset(SHELL_CODEC_FRAME_LENGTH as isize);
        i += 1;
    }
}
