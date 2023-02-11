use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
use crate::celt::celt::celt_fatal;
use crate::silk::resampler_rom::{silk_resampler_down2_0, silk_resampler_down2_1};

#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_resampler_down2(
    S: *mut i32,
    out: *mut i16,
    in_0: *const i16,
    inLen: i32,
) {
    let mut k: i32 = 0;
    let len2: i32 = inLen >> 1 as libc::c_int;
    let mut in32: i32 = 0;
    let mut out32: i32 = 0;
    let mut Y: i32 = 0;
    let mut X: i32 = 0;
    if !(silk_resampler_down2_0 as libc::c_int > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: silk_resampler_down2_0 > 0\0" as *const u8 as *const libc::c_char,
            b"silk/resampler_down2.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        );
    }
    if !((silk_resampler_down2_1 as libc::c_int) < 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: silk_resampler_down2_1 < 0\0" as *const u8 as *const libc::c_char,
            b"silk/resampler_down2.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
        );
    }
    k = 0 as libc::c_int;
    while k < len2 {
        in32 = ((*in_0.offset((2 as libc::c_int * k) as isize) as i32 as u32) << 10 as libc::c_int)
            as i32;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y as libc::c_long
            + (Y as libc::c_long * silk_resampler_down2_1 as i64 >> 16 as libc::c_int))
            as i32;
        out32 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        in32 = ((*in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) as i32 as u32)
            << 10 as libc::c_int) as i32;
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_down2_0 as i64 >> 16 as libc::c_int) as i32;
        out32 = out32 + *S.offset(1 as libc::c_int as isize);
        out32 = out32 + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        *out.offset(k as isize) = (if (if 11 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 11 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 11 as libc::c_int == 1 as libc::c_int {
            (out32 >> 1 as libc::c_int) + (out32 & 1 as libc::c_int)
        } else {
            (out32 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        k += 1;
    }
}
