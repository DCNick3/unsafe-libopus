use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:39"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:39"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:39"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:39"]
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
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn silk_biquad_alt_stride1(
    mut in_0: *const opus_int16,
    mut B_Q28: *const opus_int32,
    mut A_Q28: *const opus_int32,
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    len: opus_int32,
) {
    let mut k: libc::c_int = 0;
    let mut inval: opus_int32 = 0;
    let mut A0_U_Q28: opus_int32 = 0;
    let mut A0_L_Q28: opus_int32 = 0;
    let mut A1_U_Q28: opus_int32 = 0;
    let mut A1_L_Q28: opus_int32 = 0;
    let mut out32_Q14: opus_int32 = 0;
    A0_L_Q28 = -*A_Q28.offset(0 as libc::c_int as isize) & 0x3fff as libc::c_int;
    A0_U_Q28 = -*A_Q28.offset(0 as libc::c_int as isize) >> 14 as libc::c_int;
    A1_L_Q28 = -*A_Q28.offset(1 as libc::c_int as isize) & 0x3fff as libc::c_int;
    A1_U_Q28 = -*A_Q28.offset(1 as libc::c_int as isize) >> 14 as libc::c_int;
    k = 0 as libc::c_int;
    while k < len {
        inval = *in_0.offset(k as isize) as opus_int32;
        out32_Q14 = (((*S.offset(0 as libc::c_int as isize) as libc::c_long
            + (*B_Q28.offset(0 as libc::c_int as isize) as libc::c_long
                * inval as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32 as opus_uint32)
            << 2 as libc::c_int) as opus_int32;
        *S.offset(0 as libc::c_int as isize) = *S.offset(1 as libc::c_int as isize)
            + (if 14 as libc::c_int == 1 as libc::c_int {
                ((out32_Q14 as libc::c_long * A0_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    >> 1 as libc::c_int)
                    + ((out32_Q14 as libc::c_long * A0_L_Q28 as opus_int16 as opus_int64
                        >> 16 as libc::c_int) as opus_int32
                        & 1 as libc::c_int)
            } else {
                ((out32_Q14 as libc::c_long * A0_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    >> 14 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
        *S.offset(0 as libc::c_int as isize) = (*S.offset(0 as libc::c_int as isize)
            as libc::c_long
            + (out32_Q14 as libc::c_long * A0_U_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(0 as libc::c_int as isize) = (*S.offset(0 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(1 as libc::c_int as isize) as libc::c_long
                * inval as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(1 as libc::c_int as isize) = if 14 as libc::c_int == 1 as libc::c_int {
            ((out32_Q14 as libc::c_long * A1_L_Q28 as opus_int16 as opus_int64 >> 16 as libc::c_int)
                as opus_int32
                >> 1 as libc::c_int)
                + ((out32_Q14 as libc::c_long * A1_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    & 1 as libc::c_int)
        } else {
            ((out32_Q14 as libc::c_long * A1_L_Q28 as opus_int16 as opus_int64 >> 16 as libc::c_int)
                as opus_int32
                >> 14 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *S.offset(1 as libc::c_int as isize) = (*S.offset(1 as libc::c_int as isize)
            as libc::c_long
            + (out32_Q14 as libc::c_long * A1_U_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(1 as libc::c_int as isize) = (*S.offset(1 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(2 as libc::c_int as isize) as libc::c_long
                * inval as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *out.offset(k as isize) = (if out32_Q14 + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int
            >> 14 as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (out32_Q14 + ((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int
            >> 14 as libc::c_int)
            < 0x8000 as libc::c_int as opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else {
            out32_Q14 + ((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int
                >> 14 as libc::c_int
        }) as opus_int16;
        k += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn silk_biquad_alt_stride2_c(
    mut in_0: *const opus_int16,
    mut B_Q28: *const opus_int32,
    mut A_Q28: *const opus_int32,
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    len: opus_int32,
) {
    let mut k: libc::c_int = 0;
    let mut A0_U_Q28: opus_int32 = 0;
    let mut A0_L_Q28: opus_int32 = 0;
    let mut A1_U_Q28: opus_int32 = 0;
    let mut A1_L_Q28: opus_int32 = 0;
    let mut out32_Q14: [opus_int32; 2] = [0; 2];
    A0_L_Q28 = -*A_Q28.offset(0 as libc::c_int as isize) & 0x3fff as libc::c_int;
    A0_U_Q28 = -*A_Q28.offset(0 as libc::c_int as isize) >> 14 as libc::c_int;
    A1_L_Q28 = -*A_Q28.offset(1 as libc::c_int as isize) & 0x3fff as libc::c_int;
    A1_U_Q28 = -*A_Q28.offset(1 as libc::c_int as isize) >> 14 as libc::c_int;
    k = 0 as libc::c_int;
    while k < len {
        out32_Q14[0 as libc::c_int as usize] = (((*S.offset(0 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(0 as libc::c_int as isize) as libc::c_long
                * *in_0.offset((2 as libc::c_int * k + 0 as libc::c_int) as isize) as opus_int64
                >> 16 as libc::c_int))
            as opus_int32 as opus_uint32)
            << 2 as libc::c_int) as opus_int32;
        out32_Q14[1 as libc::c_int as usize] = (((*S.offset(2 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(0 as libc::c_int as isize) as libc::c_long
                * *in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) as opus_int64
                >> 16 as libc::c_int))
            as opus_int32 as opus_uint32)
            << 2 as libc::c_int) as opus_int32;
        *S.offset(0 as libc::c_int as isize) = *S.offset(1 as libc::c_int as isize)
            + (if 14 as libc::c_int == 1 as libc::c_int {
                ((out32_Q14[0 as libc::c_int as usize] as libc::c_long
                    * A0_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    >> 1 as libc::c_int)
                    + ((out32_Q14[0 as libc::c_int as usize] as libc::c_long
                        * A0_L_Q28 as opus_int16 as opus_int64
                        >> 16 as libc::c_int) as opus_int32
                        & 1 as libc::c_int)
            } else {
                ((out32_Q14[0 as libc::c_int as usize] as libc::c_long
                    * A0_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    >> 14 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
        *S.offset(2 as libc::c_int as isize) = *S.offset(3 as libc::c_int as isize)
            + (if 14 as libc::c_int == 1 as libc::c_int {
                ((out32_Q14[1 as libc::c_int as usize] as libc::c_long
                    * A0_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    >> 1 as libc::c_int)
                    + ((out32_Q14[1 as libc::c_int as usize] as libc::c_long
                        * A0_L_Q28 as opus_int16 as opus_int64
                        >> 16 as libc::c_int) as opus_int32
                        & 1 as libc::c_int)
            } else {
                ((out32_Q14[1 as libc::c_int as usize] as libc::c_long
                    * A0_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    >> 14 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int
                    >> 1 as libc::c_int
            });
        *S.offset(0 as libc::c_int as isize) = (*S.offset(0 as libc::c_int as isize)
            as libc::c_long
            + (out32_Q14[0 as libc::c_int as usize] as libc::c_long
                * A0_U_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(2 as libc::c_int as isize) = (*S.offset(2 as libc::c_int as isize)
            as libc::c_long
            + (out32_Q14[1 as libc::c_int as usize] as libc::c_long
                * A0_U_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(0 as libc::c_int as isize) = (*S.offset(0 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(1 as libc::c_int as isize) as libc::c_long
                * *in_0.offset((2 as libc::c_int * k + 0 as libc::c_int) as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(2 as libc::c_int as isize) = (*S.offset(2 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(1 as libc::c_int as isize) as libc::c_long
                * *in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(1 as libc::c_int as isize) = if 14 as libc::c_int == 1 as libc::c_int {
            ((out32_Q14[0 as libc::c_int as usize] as libc::c_long
                * A1_L_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32
                >> 1 as libc::c_int)
                + ((out32_Q14[0 as libc::c_int as usize] as libc::c_long
                    * A1_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    & 1 as libc::c_int)
        } else {
            ((out32_Q14[0 as libc::c_int as usize] as libc::c_long
                * A1_L_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32
                >> 14 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *S.offset(3 as libc::c_int as isize) = if 14 as libc::c_int == 1 as libc::c_int {
            ((out32_Q14[1 as libc::c_int as usize] as libc::c_long
                * A1_L_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32
                >> 1 as libc::c_int)
                + ((out32_Q14[1 as libc::c_int as usize] as libc::c_long
                    * A1_L_Q28 as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32
                    & 1 as libc::c_int)
        } else {
            ((out32_Q14[1 as libc::c_int as usize] as libc::c_long
                * A1_L_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int) as opus_int32
                >> 14 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *S.offset(1 as libc::c_int as isize) = (*S.offset(1 as libc::c_int as isize)
            as libc::c_long
            + (out32_Q14[0 as libc::c_int as usize] as libc::c_long
                * A1_U_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(3 as libc::c_int as isize) = (*S.offset(3 as libc::c_int as isize)
            as libc::c_long
            + (out32_Q14[1 as libc::c_int as usize] as libc::c_long
                * A1_U_Q28 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(1 as libc::c_int as isize) = (*S.offset(1 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(2 as libc::c_int as isize) as libc::c_long
                * *in_0.offset((2 as libc::c_int * k + 0 as libc::c_int) as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(3 as libc::c_int as isize) = (*S.offset(3 as libc::c_int as isize)
            as libc::c_long
            + (*B_Q28.offset(2 as libc::c_int as isize) as libc::c_long
                * *in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *out.offset((2 as libc::c_int * k + 0 as libc::c_int) as isize) = (if out32_Q14
            [0 as libc::c_int as usize]
            + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int
            >> 14 as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (out32_Q14[0 as libc::c_int as usize] + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int
            >> 14 as libc::c_int)
            < 0x8000 as libc::c_int as opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else {
            out32_Q14[0 as libc::c_int as usize] + ((1 as libc::c_int) << 14 as libc::c_int)
                - 1 as libc::c_int
                >> 14 as libc::c_int
        }) as opus_int16;
        *out.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) = (if out32_Q14
            [1 as libc::c_int as usize]
            + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int
            >> 14 as libc::c_int
            > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (out32_Q14[1 as libc::c_int as usize] + ((1 as libc::c_int) << 14 as libc::c_int)
            - 1 as libc::c_int
            >> 14 as libc::c_int)
            < 0x8000 as libc::c_int as opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else {
            out32_Q14[1 as libc::c_int as usize] + ((1 as libc::c_int) << 14 as libc::c_int)
                - 1 as libc::c_int
                >> 14 as libc::c_int
        }) as opus_int16;
        k += 1;
    }
}
