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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_rom.h:33"]
pub mod resampler_rom_h {
    #[c2rust::src_loc = "45:25"]
    pub static mut silk_resampler_down2_0: i16 = 9872 as libc::c_int as i16;
    #[c2rust::src_loc = "46:25"]
    pub static mut silk_resampler_down2_1: i16 =
        (39809 as libc::c_int - 65536 as libc::c_int) as i16;
}
use self::arch_h::celt_fatal;
pub use self::resampler_rom_h::{silk_resampler_down2_0, silk_resampler_down2_1};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[no_mangle]
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
