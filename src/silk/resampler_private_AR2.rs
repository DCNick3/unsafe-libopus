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
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_resampler_private_AR2(
    mut S: *mut opus_int32,
    mut out_Q8: *mut opus_int32,
    mut in_0: *const opus_int16,
    mut A_Q14: *const opus_int16,
    mut len: opus_int32,
) {
    let mut k: opus_int32 = 0;
    let mut out32: opus_int32 = 0;
    k = 0 as libc::c_int;
    while k < len {
        out32 = *S.offset(0 as libc::c_int as isize)
            + ((*in_0.offset(k as isize) as opus_int32 as opus_uint32) << 8 as libc::c_int)
                as opus_int32;
        *out_Q8.offset(k as isize) = out32;
        out32 = ((out32 as opus_uint32) << 2 as libc::c_int) as opus_int32;
        *S.offset(0 as libc::c_int as isize) = (*S.offset(1 as libc::c_int as isize)
            as libc::c_long
            + (out32 as libc::c_long * *A_Q14.offset(0 as libc::c_int as isize) as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *S.offset(1 as libc::c_int as isize) = (out32 as libc::c_long
            * *A_Q14.offset(1 as libc::c_int as isize) as opus_int64
            >> 16 as libc::c_int) as opus_int32;
        k += 1;
    }
}
