use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:36"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:36"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:36"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:36"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:36"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:36"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:37"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[no_mangle]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn celt_fatal(
        mut str: *const libc::c_char,
        mut file: *const libc::c_char,
        mut line: libc::c_int,
    ) -> ! {
        fprintf(
            stderr,
            b"Fatal (internal) error in %s, line %d: %s\n\0" as *const u8
                as *const libc::c_char,
            file,
            line,
            str,
        );
        abort();
    }
    use super::stdio_h::{fprintf, stderr};
    use super::stdlib_h::abort;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/kiss_fft.h:37"]
pub mod kiss_fft_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct kiss_twiddle_cpx {
        pub r: libc::c_float,
        pub i: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct arch_fft_state {
        pub is_supported: libc::c_int,
        pub priv_0: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct kiss_fft_state {
        pub nfft: libc::c_int,
        pub scale: opus_val16,
        pub shift: libc::c_int,
        pub factors: [opus_int16; 16],
        pub bitrev: *const opus_int16,
        pub twiddles: *const kiss_twiddle_cpx,
        pub arch_fft: *mut arch_fft_state,
    }
    use super::arch_h::opus_val16;
    use super::opus_types_h::opus_int16;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mdct.h:37"]
pub mod mdct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct mdct_lookup {
        pub n: libc::c_int,
        pub maxshift: libc::c_int,
        pub kfft: [*const kiss_fft_state; 4],
        pub trig: *const libc::c_float,
    }
    use super::kiss_fft_h::kiss_fft_state;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:40"]
pub mod modes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct OpusCustomMode {
        pub Fs: opus_int32,
        pub overlap: libc::c_int,
        pub nbEBands: libc::c_int,
        pub effEBands: libc::c_int,
        pub preemph: [opus_val16; 4],
        pub eBands: *const opus_int16,
        pub maxLM: libc::c_int,
        pub nbShortMdcts: libc::c_int,
        pub shortMdctSize: libc::c_int,
        pub nbAllocVectors: libc::c_int,
        pub allocVectors: *const libc::c_uchar,
        pub logN: *const opus_int16,
        pub window: *const opus_val16,
        pub mdct: mdct_lookup,
        pub cache: PulseCache,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:9"]
    pub struct PulseCache {
        pub size: libc::c_int,
        pub index: *const opus_int16,
        pub bits: *const libc::c_uchar,
        pub caps: *const libc::c_uchar,
    }
    use super::opus_types_h::{opus_int32, opus_int16};
    use super::arch_h::opus_val16;
    use super::mdct_h::mdct_lookup;
}
#[c2rust::header_src = "/usr/include/stdio.h:36"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:36"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "611:13"]
        pub fn abort() -> !;
    }
}
pub use self::types_h::{__int16_t, __int32_t, __off_t, __off64_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::opus_types_h::{opus_int16, opus_int32};
pub use self::stddef_h::size_t;
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::arch_h::{opus_val16, opus_val32, celt_fatal};
pub use self::kiss_fft_h::{kiss_twiddle_cpx, arch_fft_state, kiss_fft_state};
pub use self::mdct_h::mdct_lookup;
pub use self::modes_h::{OpusCustomMode, PulseCache};
use self::stdio_h::{stderr, fprintf};
use self::string_h::memmove;
use self::stdlib_h::abort;
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn resampling_factor(mut rate: opus_int32) -> libc::c_int {
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
    mut y: *mut opus_val32,
    mut x: *mut opus_val32,
    mut T: libc::c_int,
    mut N: libc::c_int,
    mut g10: opus_val16,
    mut g11: opus_val16,
    mut g12: opus_val16,
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
        *y
            .offset(
                i as isize,
            ) = *x.offset(i as isize) + g10 * x2 + g11 * (x1 + x3) + g12 * (x0 + x4);
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
    mut y: *mut opus_val32,
    mut x: *mut opus_val32,
    mut T0: libc::c_int,
    mut T1: libc::c_int,
    mut N: libc::c_int,
    mut g0: opus_val16,
    mut g1: opus_val16,
    mut tapset0: libc::c_int,
    mut tapset1: libc::c_int,
    mut window: *const opus_val16,
    mut overlap: libc::c_int,
    mut arch: libc::c_int,
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
    if g0 == 0 as libc::c_int as libc::c_float && g1 == 0 as libc::c_int as libc::c_float
    {
        if x != y {
            memmove(
                y as *mut libc::c_void,
                x as *const libc::c_void,
                (N as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val32>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * y.offset_from(x) as libc::c_long) as libc::c_ulong,
                    ),
            );
        }
        return;
    }
    T0 = if T0 > 15 as libc::c_int { T0 } else { 15 as libc::c_int };
    T1 = if T1 > 15 as libc::c_int { T1 } else { 15 as libc::c_int };
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
        *y
            .offset(
                i as isize,
            ) = *x.offset(i as isize) + (1.0f32 - f) * g00 * *x.offset((i - T0) as isize)
            + (1.0f32 - f) * g01
                * (*x.offset((i - T0 + 1 as libc::c_int) as isize)
                    + *x.offset((i - T0 - 1 as libc::c_int) as isize))
            + (1.0f32 - f) * g02
                * (*x.offset((i - T0 + 2 as libc::c_int) as isize)
                    + *x.offset((i - T0 - 2 as libc::c_int) as isize)) + f * g10 * x2
            + f * g11 * (x1 + x3) + f * g12 * (x0 + x4);
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
                            * y
                                .offset(overlap as isize)
                                .offset_from(x.offset(overlap as isize)) as libc::c_long)
                            as libc::c_ulong,
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
    mut m: *const OpusCustomMode,
    mut cap: *mut libc::c_int,
    mut LM: libc::c_int,
    mut C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*m).nbEBands {
        let mut N: libc::c_int = 0;
        N = (*((*m).eBands).offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(i as isize) as libc::c_int) << LM;
        *cap
            .offset(
                i as isize,
            ) = (*((*m).cache.caps)
            .offset(
                ((*m).nbEBands * (2 as libc::c_int * LM + C - 1 as libc::c_int) + i)
                    as isize,
            ) as libc::c_int + 64 as libc::c_int) * C * N >> 2 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "285:1"]
pub unsafe extern "C" fn opus_strerror(mut error: libc::c_int) -> *const libc::c_char {
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
        return b"unknown error\0" as *const u8 as *const libc::c_char
    } else {
        return error_strings[-error as usize]
    };
}
#[no_mangle]
#[c2rust::src_loc = "303:1"]
pub unsafe extern "C" fn opus_get_version_string() -> *const libc::c_char {
    return b"libopus 1.3.1\0" as *const u8 as *const libc::c_char;
}
