use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
pub use self::types_h::{__int16_t, __int32_t, __uint32_t, __int64_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32, opus_int64};
pub use self::typedef_h::{silk_int16_MIN, silk_int16_MAX};
#[c2rust::src_loc = "35:19"]
static mut A_fb1_20: opus_int16 = ((5394 as libc::c_int) << 1 as libc::c_int)
    as opus_int16;
#[c2rust::src_loc = "36:19"]
static mut A_fb1_21: opus_int16 = -(24290 as libc::c_int) as opus_int16;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_ana_filt_bank_1(
    mut in_0: *const opus_int16,
    mut S: *mut opus_int32,
    mut outL: *mut opus_int16,
    mut outH: *mut opus_int16,
    N: opus_int32,
) {
    let mut k: libc::c_int = 0;
    let mut N2: libc::c_int = N >> 1 as libc::c_int;
    let mut in32: opus_int32 = 0;
    let mut X: opus_int32 = 0;
    let mut Y: opus_int32 = 0;
    let mut out_1: opus_int32 = 0;
    let mut out_2: opus_int32 = 0;
    k = 0 as libc::c_int;
    while k < N2 {
        in32 = ((*in_0.offset((2 as libc::c_int * k) as isize) as opus_int32
            as opus_uint32) << 10 as libc::c_int) as opus_int32;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y as libc::c_long
            + (Y as libc::c_long * A_fb1_21 as opus_int64 >> 16 as libc::c_int))
            as opus_int32;
        out_1 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        in32 = ((*in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize)
            as opus_int32 as opus_uint32) << 10 as libc::c_int) as opus_int32;
        Y = in32 - *S.offset(1 as libc::c_int as isize);
        X = (Y as libc::c_long * A_fb1_20 as opus_int64 >> 16 as libc::c_int)
            as opus_int32;
        out_2 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = in32 + X;
        *outL
            .offset(
                k as isize,
            ) = (if (if 11 as libc::c_int == 1 as libc::c_int {
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
        }) as opus_int16;
        *outH
            .offset(
                k as isize,
            ) = (if (if 11 as libc::c_int == 1 as libc::c_int {
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
        }) as opus_int16;
        k += 1;
    }
}
