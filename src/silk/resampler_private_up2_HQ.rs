use crate::silk::resampler_rom::{silk_resampler_up2_hq_0, silk_resampler_up2_hq_1};

pub mod typedef_h {
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
    pub const silk_int16_MIN: i32 = i16::MIN as i32;
}
use crate::silk::resampler_structs::silk_resampler_state_struct;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

pub unsafe fn silk_resampler_private_up2_HQ(
    S: *mut i32,
    out: *mut i16,
    in_0: *const i16,
    len: i32,
) {
    let mut k: i32 = 0;
    let mut in32: i32 = 0;
    let mut out32_1: i32 = 0;
    let mut out32_2: i32 = 0;
    let mut Y: i32 = 0;
    let mut X: i32 = 0;
    k = 0;
    while k < len {
        in32 = ((*in_0.offset(k as isize) as i32 as u32) << 10) as i32;
        Y = in32 - *S.offset(0 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_0[0 as usize] as i64 >> 16) as i32;
        out32_1 = *S.offset(0 as isize) + X;
        *S.offset(0 as isize) = in32 + X;
        Y = out32_1 - *S.offset(1 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_0[1 as usize] as i64 >> 16) as i32;
        out32_2 = *S.offset(1 as isize) + X;
        *S.offset(1 as isize) = out32_1 + X;
        Y = out32_2 - *S.offset(2 as isize);
        X = (Y as i64 + (Y as i64 * silk_resampler_up2_hq_0[2 as usize] as i64 >> 16)) as i32;
        out32_1 = *S.offset(2 as isize) + X;
        *S.offset(2 as isize) = out32_2 + X;
        *out.offset((2 * k) as isize) = (if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) as i16;
        Y = in32 - *S.offset(3 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_1[0 as usize] as i64 >> 16) as i32;
        out32_1 = *S.offset(3 as isize) + X;
        *S.offset(3 as isize) = in32 + X;
        Y = out32_1 - *S.offset(4 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_1[1 as usize] as i64 >> 16) as i32;
        out32_2 = *S.offset(4 as isize) + X;
        *S.offset(4 as isize) = out32_1 + X;
        Y = out32_2 - *S.offset(5 as isize);
        X = (Y as i64 + (Y as i64 * silk_resampler_up2_hq_1[2 as usize] as i64 >> 16)) as i32;
        out32_1 = *S.offset(5 as isize) + X;
        *S.offset(5 as isize) = out32_2 + X;
        *out.offset((2 * k + 1) as isize) = (if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 == 1 {
            (out32_1 >> 1) + (out32_1 & 1)
        } else {
            (out32_1 >> 10 - 1) + 1 >> 1
        }) as i16;
        k += 1;
    }
}
pub unsafe fn silk_resampler_private_up2_HQ_wrapper(
    SS: *mut core::ffi::c_void,
    out: *mut i16,
    in_0: *const i16,
    len: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    silk_resampler_private_up2_HQ(((*S).sIIR).as_mut_ptr(), out, in_0, len);
}
