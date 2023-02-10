use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
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
use self::arch_h::celt_fatal;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::types_h::{__int16_t, __int32_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_interpolate(
    xi: *mut i16,
    x0: *const i16,
    x1: *const i16,
    ifact_Q2: libc::c_int,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if !(ifact_Q2 >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ifact_Q2 >= 0\0" as *const u8 as *const libc::c_char,
            b"silk/interpolate.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
        );
    }
    if !(ifact_Q2 <= 4 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ifact_Q2 <= 4\0" as *const u8 as *const libc::c_char,
            b"silk/interpolate.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < d {
        *xi.offset(i as isize) = (*x0.offset(i as isize) as libc::c_int
            + ((*x1.offset(i as isize) as libc::c_int - *x0.offset(i as isize) as libc::c_int)
                as i16 as i32
                * ifact_Q2 as i16 as i32
                >> 2 as libc::c_int)) as i16;
        i += 1;
    }
}
