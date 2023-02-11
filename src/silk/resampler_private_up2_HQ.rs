use crate::silk::resampler_rom::{silk_resampler_up2_hq_0, silk_resampler_up2_hq_1};
use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
use crate::silk::resampler_structs::silk_resampler_state_struct;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

#[c2rust::src_loc = "38:1"]
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
    k = 0 as libc::c_int;
    while k < len {
        in32 = ((*in_0.offset(k as isize) as i32 as u32) << 10 as libc::c_int) as i32;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_0[0 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_1 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        Y = out32_1 - *S.offset(1 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_0[1 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_2 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = out32_1 + X;
        Y = out32_2 - *S.offset(2 as libc::c_int as isize);
        X = (Y as libc::c_long
            + (Y as libc::c_long * silk_resampler_up2_hq_0[2 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        out32_1 = *S.offset(2 as libc::c_int as isize) + X;
        *S.offset(2 as libc::c_int as isize) = out32_2 + X;
        *out.offset((2 as libc::c_int * k) as isize) = (if (if 10 as libc::c_int == 1 as libc::c_int
        {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        Y = in32 - *S.offset(3 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_1[0 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_1 = *S.offset(3 as libc::c_int as isize) + X;
        *S.offset(3 as libc::c_int as isize) = in32 + X;
        Y = out32_1 - *S.offset(4 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_1[1 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_2 = *S.offset(4 as libc::c_int as isize) + X;
        *S.offset(4 as libc::c_int as isize) = out32_1 + X;
        Y = out32_2 - *S.offset(5 as libc::c_int as isize);
        X = (Y as libc::c_long
            + (Y as libc::c_long * silk_resampler_up2_hq_1[2 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        out32_1 = *S.offset(5 as libc::c_int as isize) + X;
        *S.offset(5 as libc::c_int as isize) = out32_2 + X;
        *out.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) = (if (if 10 as libc::c_int
            == 1 as libc::c_int
        {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        k += 1;
    }
}
#[c2rust::src_loc = "104:1"]
pub unsafe fn silk_resampler_private_up2_HQ_wrapper(
    SS: *mut libc::c_void,
    out: *mut i16,
    in_0: *const i16,
    len: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    silk_resampler_private_up2_HQ(((*S).sIIR).as_mut_ptr(), out, in_0, len);
}
