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
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: libc::c_int = 0x7fffffff as libc::c_int;
}
pub use self::types_h::{__int16_t, __int32_t, __uint32_t, __int64_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32, opus_int64};
pub use self::typedef_h::silk_int32_MAX;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_log2lin(inLog_Q7: opus_int32) -> opus_int32 {
    let mut out: opus_int32 = 0;
    let mut frac_Q7: opus_int32 = 0;
    if inLog_Q7 < 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if inLog_Q7 >= 3967 as libc::c_int {
            return silk_int32_MAX;
        }
    }
    out = ((1 as libc::c_int as opus_uint32) << (inLog_Q7 >> 7 as libc::c_int))
        as opus_int32;
    frac_Q7 = inLog_Q7 & 0x7f as libc::c_int;
    if inLog_Q7 < 2048 as libc::c_int {
        out = out
            + (out
                * (frac_Q7 as libc::c_long
                    + ((frac_Q7 as opus_int16 as opus_int32
                        * (128 as libc::c_int - frac_Q7) as opus_int16 as opus_int32)
                        as libc::c_long
                        * -(174 as libc::c_int) as opus_int16 as opus_int64
                        >> 16 as libc::c_int)) as opus_int32 >> 7 as libc::c_int);
    } else {
        out = out
            + (out >> 7 as libc::c_int)
                * (frac_Q7 as libc::c_long
                    + ((frac_Q7 as opus_int16 as opus_int32
                        * (128 as libc::c_int - frac_Q7) as opus_int16 as opus_int32)
                        as libc::c_long
                        * -(174 as libc::c_int) as opus_int16 as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
    }
    return out;
}
