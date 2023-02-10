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

#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:37"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "58:4"]
    pub type opus_uint64 = uint64_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::{uint32_t, uint64_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:37"]
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "178:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "258:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "356:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "675:15"]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
        #[c2rust::src_loc = "681:15"]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
        #[c2rust::src_loc = "713:1"]
        pub fn fseek(
            __stream: *mut FILE,
            __off: libc::c_long,
            __whence: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "718:1"]
        pub fn ftell(__stream: *mut FILE) -> libc::c_long;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "366:1"]
    pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
        return strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
    }
    extern "C" {
        #[c2rust::src_loc = "177:17"]
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;
        #[c2rust::src_loc = "454:1"]
        pub fn rand() -> libc::c_int;
        #[c2rust::src_loc = "553:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "556:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "568:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "861:12"]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:35"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:37"]
pub mod opus_defines_h {
    extern "C" {
        #[c2rust::src_loc = "782:1"]
        pub fn opus_strerror(error: libc::c_int) -> *const libc::c_char;
        #[c2rust::src_loc = "792:1"]
        pub fn opus_get_version_string() -> *const libc::c_char;
    }
}
use self::mathcalls_h::sqrt;
use self::opus_defines_h::{opus_get_version_string, opus_strerror};
pub use self::opus_types_h::{opus_int32, opus_uint32, opus_uint64};
pub use self::stddef_h::size_t;
use self::stdio_h::{fclose, fopen, fprintf, fread, fseek, ftell, fwrite, printf, stderr};
pub use self::stdlib_h::{abs, atoi, atol, calloc, free, malloc, rand};
use self::string_h::strcmp;
pub use self::FILE_h::FILE;
use libopus_unsafe::src::opus::opus_packet_get_samples_per_frame;
use libopus_unsafe::src::opus_decoder::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy,
    opus_packet_get_nb_frames, OpusDecoder,
};
use libopus_unsafe::src::opus_encoder::{
    opus_encode, opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, OpusEncoder,
};
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn print_usage(mut argv: *mut *mut libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s [-e] <application> <sampling rate (Hz)> <channels (1/2)> <bits per second>  [options] <input> <output>\n\0"
            as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    fprintf(
        stderr,
        b"       %s -d <sampling rate (Hz)> <channels (1/2)> [options] <input> <output>\n\n\0"
            as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    fprintf(
        stderr,
        b"application: voip | audio | restricted-lowdelay\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"options:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"-e                   : only runs the encoder (output the bit-stream)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-d                   : only runs the decoder (reads the bit-stream as input)\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-cbr                 : enable constant bitrate; default: variable bitrate\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-cvbr                : enable constrained variable bitrate; default: unconstrained\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-delayed-decision    : use look-ahead for speech/music detection (experts only); default: disabled\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-bandwidth <NB|MB|WB|SWB|FB> : audio bandwidth (from narrowband to fullband); default: sampling rate\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-framesize <2.5|5|10|20|40|60|80|100|120> : frame size in ms; default: 20 \n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-max_payload <bytes> : maximum payload size in bytes, default: 1024\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-complexity <comp>   : complexity, 0 (lowest) ... 10 (highest); default: 10\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-inbandfec           : enable SILK inband FEC\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-forcemono           : force mono encoding, even for stereo input\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-dtx                 : enable SILK DTX\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"-loss <perc>         : simulate packet loss, in percent (0-100); default: 0\n\0"
            as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn int_to_char(mut i: opus_uint32, mut ch: *mut libc::c_uchar) {
    *ch.offset(0 as libc::c_int as isize) = (i >> 24 as libc::c_int) as libc::c_uchar;
    *ch.offset(1 as libc::c_int as isize) =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *ch.offset(2 as libc::c_int as isize) =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *ch.offset(3 as libc::c_int as isize) =
        (i & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn char_to_int(mut ch: *mut libc::c_uchar) -> opus_uint32 {
    return (*ch.offset(0 as libc::c_int as isize) as opus_uint32) << 24 as libc::c_int
        | (*ch.offset(1 as libc::c_int as isize) as opus_uint32) << 16 as libc::c_int
        | (*ch.offset(2 as libc::c_int as isize) as opus_uint32) << 8 as libc::c_int
        | *ch.offset(3 as libc::c_int as isize) as opus_uint32;
}
#[c2rust::src_loc = "84:18"]
static mut silk8_test: [[libc::c_int; 4]; 8] = [
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int * 3 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int * 3 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1101 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "95:18"]
static mut silk12_test: [[libc::c_int; 4]; 8] = [
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        960 as libc::c_int * 3 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        960 as libc::c_int * 3 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1102 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "106:18"]
static mut silk16_test: [[libc::c_int; 4]; 8] = [
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int * 3 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int * 3 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int * 2 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1103 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "117:18"]
static mut hybrid24_test: [[libc::c_int; 4]; 4] = [
    [
        1000 as libc::c_int,
        1104 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1104 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1104 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1104 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "124:18"]
static mut hybrid48_test: [[libc::c_int; 4]; 4] = [
    [
        1000 as libc::c_int,
        1105 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1105 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1105 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1000 as libc::c_int,
        1105 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "131:18"]
static mut celt_test: [[libc::c_int; 4]; 32] = [
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        480 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        240 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        240 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        240 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        240 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        120 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        120 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        120 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        120 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        240 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        240 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        240 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        240 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        120 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1104 as libc::c_int,
        120 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1103 as libc::c_int,
        120 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1101 as libc::c_int,
        120 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "174:18"]
static mut celt_hq_test: [[libc::c_int; 4]; 4] = [
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        960 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        480 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        240 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        1002 as libc::c_int,
        1105 as libc::c_int,
        120 as libc::c_int,
        2 as libc::c_int,
    ],
];
#[c2rust::src_loc = "210:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut err: libc::c_int = 0;
    let mut inFile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outFile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fin: *mut FILE = 0 as *mut FILE;
    let mut fout: *mut FILE = 0 as *mut FILE;
    let mut enc: *mut OpusEncoder = 0 as *mut OpusEncoder;
    let mut dec: *mut OpusDecoder = 0 as *mut OpusDecoder;
    let mut args: libc::c_int = 0;
    let mut len: [libc::c_int; 2] = [0; 2];
    let mut frame_size: libc::c_int = 0;
    let mut channels: libc::c_int = 0;
    let mut bitrate_bps: opus_int32 = 0 as libc::c_int;
    let mut data: [*mut libc::c_uchar; 2] = [0 as *mut libc::c_uchar, 0 as *mut libc::c_uchar];
    let mut fbytes: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sampling_rate: opus_int32 = 0;
    let mut use_vbr: libc::c_int = 0;
    let mut max_payload_bytes: libc::c_int = 0;
    let mut complexity: libc::c_int = 0;
    let mut use_inbandfec: libc::c_int = 0;
    let mut use_dtx: libc::c_int = 0;
    let mut forcechannels: libc::c_int = 0;
    let mut cvbr: libc::c_int = 0 as libc::c_int;
    let mut packet_loss_perc: libc::c_int = 0;
    let mut count: opus_int32 = 0 as libc::c_int;
    let mut count_act: opus_int32 = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut skip: opus_int32 = 0 as libc::c_int;
    let mut stop: libc::c_int = 0 as libc::c_int;
    let mut in_0: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut out: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut application: libc::c_int = 2049 as libc::c_int;
    let mut bits: libc::c_double = 0.0f64;
    let mut bits_max: libc::c_double = 0.0f64;
    let mut bits_act: libc::c_double = 0.0f64;
    let mut bits2: libc::c_double = 0.0f64;
    let mut nrg: libc::c_double = 0.;
    let mut tot_samples: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tot_in: opus_uint64 = 0;
    let mut tot_out: opus_uint64 = 0;
    let mut bandwidth: libc::c_int = -(1000 as libc::c_int);
    let mut bandwidth_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut lost: libc::c_int = 0 as libc::c_int;
    let mut lost_prev: libc::c_int = 1 as libc::c_int;
    let mut toggle: libc::c_int = 0 as libc::c_int;
    let mut enc_final_range: [opus_uint32; 2] = [0; 2];
    let mut dec_final_range: opus_uint32 = 0;
    let mut encode_only: libc::c_int = 0 as libc::c_int;
    let mut decode_only: libc::c_int = 0 as libc::c_int;
    let mut max_frame_size: libc::c_int = 48000 as libc::c_int * 2 as libc::c_int;
    let mut num_read: size_t = 0;
    let mut curr_read: libc::c_int = 0 as libc::c_int;
    let mut sweep_bps: libc::c_int = 0 as libc::c_int;
    let mut random_framesize: libc::c_int = 0 as libc::c_int;
    let mut newsize: libc::c_int = 0 as libc::c_int;
    let mut delayed_celt: libc::c_int = 0 as libc::c_int;
    let mut sweep_max: libc::c_int = 0 as libc::c_int;
    let mut sweep_min: libc::c_int = 0 as libc::c_int;
    let mut random_fec: libc::c_int = 0 as libc::c_int;
    let mut mode_list: *const [libc::c_int; 4] = 0 as *const [libc::c_int; 4];
    let mut nb_modes_in_list: libc::c_int = 0 as libc::c_int;
    let mut curr_mode: libc::c_int = 0 as libc::c_int;
    let mut curr_mode_count: libc::c_int = 0 as libc::c_int;
    let mut mode_switch_time: libc::c_int = 48000 as libc::c_int;
    let mut nb_encoded: libc::c_int = 0 as libc::c_int;
    let mut remaining: libc::c_int = 0 as libc::c_int;
    let mut variable_duration: libc::c_int = 5000 as libc::c_int;
    let mut delayed_decision: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 1 as libc::c_int;
    if argc < 5 as libc::c_int {
        print_usage(argv);
    } else {
        tot_out = 0 as libc::c_int as opus_uint64;
        tot_in = tot_out;
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            opus_get_version_string(),
        );
        args = 1 as libc::c_int;
        if strcmp(
            *argv.offset(args as isize),
            b"-e\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            encode_only = 1 as libc::c_int;
            args += 1;
        } else if strcmp(
            *argv.offset(args as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            decode_only = 1 as libc::c_int;
            args += 1;
        }
        if decode_only == 0 && argc < 7 as libc::c_int {
            print_usage(argv);
        } else {
            if decode_only == 0 {
                if strcmp(
                    *argv.offset(args as isize),
                    b"voip\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    application = 2048 as libc::c_int;
                    current_block = 6450597802325118133;
                } else if strcmp(
                    *argv.offset(args as isize),
                    b"restricted-lowdelay\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    application = 2051 as libc::c_int;
                    current_block = 6450597802325118133;
                } else if strcmp(
                    *argv.offset(args as isize),
                    b"audio\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"unknown application: %s\n\0" as *const u8 as *const libc::c_char,
                        *argv.offset(args as isize),
                    );
                    print_usage(argv);
                    current_block = 14460699602747363466;
                } else {
                    current_block = 6450597802325118133;
                }
                match current_block {
                    14460699602747363466 => {}
                    _ => {
                        args += 1;
                        current_block = 18435049525520518667;
                    }
                }
            } else {
                current_block = 18435049525520518667;
            }
            match current_block {
                14460699602747363466 => {}
                _ => {
                    sampling_rate = atol(*argv.offset(args as isize)) as opus_int32;
                    args += 1;
                    if sampling_rate != 8000 as libc::c_int
                        && sampling_rate != 12000 as libc::c_int
                        && sampling_rate != 16000 as libc::c_int
                        && sampling_rate != 24000 as libc::c_int
                        && sampling_rate != 48000 as libc::c_int
                    {
                        fprintf(
                            stderr,
                            b"Supported sampling rates are 8000, 12000, 16000, 24000 and 48000.\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else {
                        frame_size = sampling_rate / 50 as libc::c_int;
                        channels = atoi(*argv.offset(args as isize));
                        args += 1;
                        if channels < 1 as libc::c_int || channels > 2 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Opus_demo supports only 1 or 2 channels.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            if decode_only == 0 {
                                bitrate_bps = atol(*argv.offset(args as isize)) as opus_int32;
                                args += 1;
                            }
                            use_vbr = 1 as libc::c_int;
                            max_payload_bytes = 1500 as libc::c_int;
                            complexity = 10 as libc::c_int;
                            use_inbandfec = 0 as libc::c_int;
                            forcechannels = -(1000 as libc::c_int);
                            use_dtx = 0 as libc::c_int;
                            packet_loss_perc = 0 as libc::c_int;
                            loop {
                                if !(args < argc - 2 as libc::c_int) {
                                    current_block = 10024259685434459487;
                                    break;
                                }
                                if strcmp(
                                    *argv.offset(args as isize),
                                    b"-cbr\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-cbr\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        use_vbr = 0 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-bandwidth\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-bandwidth\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"NB\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            bandwidth = 1101 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"MB\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            bandwidth = 1102 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"WB\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            bandwidth = 1103 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"SWB\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            bandwidth = 1104 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"FB\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            bandwidth = 1105 as libc::c_int;
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"Unknown bandwidth %s. Supported are NB, MB, WB, SWB, FB.\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                *argv.offset((args + 1 as libc::c_int) as isize),
                                            );
                                            current_block = 14460699602747363466;
                                            break;
                                        }
                                        args += 2 as libc::c_int;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-framesize\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-framesize\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"2.5\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = sampling_rate / 400 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"5\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = sampling_rate / 200 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"10\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = sampling_rate / 100 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"20\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = sampling_rate / 50 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"40\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = sampling_rate / 25 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"60\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = 3 as libc::c_int * sampling_rate
                                                / 50 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"80\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = 4 as libc::c_int * sampling_rate
                                                / 50 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"100\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = 5 as libc::c_int * sampling_rate
                                                / 50 as libc::c_int;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as libc::c_int) as isize),
                                            b"120\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            frame_size = 6 as libc::c_int * sampling_rate
                                                / 50 as libc::c_int;
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"Unsupported frame size: %s ms. Supported are 2.5, 5, 10, 20, 40, 60, 80, 100, 120.\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                *argv.offset((args + 1 as libc::c_int) as isize),
                                            );
                                            current_block = 14460699602747363466;
                                            break;
                                        }
                                        args += 2 as libc::c_int;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-max_payload\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-max_payload\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        max_payload_bytes =
                                            atoi(*argv.offset((args + 1 as libc::c_int) as isize));
                                        args += 2 as libc::c_int;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-complexity\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-complexity\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        complexity =
                                            atoi(*argv.offset((args + 1 as libc::c_int) as isize));
                                        args += 2 as libc::c_int;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-inbandfec\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    use_inbandfec = 1 as libc::c_int;
                                    args += 1;
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-forcemono\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-forcemono\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        forcechannels = 1 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-cvbr\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-cvbr\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        cvbr = 1 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-delayed-decision\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-delayed-decision\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        delayed_decision = 1 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-dtx\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-dtx\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        use_dtx = 1 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-loss\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    packet_loss_perc =
                                        atoi(*argv.offset((args + 1 as libc::c_int) as isize));
                                    args += 2 as libc::c_int;
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-sweep\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-sweep\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        sweep_bps =
                                            atoi(*argv.offset((args + 1 as libc::c_int) as isize));
                                        args += 2 as libc::c_int;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-random_framesize\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-random_framesize\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        random_framesize = 1 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-sweep_max\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-sweep_max\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        sweep_max =
                                            atoi(*argv.offset((args + 1 as libc::c_int) as isize));
                                        args += 2 as libc::c_int;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-random_fec\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-random_fec\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        random_fec = 1 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-silk8k_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-silk8k_test\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = silk8_test.as_ptr();
                                        nb_modes_in_list = 8 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-silk12k_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-silk12k_test\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = silk12_test.as_ptr();
                                        nb_modes_in_list = 8 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-silk16k_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-silk16k_test\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = silk16_test.as_ptr();
                                        nb_modes_in_list = 8 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-hybrid24k_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-hybrid24k_test\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = hybrid24_test.as_ptr();
                                        nb_modes_in_list = 4 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-hybrid48k_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-hybrid48k_test\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = hybrid48_test.as_ptr();
                                        nb_modes_in_list = 4 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-celt_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-celt_test\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = celt_test.as_ptr();
                                        nb_modes_in_list = 32 as libc::c_int;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-celt_hq_test\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr,
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const libc::c_char,
                                            b"-celt_hq_test\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = celt_hq_test.as_ptr();
                                        nb_modes_in_list = 4 as libc::c_int;
                                        args += 1;
                                    }
                                } else {
                                    printf(
                                        b"Error: unrecognized setting: %s\n\n\0" as *const u8
                                            as *const libc::c_char,
                                        *argv.offset(args as isize),
                                    );
                                    print_usage(argv);
                                    current_block = 14460699602747363466;
                                    break;
                                }
                            }
                            match current_block {
                                14460699602747363466 => {}
                                _ => {
                                    if sweep_max != 0 {
                                        sweep_min = bitrate_bps;
                                    }
                                    if max_payload_bytes < 0 as libc::c_int
                                        || max_payload_bytes > 1500 as libc::c_int
                                    {
                                        fprintf(
                                            stderr,
                                            b"max_payload_bytes must be between 0 and %d\n\0"
                                                as *const u8
                                                as *const libc::c_char,
                                            1500 as libc::c_int,
                                        );
                                    } else {
                                        inFile = *argv.offset((argc - 2 as libc::c_int) as isize);
                                        fin = fopen(
                                            inFile,
                                            b"rb\0" as *const u8 as *const libc::c_char,
                                        );
                                        if fin.is_null() {
                                            fprintf(
                                                stderr,
                                                b"Could not open input file %s\n\0" as *const u8
                                                    as *const libc::c_char,
                                                *argv.offset((argc - 2 as libc::c_int) as isize),
                                            );
                                        } else {
                                            if !mode_list.is_null() {
                                                let mut size: libc::c_int = 0;
                                                fseek(
                                                    fin,
                                                    0 as libc::c_int as libc::c_long,
                                                    2 as libc::c_int,
                                                );
                                                size = ftell(fin) as libc::c_int;
                                                fprintf(
                                                    stderr,
                                                    b"File size is %d bytes\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    size,
                                                );
                                                fseek(
                                                    fin,
                                                    0 as libc::c_int as libc::c_long,
                                                    0 as libc::c_int,
                                                );
                                                mode_switch_time = (size as libc::c_ulong)
                                                    .wrapping_div(::core::mem::size_of::<
                                                        libc::c_short,
                                                    >(
                                                    )
                                                        as libc::c_ulong)
                                                    .wrapping_div(channels as libc::c_ulong)
                                                    .wrapping_div(nb_modes_in_list as libc::c_ulong)
                                                    as libc::c_int;
                                                fprintf(
                                                    stderr,
                                                    b"Switching mode every %d samples\n\0"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    mode_switch_time,
                                                );
                                            }
                                            outFile =
                                                *argv.offset((argc - 1 as libc::c_int) as isize);
                                            fout = fopen(
                                                outFile,
                                                b"wb+\0" as *const u8 as *const libc::c_char,
                                            );
                                            if fout.is_null() {
                                                fprintf(
                                                    stderr,
                                                    b"Could not open output file %s\n\0"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    *argv
                                                        .offset((argc - 1 as libc::c_int) as isize),
                                                );
                                            } else {
                                                if decode_only == 0 {
                                                    enc = opus_encoder_create(
                                                        sampling_rate,
                                                        channels,
                                                        application,
                                                        &mut err,
                                                    );
                                                    if err != 0 as libc::c_int {
                                                        fprintf(
                                                            stderr,
                                                            b"Cannot create encoder: %s\n\0"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            opus_strerror(err),
                                                        );
                                                        current_block = 14460699602747363466;
                                                    } else {
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4002 as libc::c_int,
                                                            bitrate_bps,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4008 as libc::c_int,
                                                            bandwidth,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4006 as libc::c_int,
                                                            use_vbr,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4020 as libc::c_int,
                                                            cvbr,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4010 as libc::c_int,
                                                            complexity,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4012 as libc::c_int,
                                                            use_inbandfec,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4022 as libc::c_int,
                                                            forcechannels,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4016 as libc::c_int,
                                                            use_dtx,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4014 as libc::c_int,
                                                            packet_loss_perc,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4027 as libc::c_int,
                                                            (&mut skip as *mut opus_int32).offset(
                                                                (&mut skip as *mut opus_int32)
                                                                    .offset_from(
                                                                        &mut skip
                                                                            as *mut opus_int32,
                                                                    )
                                                                    as libc::c_long
                                                                    as isize,
                                                            ),
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4036 as libc::c_int,
                                                            16 as libc::c_int,
                                                        );
                                                        opus_encoder_ctl(
                                                            enc,
                                                            4040 as libc::c_int,
                                                            variable_duration,
                                                        );
                                                        current_block = 4253140123906974056;
                                                    }
                                                } else {
                                                    current_block = 4253140123906974056;
                                                }
                                                match current_block {
                                                    14460699602747363466 => {}
                                                    _ => {
                                                        if encode_only == 0 {
                                                            dec = opus_decoder_create(
                                                                sampling_rate,
                                                                channels,
                                                                &mut err,
                                                            );
                                                            if err != 0 as libc::c_int {
                                                                fprintf(
                                                                    stderr,
                                                                    b"Cannot create decoder: %s\n\0"
                                                                        as *const u8
                                                                        as *const libc::c_char,
                                                                    opus_strerror(err),
                                                                );
                                                                current_block =
                                                                    14460699602747363466;
                                                            } else {
                                                                current_block =
                                                                    13095187161273680990;
                                                            }
                                                        } else {
                                                            current_block = 13095187161273680990;
                                                        }
                                                        match current_block {
                                                            14460699602747363466 => {}
                                                            _ => {
                                                                match bandwidth {
                                                                    1101 => {
                                                                        bandwidth_string = b"narrowband\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                    1102 => {
                                                                        bandwidth_string = b"mediumband\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                    1103 => {
                                                                        bandwidth_string = b"wideband\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                    1104 => {
                                                                        bandwidth_string = b"superwideband\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                    1105 => {
                                                                        bandwidth_string = b"fullband\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                    -1000 => {
                                                                        bandwidth_string = b"auto bandwidth\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                    _ => {
                                                                        bandwidth_string = b"unknown\0" as *const u8
                                                                            as *const libc::c_char;
                                                                    }
                                                                }
                                                                if decode_only != 0 {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"Decoding with %ld Hz output (%d channels)\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        sampling_rate as libc::c_long,
                                                                        channels,
                                                                    );
                                                                } else {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"Encoding %ld Hz input at %.3f kb/s in %s with %d-sample frames.\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        sampling_rate as libc::c_long,
                                                                        bitrate_bps as libc::c_double * 0.001f64,
                                                                        bandwidth_string,
                                                                        frame_size,
                                                                    );
                                                                }
                                                                in_0 = malloc(
                                                                    ((max_frame_size * channels)
                                                                        as libc::c_ulong)
                                                                        .wrapping_mul(
                                                                            ::core::mem::size_of::<
                                                                                libc::c_short,
                                                                            >(
                                                                            )
                                                                                as libc::c_ulong,
                                                                        ),
                                                                )
                                                                    as *mut libc::c_short;
                                                                out = malloc(
                                                                    ((max_frame_size * channels)
                                                                        as libc::c_ulong)
                                                                        .wrapping_mul(
                                                                            ::core::mem::size_of::<
                                                                                libc::c_short,
                                                                            >(
                                                                            )
                                                                                as libc::c_ulong,
                                                                        ),
                                                                )
                                                                    as *mut libc::c_short;
                                                                fbytes = malloc(
                                                                    ((max_frame_size * channels)
                                                                        as libc::c_ulong)
                                                                        .wrapping_mul(
                                                                            ::core::mem::size_of::<
                                                                                libc::c_short,
                                                                            >(
                                                                            )
                                                                                as libc::c_ulong,
                                                                        ),
                                                                )
                                                                    as *mut libc::c_uchar;
                                                                data[0 as libc::c_int as usize] =
                                                                    calloc(
                                                                        max_payload_bytes
                                                                            as libc::c_ulong,
                                                                        ::core::mem::size_of::<
                                                                            libc::c_uchar,
                                                                        >(
                                                                        )
                                                                            as libc::c_ulong,
                                                                    )
                                                                        as *mut libc::c_uchar;
                                                                if use_inbandfec != 0 {
                                                                    data[1 as libc::c_int
                                                                        as usize] = calloc(
                                                                        max_payload_bytes
                                                                            as libc::c_ulong,
                                                                        ::core::mem::size_of::<
                                                                            libc::c_uchar,
                                                                        >(
                                                                        )
                                                                            as libc::c_ulong,
                                                                    )
                                                                        as *mut libc::c_uchar;
                                                                }
                                                                if delayed_decision != 0 {
                                                                    if frame_size
                                                                        == sampling_rate
                                                                            / 400 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5001 as libc::c_int;
                                                                    } else if frame_size
                                                                        == sampling_rate
                                                                            / 200 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5002 as libc::c_int;
                                                                    } else if frame_size
                                                                        == sampling_rate
                                                                            / 100 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5003 as libc::c_int;
                                                                    } else if frame_size
                                                                        == sampling_rate
                                                                            / 50 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5004 as libc::c_int;
                                                                    } else if frame_size
                                                                        == sampling_rate
                                                                            / 25 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5005 as libc::c_int;
                                                                    } else if frame_size
                                                                        == 3 as libc::c_int
                                                                            * sampling_rate
                                                                            / 50 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5006 as libc::c_int;
                                                                    } else if frame_size
                                                                        == 4 as libc::c_int
                                                                            * sampling_rate
                                                                            / 50 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5007 as libc::c_int;
                                                                    } else if frame_size
                                                                        == 5 as libc::c_int
                                                                            * sampling_rate
                                                                            / 50 as libc::c_int
                                                                    {
                                                                        variable_duration =
                                                                            5008 as libc::c_int;
                                                                    } else {
                                                                        variable_duration =
                                                                            5009 as libc::c_int;
                                                                    }
                                                                    opus_encoder_ctl(
                                                                        enc,
                                                                        4040 as libc::c_int,
                                                                        variable_duration,
                                                                    );
                                                                    frame_size = 2 as libc::c_int
                                                                        * 48000 as libc::c_int;
                                                                }
                                                                loop {
                                                                    if !(stop == 0) {
                                                                        current_block =
                                                                            15240930316249348783;
                                                                        break;
                                                                    }
                                                                    if delayed_celt != 0 {
                                                                        frame_size = newsize;
                                                                        delayed_celt =
                                                                            0 as libc::c_int;
                                                                    } else if random_framesize != 0
                                                                        && rand()
                                                                            % 20 as libc::c_int
                                                                            == 0 as libc::c_int
                                                                    {
                                                                        newsize = rand()
                                                                            % 6 as libc::c_int;
                                                                        match newsize {
                                                                            0 => {
                                                                                newsize = sampling_rate / 400 as libc::c_int;
                                                                            }
                                                                            1 => {
                                                                                newsize = sampling_rate / 200 as libc::c_int;
                                                                            }
                                                                            2 => {
                                                                                newsize = sampling_rate / 100 as libc::c_int;
                                                                            }
                                                                            3 => {
                                                                                newsize = sampling_rate / 50 as libc::c_int;
                                                                            }
                                                                            4 => {
                                                                                newsize = sampling_rate / 25 as libc::c_int;
                                                                            }
                                                                            5 => {
                                                                                newsize = 3 as libc::c_int * sampling_rate
                                                                                    / 50 as libc::c_int;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        while newsize < sampling_rate / 25 as libc::c_int
                                                                            && bitrate_bps - abs(sweep_bps)
                                                                                <= 3 as libc::c_int * 12 as libc::c_int * sampling_rate
                                                                                    / newsize
                                                                        {
                                                                            newsize *= 2 as libc::c_int;
                                                                        }
                                                                        if newsize < sampling_rate / 100 as libc::c_int
                                                                            && frame_size >= sampling_rate / 100 as libc::c_int
                                                                        {
                                                                            opus_encoder_ctl(
                                                                                enc,
                                                                                11002 as libc::c_int,
                                                                                1002 as libc::c_int,
                                                                            );
                                                                            delayed_celt = 1 as libc::c_int;
                                                                        } else {
                                                                            frame_size = newsize;
                                                                        }
                                                                    }
                                                                    if random_fec != 0
                                                                        && rand()
                                                                            % 30 as libc::c_int
                                                                            == 0 as libc::c_int
                                                                    {
                                                                        rand();
                                                                        opus_encoder_ctl(
                                                                            enc,
                                                                            4012 as libc::c_int,
                                                                            (rand()
                                                                                % 4 as libc::c_int
                                                                                == 0 as libc::c_int)
                                                                                as libc::c_int,
                                                                        );
                                                                    }
                                                                    if decode_only != 0 {
                                                                        let mut ch: [libc::c_uchar;
                                                                            4] = [0; 4];
                                                                        num_read = fread(
                                                                            ch.as_mut_ptr() as *mut libc::c_void,
                                                                            1 as libc::c_int as libc::c_ulong,
                                                                            4 as libc::c_int as libc::c_ulong,
                                                                            fin,
                                                                        );
                                                                        if num_read
                                                                            != 4 as libc::c_int
                                                                                as libc::c_ulong
                                                                        {
                                                                            current_block = 15240930316249348783;
                                                                            break;
                                                                        }
                                                                        len[toggle as usize] =
                                                                            char_to_int(
                                                                                ch.as_mut_ptr(),
                                                                            )
                                                                                as libc::c_int;
                                                                        if len[toggle as usize]
                                                                            > max_payload_bytes
                                                                            || len[toggle as usize]
                                                                                < 0 as libc::c_int
                                                                        {
                                                                            fprintf(
                                                                                stderr,
                                                                                b"Invalid payload length: %d\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                len[toggle as usize],
                                                                            );
                                                                            current_block = 15240930316249348783;
                                                                            break;
                                                                        } else {
                                                                            num_read = fread(
                                                                                ch.as_mut_ptr() as *mut libc::c_void,
                                                                                1 as libc::c_int as libc::c_ulong,
                                                                                4 as libc::c_int as libc::c_ulong,
                                                                                fin,
                                                                            );
                                                                            if num_read
                                                                                != 4 as libc::c_int
                                                                                    as libc::c_ulong
                                                                            {
                                                                                current_block = 15240930316249348783;
                                                                                break;
                                                                            }
                                                                            enc_final_range
                                                                                [toggle as usize] =
                                                                                char_to_int(
                                                                                    ch.as_mut_ptr(),
                                                                                );
                                                                            num_read = fread(
                                                                                data[toggle as usize] as *mut libc::c_void,
                                                                                1 as libc::c_int as libc::c_ulong,
                                                                                len[toggle as usize] as libc::c_ulong,
                                                                                fin,
                                                                            );
                                                                            if num_read
                                                                                != len[toggle
                                                                                    as usize]
                                                                                    as size_t
                                                                            {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"Ran out of input, expecting %d bytes got %d\n\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                    len[toggle as usize],
                                                                                    num_read as libc::c_int,
                                                                                );
                                                                                current_block = 15240930316249348783;
                                                                                break;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let mut i: libc::c_int = 0;
                                                                        if !mode_list.is_null() {
                                                                            opus_encoder_ctl(
                                                                                enc,
                                                                                4008 as libc::c_int,
                                                                                (*mode_list
                                                                                    .offset(curr_mode as isize))[1 as libc::c_int as usize],
                                                                            );
                                                                            opus_encoder_ctl(
                                                                                enc,
                                                                                11002 as libc::c_int,
                                                                                (*mode_list
                                                                                    .offset(curr_mode as isize))[0 as libc::c_int as usize],
                                                                            );
                                                                            opus_encoder_ctl(
                                                                                enc,
                                                                                4022 as libc::c_int,
                                                                                (*mode_list
                                                                                    .offset(curr_mode as isize))[3 as libc::c_int as usize],
                                                                            );
                                                                            frame_size = (*mode_list
                                                                                .offset(curr_mode as isize))[2 as libc::c_int as usize];
                                                                        }
                                                                        num_read = fread(
                                                                            fbytes as *mut libc::c_void,
                                                                            (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                                                                .wrapping_mul(channels as libc::c_ulong),
                                                                            (frame_size - remaining) as libc::c_ulong,
                                                                            fin,
                                                                        );
                                                                        curr_read =
                                                                            num_read as libc::c_int;
                                                                        tot_in = (tot_in as libc::c_ulong)
                                                                            .wrapping_add(curr_read as libc::c_ulong) as opus_uint64
                                                                            as opus_uint64;
                                                                        i = 0 as libc::c_int;
                                                                        while i < curr_read
                                                                            * channels
                                                                        {
                                                                            let mut s: opus_int32 =
                                                                                0;
                                                                            s = (*fbytes
                                                                                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                                                                                as libc::c_int) << 8 as libc::c_int
                                                                                | *fbytes.offset((2 as libc::c_int * i) as isize)
                                                                                    as libc::c_int;
                                                                            s = (s & 0xffff
                                                                                as libc::c_int
                                                                                ^ 0x8000
                                                                                    as libc::c_int)
                                                                                - 0x8000
                                                                                    as libc::c_int;
                                                                            *in_0.offset(
                                                                                (i + remaining
                                                                                    * channels)
                                                                                    as isize,
                                                                            ) = s as libc::c_short;
                                                                            i += 1;
                                                                        }
                                                                        if curr_read + remaining
                                                                            < frame_size
                                                                        {
                                                                            i = (curr_read
                                                                                + remaining)
                                                                                * channels;
                                                                            while i < frame_size
                                                                                * channels
                                                                            {
                                                                                *in_0
                                                                                    .offset(i as isize) = 0 as libc::c_int as libc::c_short;
                                                                                i += 1;
                                                                            }
                                                                            if encode_only != 0
                                                                                || decode_only != 0
                                                                            {
                                                                                stop = 1
                                                                                    as libc::c_int;
                                                                            }
                                                                        }
                                                                        len[toggle as usize] =
                                                                            opus_encode(
                                                                                enc,
                                                                                in_0,
                                                                                frame_size,
                                                                                data[toggle
                                                                                    as usize],
                                                                                max_payload_bytes,
                                                                            );
                                                                        nb_encoded = opus_packet_get_samples_per_frame(
                                                                            data[toggle as usize],
                                                                            sampling_rate,
                                                                        )
                                                                            * opus_packet_get_nb_frames(
                                                                                data[toggle as usize] as *const libc::c_uchar,
                                                                                len[toggle as usize],
                                                                            );
                                                                        remaining =
                                                                            frame_size - nb_encoded;
                                                                        i = 0 as libc::c_int;
                                                                        while i < remaining
                                                                            * channels
                                                                        {
                                                                            *in_0.offset(
                                                                                i as isize,
                                                                            ) = *in_0.offset(
                                                                                (nb_encoded
                                                                                    * channels
                                                                                    + i)
                                                                                    as isize,
                                                                            );
                                                                            i += 1;
                                                                        }
                                                                        if sweep_bps
                                                                            != 0 as libc::c_int
                                                                        {
                                                                            bitrate_bps +=
                                                                                sweep_bps;
                                                                            if sweep_max != 0 {
                                                                                if bitrate_bps > sweep_max {
                                                                                    sweep_bps = -sweep_bps;
                                                                                } else if bitrate_bps < sweep_min {
                                                                                    sweep_bps = -sweep_bps;
                                                                                }
                                                                            }
                                                                            if bitrate_bps
                                                                                < 1000
                                                                                    as libc::c_int
                                                                            {
                                                                                bitrate_bps = 1000
                                                                                    as libc::c_int;
                                                                            }
                                                                            opus_encoder_ctl(
                                                                                enc,
                                                                                4002 as libc::c_int,
                                                                                bitrate_bps,
                                                                            );
                                                                        }
                                                                        opus_encoder_ctl(
                                                                            enc,
                                                                            4031 as libc::c_int,
                                                                            (&mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
                                                                                as *mut opus_uint32)
                                                                                .offset(
                                                                                    (&mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
                                                                                        as *mut opus_uint32)
                                                                                        .offset_from(
                                                                                            &mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
                                                                                                as *mut opus_uint32,
                                                                                        ) as libc::c_long as isize,
                                                                                ),
                                                                        );
                                                                        if len[toggle as usize]
                                                                            < 0 as libc::c_int
                                                                        {
                                                                            fprintf(
                                                                                stderr,
                                                                                b"opus_encode() returned %d\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                len[toggle as usize],
                                                                            );
                                                                            current_block = 14460699602747363466;
                                                                            break;
                                                                        } else {
                                                                            curr_mode_count +=
                                                                                frame_size;
                                                                            if curr_mode_count > mode_switch_time
                                                                                && curr_mode < nb_modes_in_list - 1 as libc::c_int
                                                                            {
                                                                                curr_mode += 1;
                                                                                curr_mode_count = 0 as libc::c_int;
                                                                            }
                                                                        }
                                                                    }
                                                                    if encode_only != 0 {
                                                                        let mut int_field: [libc::c_uchar; 4] = [0; 4];
                                                                        int_to_char(
                                                                            len[toggle as usize]
                                                                                as opus_uint32,
                                                                            int_field.as_mut_ptr(),
                                                                        );
                                                                        if fwrite(
                                                                            int_field.as_mut_ptr() as *const libc::c_void,
                                                                            1 as libc::c_int as libc::c_ulong,
                                                                            4 as libc::c_int as libc::c_ulong,
                                                                            fout,
                                                                        ) != 4 as libc::c_int as libc::c_ulong
                                                                        {
                                                                            fprintf(
                                                                                stderr,
                                                                                b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            current_block = 14460699602747363466;
                                                                            break;
                                                                        } else {
                                                                            int_to_char(
                                                                                enc_final_range[toggle as usize],
                                                                                int_field.as_mut_ptr(),
                                                                            );
                                                                            if fwrite(
                                                                                int_field.as_mut_ptr() as *const libc::c_void,
                                                                                1 as libc::c_int as libc::c_ulong,
                                                                                4 as libc::c_int as libc::c_ulong,
                                                                                fout,
                                                                            ) != 4 as libc::c_int as libc::c_ulong
                                                                            {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                                                                                );
                                                                                current_block = 14460699602747363466;
                                                                                break;
                                                                            } else if fwrite(
                                                                                data[toggle as usize] as *const libc::c_void,
                                                                                1 as libc::c_int as libc::c_ulong,
                                                                                len[toggle as usize] as libc::c_ulong,
                                                                                fout,
                                                                            ) != len[toggle as usize] as libc::c_uint as libc::c_ulong
                                                                            {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                                                                                );
                                                                                current_block = 14460699602747363466;
                                                                                break;
                                                                            } else {
                                                                                tot_samples += nb_encoded as libc::c_double;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let mut output_samples: opus_int32 = 0;
                                                                        lost = (len[toggle as usize] == 0 as libc::c_int
                                                                            || packet_loss_perc > 0 as libc::c_int
                                                                                && (rand() % 100 as libc::c_int) < packet_loss_perc)
                                                                            as libc::c_int;
                                                                        if lost != 0 {
                                                                            opus_decoder_ctl(
                                                                                dec,
                                                                                4039 as libc::c_int,
                                                                                (&mut output_samples as *mut opus_int32)
                                                                                    .offset(
                                                                                        (&mut output_samples as *mut opus_int32)
                                                                                            .offset_from(&mut output_samples as *mut opus_int32)
                                                                                            as libc::c_long as isize,
                                                                                    ),
                                                                            );
                                                                        } else {
                                                                            output_samples =
                                                                                max_frame_size;
                                                                        }
                                                                        if count >= use_inbandfec {
                                                                            if use_inbandfec != 0 {
                                                                                if lost_prev != 0 {
                                                                                    opus_decoder_ctl(
                                                                                        dec,
                                                                                        4039 as libc::c_int,
                                                                                        (&mut output_samples as *mut opus_int32)
                                                                                            .offset(
                                                                                                (&mut output_samples as *mut opus_int32)
                                                                                                    .offset_from(&mut output_samples as *mut opus_int32)
                                                                                                    as libc::c_long as isize,
                                                                                            ),
                                                                                    );
                                                                                    output_samples = opus_decode(
                                                                                        dec,
                                                                                        if lost != 0 {
                                                                                            0 as *mut libc::c_uchar
                                                                                        } else {
                                                                                            data[toggle as usize]
                                                                                        },
                                                                                        len[toggle as usize],
                                                                                        out,
                                                                                        output_samples,
                                                                                        1 as libc::c_int,
                                                                                    );
                                                                                } else {
                                                                                    output_samples = max_frame_size;
                                                                                    output_samples = opus_decode(
                                                                                        dec,
                                                                                        data[(1 as libc::c_int - toggle) as usize],
                                                                                        len[(1 as libc::c_int - toggle) as usize],
                                                                                        out,
                                                                                        output_samples,
                                                                                        0 as libc::c_int,
                                                                                    );
                                                                                }
                                                                            } else {
                                                                                output_samples = opus_decode(
                                                                                    dec,
                                                                                    if lost != 0 {
                                                                                        0 as *mut libc::c_uchar
                                                                                    } else {
                                                                                        data[toggle as usize]
                                                                                    },
                                                                                    len[toggle as usize],
                                                                                    out,
                                                                                    output_samples,
                                                                                    0 as libc::c_int,
                                                                                );
                                                                            }
                                                                            if output_samples
                                                                                > 0 as libc::c_int
                                                                            {
                                                                                if decode_only == 0
                                                                                    && tot_out.wrapping_add(output_samples as libc::c_ulong)
                                                                                        > tot_in
                                                                                {
                                                                                    stop = 1 as libc::c_int;
                                                                                    output_samples = tot_in.wrapping_sub(tot_out) as opus_int32;
                                                                                }
                                                                                if output_samples
                                                                                    > skip
                                                                                {
                                                                                    let mut i_0: libc::c_int = 0;
                                                                                    i_0 = 0 as libc::c_int;
                                                                                    while i_0 < (output_samples - skip) * channels {
                                                                                        let mut s_0: libc::c_short = 0;
                                                                                        s_0 = *out.offset((i_0 + skip * channels) as isize);
                                                                                        *fbytes
                                                                                            .offset(
                                                                                                (2 as libc::c_int * i_0) as isize,
                                                                                            ) = (s_0 as libc::c_int & 0xff as libc::c_int)
                                                                                            as libc::c_uchar;
                                                                                        *fbytes
                                                                                            .offset(
                                                                                                (2 as libc::c_int * i_0 + 1 as libc::c_int) as isize,
                                                                                            ) = (s_0 as libc::c_int >> 8 as libc::c_int
                                                                                            & 0xff as libc::c_int) as libc::c_uchar;
                                                                                        i_0 += 1;
                                                                                    }
                                                                                    if fwrite(
                                                                                        fbytes as *const libc::c_void,
                                                                                        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                                                                                            .wrapping_mul(channels as libc::c_ulong),
                                                                                        (output_samples - skip) as libc::c_ulong,
                                                                                        fout,
                                                                                    )
                                                                                        != (output_samples - skip) as libc::c_uint as libc::c_ulong
                                                                                    {
                                                                                        fprintf(
                                                                                            stderr,
                                                                                            b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        current_block = 14460699602747363466;
                                                                                        break;
                                                                                    } else {
                                                                                        tot_out = (tot_out as libc::c_ulong)
                                                                                            .wrapping_add((output_samples - skip) as libc::c_ulong)
                                                                                            as opus_uint64 as opus_uint64;
                                                                                    }
                                                                                }
                                                                                if output_samples
                                                                                    < skip
                                                                                {
                                                                                    skip -= output_samples;
                                                                                } else {
                                                                                    skip = 0 as libc::c_int;
                                                                                }
                                                                            } else {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"error decoding frame: %s\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    opus_strerror(output_samples),
                                                                                );
                                                                            }
                                                                            tot_samples += output_samples as libc::c_double;
                                                                        }
                                                                    }
                                                                    if encode_only == 0 {
                                                                        opus_decoder_ctl(
                                                                            dec,
                                                                            4031 as libc::c_int,
                                                                            (&mut dec_final_range as *mut opus_uint32)
                                                                                .offset(
                                                                                    (&mut dec_final_range as *mut opus_uint32)
                                                                                        .offset_from(&mut dec_final_range as *mut opus_uint32)
                                                                                        as libc::c_long as isize,
                                                                                ),
                                                                        );
                                                                    }
                                                                    if enc_final_range[(toggle
                                                                        ^ use_inbandfec)
                                                                        as usize]
                                                                        != 0 as libc::c_int
                                                                            as libc::c_uint
                                                                        && encode_only == 0
                                                                        && lost == 0
                                                                        && lost_prev == 0
                                                                        && dec_final_range
                                                                            != enc_final_range
                                                                                [(toggle
                                                                                    ^ use_inbandfec)
                                                                                    as usize]
                                                                    {
                                                                        fprintf(
                                                                            stderr,
                                                                            b"Error: Range coder state mismatch between encoder and decoder in frame %ld: 0x%8lx vs 0x%8lx\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            count as libc::c_long,
                                                                            enc_final_range[(toggle ^ use_inbandfec) as usize]
                                                                                as libc::c_ulong,
                                                                            dec_final_range as libc::c_ulong,
                                                                        );
                                                                        current_block =
                                                                            14460699602747363466;
                                                                        break;
                                                                    } else {
                                                                        lost_prev = lost;
                                                                        if count >= use_inbandfec {
                                                                            bits += (len
                                                                                [toggle as usize]
                                                                                * 8 as libc::c_int)
                                                                                as libc::c_double;
                                                                            bits_max = if (len
                                                                                [toggle as usize]
                                                                                * 8 as libc::c_int)
                                                                                as libc::c_double
                                                                                > bits_max
                                                                            {
                                                                                (len[toggle as usize] * 8 as libc::c_int) as libc::c_double
                                                                            } else {
                                                                                bits_max
                                                                            };
                                                                            bits2 += (len
                                                                                [toggle as usize]
                                                                                * len[toggle
                                                                                    as usize]
                                                                                * 64 as libc::c_int)
                                                                                as libc::c_double;
                                                                            if decode_only == 0 {
                                                                                nrg = 0.0f64;
                                                                                k = 0
                                                                                    as libc::c_int;
                                                                                while k < frame_size
                                                                                    * channels
                                                                                {
                                                                                    nrg
                                                                                        += *in_0.offset(k as isize) as libc::c_int as libc::c_double
                                                                                            * *in_0.offset(k as isize) as libc::c_double;
                                                                                    k += 1;
                                                                                }
                                                                                nrg /= (frame_size * channels) as libc::c_double;
                                                                                if nrg > 1e5f64 {
                                                                                    bits_act
                                                                                        += (len[toggle as usize] * 8 as libc::c_int)
                                                                                            as libc::c_double;
                                                                                    count_act += 1;
                                                                                }
                                                                            }
                                                                        }
                                                                        count += 1;
                                                                        toggle = toggle
                                                                            + use_inbandfec
                                                                            & 1 as libc::c_int;
                                                                    }
                                                                }
                                                                match current_block {
                                                                    14460699602747363466 => {}
                                                                    _ => {
                                                                        if decode_only != 0
                                                                            && count
                                                                                > 0 as libc::c_int
                                                                        {
                                                                            frame_size = (tot_samples / count as libc::c_double)
                                                                                as libc::c_int;
                                                                        }
                                                                        count -= use_inbandfec;
                                                                        if tot_samples
                                                                            >= 1 as libc::c_int
                                                                                as libc::c_double
                                                                            && count
                                                                                > 0 as libc::c_int
                                                                            && frame_size != 0
                                                                        {
                                                                            let mut var: libc::c_double = 0.;
                                                                            fprintf(
                                                                                stderr,
                                                                                b"average bitrate:             %7.3f kb/s\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                1e-3f64 * bits * sampling_rate as libc::c_double
                                                                                    / tot_samples,
                                                                            );
                                                                            fprintf(
                                                                                stderr,
                                                                                b"maximum bitrate:             %7.3f kb/s\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                1e-3f64 * bits_max * sampling_rate as libc::c_double
                                                                                    / frame_size as libc::c_double,
                                                                            );
                                                                            if decode_only == 0 {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"active bitrate:              %7.3f kb/s\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    1e-3f64 * bits_act * sampling_rate as libc::c_double
                                                                                        / (1e-15f64
                                                                                            + frame_size as libc::c_double
                                                                                                * count_act as libc::c_double),
                                                                                );
                                                                            }
                                                                            var = bits2 / count as libc::c_double
                                                                                - bits * bits
                                                                                    / (count as libc::c_double * count as libc::c_double);
                                                                            if var < 0
                                                                                as libc::c_int
                                                                                as libc::c_double
                                                                            {
                                                                                var = 0 as libc::c_int as libc::c_double;
                                                                            }
                                                                            fprintf(
                                                                                stderr,
                                                                                b"bitrate standard deviation:  %7.3f kb/s\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                1e-3f64 * sqrt(var) * sampling_rate as libc::c_double
                                                                                    / frame_size as libc::c_double,
                                                                            );
                                                                        } else {
                                                                            fprintf(
                                                                                stderr,
                                                                                b"bitrate statistics are undefined\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                            );
                                                                        }
                                                                        ret = 0 as libc::c_int;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    opus_encoder_destroy(enc);
    opus_decoder_destroy(dec);
    free(data[0 as libc::c_int as usize] as *mut libc::c_void);
    free(data[1 as libc::c_int as usize] as *mut libc::c_void);
    if !fin.is_null() {
        fclose(fin);
    }
    if !fout.is_null() {
        fclose(fout);
    }
    free(in_0 as *mut libc::c_void);
    free(out as *mut libc::c_void);
    free(fbytes as *mut libc::c_void);
    return ret;
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
