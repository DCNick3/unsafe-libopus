use ::libc;

use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::define::SHELL_CODEC_FRAME_LENGTH;
use crate::silk::tables_pulses_per_block::silk_sign_iCDF;

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
