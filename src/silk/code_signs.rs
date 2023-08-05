use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::define::SHELL_CODEC_FRAME_LENGTH;
use crate::silk::tables_pulses_per_block::silk_sign_iCDF;

pub unsafe fn silk_encode_signs(
    psRangeEnc: &mut ec_enc,
    pulses: *const i8,
    mut length: i32,
    signalType: i32,
    quantOffsetType: i32,
    sum_pulses: *const i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut icdf: [u8; 2] = [0; 2];
    let mut q_ptr: *const i8 = 0 as *const i8;
    let mut icdf_ptr: *const u8 = 0 as *const u8;
    icdf[1 as usize] = 0;
    q_ptr = pulses;
    i = 7 * (quantOffsetType + ((signalType as u32) << 1) as i32) as i16 as i32;
    icdf_ptr = &*silk_sign_iCDF.as_ptr().offset(i as isize) as *const u8;
    length = length + 16 / 2 >> 4;
    i = 0;
    while i < length {
        p = *sum_pulses.offset(i as isize);
        if p > 0 {
            icdf[0 as usize] =
                *icdf_ptr.offset((if (p & 0x1f) < 6 { p & 0x1f } else { 6 }) as isize);
            j = 0;
            while j < SHELL_CODEC_FRAME_LENGTH {
                if *q_ptr.offset(j as isize) as i32 != 0 {
                    ec_enc_icdf(
                        psRangeEnc,
                        (*q_ptr.offset(j as isize) as i32 >> 15) + 1,
                        icdf.as_mut_ptr(),
                        8,
                    );
                }
                j += 1;
            }
        }
        q_ptr = q_ptr.offset(SHELL_CODEC_FRAME_LENGTH as isize);
        i += 1;
    }
}
pub unsafe fn silk_decode_signs(
    psRangeDec: &mut ec_dec,
    pulses: *mut i16,
    mut length: i32,
    signalType: i32,
    quantOffsetType: i32,
    sum_pulses: *const i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut icdf: [u8; 2] = [0; 2];
    let mut q_ptr: *mut i16 = 0 as *mut i16;
    let mut icdf_ptr: *const u8 = 0 as *const u8;
    icdf[1 as usize] = 0;
    q_ptr = pulses;
    i = 7 * (quantOffsetType + ((signalType as u32) << 1) as i32) as i16 as i32;
    icdf_ptr = &*silk_sign_iCDF.as_ptr().offset(i as isize) as *const u8;
    length = length + 16 / 2 >> 4;
    i = 0;
    while i < length {
        p = *sum_pulses.offset(i as isize);
        if p > 0 {
            icdf[0 as usize] =
                *icdf_ptr.offset((if (p & 0x1f) < 6 { p & 0x1f } else { 6 }) as isize);
            j = 0;
            while j < SHELL_CODEC_FRAME_LENGTH {
                if *q_ptr.offset(j as isize) as i32 > 0 {
                    let ref mut fresh0 = *q_ptr.offset(j as isize);
                    *fresh0 = (*fresh0 as i32
                        * (((ec_dec_icdf(psRangeDec, icdf.as_mut_ptr(), 8) as u32) << 1) as i32
                            - 1)) as i16;
                }
                j += 1;
            }
        }
        q_ptr = q_ptr.offset(SHELL_CODEC_FRAME_LENGTH as isize);
        i += 1;
    }
}
