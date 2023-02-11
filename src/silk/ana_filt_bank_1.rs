use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

#[c2rust::src_loc = "35:19"]
static mut A_fb1_20: i16 = ((5394 as libc::c_int) << 1 as libc::c_int) as i16;
#[c2rust::src_loc = "36:19"]
static mut A_fb1_21: i16 = -(24290 as libc::c_int) as i16;
#[c2rust::src_loc = "39:1"]
pub unsafe fn silk_ana_filt_bank_1(
    in_0: *const i16,
    S: *mut i32,
    outL: *mut i16,
    outH: *mut i16,
    N: i32,
) {
    let mut k: libc::c_int = 0;
    let N2: libc::c_int = N >> 1 as libc::c_int;
    let mut in32: i32 = 0;
    let mut X: i32 = 0;
    let mut Y: i32 = 0;
    let mut out_1: i32 = 0;
    let mut out_2: i32 = 0;
    k = 0 as libc::c_int;
    while k < N2 {
        in32 = ((*in_0.offset((2 as libc::c_int * k) as isize) as i32 as u32) << 10 as libc::c_int)
            as i32;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y as libc::c_long + (Y as libc::c_long * A_fb1_21 as i64 >> 16 as libc::c_int)) as i32;
        out_1 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        in32 = ((*in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) as i32 as u32)
            << 10 as libc::c_int) as i32;
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = (Y as libc::c_long * A_fb1_20 as i64 >> 16 as libc::c_int) as i32;
        out_2 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        *outL.offset(k as isize) = (if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 + out_1 >> 1 as libc::c_int) + (out_2 + out_1 & 1 as libc::c_int)
        } else {
            (out_2 + out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 + out_1 >> 1 as libc::c_int) + (out_2 + out_1 & 1 as libc::c_int)
        } else {
            (out_2 + out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 + out_1 >> 1 as libc::c_int) + (out_2 + out_1 & 1 as libc::c_int)
        } else {
            (out_2 + out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as i16;
        *outH.offset(k as isize) = (if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 - out_1 >> 1 as libc::c_int) + (out_2 - out_1 & 1 as libc::c_int)
        } else {
            (out_2 - out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 - out_1 >> 1 as libc::c_int) + (out_2 - out_1 & 1 as libc::c_int)
        } else {
            (out_2 - out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 11 as libc::c_int == 1 as libc::c_int {
            (out_2 - out_1 >> 1 as libc::c_int) + (out_2 - out_1 & 1 as libc::c_int)
        } else {
            (out_2 - out_1 >> 11 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        }) as i16;
        k += 1;
    }
}
