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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [opus_int16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: opus_int32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    use super::opus_types_h::{opus_int16, opus_int32};
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
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_rom.h:33"]
pub mod resampler_rom_h {
    #[c2rust::src_loc = "39:9"]
    pub const RESAMPLER_DOWN_ORDER_FIR0: libc::c_int = 18;
    #[c2rust::src_loc = "40:9"]
    pub const RESAMPLER_DOWN_ORDER_FIR1: libc::c_int = 24;
    #[c2rust::src_loc = "41:9"]
    pub const RESAMPLER_DOWN_ORDER_FIR2: libc::c_int = 36;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_private.h:33"]
pub mod resampler_private_h {
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn silk_resampler_private_AR2(
            S: *mut opus_int32,
            out_Q8: *mut opus_int32,
            in_0: *const opus_int16,
            A_Q14: *const opus_int16,
            len: opus_int32,
        );
    }
}
use self::arch_h::celt_fatal;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
use self::resampler_private_h::silk_resampler_private_AR2;
pub use self::resampler_rom_h::{
    RESAMPLER_DOWN_ORDER_FIR0, RESAMPLER_DOWN_ORDER_FIR1, RESAMPLER_DOWN_ORDER_FIR2,
};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
use self::string_h::memcpy;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[inline]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn silk_resampler_private_down_FIR_INTERPOL(
    mut out: *mut opus_int16,
    buf: *mut opus_int32,
    FIR_Coefs: *const opus_int16,
    FIR_Order: libc::c_int,
    FIR_Fracs: libc::c_int,
    max_index_Q16: opus_int32,
    index_increment_Q16: opus_int32,
) -> *mut opus_int16 {
    let mut index_Q16: opus_int32 = 0;
    let mut res_Q6: opus_int32 = 0;
    let mut buf_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut interpol_ind: opus_int32 = 0;
    let mut interpol_ptr: *const opus_int16 = 0 as *const opus_int16;
    match FIR_Order {
        RESAMPLER_DOWN_ORDER_FIR0 => {
            index_Q16 = 0 as libc::c_int;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as libc::c_int) as isize);
                interpol_ind = ((index_Q16 & 0xffff as libc::c_int) as libc::c_long
                    * FIR_Fracs as opus_int16 as opus_int64
                    >> 16 as libc::c_int) as opus_int32;
                interpol_ptr = &*FIR_Coefs
                    .offset((RESAMPLER_DOWN_ORDER_FIR0 / 2 as libc::c_int * interpol_ind) as isize)
                    as *const opus_int16;
                res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                    * *interpol_ptr.offset(0 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(1 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(2 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(2 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(3 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(3 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(4 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(4 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(5 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(5 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(6 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(6 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(7 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(7 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(8 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(8 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                interpol_ptr = &*FIR_Coefs.offset(
                    (RESAMPLER_DOWN_ORDER_FIR0 / 2 as libc::c_int
                        * (FIR_Fracs - 1 as libc::c_int - interpol_ind))
                        as isize,
                ) as *const opus_int16;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(17 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(0 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(16 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(1 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(15 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(2 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(14 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(3 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(13 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(4 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(12 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(5 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(11 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(6 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(10 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(7 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(9 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(8 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                let fresh0 = out;
                out = out.offset(1);
                *fresh0 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as opus_int16;
                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR1 => {
            index_Q16 = 0 as libc::c_int;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as libc::c_int) as isize);
                res_Q6 = ((*buf_ptr.offset(0 as libc::c_int as isize)
                    + *buf_ptr.offset(23 as libc::c_int as isize))
                    as libc::c_long
                    * *FIR_Coefs.offset(0 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(1 as libc::c_int as isize)
                        + *buf_ptr.offset(22 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(1 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(2 as libc::c_int as isize)
                        + *buf_ptr.offset(21 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(2 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(3 as libc::c_int as isize)
                        + *buf_ptr.offset(20 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(3 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(4 as libc::c_int as isize)
                        + *buf_ptr.offset(19 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(4 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(5 as libc::c_int as isize)
                        + *buf_ptr.offset(18 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(5 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(6 as libc::c_int as isize)
                        + *buf_ptr.offset(17 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(6 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(7 as libc::c_int as isize)
                        + *buf_ptr.offset(16 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(7 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(8 as libc::c_int as isize)
                        + *buf_ptr.offset(15 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(8 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(9 as libc::c_int as isize)
                        + *buf_ptr.offset(14 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(9 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(10 as libc::c_int as isize)
                        + *buf_ptr.offset(13 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(10 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(11 as libc::c_int as isize)
                        + *buf_ptr.offset(12 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(11 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                let fresh1 = out;
                out = out.offset(1);
                *fresh1 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as opus_int16;
                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR2 => {
            index_Q16 = 0 as libc::c_int;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as libc::c_int) as isize);
                res_Q6 = ((*buf_ptr.offset(0 as libc::c_int as isize)
                    + *buf_ptr.offset(35 as libc::c_int as isize))
                    as libc::c_long
                    * *FIR_Coefs.offset(0 as libc::c_int as isize) as opus_int64
                    >> 16 as libc::c_int) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(1 as libc::c_int as isize)
                        + *buf_ptr.offset(34 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(1 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(2 as libc::c_int as isize)
                        + *buf_ptr.offset(33 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(2 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(3 as libc::c_int as isize)
                        + *buf_ptr.offset(32 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(3 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(4 as libc::c_int as isize)
                        + *buf_ptr.offset(31 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(4 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(5 as libc::c_int as isize)
                        + *buf_ptr.offset(30 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(5 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(6 as libc::c_int as isize)
                        + *buf_ptr.offset(29 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(6 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(7 as libc::c_int as isize)
                        + *buf_ptr.offset(28 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(7 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(8 as libc::c_int as isize)
                        + *buf_ptr.offset(27 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(8 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(9 as libc::c_int as isize)
                        + *buf_ptr.offset(26 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(9 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(10 as libc::c_int as isize)
                        + *buf_ptr.offset(25 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(10 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(11 as libc::c_int as isize)
                        + *buf_ptr.offset(24 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(11 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(12 as libc::c_int as isize)
                        + *buf_ptr.offset(23 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(12 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(13 as libc::c_int as isize)
                        + *buf_ptr.offset(22 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(13 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(14 as libc::c_int as isize)
                        + *buf_ptr.offset(21 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(14 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(15 as libc::c_int as isize)
                        + *buf_ptr.offset(20 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(15 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(16 as libc::c_int as isize)
                        + *buf_ptr.offset(19 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(16 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(17 as libc::c_int as isize)
                        + *buf_ptr.offset(18 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(17 as libc::c_int as isize) as opus_int64
                        >> 16 as libc::c_int)) as opus_int32;
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as opus_int16;
                index_Q16 += index_increment_Q16;
            }
        }
        _ => {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/resampler_private_down_FIR.c\0" as *const u8 as *const libc::c_char,
                    139 as libc::c_int,
                );
            }
        }
    }
    return out;
}
#[no_mangle]
#[c2rust::src_loc = "145:1"]
pub unsafe extern "C" fn silk_resampler_private_down_FIR(
    SS: *mut libc::c_void,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut inLen: opus_int32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: opus_int32 = 0;
    let mut max_index_Q16: opus_int32 = 0;
    let mut index_increment_Q16: opus_int32 = 0;
    let mut FIR_Coefs: *const opus_int16 = 0 as *const opus_int16;
    let vla = ((*S).batchSize + (*S).FIR_Order) as usize;
    let mut buf: Vec<opus_int32> = ::std::vec::from_elem(0, vla);
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*S).sFIR.i32_0).as_mut_ptr() as *const libc::c_void,
        ((*S).FIR_Order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
    FIR_Coefs = &*((*S).Coefs).offset(2 as libc::c_int as isize) as *const opus_int16;
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize {
            inLen
        } else {
            (*S).batchSize
        };
        silk_resampler_private_AR2(
            ((*S).sIIR).as_mut_ptr(),
            &mut *buf.as_mut_ptr().offset((*S).FIR_Order as isize),
            in_0,
            (*S).Coefs,
            nSamplesIn,
        );
        max_index_Q16 = ((nSamplesIn as opus_uint32) << 16 as libc::c_int) as opus_int32;
        out = silk_resampler_private_down_FIR_INTERPOL(
            out,
            buf.as_mut_ptr(),
            FIR_Coefs,
            (*S).FIR_Order,
            (*S).FIR_Fracs,
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 1 as libc::c_int) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut opus_int32
                as *const libc::c_void,
            ((*S).FIR_Order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*S).sFIR.i32_0).as_mut_ptr() as *mut libc::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut opus_int32
            as *const libc::c_void,
        ((*S).FIR_Order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
}
