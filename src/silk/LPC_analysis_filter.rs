use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
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
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
pub use self::types_h::{__int16_t, __int32_t, __uint32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
use self::arch_h::celt_fatal;
use self::string_h::memset;
pub use self::typedef_h::{silk_int16_MIN, silk_int16_MAX};
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn silk_LPC_analysis_filter(
    out: *mut opus_int16,
    in_0: *const opus_int16,
    B: *const opus_int16,
    len: opus_int32,
    d: opus_int32,
    _arch: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut out32_Q12: opus_int32 = 0;
    let mut out32: opus_int32 = 0;
    let mut in_ptr: *const opus_int16 = 0 as *const opus_int16;
    if !(d >= 6 as libc::c_int) {
        celt_fatal(
            b"assertion failed: d >= 6\0" as *const u8 as *const libc::c_char,
            b"silk/LPC_analysis_filter.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
        );
    }
    if !(d & 1 as libc::c_int == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: (d & 1) == 0\0" as *const u8 as *const libc::c_char,
            b"silk/LPC_analysis_filter.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
        );
    }
    if !(d <= len) {
        celt_fatal(
            b"assertion failed: d <= len\0" as *const u8 as *const libc::c_char,
            b"silk/LPC_analysis_filter.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
        );
    }
    ix = d;
    while ix < len {
        in_ptr = &*in_0.offset((ix - 1 as libc::c_int) as isize) as *const opus_int16;
        out32_Q12 = *in_ptr.offset(0 as libc::c_int as isize) as opus_int32
            * *B.offset(0 as libc::c_int as isize) as opus_int32;
        out32_Q12 = (out32_Q12 as opus_uint32)
            .wrapping_add(
                (*in_ptr.offset(-(1 as libc::c_int) as isize) as opus_int32
                    * *B.offset(1 as libc::c_int as isize) as opus_int32) as opus_uint32,
            ) as opus_int32;
        out32_Q12 = (out32_Q12 as opus_uint32)
            .wrapping_add(
                (*in_ptr.offset(-(2 as libc::c_int) as isize) as opus_int32
                    * *B.offset(2 as libc::c_int as isize) as opus_int32) as opus_uint32,
            ) as opus_int32;
        out32_Q12 = (out32_Q12 as opus_uint32)
            .wrapping_add(
                (*in_ptr.offset(-(3 as libc::c_int) as isize) as opus_int32
                    * *B.offset(3 as libc::c_int as isize) as opus_int32) as opus_uint32,
            ) as opus_int32;
        out32_Q12 = (out32_Q12 as opus_uint32)
            .wrapping_add(
                (*in_ptr.offset(-(4 as libc::c_int) as isize) as opus_int32
                    * *B.offset(4 as libc::c_int as isize) as opus_int32) as opus_uint32,
            ) as opus_int32;
        out32_Q12 = (out32_Q12 as opus_uint32)
            .wrapping_add(
                (*in_ptr.offset(-(5 as libc::c_int) as isize) as opus_int32
                    * *B.offset(5 as libc::c_int as isize) as opus_int32) as opus_uint32,
            ) as opus_int32;
        j = 6 as libc::c_int;
        while j < d {
            out32_Q12 = (out32_Q12 as opus_uint32)
                .wrapping_add(
                    (*in_ptr.offset(-j as isize) as opus_int32
                        * *B.offset(j as isize) as opus_int32) as opus_uint32,
                ) as opus_int32;
            out32_Q12 = (out32_Q12 as opus_uint32)
                .wrapping_add(
                    (*in_ptr.offset((-j - 1 as libc::c_int) as isize) as opus_int32
                        * *B.offset((j + 1 as libc::c_int) as isize) as opus_int32)
                        as opus_uint32,
                ) as opus_int32;
            j += 2 as libc::c_int;
        }
        out32_Q12 = (((*in_ptr.offset(1 as libc::c_int as isize) as opus_int32
            as opus_uint32) << 12 as libc::c_int) as opus_int32 as opus_uint32)
            .wrapping_sub(out32_Q12 as opus_uint32) as opus_int32;
        out32 = if 12 as libc::c_int == 1 as libc::c_int {
            (out32_Q12 >> 1 as libc::c_int) + (out32_Q12 & 1 as libc::c_int)
        } else {
            (out32_Q12 >> 12 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                >> 1 as libc::c_int
        };
        *out
            .offset(
                ix as isize,
            ) = (if out32 > silk_int16_MAX {
            silk_int16_MAX
        } else if out32 < silk_int16_MIN {
            silk_int16_MIN
        } else {
            out32
        }) as opus_int16;
        ix += 1;
    }
    memset(
        out as *mut libc::c_void,
        0 as libc::c_int,
        (d as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
}
