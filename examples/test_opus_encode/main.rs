#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![feature(stdsimd)]
#![register_tool(c2rust)]

use ::libc;
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:32"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:33"]
pub mod time_t_h {
    #[c2rust::src_loc = "10:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "144:14"]
        pub static mut stdout: *mut FILE;
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        strtol(
            __nptr,
            std::ptr::null_mut::<libc::c_void>() as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int
    }
    extern "C" {
        #[c2rust::src_loc = "177:17"]
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "611:13"]
        pub fn abort() -> !;
        #[c2rust::src_loc = "654:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:37"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:38"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:40"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[c2rust::src_loc = "650:1"]
        pub fn getpid() -> __pid_t;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/tests/test_opus_common.h:48"]
pub mod test_opus_common_h {
    #[inline]
    #[c2rust::src_loc = "28:1"]
    pub unsafe extern "C" fn deb2_impl(
        mut _t: *mut libc::c_uchar,
        mut _p: *mut *mut libc::c_uchar,
        mut _k: libc::c_int,
        mut _x: libc::c_int,
        mut _y: libc::c_int,
    ) {
        let mut i: libc::c_int = 0;
        if _x > 2 as libc::c_int {
            if _y < 3 as libc::c_int {
                i = 0 as libc::c_int;
                while i < _y {
                    *_p = (*_p).offset(-1);
                    **_p = *_t.offset((i + 1 as libc::c_int) as isize);
                    i += 1;
                }
            }
        } else {
            *_t.offset(_x as isize) = *_t.offset((_x - _y) as isize);
            deb2_impl(_t, _p, _k, _x + 1 as libc::c_int, _y);
            i = *_t.offset((_x - _y) as isize) as libc::c_int + 1 as libc::c_int;
            while i < _k {
                *_t.offset(_x as isize) = i as libc::c_uchar;
                deb2_impl(_t, _p, _k, _x + 1 as libc::c_int, _x);
                i += 1;
            }
        };
    }
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn debruijn2(mut _k: libc::c_int, mut _res: *mut libc::c_uchar) {
        let mut p: *mut libc::c_uchar = std::ptr::null_mut::<libc::c_uchar>();
        let mut t: *mut libc::c_uchar = std::ptr::null_mut::<libc::c_uchar>();
        t = malloc(
            (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_mul(_k as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_uchar;
        memset(
            t as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_mul(_k as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
        p = &mut *_res.offset((_k * _k) as isize) as *mut libc::c_uchar;
        deb2_impl(t, &mut p, _k, 1 as libc::c_int, 1 as libc::c_int);
        free(t as *mut libc::c_void);
    }
    #[c2rust::src_loc = "56:20"]
    pub static mut Rz: u32 = 0;
    #[c2rust::src_loc = "56:24"]
    pub static mut Rw: u32 = 0;
    #[inline]
    #[c2rust::src_loc = "57:1"]
    pub unsafe extern "C" fn fast_rand() -> u32 {
        Rz = (36969 as libc::c_int as libc::c_uint)
            .wrapping_mul(Rz & 65535 as libc::c_int as libc::c_uint)
            .wrapping_add(Rz >> 16 as libc::c_int);
        Rw = (18000 as libc::c_int as libc::c_uint)
            .wrapping_mul(Rw & 65535 as libc::c_int as libc::c_uint)
            .wrapping_add(Rw >> 16 as libc::c_int);
        (Rz << 16 as libc::c_int).wrapping_add(Rw)
    }
    #[c2rust::src_loc = "63:20"]
    pub static mut iseed: u32 = 0;
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn _test_failed(
        mut file: *const libc::c_char,
        mut line: libc::c_int,
    ) -> ! {
        fprintf(
            stderr,
            b"\n ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b" ***         A fatal error was detected.         ***\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b" ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Please report this failure and include\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"'make check SEED=%u fails %s at line %d for %s'\n\0" as *const u8
                as *const libc::c_char,
            iseed,
            file,
            line,
            opus_get_version_string(),
        );
        fprintf(
            stderr,
            b"and any relevant details about your system.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }

    use super::stdio_h::{fprintf, stderr};
    use super::stdlib_h::{abort, free, malloc};
    use super::string_h::memset;
    use libopus_unsafe::opus_get_version_string;
}
use self::stdio_h::{fprintf, stderr, stdout};
pub use self::stdlib_h::{atoi, free, getenv, malloc, strtol};
use self::string_h::{memcpy, memset, strcmp};
pub use self::test_opus_common_h::{debruijn2, Rw, Rz, _test_failed, fast_rand, iseed};
use self::time_h::time;
pub use self::time_t_h::time_t;
use self::unistd_h::getpid;
use libopus_unsafe::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy,
    opus_decoder_get_size, opus_encode, opus_encoder_create, opus_encoder_ctl,
    opus_encoder_destroy, opus_encoder_get_size, opus_get_version_string, opus_multistream_decode,
    opus_multistream_decoder_create, opus_multistream_decoder_ctl,
    opus_multistream_decoder_destroy, opus_multistream_encode, opus_multistream_encoder_create,
    opus_multistream_encoder_ctl, opus_multistream_encoder_destroy, opus_multistream_packet_pad,
    opus_multistream_packet_unpad, opus_packet_pad, opus_packet_parse, opus_packet_unpad,
    OpusDecoder, OpusEncoder, OpusMSDecoder, OpusMSEncoder,
};

mod opus_encode_regressions;
use opus_encode_regressions::regression_test;

#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn generate_music(mut buf: *mut libc::c_short, mut len: i32) {
    let mut a1: i32 = 0;
    let mut b1: i32 = 0;
    let mut a2: i32 = 0;
    let mut b2: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut d1: i32 = 0;
    let mut d2: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    b2 = 0 as libc::c_int;
    a2 = b2;
    b1 = a2;
    a1 = b1;
    d2 = 0 as libc::c_int;
    d1 = d2;
    c2 = d1;
    c1 = c2;
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 2880 as libc::c_int {
        let fresh0 = &mut (*buf.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize));
        *fresh0 = 0 as libc::c_int as libc::c_short;
        *buf.offset((i * 2 as libc::c_int) as isize) = *fresh0;
        i += 1;
    }
    i = 2880 as libc::c_int;
    while i < len {
        let mut r: u32 = 0;
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        v2 = (((j
            * (j >> 12 as libc::c_int
                ^ (j >> 10 as libc::c_int | j >> 12 as libc::c_int)
                    & 26 as libc::c_int
                    & j >> 7 as libc::c_int))
            & 128 as libc::c_int)
            + 128 as libc::c_int)
            << 15 as libc::c_int;
        v1 = v2;
        r = fast_rand();
        v1 = (v1 as libc::c_uint).wrapping_add(r & 65535 as libc::c_int as libc::c_uint) as i32
            as i32;
        v1 = (v1 as libc::c_uint).wrapping_sub(r >> 16 as libc::c_int) as i32 as i32;
        r = fast_rand();
        v2 = (v2 as libc::c_uint).wrapping_add(r & 65535 as libc::c_int as libc::c_uint) as i32
            as i32;
        v2 = (v2 as libc::c_uint).wrapping_sub(r >> 16 as libc::c_int) as i32 as i32;
        b1 = v1 - a1 + ((b1 * 61 as libc::c_int + 32 as libc::c_int) >> 6 as libc::c_int);
        a1 = v1;
        b2 = v2 - a2 + ((b2 * 61 as libc::c_int + 32 as libc::c_int) >> 6 as libc::c_int);
        a2 = v2;
        c1 = (30 as libc::c_int * (c1 + b1 + d1) + 32 as libc::c_int) >> 6 as libc::c_int;
        d1 = b1;
        c2 = (30 as libc::c_int * (c2 + b2 + d2) + 32 as libc::c_int) >> 6 as libc::c_int;
        d2 = b2;
        v1 = (c1 + 128 as libc::c_int) >> 8 as libc::c_int;
        v2 = (c2 + 128 as libc::c_int) >> 8 as libc::c_int;
        *buf.offset((i * 2 as libc::c_int) as isize) = (if v1 > 32767 as libc::c_int {
            32767 as libc::c_int
        } else if v1 < -(32768 as libc::c_int) {
            -(32768 as libc::c_int)
        } else {
            v1
        }) as libc::c_short;
        *buf.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize) =
            (if v2 > 32767 as libc::c_int {
                32767 as libc::c_int
            } else if v2 < -(32768 as libc::c_int) {
                -(32768 as libc::c_int)
            } else {
                v2
            }) as libc::c_short;
        if i % 6 as libc::c_int == 0 as libc::c_int {
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn get_frame_size_enum(
    mut frame_size: libc::c_int,
    mut sampling_rate: libc::c_int,
) -> libc::c_int {
    let mut frame_size_enum: libc::c_int = 0;
    if frame_size == sampling_rate / 400 as libc::c_int {
        frame_size_enum = 5001 as libc::c_int;
    } else if frame_size == sampling_rate / 200 as libc::c_int {
        frame_size_enum = 5002 as libc::c_int;
    } else if frame_size == sampling_rate / 100 as libc::c_int {
        frame_size_enum = 5003 as libc::c_int;
    } else if frame_size == sampling_rate / 50 as libc::c_int {
        frame_size_enum = 5004 as libc::c_int;
    } else if frame_size == sampling_rate / 25 as libc::c_int {
        frame_size_enum = 5005 as libc::c_int;
    } else if frame_size == 3 as libc::c_int * sampling_rate / 50 as libc::c_int {
        frame_size_enum = 5006 as libc::c_int;
    } else if frame_size == 4 as libc::c_int * sampling_rate / 50 as libc::c_int {
        frame_size_enum = 5007 as libc::c_int;
    } else if frame_size == 5 as libc::c_int * sampling_rate / 50 as libc::c_int {
        frame_size_enum = 5008 as libc::c_int;
    } else if frame_size == 6 as libc::c_int * sampling_rate / 50 as libc::c_int {
        frame_size_enum = 5009 as libc::c_int;
    } else {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
        );
    }
    frame_size_enum
}
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn test_encode(
    mut enc: *mut OpusEncoder,
    mut channels: libc::c_int,
    mut frame_size: libc::c_int,
    mut dec: *mut OpusDecoder,
) -> libc::c_int {
    let mut samp_count: libc::c_int = 0 as libc::c_int;
    let mut inbuf: *mut i16 = std::ptr::null_mut::<i16>();
    let mut packet: [libc::c_uchar; 1757] = [0; 1757];
    let mut len: libc::c_int = 0;
    let mut outbuf: *mut i16 = std::ptr::null_mut::<i16>();
    let mut out_samples: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    inbuf = malloc(
        (::core::mem::size_of::<i16>() as libc::c_ulong).wrapping_mul(
            (48000 as libc::c_int * 30 as libc::c_int / 3 as libc::c_int) as libc::c_ulong,
        ),
    ) as *mut i16;
    generate_music(
        inbuf,
        48000 as libc::c_int * 30 as libc::c_int / 3 as libc::c_int / 2 as libc::c_int,
    );
    outbuf = malloc(
        (::core::mem::size_of::<i16>() as libc::c_ulong)
            .wrapping_mul(5760 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    ) as *mut i16;
    loop {
        len = opus_encode(
            enc,
            &mut *inbuf.offset((samp_count * channels) as isize),
            frame_size,
            packet.as_mut_ptr(),
            1500 as libc::c_int,
        );
        if len < 0 as libc::c_int || len > 1500 as libc::c_int {
            fprintf(
                stderr,
                b"opus_encode() returned %d\n\0" as *const u8 as *const libc::c_char,
                len,
            );
            ret = -(1 as libc::c_int);
            break;
        } else {
            out_samples = opus_decode(
                dec,
                packet.as_mut_ptr(),
                len,
                outbuf,
                5760 as libc::c_int,
                0 as libc::c_int,
            );
            if out_samples != frame_size {
                fprintf(
                    stderr,
                    b"opus_decode() returned %d\n\0" as *const u8 as *const libc::c_char,
                    out_samples,
                );
                ret = -(1 as libc::c_int);
                break;
            } else {
                samp_count += frame_size;
                if samp_count
                    >= 48000 as libc::c_int * 30 as libc::c_int
                        / 3 as libc::c_int
                        / 2 as libc::c_int
                        - 5760 as libc::c_int
                {
                    break;
                }
            }
        }
    }
    free(inbuf as *mut libc::c_void);
    free(outbuf as *mut libc::c_void);
    ret
}
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn fuzz_encoder_settings(
    num_encoders: libc::c_int,
    num_setting_changes: libc::c_int,
) {
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut sampling_rates: [libc::c_int; 5] = [
        8000 as libc::c_int,
        12000 as libc::c_int,
        16000 as libc::c_int,
        24000 as libc::c_int,
        48000 as libc::c_int,
    ];
    let mut channels: [libc::c_int; 2] = [1 as libc::c_int, 2 as libc::c_int];
    let mut applications: [libc::c_int; 3] = [
        2049 as libc::c_int,
        2048 as libc::c_int,
        2051 as libc::c_int,
    ];
    let mut bitrates: [libc::c_int; 11] = [
        6000 as libc::c_int,
        12000 as libc::c_int,
        16000 as libc::c_int,
        24000 as libc::c_int,
        32000 as libc::c_int,
        48000 as libc::c_int,
        64000 as libc::c_int,
        96000 as libc::c_int,
        510000 as libc::c_int,
        -(1000 as libc::c_int),
        -(1 as libc::c_int),
    ];
    let mut force_channels: [libc::c_int; 4] = [
        -(1000 as libc::c_int),
        -(1000 as libc::c_int),
        1 as libc::c_int,
        2 as libc::c_int,
    ];
    let mut use_vbr: [libc::c_int; 3] = [0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int];
    let mut vbr_constraints: [libc::c_int; 3] =
        [0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int];
    let mut complexities: [libc::c_int; 11] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
    ];
    let mut max_bandwidths: [libc::c_int; 6] = [
        1101 as libc::c_int,
        1102 as libc::c_int,
        1103 as libc::c_int,
        1104 as libc::c_int,
        1105 as libc::c_int,
        1105 as libc::c_int,
    ];
    let mut signals: [libc::c_int; 4] = [
        -(1000 as libc::c_int),
        -(1000 as libc::c_int),
        3001 as libc::c_int,
        3002 as libc::c_int,
    ];
    let mut inband_fecs: [libc::c_int; 3] = [0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int];
    let mut packet_loss_perc: [libc::c_int; 4] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        5 as libc::c_int,
    ];
    let mut lsb_depths: [libc::c_int; 2] = [8 as libc::c_int, 24 as libc::c_int];
    let mut prediction_disabled: [libc::c_int; 3] =
        [0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int];
    let mut use_dtx: [libc::c_int; 2] = [0 as libc::c_int, 1 as libc::c_int];
    let mut frame_sizes_ms_x2: [libc::c_int; 9] = [
        5 as libc::c_int,
        10 as libc::c_int,
        20 as libc::c_int,
        40 as libc::c_int,
        80 as libc::c_int,
        120 as libc::c_int,
        160 as libc::c_int,
        200 as libc::c_int,
        240 as libc::c_int,
    ];
    i = 0 as libc::c_int;
    while i < num_encoders {
        let mut sampling_rate: libc::c_int = sampling_rates[(fast_rand() as libc::c_ulong)
            .wrapping_rem(::core::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as usize];
        let mut num_channels: libc::c_int = channels[(fast_rand() as libc::c_ulong)
            .wrapping_rem(::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as usize];
        let mut application: libc::c_int = applications[(fast_rand() as libc::c_ulong)
            .wrapping_rem(::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as usize];
        dec = opus_decoder_create(sampling_rate, num_channels, &mut err);
        if err != 0 as libc::c_int || dec.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                217 as libc::c_int,
            );
        }
        enc = opus_encoder_create(sampling_rate, num_channels, application, &mut err);
        if err != 0 as libc::c_int || enc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int,
            );
        }
        j = 0 as libc::c_int;
        while j < num_setting_changes {
            let mut bitrate: libc::c_int = bitrates[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 11]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut force_channel: libc::c_int = force_channels[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut vbr: libc::c_int = use_vbr[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut vbr_constraint: libc::c_int = vbr_constraints[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut complexity: libc::c_int = complexities[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 11]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut max_bw: libc::c_int = max_bandwidths[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut sig: libc::c_int = signals[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut inband_fec: libc::c_int = inband_fecs[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut pkt_loss: libc::c_int = packet_loss_perc[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut lsb_depth: libc::c_int = lsb_depths[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut pred_disabled: libc::c_int = prediction_disabled[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut dtx: libc::c_int = use_dtx[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut frame_size_ms_x2: libc::c_int = frame_sizes_ms_x2[(fast_rand() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<[libc::c_int; 9]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as usize];
            let mut frame_size: libc::c_int =
                frame_size_ms_x2 * sampling_rate / 2000 as libc::c_int;
            let mut frame_size_enum: libc::c_int = get_frame_size_enum(frame_size, sampling_rate);
            force_channel = if force_channel < num_channels {
                force_channel
            } else {
                num_channels
            };
            if opus_encoder_ctl(enc, 4002 as libc::c_int, bitrate) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    240 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4022 as libc::c_int, force_channel) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    241 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4006 as libc::c_int, vbr) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    242 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4020 as libc::c_int, vbr_constraint) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    243 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4010 as libc::c_int, complexity) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    244 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4004 as libc::c_int, max_bw) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    245 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4024 as libc::c_int, sig) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    246 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4012 as libc::c_int, inband_fec) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    247 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4014 as libc::c_int, pkt_loss) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    248 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4036 as libc::c_int, lsb_depth) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    249 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4042 as libc::c_int, pred_disabled) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    250 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4016 as libc::c_int, dtx) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    251 as libc::c_int,
                );
            }
            if opus_encoder_ctl(enc, 4040 as libc::c_int, frame_size_enum) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    252 as libc::c_int,
                );
            }
            if test_encode(enc, num_channels, frame_size, dec) != 0 {
                fprintf(
                    stderr,
                    b"fuzz_encoder_settings: %d kHz, %d ch, application: %d, %d bps, force ch: %d, vbr: %d, vbr constraint: %d, complexity: %d, max bw: %d, signal: %d, inband fec: %d, pkt loss: %d%%, lsb depth: %d, pred disabled: %d, dtx: %d, (%d/2) ms\n\0"
                        as *const u8 as *const libc::c_char,
                    sampling_rate / 1000 as libc::c_int,
                    num_channels,
                    application,
                    bitrate,
                    force_channel,
                    vbr,
                    vbr_constraint,
                    complexity,
                    max_bw,
                    sig,
                    inband_fec,
                    pkt_loss,
                    lsb_depth,
                    pred_disabled,
                    dtx,
                    frame_size_ms_x2,
                );
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    263 as libc::c_int,
                );
            }
            j += 1;
        }
        opus_encoder_destroy(enc);
        opus_decoder_destroy(dec);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "272:1"]
pub unsafe extern "C" fn run_test1(mut no_fuzz: libc::c_int) -> libc::c_int {
    static mut fsizes: [libc::c_int; 6] = [
        960 as libc::c_int * 3 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        120 as libc::c_int,
        240 as libc::c_int,
        480 as libc::c_int,
        960 as libc::c_int,
    ];
    static mut mstrings: [*const libc::c_char; 3] = [
        b"    LP\0" as *const u8 as *const libc::c_char,
        b"Hybrid\0" as *const u8 as *const libc::c_char,
        b"  MDCT\0" as *const u8 as *const libc::c_char,
    ];
    let mut mapping: [libc::c_uchar; 256] = [
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut db62: [libc::c_uchar; 36] = [0; 36];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut rc: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut MSenc: *mut OpusMSEncoder = std::ptr::null_mut::<OpusMSEncoder>();
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut MSdec: *mut OpusMSDecoder = std::ptr::null_mut::<OpusMSDecoder>();
    let mut MSdec_err: *mut OpusMSDecoder = std::ptr::null_mut::<OpusMSDecoder>();
    let mut dec_err: [*mut OpusDecoder; 10] = [std::ptr::null_mut::<OpusDecoder>(); 10];
    let mut inbuf: *mut libc::c_short = std::ptr::null_mut::<libc::c_short>();
    let mut outbuf: *mut libc::c_short = std::ptr::null_mut::<libc::c_short>();
    let mut out2buf: *mut libc::c_short = std::ptr::null_mut::<libc::c_short>();
    let mut bitrate_bps: i32 = 0;
    let mut packet: [libc::c_uchar; 1757] = [0; 1757];
    let mut enc_final_range: u32 = 0;
    let mut dec_final_range: u32 = 0;
    let mut fswitch: libc::c_int = 0;
    let mut fsize: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    fprintf(
        stdout,
        b"  Encode+Decode tests.\n\0" as *const u8 as *const libc::c_char,
    );
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        2048 as libc::c_int,
        &mut err,
    );
    if err != 0 as libc::c_int || enc.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut ret_err: *mut libc::c_int = std::ptr::null_mut::<libc::c_int>();
        ret_err = if i != 0 {
            std::ptr::null_mut::<libc::c_int>()
        } else {
            &mut err
        };
        MSenc = opus_multistream_encoder_create(
            8000 as libc::c_int,
            2 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            -(5 as libc::c_int),
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            2048 as libc::c_int,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                312 as libc::c_int,
            );
        }
        MSenc = opus_multistream_encoder_create(
            44100 as libc::c_int,
            2 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            2048 as libc::c_int,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                315 as libc::c_int,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as libc::c_int,
            2 as libc::c_int,
            2 as libc::c_int,
            3 as libc::c_int,
            mapping.as_mut_ptr(),
            2048 as libc::c_int,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as libc::c_int,
            2 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            2048 as libc::c_int,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as libc::c_int,
            256 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            2048 as libc::c_int,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                324 as libc::c_int,
            );
        }
        i += 1;
    }
    MSenc = opus_multistream_encoder_create(
        8000 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        mapping.as_mut_ptr(),
        2049 as libc::c_int,
        &mut err,
    );
    if err != 0 as libc::c_int || MSenc.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            328 as libc::c_int,
        );
    }
    if opus_multistream_encoder_ctl(
        MSenc,
        4003 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    ) != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            331 as libc::c_int,
        );
    }
    if opus_multistream_encoder_ctl(
        MSenc,
        4037 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    ) != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            332 as libc::c_int,
        );
    }
    if i < 16 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            333 as libc::c_int,
        );
    }
    let mut tmp_enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    if opus_multistream_encoder_ctl(
        MSenc,
        5120 as libc::c_int,
        1 as libc::c_int,
        (&mut tmp_enc as *mut *mut OpusEncoder).offset(
            (&mut tmp_enc as *mut *mut OpusEncoder)
                .offset_from(&mut tmp_enc as *mut *mut OpusEncoder) as libc::c_long
                as isize,
        ),
    ) != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int,
        );
    }
    if opus_encoder_ctl(
        tmp_enc,
        4037 as libc::c_int,
        (&mut j as *mut i32)
            .offset((&mut j as *mut i32).offset_from(&mut j as *mut i32) as libc::c_long as isize),
    ) != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
        );
    }
    if i != j {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
        );
    }
    if opus_multistream_encoder_ctl(
        MSenc,
        5120 as libc::c_int,
        2 as libc::c_int,
        (&mut tmp_enc as *mut *mut OpusEncoder).offset(
            (&mut tmp_enc as *mut *mut OpusEncoder)
                .offset_from(&mut tmp_enc as *mut *mut OpusEncoder) as libc::c_long
                as isize,
        ),
    ) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            340 as libc::c_int,
        );
    }
    dec = opus_decoder_create(48000 as libc::c_int, 2 as libc::c_int, &mut err);
    if err != 0 as libc::c_int || dec.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            344 as libc::c_int,
        );
    }
    MSdec = opus_multistream_decoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as libc::c_int || MSdec.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
        );
    }
    MSdec_err = opus_multistream_decoder_create(
        48000 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as libc::c_int || MSdec_err.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int,
        );
    }
    dec_err[0 as libc::c_int as usize] =
        malloc(opus_decoder_get_size(2 as libc::c_int) as libc::c_ulong) as *mut OpusDecoder;
    memcpy(
        dec_err[0 as libc::c_int as usize] as *mut libc::c_void,
        dec as *const libc::c_void,
        opus_decoder_get_size(2 as libc::c_int) as libc::c_ulong,
    );
    dec_err[1 as libc::c_int as usize] =
        opus_decoder_create(48000 as libc::c_int, 1 as libc::c_int, &mut err);
    dec_err[2 as libc::c_int as usize] =
        opus_decoder_create(24000 as libc::c_int, 2 as libc::c_int, &mut err);
    dec_err[3 as libc::c_int as usize] =
        opus_decoder_create(24000 as libc::c_int, 1 as libc::c_int, &mut err);
    dec_err[4 as libc::c_int as usize] =
        opus_decoder_create(16000 as libc::c_int, 2 as libc::c_int, &mut err);
    dec_err[5 as libc::c_int as usize] =
        opus_decoder_create(16000 as libc::c_int, 1 as libc::c_int, &mut err);
    dec_err[6 as libc::c_int as usize] =
        opus_decoder_create(12000 as libc::c_int, 2 as libc::c_int, &mut err);
    dec_err[7 as libc::c_int as usize] =
        opus_decoder_create(12000 as libc::c_int, 1 as libc::c_int, &mut err);
    dec_err[8 as libc::c_int as usize] =
        opus_decoder_create(8000 as libc::c_int, 2 as libc::c_int, &mut err);
    dec_err[9 as libc::c_int as usize] =
        opus_decoder_create(8000 as libc::c_int, 1 as libc::c_int, &mut err);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if (dec_err[i as usize]).is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                363 as libc::c_int,
            );
        }
        i += 1;
    }
    let mut enccpy: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    enccpy = malloc(opus_encoder_get_size(2 as libc::c_int) as libc::c_ulong) as *mut OpusEncoder;
    memcpy(
        enccpy as *mut libc::c_void,
        enc as *const libc::c_void,
        opus_encoder_get_size(2 as libc::c_int) as libc::c_ulong,
    );
    memset(
        enc as *mut libc::c_void,
        255 as libc::c_int,
        opus_encoder_get_size(2 as libc::c_int) as libc::c_ulong,
    );
    opus_encoder_destroy(enc);
    enc = enccpy;
    inbuf = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_mul((48000 as libc::c_int * 30 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_short;
    outbuf = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_mul((48000 as libc::c_int * 30 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_short;
    out2buf = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_mul(5760 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_short;
    if inbuf.is_null() || outbuf.is_null() || out2buf.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
        );
    }
    generate_music(inbuf, 48000 as libc::c_int * 30 as libc::c_int);
    if opus_encoder_ctl(enc, 4008 as libc::c_int, -(1000 as libc::c_int)) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
        );
    }
    if opus_encoder_ctl(enc, 11002 as libc::c_int, -(2 as libc::c_int)) != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int,
        );
    }
    if opus_encode(
        enc,
        inbuf,
        500 as libc::c_int,
        packet.as_mut_ptr(),
        1500 as libc::c_int,
    ) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
        );
    }
    rc = 0 as libc::c_int;
    while rc < 3 as libc::c_int {
        if opus_encoder_ctl(
            enc,
            4006 as libc::c_int,
            (rc < 2 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                393 as libc::c_int,
            );
        }
        if opus_encoder_ctl(
            enc,
            4020 as libc::c_int,
            (rc == 1 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int,
            );
        }
        if opus_encoder_ctl(
            enc,
            4020 as libc::c_int,
            (rc == 1 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                395 as libc::c_int,
            );
        }
        if opus_encoder_ctl(
            enc,
            4012 as libc::c_int,
            (rc == 0 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                396 as libc::c_int,
            );
        }
        j = 0 as libc::c_int;
        while j < 13 as libc::c_int {
            let mut rate: libc::c_int = 0;
            let mut modes: [libc::c_int; 13] = [
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
            ];
            let mut rates: [libc::c_int; 13] = [
                6000 as libc::c_int,
                12000 as libc::c_int,
                48000 as libc::c_int,
                16000 as libc::c_int,
                32000 as libc::c_int,
                48000 as libc::c_int,
                64000 as libc::c_int,
                512000 as libc::c_int,
                13000 as libc::c_int,
                24000 as libc::c_int,
                48000 as libc::c_int,
                64000 as libc::c_int,
                96000 as libc::c_int,
            ];
            let mut frame: [libc::c_int; 13] = [
                960 as libc::c_int * 2 as libc::c_int,
                960 as libc::c_int,
                480 as libc::c_int,
                960 as libc::c_int,
                960 as libc::c_int,
                960 as libc::c_int,
                480 as libc::c_int,
                960 as libc::c_int * 3 as libc::c_int,
                960 as libc::c_int * 3 as libc::c_int,
                960 as libc::c_int,
                480 as libc::c_int,
                240 as libc::c_int,
                120 as libc::c_int,
            ];
            rate = (rates[j as usize] as libc::c_uint)
                .wrapping_add((fast_rand()).wrapping_rem(rates[j as usize] as libc::c_uint))
                as libc::c_int;
            i = 0 as libc::c_int;
            count = i;
            loop {
                let mut bw: libc::c_int = 0;
                let mut len: libc::c_int = 0;
                let mut out_samples: libc::c_int = 0;
                let mut frame_size: libc::c_int = 0;
                frame_size = frame[j as usize];
                if fast_rand() & 255 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if opus_encoder_ctl(enc, 4028 as libc::c_int) != 0 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            410 as libc::c_int,
                        );
                    }
                    if opus_decoder_ctl(dec, 4028 as libc::c_int) != 0 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            411 as libc::c_int,
                        );
                    }
                    if fast_rand() & 1 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                        && opus_decoder_ctl(
                            dec_err[(fast_rand() & 1 as libc::c_int as libc::c_uint) as usize],
                            4028 as libc::c_int,
                        ) != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            414 as libc::c_int,
                        );
                    }
                }
                if fast_rand() & 127 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                    && opus_decoder_ctl(
                        dec_err[(fast_rand() & 1 as libc::c_int as libc::c_uint) as usize],
                        4028 as libc::c_int,
                    ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        419 as libc::c_int,
                    );
                }
                if (fast_rand()).wrapping_rem(10 as libc::c_int as libc::c_uint)
                    == 0 as libc::c_int as libc::c_uint
                {
                    let mut complex: libc::c_int = (fast_rand())
                        .wrapping_rem(11 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    if opus_encoder_ctl(enc, 4010 as libc::c_int, complex) != 0 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            423 as libc::c_int,
                        );
                    }
                }
                if (fast_rand()).wrapping_rem(50 as libc::c_int as libc::c_uint)
                    == 0 as libc::c_int as libc::c_uint
                {
                    opus_decoder_ctl(dec, 4028 as libc::c_int);
                }
                if opus_encoder_ctl(
                    enc,
                    4012 as libc::c_int,
                    (rc == 0 as libc::c_int) as libc::c_int,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        426 as libc::c_int,
                    );
                }
                if opus_encoder_ctl(
                    enc,
                    11002 as libc::c_int,
                    1000 as libc::c_int + modes[j as usize],
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        427 as libc::c_int,
                    );
                }
                fast_rand();
                if opus_encoder_ctl(
                    enc,
                    4016 as libc::c_int,
                    (fast_rand() & 1 as libc::c_int as libc::c_uint) as i32,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        428 as libc::c_int,
                    );
                }
                if opus_encoder_ctl(enc, 4002 as libc::c_int, rate) != 0 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        429 as libc::c_int,
                    );
                }
                if rates[j as usize] >= 64000 as libc::c_int {
                } else {
                };
                if opus_encoder_ctl(
                    enc,
                    4022 as libc::c_int,
                    if rates[j as usize] >= 64000 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    },
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        430 as libc::c_int,
                    );
                }
                if opus_encoder_ctl(
                    enc,
                    4010 as libc::c_int,
                    (count >> 2 as libc::c_int) % 11 as libc::c_int,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        431 as libc::c_int,
                    );
                }
                fast_rand();
                fast_rand();
                if opus_encoder_ctl(
                    enc,
                    4014 as libc::c_int,
                    (fast_rand()
                        & 15 as libc::c_int as libc::c_uint
                        & (fast_rand()).wrapping_rem(15 as libc::c_int as libc::c_uint))
                        as i32,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        432 as libc::c_int,
                    );
                }
                bw = (if modes[j as usize] == 0 as libc::c_int {
                    (1101 as libc::c_int as libc::c_uint)
                        .wrapping_add((fast_rand()).wrapping_rem(3 as libc::c_int as libc::c_uint))
                } else if modes[j as usize] == 1 as libc::c_int {
                    (1104 as libc::c_int as libc::c_uint)
                        .wrapping_add(fast_rand() & 1 as libc::c_int as libc::c_uint)
                } else {
                    (1101 as libc::c_int as libc::c_uint)
                        .wrapping_add((fast_rand()).wrapping_rem(5 as libc::c_int as libc::c_uint))
                }) as libc::c_int;
                if modes[j as usize] == 2 as libc::c_int && bw == 1102 as libc::c_int {
                    bw += 3 as libc::c_int;
                }
                if opus_encoder_ctl(enc, 4008 as libc::c_int, bw) != 0 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        437 as libc::c_int,
                    );
                }
                len = opus_encode(
                    enc,
                    &mut *inbuf.offset((i << 1 as libc::c_int) as isize),
                    frame_size,
                    packet.as_mut_ptr(),
                    1500 as libc::c_int,
                );
                if len < 0 as libc::c_int || len > 1500 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        439 as libc::c_int,
                    );
                }
                if opus_encoder_ctl(
                    enc,
                    4031 as libc::c_int,
                    (&mut enc_final_range as *mut u32).offset(
                        (&mut enc_final_range as *mut u32)
                            .offset_from(&mut enc_final_range as *mut u32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        440 as libc::c_int,
                    );
                }
                if fast_rand() & 3 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if opus_packet_pad(packet.as_mut_ptr(), len, len + 1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            443 as libc::c_int,
                        );
                    }
                    len += 1;
                }
                if fast_rand() & 7 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if opus_packet_pad(packet.as_mut_ptr(), len, len + 256 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            448 as libc::c_int,
                        );
                    }
                    len += 256 as libc::c_int;
                }
                if fast_rand() & 3 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    len = opus_packet_unpad(packet.as_mut_ptr(), len);
                    if len < 1 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            454 as libc::c_int,
                        );
                    }
                }
                out_samples = opus_decode(
                    dec,
                    packet.as_mut_ptr(),
                    len,
                    &mut *outbuf.offset((i << 1 as libc::c_int) as isize),
                    5760 as libc::c_int,
                    0 as libc::c_int,
                );
                if out_samples != frame_size {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        457 as libc::c_int,
                    );
                }
                if opus_decoder_ctl(
                    dec,
                    4031 as libc::c_int,
                    (&mut dec_final_range as *mut u32).offset(
                        (&mut dec_final_range as *mut u32)
                            .offset_from(&mut dec_final_range as *mut u32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        458 as libc::c_int,
                    );
                }
                if enc_final_range != dec_final_range {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        459 as libc::c_int,
                    );
                }
                out_samples = opus_decode(
                    dec_err[0 as libc::c_int as usize],
                    packet.as_mut_ptr(),
                    len,
                    out2buf,
                    frame_size,
                    (fast_rand() & 3 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint) as libc::c_int,
                );
                if out_samples != frame_size {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        462 as libc::c_int,
                    );
                }
                out_samples = opus_decode(
                    dec_err[1 as libc::c_int as usize],
                    packet.as_mut_ptr(),
                    if fast_rand() & 3 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        0 as libc::c_int
                    } else {
                        len
                    },
                    out2buf,
                    5760 as libc::c_int,
                    (fast_rand() & 7 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint) as libc::c_int,
                );
                if out_samples < 120 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        464 as libc::c_int,
                    );
                }
                i += frame_size;
                count += 1;
                if i >= 48000 as libc::c_int * 30 as libc::c_int / 3 as libc::c_int
                    - 5760 as libc::c_int
                {
                    break;
                }
            }
            fprintf(
                stdout,
                b"    Mode %s FB encode %s, %6d bps OK.\n\0" as *const u8 as *const libc::c_char,
                mstrings[modes[j as usize] as usize],
                if rc == 0 as libc::c_int {
                    b" VBR\0" as *const u8 as *const libc::c_char
                } else if rc == 1 as libc::c_int {
                    b"CVBR\0" as *const u8 as *const libc::c_char
                } else {
                    b" CBR\0" as *const u8 as *const libc::c_char
                },
                rate,
            );
            j += 1;
        }
        rc += 1;
    }
    if opus_encoder_ctl(enc, 11002 as libc::c_int, -(1000 as libc::c_int)) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int,
        );
    }
    if opus_encoder_ctl(enc, 4022 as libc::c_int, -(1000 as libc::c_int)) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int,
        );
    }
    if opus_encoder_ctl(enc, 4012 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            474 as libc::c_int,
        );
    }
    if opus_encoder_ctl(enc, 4016 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            475 as libc::c_int,
        );
    }
    rc = 0 as libc::c_int;
    while rc < 3 as libc::c_int {
        if opus_multistream_encoder_ctl(
            MSenc,
            4006 as libc::c_int,
            (rc < 2 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                479 as libc::c_int,
            );
        }
        if opus_multistream_encoder_ctl(
            MSenc,
            4020 as libc::c_int,
            (rc == 1 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                480 as libc::c_int,
            );
        }
        if opus_multistream_encoder_ctl(
            MSenc,
            4020 as libc::c_int,
            (rc == 1 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                481 as libc::c_int,
            );
        }
        if opus_multistream_encoder_ctl(
            MSenc,
            4012 as libc::c_int,
            (rc == 0 as libc::c_int) as libc::c_int,
        ) != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                482 as libc::c_int,
            );
        }
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            let mut rate_0: libc::c_int = 0;
            let mut modes_0: [libc::c_int; 16] = [
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
            ];
            let mut rates_0: [libc::c_int; 16] = [
                4000 as libc::c_int,
                12000 as libc::c_int,
                32000 as libc::c_int,
                8000 as libc::c_int,
                16000 as libc::c_int,
                32000 as libc::c_int,
                48000 as libc::c_int,
                88000 as libc::c_int,
                4000 as libc::c_int,
                12000 as libc::c_int,
                32000 as libc::c_int,
                8000 as libc::c_int,
                16000 as libc::c_int,
                32000 as libc::c_int,
                48000 as libc::c_int,
                88000 as libc::c_int,
            ];
            let mut frame_0: [libc::c_int; 16] = [
                160 as libc::c_int * 1 as libc::c_int,
                160 as libc::c_int,
                80 as libc::c_int,
                160 as libc::c_int,
                160 as libc::c_int,
                80 as libc::c_int,
                40 as libc::c_int,
                20 as libc::c_int,
                160 as libc::c_int * 1 as libc::c_int,
                160 as libc::c_int,
                80 as libc::c_int,
                160 as libc::c_int,
                160 as libc::c_int,
                80 as libc::c_int,
                40 as libc::c_int,
                20 as libc::c_int,
            ];
            (rc == 0 as libc::c_int && j == 1 as libc::c_int) as libc::c_int;
            if opus_multistream_encoder_ctl(
                MSenc,
                4012 as libc::c_int,
                (rc == 0 as libc::c_int && j == 1 as libc::c_int) as libc::c_int,
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    489 as libc::c_int,
                );
            }
            if opus_multistream_encoder_ctl(
                MSenc,
                11002 as libc::c_int,
                1000 as libc::c_int + modes_0[j as usize],
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    490 as libc::c_int,
                );
            }
            rate_0 = (rates_0[j as usize] as libc::c_uint)
                .wrapping_add((fast_rand()).wrapping_rem(rates_0[j as usize] as libc::c_uint))
                as libc::c_int;
            fast_rand();
            if opus_multistream_encoder_ctl(
                MSenc,
                4016 as libc::c_int,
                (fast_rand() & 1 as libc::c_int as libc::c_uint) as i32,
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    492 as libc::c_int,
                );
            }
            if opus_multistream_encoder_ctl(MSenc, 4002 as libc::c_int, rate_0) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                    493 as libc::c_int,
                );
            }
            i = 0 as libc::c_int;
            count = i;
            loop {
                let mut len_0: libc::c_int = 0;
                let mut out_samples_0: libc::c_int = 0;
                let mut frame_size_0: libc::c_int = 0;
                let mut loss: libc::c_int = 0;
                let mut pred: i32 = 0;
                if opus_multistream_encoder_ctl(
                    MSenc,
                    4043 as libc::c_int,
                    (&mut pred as *mut i32)
                        .offset((&mut pred as *mut i32).offset_from(&mut pred as *mut i32)
                            as libc::c_long as isize),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        498 as libc::c_int,
                    );
                }
                fast_rand();
                if pred != 0 {
                } else {
                };
                if opus_multistream_encoder_ctl(
                    MSenc,
                    4042 as libc::c_int,
                    (((fast_rand() & 15 as libc::c_int as libc::c_uint) as libc::c_int)
                        < (if pred != 0 {
                            11 as libc::c_int
                        } else {
                            4 as libc::c_int
                        })) as libc::c_int,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        499 as libc::c_int,
                    );
                }
                frame_size_0 = frame_0[j as usize];
                if opus_multistream_encoder_ctl(
                    MSenc,
                    4010 as libc::c_int,
                    (count >> 2 as libc::c_int) % 11 as libc::c_int,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        501 as libc::c_int,
                    );
                }
                fast_rand();
                fast_rand();
                if opus_multistream_encoder_ctl(
                    MSenc,
                    4014 as libc::c_int,
                    (fast_rand()
                        & 15 as libc::c_int as libc::c_uint
                        & (fast_rand()).wrapping_rem(15 as libc::c_int as libc::c_uint))
                        as i32,
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        502 as libc::c_int,
                    );
                }
                if fast_rand() & 255 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if opus_multistream_encoder_ctl(MSenc, 4028 as libc::c_int) != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            505 as libc::c_int,
                        );
                    }
                    if opus_multistream_decoder_ctl(MSdec, 4028 as libc::c_int) != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            506 as libc::c_int,
                        );
                    }
                    if fast_rand() & 3 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                        && opus_multistream_decoder_ctl(MSdec_err, 4028 as libc::c_int)
                            != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            509 as libc::c_int,
                        );
                    }
                }
                if fast_rand() & 255 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                    && opus_multistream_decoder_ctl(MSdec_err, 4028 as libc::c_int)
                        != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        514 as libc::c_int,
                    );
                }
                len_0 = opus_multistream_encode(
                    MSenc,
                    &mut *inbuf.offset((i << 1 as libc::c_int) as isize),
                    frame_size_0,
                    packet.as_mut_ptr(),
                    1500 as libc::c_int,
                );
                if len_0 < 0 as libc::c_int || len_0 > 1500 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        517 as libc::c_int,
                    );
                }
                if opus_multistream_encoder_ctl(
                    MSenc,
                    4031 as libc::c_int,
                    (&mut enc_final_range as *mut u32).offset(
                        (&mut enc_final_range as *mut u32)
                            .offset_from(&mut enc_final_range as *mut u32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        518 as libc::c_int,
                    );
                }
                if fast_rand() & 3 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if opus_multistream_packet_pad(
                        packet.as_mut_ptr(),
                        len_0,
                        len_0 + 1 as libc::c_int,
                        2 as libc::c_int,
                    ) != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            521 as libc::c_int,
                        );
                    }
                    len_0 += 1;
                }
                if fast_rand() & 7 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if opus_multistream_packet_pad(
                        packet.as_mut_ptr(),
                        len_0,
                        len_0 + 256 as libc::c_int,
                        2 as libc::c_int,
                    ) != 0 as libc::c_int
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            526 as libc::c_int,
                        );
                    }
                    len_0 += 256 as libc::c_int;
                }
                if fast_rand() & 3 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    len_0 =
                        opus_multistream_packet_unpad(packet.as_mut_ptr(), len_0, 2 as libc::c_int);
                    if len_0 < 1 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                            532 as libc::c_int,
                        );
                    }
                }
                out_samples_0 = opus_multistream_decode(
                    MSdec,
                    packet.as_mut_ptr(),
                    len_0,
                    out2buf,
                    5760 as libc::c_int,
                    0 as libc::c_int,
                );
                if out_samples_0 != frame_size_0 * 6 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        535 as libc::c_int,
                    );
                }
                if opus_multistream_decoder_ctl(
                    MSdec,
                    4031 as libc::c_int,
                    (&mut dec_final_range as *mut u32).offset(
                        (&mut dec_final_range as *mut u32)
                            .offset_from(&mut dec_final_range as *mut u32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        536 as libc::c_int,
                    );
                }
                if enc_final_range != dec_final_range {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        537 as libc::c_int,
                    );
                }
                loss = (fast_rand() & 63 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint) as libc::c_int;
                out_samples_0 = opus_multistream_decode(
                    MSdec_err,
                    packet.as_mut_ptr(),
                    if loss != 0 { 0 as libc::c_int } else { len_0 },
                    out2buf,
                    frame_size_0 * 6 as libc::c_int,
                    (fast_rand() & 3 as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint) as libc::c_int,
                );
                if out_samples_0 != frame_size_0 * 6 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                        541 as libc::c_int,
                    );
                }
                i += frame_size_0;
                count += 1;
                if i >= 48000 as libc::c_int * 30 as libc::c_int
                    / 3 as libc::c_int
                    / 12 as libc::c_int
                    - 5760 as libc::c_int
                {
                    break;
                }
            }
            fprintf(
                stdout,
                b"    Mode %s NB dual-mono MS encode %s, %6d bps OK.\n\0" as *const u8
                    as *const libc::c_char,
                mstrings[modes_0[j as usize] as usize],
                if rc == 0 as libc::c_int {
                    b" VBR\0" as *const u8 as *const libc::c_char
                } else if rc == 1 as libc::c_int {
                    b"CVBR\0" as *const u8 as *const libc::c_char
                } else {
                    b" CBR\0" as *const u8 as *const libc::c_char
                },
                rate_0,
            );
            j += 1;
        }
        rc += 1;
    }
    bitrate_bps = 512000 as libc::c_int;
    fsize = (fast_rand()).wrapping_rem(31 as libc::c_int as libc::c_uint) as libc::c_int;
    fswitch = 100 as libc::c_int;
    debruijn2(6 as libc::c_int, db62.as_mut_ptr());
    i = 0 as libc::c_int;
    count = i;
    loop {
        let mut toc: libc::c_uchar = 0;
        let mut frames: [*const libc::c_uchar; 48] = [std::ptr::null::<libc::c_uchar>(); 48];
        let mut size: [libc::c_short; 48] = [0; 48];
        let mut payload_offset: libc::c_int = 0;
        let mut dec_final_range2: u32 = 0;
        let mut jj: libc::c_int = 0;
        let mut dec2: libc::c_int = 0;
        let mut len_1: libc::c_int = 0;
        let mut out_samples_1: libc::c_int = 0;
        let mut frame_size_1: libc::c_int = fsizes[db62[fsize as usize] as usize];
        let mut offset: i32 = i % (48000 as libc::c_int * 30 as libc::c_int - 5760 as libc::c_int);
        opus_encoder_ctl(enc, 4002 as libc::c_int, bitrate_bps);
        len_1 = opus_encode(
            enc,
            &mut *inbuf.offset((offset << 1 as libc::c_int) as isize),
            frame_size_1,
            packet.as_mut_ptr(),
            1500 as libc::c_int,
        );
        if len_1 < 0 as libc::c_int || len_1 > 1500 as libc::c_int {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                569 as libc::c_int,
            );
        }
        count += 1;
        opus_encoder_ctl(
            enc,
            4031 as libc::c_int,
            (&mut enc_final_range as *mut u32).offset(
                (&mut enc_final_range as *mut u32).offset_from(&mut enc_final_range as *mut u32)
                    as libc::c_long as isize,
            ),
        );
        out_samples_1 = opus_decode(
            dec,
            packet.as_mut_ptr(),
            len_1,
            &mut *outbuf.offset((offset << 1 as libc::c_int) as isize),
            5760 as libc::c_int,
            0 as libc::c_int,
        );
        if out_samples_1 != frame_size_1 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                575 as libc::c_int,
            );
        }
        opus_decoder_ctl(
            dec,
            4031 as libc::c_int,
            (&mut dec_final_range as *mut u32).offset(
                (&mut dec_final_range as *mut u32).offset_from(&mut dec_final_range as *mut u32)
                    as libc::c_long as isize,
            ),
        );
        if dec_final_range != enc_final_range {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                580 as libc::c_int,
            );
        }
        if opus_packet_parse(
            packet.as_mut_ptr(),
            len_1,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        ) <= 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                585 as libc::c_int,
            );
        }
        if fast_rand() & 1023 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            len_1 = 0 as libc::c_int;
        }
        j = (frames[0 as libc::c_int as usize]).offset_from(packet.as_mut_ptr()) as libc::c_long
            as i32;
        while j < len_1 {
            jj = 0 as libc::c_int;
            while jj < 8 as libc::c_int {
                packet[j as usize] = (packet[j as usize] as libc::c_int
                    ^ ((no_fuzz == 0
                        && fast_rand() & 1023 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint)
                        as libc::c_int)
                        << jj) as libc::c_uchar;
                jj += 1;
            }
            j += 1;
        }
        out_samples_1 = opus_decode(
            dec_err[0 as libc::c_int as usize],
            if len_1 > 0 as libc::c_int {
                packet.as_mut_ptr()
            } else {
                std::ptr::null_mut::<libc::c_uchar>()
            },
            len_1,
            out2buf,
            5760 as libc::c_int,
            0 as libc::c_int,
        );
        if out_samples_1 < 0 as libc::c_int || out_samples_1 > 5760 as libc::c_int {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                589 as libc::c_int,
            );
        }
        if len_1 > 0 as libc::c_int && out_samples_1 != frame_size_1 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                590 as libc::c_int,
            );
        }
        opus_decoder_ctl(
            dec_err[0 as libc::c_int as usize],
            4031 as libc::c_int,
            (&mut dec_final_range as *mut u32).offset(
                (&mut dec_final_range as *mut u32).offset_from(&mut dec_final_range as *mut u32)
                    as libc::c_long as isize,
            ),
        );
        dec2 = (fast_rand())
            .wrapping_rem(9 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
        out_samples_1 = opus_decode(
            dec_err[dec2 as usize],
            if len_1 > 0 as libc::c_int {
                packet.as_mut_ptr()
            } else {
                std::ptr::null_mut::<libc::c_uchar>()
            },
            len_1,
            out2buf,
            5760 as libc::c_int,
            0 as libc::c_int,
        );
        if out_samples_1 < 0 as libc::c_int || out_samples_1 > 5760 as libc::c_int {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                597 as libc::c_int,
            );
        }
        opus_decoder_ctl(
            dec_err[dec2 as usize],
            4031 as libc::c_int,
            (&mut dec_final_range2 as *mut u32).offset(
                (&mut dec_final_range2 as *mut u32).offset_from(&mut dec_final_range2 as *mut u32)
                    as libc::c_long as isize,
            ),
        );
        if len_1 > 0 as libc::c_int && dec_final_range != dec_final_range2 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
                600 as libc::c_int,
            );
        }
        fswitch -= 1;
        if fswitch < 1 as libc::c_int {
            let mut new_size: libc::c_int = 0;
            fsize = (fsize + 1 as libc::c_int) % 36 as libc::c_int;
            new_size = fsizes[db62[fsize as usize] as usize];
            if new_size == 960 as libc::c_int || new_size == 480 as libc::c_int {
                fswitch = ((2880 as libc::c_int / new_size) as libc::c_uint).wrapping_mul(
                    (fast_rand())
                        .wrapping_rem(19 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                ) as libc::c_int;
            } else {
                fswitch = (fast_rand())
                    .wrapping_rem((2880 as libc::c_int / new_size) as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
        }
        bitrate_bps = ((fast_rand())
            .wrapping_rem(508000 as libc::c_int as libc::c_uint)
            .wrapping_add(4000 as libc::c_int as libc::c_uint)
            .wrapping_add(bitrate_bps as libc::c_uint)
            >> 1 as libc::c_int) as i32;
        i += frame_size_1;
        if i >= 48000 as libc::c_int * 30 as libc::c_int * 4 as libc::c_int {
            break;
        }
    }
    fprintf(
        stdout,
        b"    All framesize pairs switching encode, %d frames OK.\n\0" as *const u8
            as *const libc::c_char,
        count,
    );
    if opus_encoder_ctl(enc, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            616 as libc::c_int,
        );
    }
    opus_encoder_destroy(enc);
    if opus_multistream_encoder_ctl(MSenc, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int,
        );
    }
    opus_multistream_encoder_destroy(MSenc);
    if opus_decoder_ctl(dec, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            620 as libc::c_int,
        );
    }
    opus_decoder_destroy(dec);
    if opus_multistream_decoder_ctl(MSdec, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int,
        );
    }
    opus_multistream_decoder_destroy(MSdec);
    opus_multistream_decoder_destroy(MSdec_err);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        opus_decoder_destroy(dec_err[i as usize]);
        i += 1;
    }
    free(inbuf as *mut libc::c_void);
    free(outbuf as *mut libc::c_void);
    free(out2buf as *mut libc::c_void);
    0 as libc::c_int
}
#[no_mangle]
#[c2rust::src_loc = "632:1"]
pub unsafe extern "C" fn print_usage(mut _argv: *mut *mut libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s [<seed>] [-fuzz <num_encoders> <num_settings_per_encoder>]\n\0" as *const u8
            as *const libc::c_char,
        *_argv.offset(0 as libc::c_int as isize),
    );
}
#[c2rust::src_loc = "637:1"]
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut args: libc::c_int = 1 as libc::c_int;
    let mut strtol_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut oversion: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut env_seed: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut env_used: libc::c_int = 0;
    let mut num_encoders_to_fuzz: libc::c_int = 5 as libc::c_int;
    let mut num_setting_changes: libc::c_int = 40 as libc::c_int;
    env_used = 0 as libc::c_int;
    env_seed = getenv(b"SEED\0" as *const u8 as *const libc::c_char);
    if _argc > 1 as libc::c_int {
        iseed = strtol(
            *_argv.offset(1 as libc::c_int as isize),
            &mut strtol_str,
            10 as libc::c_int,
        ) as u32;
    }
    if !strtol_str.is_null()
        && *strtol_str.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        args += 1;
    } else if !env_seed.is_null() {
        iseed = atoi(env_seed) as u32;
        env_used = 1 as libc::c_int;
    } else {
        iseed = time(std::ptr::null_mut::<time_t>()) as u32
            ^ (getpid() as u32 & 65535 as libc::c_int as libc::c_uint) << 16 as libc::c_int;
    }
    Rz = iseed;
    Rw = Rz;
    while args < _argc {
        if strcmp(
            *_argv.offset(args as isize),
            b"-fuzz\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && _argc == args + 3 as libc::c_int
        {
            num_encoders_to_fuzz = strtol(
                *_argv.offset((args + 1 as libc::c_int) as isize),
                &mut strtol_str,
                10 as libc::c_int,
            ) as libc::c_int;
            if *strtol_str.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
                || num_encoders_to_fuzz <= 0 as libc::c_int
            {
                print_usage(_argv);
                return 1 as libc::c_int;
            }
            num_setting_changes = strtol(
                *_argv.offset((args + 2 as libc::c_int) as isize),
                &mut strtol_str,
                10 as libc::c_int,
            ) as libc::c_int;
            if *strtol_str.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
                || num_setting_changes <= 0 as libc::c_int
            {
                print_usage(_argv);
                return 1 as libc::c_int;
            }
            args += 3 as libc::c_int;
        } else {
            print_usage(_argv);
            return 1 as libc::c_int;
        }
    }
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const libc::c_char,
            682 as libc::c_int,
        );
    }
    fprintf(
        stderr,
        b"Testing %s encoder. Random seed: %u (%.4X)\n\0" as *const u8 as *const libc::c_char,
        oversion,
        iseed,
        (fast_rand()).wrapping_rem(65535 as libc::c_int as libc::c_uint),
    );
    if env_used != 0 {
        fprintf(
            stderr,
            b"  Random seed set from the environment (SEED=%s).\n\0" as *const u8
                as *const libc::c_char,
            env_seed,
        );
    }
    regression_test();
    run_test1(
        (getenv(b"TEST_OPUS_NOFUZZ\0" as *const u8 as *const libc::c_char)
            != std::ptr::null_mut::<libc::c_void>() as *mut libc::c_char) as libc::c_int,
    );
    if (getenv(b"TEST_OPUS_NOFUZZ\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"Running fuzz_encoder_settings with %d encoder(s) and %d setting change(s) each.\n\0"
                as *const u8 as *const libc::c_char,
            num_encoders_to_fuzz,
            num_setting_changes,
        );
        fuzz_encoder_settings(num_encoders_to_fuzz, num_setting_changes);
    }
    fprintf(
        stderr,
        b"Tests completed successfully.\n\0" as *const u8 as *const libc::c_char,
    );
    0 as libc::c_int
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
