pub mod typedef_h {
    pub const silk_int16_MIN: i32 = i16::MIN as i32;
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
}
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
use crate::silk::resampler_rom::{silk_resampler_down2_0, silk_resampler_down2_1};

pub unsafe fn silk_resampler_down2(S: *mut i32, out: *mut i16, in_0: *const i16, inLen: i32) {
    let mut k: i32 = 0;
    let len2: i32 = inLen >> 1;
    let mut in32: i32 = 0;
    let mut out32: i32 = 0;
    let mut Y: i32 = 0;
    let mut X: i32 = 0;
    assert!(silk_resampler_down2_0 as i32 > 0);
    assert!((silk_resampler_down2_1 as i32) < 0);
    k = 0;
    while k < len2 {
        in32 = ((*in_0.offset((2 * k) as isize) as i32 as u32) << 10) as i32;
        Y = in32 - *S.offset(0 as isize);
        X = (Y as i64 + (Y as i64 * silk_resampler_down2_1 as i64 >> 16)) as i32;
        out32 = *S.offset(0 as isize) + X;
        *S.offset(0 as isize) = in32 + X;
        in32 = ((*in_0.offset((2 * k + 1) as isize) as i32 as u32) << 10) as i32;
        Y = in32 - *S.offset(1 as isize);
        X = (Y as i64 * silk_resampler_down2_0 as i64 >> 16) as i32;
        out32 = out32 + *S.offset(1 as isize);
        out32 = out32 + X;
        *S.offset(1 as isize) = in32 + X;
        *out.offset(k as isize) = (if (if 11 == 1 {
            (out32 >> 1) + (out32 & 1)
        } else {
            (out32 >> 11 - 1) + 1 >> 1
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 11 == 1 {
            (out32 >> 1) + (out32 & 1)
        } else {
            (out32 >> 11 - 1) + 1 >> 1
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 11 == 1 {
            (out32 >> 1) + (out32 & 1)
        } else {
            (out32 >> 11 - 1) + 1 >> 1
        }) as i16;
        k += 1;
    }
}
