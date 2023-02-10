use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:36"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:37"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "71:1"]
    pub unsafe fn celt_fatal(
        str: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    ) -> ! {
        let str = std::ffi::CStr::from_ptr(str);
        let file = std::ffi::CStr::from_ptr(file);
        panic!(
            "Fatal (internal) error in {}, line {}: {}",
            file.to_str().unwrap(),
            line,
            str.to_str().unwrap()
        );
    }
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:40"]
pub mod modes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct OpusCustomMode {
        pub Fs: i32,
        pub overlap: libc::c_int,
        pub nbEBands: libc::c_int,
        pub effEBands: libc::c_int,
        pub preemph: [opus_val16; 4],
        pub eBands: *const i16,
        pub maxLM: libc::c_int,
        pub nbShortMdcts: libc::c_int,
        pub shortMdctSize: libc::c_int,
        pub nbAllocVectors: libc::c_int,
        pub allocVectors: *const libc::c_uchar,
        pub logN: *const i16,
        pub window: *const opus_val16,
        pub mdct: mdct_lookup,
        pub cache: PulseCache,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct PulseCache {
        pub size: libc::c_int,
        pub index: *const i16,
        pub bits: *const libc::c_uchar,
        pub caps: *const libc::c_uchar,
    }
    use super::arch_h::opus_val16;
    use crate::celt::mdct::mdct_lookup;
}
pub use self::arch_h::{celt_fatal, opus_val16, opus_val32};
pub use self::modes_h::{OpusCustomMode, PulseCache};
pub use self::stddef_h::size_t;

use crate::externs::memmove;

#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn resampling_factor(rate: i32) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    match rate {
        48000 => {
            ret = 1 as libc::c_int;
        }
        24000 => {
            ret = 2 as libc::c_int;
        }
        16000 => {
            ret = 3 as libc::c_int;
        }
        12000 => {
            ret = 4 as libc::c_int;
        }
        8000 => {
            ret = 6 as libc::c_int;
        }
        _ => {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"celt/celt.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_int,
                );
            }
            ret = 0 as libc::c_int;
        }
    }
    return ret;
}
#[c2rust::src_loc = "160:1"]
unsafe extern "C" fn comb_filter_const_c(
    y: *mut opus_val32,
    x: *mut opus_val32,
    T: libc::c_int,
    N: libc::c_int,
    g10: opus_val16,
    g11: opus_val16,
    g12: opus_val16,
) {
    let mut x0: opus_val32 = 0.;
    let mut x1: opus_val32 = 0.;
    let mut x2: opus_val32 = 0.;
    let mut x3: opus_val32 = 0.;
    let mut x4: opus_val32 = 0.;
    let mut i: libc::c_int = 0;
    x4 = *x.offset((-T - 2 as libc::c_int) as isize);
    x3 = *x.offset((-T - 1 as libc::c_int) as isize);
    x2 = *x.offset(-T as isize);
    x1 = *x.offset((-T + 1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < N {
        x0 = *x.offset((i - T + 2 as libc::c_int) as isize);
        *y.offset(i as isize) =
            *x.offset(i as isize) + g10 * x2 + g11 * (x1 + x3) + g12 * (x0 + x4);
        *y.offset(i as isize) = *y.offset(i as isize);
        x4 = x3;
        x3 = x2;
        x2 = x1;
        x1 = x0;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "190:1"]
pub unsafe extern "C" fn comb_filter(
    y: *mut opus_val32,
    x: *mut opus_val32,
    mut T0: libc::c_int,
    mut T1: libc::c_int,
    N: libc::c_int,
    g0: opus_val16,
    g1: opus_val16,
    tapset0: libc::c_int,
    tapset1: libc::c_int,
    window: *const opus_val16,
    mut overlap: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut g00: opus_val16 = 0.;
    let mut g01: opus_val16 = 0.;
    let mut g02: opus_val16 = 0.;
    let mut g10: opus_val16 = 0.;
    let mut g11: opus_val16 = 0.;
    let mut g12: opus_val16 = 0.;
    let mut x0: opus_val32 = 0.;
    let mut x1: opus_val32 = 0.;
    let mut x2: opus_val32 = 0.;
    let mut x3: opus_val32 = 0.;
    let mut x4: opus_val32 = 0.;
    static mut gains: [[opus_val16; 3]; 3] = [
        [0.3066406250f32, 0.2170410156f32, 0.1296386719f32],
        [0.4638671875f32, 0.2680664062f32, 0.0f32],
        [0.7998046875f32, 0.1000976562f32, 0.0f32],
    ];
    if g0 == 0 as libc::c_int as libc::c_float && g1 == 0 as libc::c_int as libc::c_float {
        if x != y {
            memmove(
                y as *mut libc::c_void,
                x as *const libc::c_void,
                (N as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long * y.offset_from(x) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
        }
        return;
    }
    T0 = if T0 > 15 as libc::c_int {
        T0
    } else {
        15 as libc::c_int
    };
    T1 = if T1 > 15 as libc::c_int {
        T1
    } else {
        15 as libc::c_int
    };
    g00 = g0 * gains[tapset0 as usize][0 as libc::c_int as usize];
    g01 = g0 * gains[tapset0 as usize][1 as libc::c_int as usize];
    g02 = g0 * gains[tapset0 as usize][2 as libc::c_int as usize];
    g10 = g1 * gains[tapset1 as usize][0 as libc::c_int as usize];
    g11 = g1 * gains[tapset1 as usize][1 as libc::c_int as usize];
    g12 = g1 * gains[tapset1 as usize][2 as libc::c_int as usize];
    x1 = *x.offset((-T1 + 1 as libc::c_int) as isize);
    x2 = *x.offset(-T1 as isize);
    x3 = *x.offset((-T1 - 1 as libc::c_int) as isize);
    x4 = *x.offset((-T1 - 2 as libc::c_int) as isize);
    if g0 == g1 && T0 == T1 && tapset0 == tapset1 {
        overlap = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < overlap {
        let mut f: opus_val16 = 0.;
        x0 = *x.offset((i - T1 + 2 as libc::c_int) as isize);
        f = *window.offset(i as isize) * *window.offset(i as isize);
        *y.offset(i as isize) = *x.offset(i as isize)
            + (1.0f32 - f) * g00 * *x.offset((i - T0) as isize)
            + (1.0f32 - f)
                * g01
                * (*x.offset((i - T0 + 1 as libc::c_int) as isize)
                    + *x.offset((i - T0 - 1 as libc::c_int) as isize))
            + (1.0f32 - f)
                * g02
                * (*x.offset((i - T0 + 2 as libc::c_int) as isize)
                    + *x.offset((i - T0 - 2 as libc::c_int) as isize))
            + f * g10 * x2
            + f * g11 * (x1 + x3)
            + f * g12 * (x0 + x4);
        *y.offset(i as isize) = *y.offset(i as isize);
        x4 = x3;
        x3 = x2;
        x2 = x1;
        x1 = x0;
        i += 1;
    }
    if g1 == 0 as libc::c_int as libc::c_float {
        if x != y {
            memmove(
                y.offset(overlap as isize) as *mut libc::c_void,
                x.offset(overlap as isize) as *const libc::c_void,
                ((N - overlap) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * y.offset(overlap as isize)
                                .offset_from(x.offset(overlap as isize))
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
        }
        return;
    }
    comb_filter_const_c(
        y.offset(i as isize),
        x.offset(i as isize),
        T1,
        N - i,
        g10,
        g11,
        g12,
    );
}
#[no_mangle]
#[c2rust::src_loc = "263:19"]
pub static mut tf_select_table: [[libc::c_schar; 8]; 4] = [
    [
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "272:1"]
pub unsafe extern "C" fn init_caps(
    m: *const OpusCustomMode,
    cap: *mut libc::c_int,
    LM: libc::c_int,
    C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*m).nbEBands {
        let mut N: libc::c_int = 0;
        N = (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(i as isize) as libc::c_int)
            << LM;
        *cap.offset(i as isize) = (*((*m).cache.caps)
            .offset(((*m).nbEBands * (2 as libc::c_int * LM + C - 1 as libc::c_int) + i) as isize)
            as libc::c_int
            + 64 as libc::c_int)
            * C
            * N
            >> 2 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "285:1"]
pub unsafe extern "C" fn opus_strerror(error: libc::c_int) -> *const libc::c_char {
    static mut error_strings: [*const libc::c_char; 8] = [
        b"success\0" as *const u8 as *const libc::c_char,
        b"invalid argument\0" as *const u8 as *const libc::c_char,
        b"buffer too small\0" as *const u8 as *const libc::c_char,
        b"internal error\0" as *const u8 as *const libc::c_char,
        b"corrupted stream\0" as *const u8 as *const libc::c_char,
        b"request not implemented\0" as *const u8 as *const libc::c_char,
        b"invalid state\0" as *const u8 as *const libc::c_char,
        b"memory allocation failed\0" as *const u8 as *const libc::c_char,
    ];
    if error > 0 as libc::c_int || error < -(7 as libc::c_int) {
        return b"unknown error\0" as *const u8 as *const libc::c_char;
    } else {
        return error_strings[-error as usize];
    };
}
#[no_mangle]
#[c2rust::src_loc = "303:1"]
pub unsafe extern "C" fn opus_get_version_string() -> *const libc::c_char {
    return b"libopus 1.3.1\0" as *const u8 as *const libc::c_char;
}
