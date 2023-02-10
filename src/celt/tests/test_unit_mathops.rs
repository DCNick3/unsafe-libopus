use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:37"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:37"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:37"]
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
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:37"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:39"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:39"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
}
#[c2rust::header_src = "/usr/include/stdio.h:37"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:38"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "95:17"]
        pub fn exp(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "104:17"]
        pub fn log(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "162:14"]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/bands.h:40"]
pub mod bands_h {
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn bitexact_cos(x: i16) -> i16;
        #[c2rust::src_loc = "40:1"]
        pub fn bitexact_log2tan(isin: libc::c_int, icos: libc::c_int) -> libc::c_int;
    }
}
pub use self::arch_h::{opus_val16, opus_val32};
use self::bands_h::{bitexact_cos, bitexact_log2tan};
use self::mathcalls_h::{exp, fabs, log, sqrt};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
use self::stdio_h::{fprintf, stderr};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__int16_t, __int32_t, __off64_t, __off_t};
pub use self::FILE_h::FILE;
#[no_mangle]
#[c2rust::src_loc = "48:5"]
pub static mut ret: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn testdiv() {
    let mut i: i32 = 0;
    i = 1 as libc::c_int;
    while i <= 327670 as libc::c_int {
        let mut prod: libc::c_double = 0.;
        let mut val: opus_val32 = 0.;
        val = 1.0f32 / i as libc::c_float;
        prod = (val * i as libc::c_float) as libc::c_double;
        if fabs(prod - 1 as libc::c_int as libc::c_double) > 0.00025f64 {
            fprintf(
                stderr,
                b"div failed: 1/%d=%f (product = %f)\n\0" as *const u8 as *const libc::c_char,
                i,
                val as libc::c_double,
                prod,
            );
            ret = 1 as libc::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn testsqrt() {
    let mut i: i32 = 0;
    i = 1 as libc::c_int;
    while i <= 1000000000 as libc::c_int {
        let mut ratio: libc::c_double = 0.;
        let mut val: opus_val16 = 0.;
        val = sqrt(i as libc::c_double) as libc::c_float;
        ratio = val as libc::c_double / sqrt(i as libc::c_double);
        if fabs(ratio - 1 as libc::c_int as libc::c_double) > 0.0005f64
            && fabs(val as libc::c_double - sqrt(i as libc::c_double))
                > 2 as libc::c_int as libc::c_double
        {
            fprintf(
                stderr,
                b"sqrt failed: sqrt(%d)=%f (ratio = %f)\n\0" as *const u8 as *const libc::c_char,
                i,
                val as libc::c_double,
                ratio,
            );
            ret = 1 as libc::c_int;
        }
        i += i >> 10 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn testbitexactcos() {
    let mut i: libc::c_int = 0;
    let mut min_d: i32 = 0;
    let mut max_d: i32 = 0;
    let mut last: i32 = 0;
    let mut chk: i32 = 0;
    max_d = 0 as libc::c_int;
    chk = max_d;
    min_d = 32767 as libc::c_int;
    last = min_d;
    i = 64 as libc::c_int;
    while i <= 16320 as libc::c_int {
        let mut d: i32 = 0;
        let mut q: i32 = bitexact_cos(i as i16) as i32;
        chk ^= q * i;
        d = last - q;
        if d > max_d {
            max_d = d;
        }
        if d < min_d {
            min_d = d;
        }
        last = q;
        i += 1;
    }
    if chk != 89408644 as libc::c_int
        || max_d != 5 as libc::c_int
        || min_d != 0 as libc::c_int
        || bitexact_cos(64 as libc::c_int as i16) as libc::c_int != 32767 as libc::c_int
        || bitexact_cos(16320 as libc::c_int as i16) as libc::c_int != 200 as libc::c_int
        || bitexact_cos(8192 as libc::c_int as i16) as libc::c_int != 23171 as libc::c_int
    {
        fprintf(
            stderr,
            b"bitexact_cos failed\n\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
    }
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn testbitexactlog2tan() {
    let mut i: libc::c_int = 0;
    let mut fail: libc::c_int = 0;
    let mut min_d: i32 = 0;
    let mut max_d: i32 = 0;
    let mut last: i32 = 0;
    let mut chk: i32 = 0;
    max_d = 0 as libc::c_int;
    chk = max_d;
    fail = chk;
    min_d = 15059 as libc::c_int;
    last = min_d;
    i = 64 as libc::c_int;
    while i < 8193 as libc::c_int {
        let mut d: i32 = 0;
        let mut mid: i32 = bitexact_cos(i as i16) as i32;
        let mut side: i32 = bitexact_cos((16384 as libc::c_int - i) as i16) as i32;
        let mut q: i32 = bitexact_log2tan(mid, side);
        chk ^= q * i;
        d = last - q;
        if q != -(1 as libc::c_int) * bitexact_log2tan(side, mid) {
            fail = 1 as libc::c_int;
        }
        if d > max_d {
            max_d = d;
        }
        if d < min_d {
            min_d = d;
        }
        last = q;
        i += 1;
    }
    if chk != 15821257 as libc::c_int
        || max_d != 61 as libc::c_int
        || min_d != -(2 as libc::c_int)
        || fail != 0
        || bitexact_log2tan(32767 as libc::c_int, 200 as libc::c_int) != 15059 as libc::c_int
        || bitexact_log2tan(30274 as libc::c_int, 12540 as libc::c_int) != 2611 as libc::c_int
        || bitexact_log2tan(23171 as libc::c_int, 23171 as libc::c_int) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"bitexact_log2tan failed\n\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
    }
}
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn testlog2() {
    let mut x: libc::c_float = 0.;
    x = 0.001f64 as libc::c_float;
    while (x as libc::c_double) < 1677700.0f64 {
        let mut error: libc::c_float = fabs(
            1.442695040888963387f64 * log(x as libc::c_double)
                - (1.442695040888963387f64 * log(x as libc::c_double)) as libc::c_float
                    as libc::c_double,
        ) as libc::c_float;
        if error as libc::c_double > 0.0009f64 {
            fprintf(
                stderr,
                b"celt_log2 failed: fabs((1.442695040888963387*log(x))-celt_log2(x))>0.001 (x = %f, error = %f)\n\0"
                    as *const u8 as *const libc::c_char,
                x as libc::c_double,
                error as libc::c_double,
            );
            ret = 1 as libc::c_int;
        }
        x = (x as libc::c_double + x as libc::c_double / 8.0f64) as libc::c_float;
    }
}
#[no_mangle]
#[c2rust::src_loc = "157:1"]
pub unsafe extern "C" fn testexp2() {
    let mut x: libc::c_float = 0.;
    x = -11.0f64 as libc::c_float;
    while (x as libc::c_double) < 24.0f64 {
        let mut error: libc::c_float = fabs(
            x as libc::c_double
                - 1.442695040888963387f64
                    * log(
                        exp(0.6931471805599453094f64 * x as libc::c_double) as libc::c_float
                            as libc::c_double,
                    ),
        ) as libc::c_float;
        if error as libc::c_double > 0.0002f64 {
            fprintf(
                stderr,
                b"celt_exp2 failed: fabs(x-(1.442695040888963387*log(celt_exp2(x))))>0.0005 (x = %f, error = %f)\n\0"
                    as *const u8 as *const libc::c_char,
                x as libc::c_double,
                error as libc::c_double,
            );
            ret = 1 as libc::c_int;
        }
        x = (x as libc::c_double + 0.0007f64) as libc::c_float;
    }
}
#[no_mangle]
#[c2rust::src_loc = "171:1"]
pub unsafe extern "C" fn testexp2log2() {
    let mut x: libc::c_float = 0.;
    x = -11.0f64 as libc::c_float;
    while (x as libc::c_double) < 24.0f64 {
        let mut error: libc::c_float = fabs(
            (x - (1.442695040888963387f64
                * log(
                    exp(0.6931471805599453094f64 * x as libc::c_double) as libc::c_float
                        as libc::c_double,
                )) as libc::c_float) as libc::c_double,
        ) as libc::c_float;
        if error as libc::c_double > 0.001f64 {
            fprintf(
                stderr,
                b"celt_log2/celt_exp2 failed: fabs(x-(celt_log2(celt_exp2(x))))>0.001 (x = %f, error = %f)\n\0"
                    as *const u8 as *const libc::c_char,
                x as libc::c_double,
                error as libc::c_double,
            );
            ret = 1 as libc::c_int;
        }
        x = (x as libc::c_double + 0.0007f64) as libc::c_float;
    }
}
#[c2rust::src_loc = "253:1"]
unsafe fn main_0() -> libc::c_int {
    testbitexactcos();
    testbitexactlog2tan();
    testdiv();
    testsqrt();
    testlog2();
    testexp2();
    testexp2log2();
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
