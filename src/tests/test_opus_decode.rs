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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:45"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus.h:45"]
pub mod opus_h {
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "399:16"]
        pub type OpusDecoder;
        #[c2rust::src_loc = "406:1"]
        pub fn opus_decoder_get_size(channels: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "423:1"]
        pub fn opus_decoder_create(
            Fs: opus_int32,
            channels: libc::c_int,
            error: *mut libc::c_int,
        ) -> *mut OpusDecoder;
        #[c2rust::src_loc = "462:1"]
        pub fn opus_decode(
            st: *mut OpusDecoder,
            data: *const libc::c_uchar,
            len: opus_int32,
            pcm: *mut opus_int16,
            frame_size: libc::c_int,
            decode_fec: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "507:1"]
        pub fn opus_decoder_ctl(st: *mut OpusDecoder, request: libc::c_int, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "512:1"]
        pub fn opus_decoder_destroy(st: *mut OpusDecoder);
        #[c2rust::src_loc = "563:1"]
        pub fn opus_packet_get_nb_channels(data: *const libc::c_uchar) -> libc::c_int;
        #[c2rust::src_loc = "594:1"]
        pub fn opus_decoder_get_nb_samples(
            dec: *const OpusDecoder,
            packet: *const libc::c_uchar,
            len: opus_int32,
        ) -> libc::c_int;
        #[c2rust::src_loc = "606:1"]
        pub fn opus_pcm_soft_clip(
            pcm: *mut libc::c_float,
            frame_size: libc::c_int,
            channels: libc::c_int,
            softclip_mem: *mut libc::c_float,
        );
    }
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
        #[c2rust::src_loc = "356:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:45"]
pub mod opus_defines_h {
    extern "C" {
        #[c2rust::src_loc = "792:1"]
        pub fn opus_get_version_string() -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/tests/test_opus_common.h:46"]
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
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut t: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
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
    pub static mut Rz: opus_uint32 = 0;
    #[c2rust::src_loc = "56:24"]
    pub static mut Rw: opus_uint32 = 0;
    #[inline]
    #[c2rust::src_loc = "57:1"]
    pub unsafe extern "C" fn fast_rand() -> opus_uint32 {
        Rz = (36969 as libc::c_int as libc::c_uint)
            .wrapping_mul(Rz & 65535 as libc::c_int as libc::c_uint)
            .wrapping_add(Rz >> 16 as libc::c_int);
        Rw = (18000 as libc::c_int as libc::c_uint)
            .wrapping_mul(Rw & 65535 as libc::c_int as libc::c_uint)
            .wrapping_add(Rw >> 16 as libc::c_int);
        return (Rz << 16 as libc::c_int).wrapping_add(Rw);
    }
    #[c2rust::src_loc = "63:20"]
    pub static mut iseed: opus_uint32 = 0;
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
    use super::opus_defines_h::opus_get_version_string;
    use super::opus_types_h::opus_uint32;
    use super::stdio_h::{fprintf, stderr};
    use super::stdlib_h::{abort, free, malloc};
    use super::string_h::memset;
}
use self::opus_defines_h::opus_get_version_string;
use self::opus_h::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy,
    opus_decoder_get_nb_samples, opus_decoder_get_size, opus_packet_get_nb_channels,
    opus_pcm_soft_clip, OpusDecoder,
};
pub use self::opus_types_h::{opus_int16, opus_int32, opus_uint32};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::uint32_t;
use self::stdio_h::{fprintf, printf, stderr, stdout};
pub use self::stdlib_h::{abort, atoi, free, getenv, malloc, rand, strtol};
use self::string_h::{memcpy, memset};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_opus_common_h::{deb2_impl, debruijn2, Rw, Rz, _test_failed, fast_rand, iseed};
use self::time_h::time;
pub use self::time_t_h::time_t;
pub use self::types_h::{__int16_t, __int32_t, __off64_t, __off_t, __pid_t, __time_t, __uint32_t};
use self::unistd_h::getpid;
pub use self::FILE_h::FILE;
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn test_decoder_code0(mut no_fuzz: libc::c_int) -> libc::c_int {
    static mut fsv: [opus_int32; 5] = [
        48000 as libc::c_int,
        24000 as libc::c_int,
        16000 as libc::c_int,
        12000 as libc::c_int,
        8000 as libc::c_int,
    ];
    let mut err: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut plen: libc::c_int = 0;
    let mut out_samples: libc::c_int = 0;
    let mut fec: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut i: opus_int32 = 0;
    let mut dec: [*mut OpusDecoder; 10] = [0 as *mut OpusDecoder; 10];
    let mut decsize: opus_int32 = 0;
    let mut decbak: *mut OpusDecoder = 0 as *mut OpusDecoder;
    let mut dec_final_range1: opus_uint32 = 0;
    let mut dec_final_range2: opus_uint32 = 0;
    let mut dec_final_acc: opus_uint32 = 0;
    let mut packet: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut modes: [libc::c_uchar; 4096] = [0; 4096];
    let mut outbuf_int: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut outbuf: *mut libc::c_short = 0 as *mut libc::c_short;
    dec_final_range2 = 2 as libc::c_int as opus_uint32;
    dec_final_range1 = dec_final_range2;
    packet = malloc(
        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(1500 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if packet.is_null() {
        _test_failed(
            b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
        );
    }
    outbuf_int = malloc(
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_mul((5760 as libc::c_int + 16 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_short;
    i = 0 as libc::c_int;
    while i < (5760 as libc::c_int + 16 as libc::c_int) * 2 as libc::c_int {
        *outbuf_int.offset(i as isize) = 32749 as libc::c_int as libc::c_short;
        i += 1;
    }
    outbuf = &mut *outbuf_int.offset((8 as libc::c_int * 2 as libc::c_int) as isize)
        as *mut libc::c_short;
    fprintf(
        stdout,
        b"  Starting %d decoders...\n\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int * 2 as libc::c_int,
    );
    t = 0 as libc::c_int;
    while t < 5 as libc::c_int * 2 as libc::c_int {
        let mut fs: libc::c_int = fsv[(t >> 1 as libc::c_int) as usize];
        let mut c: libc::c_int = (t & 1 as libc::c_int) + 1 as libc::c_int;
        err = -(3 as libc::c_int);
        dec[t as usize] = opus_decoder_create(fs, c, &mut err);
        if err != 0 as libc::c_int || (dec[t as usize]).is_null() {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int,
            );
        }
        fprintf(
            stdout,
            b"    opus_decoder_create(%5d,%d) OK. Copy \0" as *const u8 as *const libc::c_char,
            fs,
            c,
        );
        let mut dec2: *mut OpusDecoder = 0 as *mut OpusDecoder;
        dec2 = malloc(opus_decoder_get_size(c) as libc::c_ulong) as *mut OpusDecoder;
        if dec2.is_null() {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
            );
        }
        memcpy(
            dec2 as *mut libc::c_void,
            dec[t as usize] as *const libc::c_void,
            opus_decoder_get_size(c) as libc::c_ulong,
        );
        memset(
            dec[t as usize] as *mut libc::c_void,
            255 as libc::c_int,
            opus_decoder_get_size(c) as libc::c_ulong,
        );
        opus_decoder_destroy(dec[t as usize]);
        printf(b"OK.\n\0" as *const u8 as *const libc::c_char);
        dec[t as usize] = dec2;
        t += 1;
    }
    decsize = opus_decoder_get_size(1 as libc::c_int);
    decbak = malloc(decsize as libc::c_ulong) as *mut OpusDecoder;
    if decbak.is_null() {
        _test_failed(
            b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
        );
    }
    t = 0 as libc::c_int;
    while t < 5 as libc::c_int * 2 as libc::c_int {
        let mut factor: libc::c_int = 48000 as libc::c_int / fsv[(t >> 1 as libc::c_int) as usize];
        fec = 0 as libc::c_int;
        while fec < 2 as libc::c_int {
            let mut dur: opus_int32 = 0;
            out_samples = opus_decode(
                dec[t as usize],
                0 as *const libc::c_uchar,
                0 as libc::c_int,
                outbuf,
                120 as libc::c_int / factor,
                fec,
            );
            if out_samples != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                );
            }
            if opus_decoder_ctl(
                dec[t as usize],
                4039 as libc::c_int,
                (&mut dur as *mut opus_int32).offset(
                    (&mut dur as *mut opus_int32).offset_from(&mut dur as *mut opus_int32)
                        as libc::c_long as isize,
                ),
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    111 as libc::c_int,
                );
            }
            if dur != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    112 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                0 as *const libc::c_uchar,
                0 as libc::c_int,
                outbuf,
                120 as libc::c_int / factor + 2 as libc::c_int,
                fec,
            );
            if out_samples != -(1 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    116 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                0 as *const libc::c_uchar,
                -(1 as libc::c_int),
                outbuf,
                120 as libc::c_int / factor,
                fec,
            );
            if out_samples != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    120 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                0 as *const libc::c_uchar,
                1 as libc::c_int,
                outbuf,
                120 as libc::c_int / factor,
                fec,
            );
            if out_samples != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    122 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                0 as *const libc::c_uchar,
                10 as libc::c_int,
                outbuf,
                120 as libc::c_int / factor,
                fec,
            );
            if out_samples != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    124 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                0 as *const libc::c_uchar,
                fast_rand() as opus_int32,
                outbuf,
                120 as libc::c_int / factor,
                fec,
            );
            if out_samples != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    126 as libc::c_int,
                );
            }
            if opus_decoder_ctl(
                dec[t as usize],
                4039 as libc::c_int,
                (&mut dur as *mut opus_int32).offset(
                    (&mut dur as *mut opus_int32).offset_from(&mut dur as *mut opus_int32)
                        as libc::c_long as isize,
                ),
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    127 as libc::c_int,
                );
            }
            if dur != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    128 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                0 as libc::c_int,
                outbuf,
                120 as libc::c_int / factor,
                fec,
            );
            if out_samples != 120 as libc::c_int / factor {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    132 as libc::c_int,
                );
            }
            *outbuf.offset(0 as libc::c_int as isize) = 32749 as libc::c_int as libc::c_short;
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                0 as libc::c_int,
                outbuf,
                0 as libc::c_int,
                fec,
            );
            if out_samples > 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    137 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                0 as libc::c_int,
                0 as *mut opus_int16,
                0 as libc::c_int,
                fec,
            );
            if out_samples > 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    146 as libc::c_int,
                );
            }
            if *outbuf.offset(0 as libc::c_int as isize) as libc::c_int != 32749 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    147 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                -(1 as libc::c_int),
                outbuf,
                5760 as libc::c_int,
                fec,
            );
            if out_samples >= 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    151 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                -(2147483647 as libc::c_int) - 1 as libc::c_int,
                outbuf,
                5760 as libc::c_int,
                fec,
            );
            if out_samples >= 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    153 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                -(1 as libc::c_int),
                outbuf,
                -(1 as libc::c_int),
                fec,
            );
            if out_samples >= 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    155 as libc::c_int,
                );
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                1 as libc::c_int,
                outbuf,
                5760 as libc::c_int,
                if fec != 0 {
                    -(1 as libc::c_int)
                } else {
                    2 as libc::c_int
                },
            );
            if out_samples >= 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    159 as libc::c_int,
                );
            }
            if opus_decoder_ctl(dec[t as usize], 4028 as libc::c_int) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int,
                );
            }
            fec += 1;
        }
        t += 1;
    }
    fprintf(
        stdout,
        b"  dec[all] initial frame PLC OK.\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut dur_0: opus_int32 = 0;
        let mut j: libc::c_int = 0;
        let mut expected: [libc::c_int; 10] = [0; 10];
        *packet.offset(0 as libc::c_int as isize) = (i << 2 as libc::c_int) as libc::c_uchar;
        *packet.offset(1 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
        *packet.offset(2 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
        err = opus_packet_get_nb_channels(packet);
        if err != (i & 1 as libc::c_int) + 1 as libc::c_int {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                176 as libc::c_int,
            );
        }
        t = 0 as libc::c_int;
        while t < 5 as libc::c_int * 2 as libc::c_int {
            expected[t as usize] = opus_decoder_get_nb_samples(
                dec[t as usize],
                packet as *const libc::c_uchar,
                1 as libc::c_int,
            );
            if expected[t as usize] > 2880 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    180 as libc::c_int,
                );
            }
            t += 1;
        }
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            *packet.offset(1 as libc::c_int as isize) = j as libc::c_uchar;
            t = 0 as libc::c_int;
            while t < 5 as libc::c_int * 2 as libc::c_int {
                out_samples = opus_decode(
                    dec[t as usize],
                    packet,
                    3 as libc::c_int,
                    outbuf,
                    5760 as libc::c_int,
                    0 as libc::c_int,
                );
                if out_samples != expected[t as usize] {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        189 as libc::c_int,
                    );
                }
                if opus_decoder_ctl(
                    dec[t as usize],
                    4039 as libc::c_int,
                    (&mut dur_0 as *mut opus_int32).offset(
                        (&mut dur_0 as *mut opus_int32).offset_from(&mut dur_0 as *mut opus_int32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        190 as libc::c_int,
                    );
                }
                if dur_0 != out_samples {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        191 as libc::c_int,
                    );
                }
                opus_decoder_ctl(
                    dec[t as usize],
                    4031 as libc::c_int,
                    (&mut dec_final_range1 as *mut opus_uint32).offset(
                        (&mut dec_final_range1 as *mut opus_uint32)
                            .offset_from(&mut dec_final_range1 as *mut opus_uint32)
                            as libc::c_long as isize,
                    ),
                );
                if t == 0 as libc::c_int {
                    dec_final_range2 = dec_final_range1;
                } else if dec_final_range1 != dec_final_range2 {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        194 as libc::c_int,
                    );
                }
                t += 1;
            }
            j += 1;
        }
        t = 0 as libc::c_int;
        while t < 5 as libc::c_int * 2 as libc::c_int {
            let mut factor_0: libc::c_int =
                48000 as libc::c_int / fsv[(t >> 1 as libc::c_int) as usize];
            j = 0 as libc::c_int;
            while j < 6 as libc::c_int {
                out_samples = opus_decode(
                    dec[t as usize],
                    0 as *const libc::c_uchar,
                    0 as libc::c_int,
                    outbuf,
                    expected[t as usize],
                    0 as libc::c_int,
                );
                if out_samples != expected[t as usize] {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        204 as libc::c_int,
                    );
                }
                if opus_decoder_ctl(
                    dec[t as usize],
                    4039 as libc::c_int,
                    (&mut dur_0 as *mut opus_int32).offset(
                        (&mut dur_0 as *mut opus_int32).offset_from(&mut dur_0 as *mut opus_int32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        205 as libc::c_int,
                    );
                }
                if dur_0 != out_samples {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        206 as libc::c_int,
                    );
                }
                j += 1;
            }
            if expected[t as usize] != 120 as libc::c_int / factor_0 {
                out_samples = opus_decode(
                    dec[t as usize],
                    0 as *const libc::c_uchar,
                    0 as libc::c_int,
                    outbuf,
                    120 as libc::c_int / factor_0,
                    0 as libc::c_int,
                );
                if out_samples != 120 as libc::c_int / factor_0 {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        213 as libc::c_int,
                    );
                }
                if opus_decoder_ctl(
                    dec[t as usize],
                    4039 as libc::c_int,
                    (&mut dur_0 as *mut opus_int32).offset(
                        (&mut dur_0 as *mut opus_int32).offset_from(&mut dur_0 as *mut opus_int32)
                            as libc::c_long as isize,
                    ),
                ) != 0 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        214 as libc::c_int,
                    );
                }
                if dur_0 != out_samples {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        215 as libc::c_int,
                    );
                }
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                2 as libc::c_int,
                outbuf,
                expected[t as usize] - 1 as libc::c_int,
                0 as libc::c_int,
            );
            if out_samples > 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    218 as libc::c_int,
                );
            }
            t += 1;
        }
        i += 1;
    }
    fprintf(
        stdout,
        b"  dec[all] all 2-byte prefix for length 3 and PLC, all modes (64) OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if no_fuzz != 0 {
        fprintf(
            stdout,
            b"  Skipping many tests which fuzz the decoder as requested.\n\0" as *const u8
                as *const libc::c_char,
        );
        free(decbak as *mut libc::c_void);
        t = 0 as libc::c_int;
        while t < 5 as libc::c_int * 2 as libc::c_int {
            opus_decoder_destroy(dec[t as usize]);
            t += 1;
        }
        printf(b"  Decoders stopped.\n\0" as *const u8 as *const libc::c_char);
        err = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int * 2 as libc::c_int {
            err |= (*outbuf_int.offset(i as isize) as libc::c_int != 32749 as libc::c_int)
                as libc::c_int;
            i += 1;
        }
        i = 5760 as libc::c_int * 2 as libc::c_int;
        while i < (5760 as libc::c_int + 8 as libc::c_int) * 2 as libc::c_int {
            err |=
                (*outbuf.offset(i as isize) as libc::c_int != 32749 as libc::c_int) as libc::c_int;
            i += 1;
        }
        if err != 0 {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                233 as libc::c_int,
            );
        }
        free(outbuf_int as *mut libc::c_void);
        free(packet as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    static mut cmodes: [libc::c_int; 4] = [
        16 as libc::c_int,
        20 as libc::c_int,
        24 as libc::c_int,
        28 as libc::c_int,
    ];
    static mut cres: [opus_uint32; 4] = [
        116290185 as libc::c_int as opus_uint32,
        2172123586 as libc::c_uint,
        2172123586 as libc::c_uint,
        2172123586 as libc::c_uint,
    ];
    static mut lres: [opus_uint32; 3] = [
        3285687739 as libc::c_uint,
        1481572662 as libc::c_int as opus_uint32,
        694350475 as libc::c_int as opus_uint32,
    ];
    static mut lmodes: [libc::c_int; 3] = [0 as libc::c_int, 4 as libc::c_int, 8 as libc::c_int];
    let mut mode: libc::c_int =
        (fast_rand()).wrapping_rem(4 as libc::c_int as libc::c_uint) as libc::c_int;
    *packet.offset(0 as libc::c_int as isize) =
        (cmodes[mode as usize] << 3 as libc::c_int) as libc::c_uchar;
    dec_final_acc = 0 as libc::c_int as opus_uint32;
    t = (fast_rand()).wrapping_rem(10 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 65536 as libc::c_int {
        let mut factor_1: libc::c_int =
            48000 as libc::c_int / fsv[(t >> 1 as libc::c_int) as usize];
        *packet.offset(1 as libc::c_int as isize) = (i >> 8 as libc::c_int) as libc::c_uchar;
        *packet.offset(2 as libc::c_int as isize) = (i & 255 as libc::c_int) as libc::c_uchar;
        *packet.offset(3 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
        out_samples = opus_decode(
            dec[t as usize],
            packet,
            4 as libc::c_int,
            outbuf,
            5760 as libc::c_int,
            0 as libc::c_int,
        );
        if out_samples != 120 as libc::c_int / factor_1 {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                260 as libc::c_int,
            );
        }
        opus_decoder_ctl(
            dec[t as usize],
            4031 as libc::c_int,
            (&mut dec_final_range1 as *mut opus_uint32).offset(
                (&mut dec_final_range1 as *mut opus_uint32)
                    .offset_from(&mut dec_final_range1 as *mut opus_uint32)
                    as libc::c_long as isize,
            ),
        );
        dec_final_acc = (dec_final_acc as libc::c_uint).wrapping_add(dec_final_range1)
            as opus_uint32 as opus_uint32;
        i += 1;
    }
    if dec_final_acc != cres[mode as usize] {
        _test_failed(
            b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
            264 as libc::c_int,
        );
    }
    fprintf(
        stdout,
        b"  dec[%3d] all 3-byte prefix for length 4, mode %2d OK.\n\0" as *const u8
            as *const libc::c_char,
        t,
        cmodes[mode as usize],
    );
    mode = (fast_rand()).wrapping_rem(3 as libc::c_int as libc::c_uint) as libc::c_int;
    *packet.offset(0 as libc::c_int as isize) =
        (lmodes[mode as usize] << 3 as libc::c_int) as libc::c_uchar;
    dec_final_acc = 0 as libc::c_int as opus_uint32;
    t = (fast_rand()).wrapping_rem(10 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 65536 as libc::c_int {
        let mut factor_2: libc::c_int =
            48000 as libc::c_int / fsv[(t >> 1 as libc::c_int) as usize];
        *packet.offset(1 as libc::c_int as isize) = (i >> 8 as libc::c_int) as libc::c_uchar;
        *packet.offset(2 as libc::c_int as isize) = (i & 255 as libc::c_int) as libc::c_uchar;
        *packet.offset(3 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
        out_samples = opus_decode(
            dec[t as usize],
            packet,
            4 as libc::c_int,
            outbuf,
            5760 as libc::c_int,
            0 as libc::c_int,
        );
        if out_samples != 480 as libc::c_int / factor_2 {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                278 as libc::c_int,
            );
        }
        opus_decoder_ctl(
            dec[t as usize],
            4031 as libc::c_int,
            (&mut dec_final_range1 as *mut opus_uint32).offset(
                (&mut dec_final_range1 as *mut opus_uint32)
                    .offset_from(&mut dec_final_range1 as *mut opus_uint32)
                    as libc::c_long as isize,
            ),
        );
        dec_final_acc = (dec_final_acc as libc::c_uint).wrapping_add(dec_final_range1)
            as opus_uint32 as opus_uint32;
        i += 1;
    }
    if dec_final_acc != lres[mode as usize] {
        _test_failed(
            b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
        );
    }
    fprintf(
        stdout,
        b"  dec[%3d] all 3-byte prefix for length 4, mode %2d OK.\n\0" as *const u8
            as *const libc::c_char,
        t,
        lmodes[mode as usize],
    );
    skip = (fast_rand()).wrapping_rem(7 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut j_0: libc::c_int = 0;
        let mut expected_0: [libc::c_int; 10] = [0; 10];
        *packet.offset(0 as libc::c_int as isize) = (i << 2 as libc::c_int) as libc::c_uchar;
        t = 0 as libc::c_int;
        while t < 5 as libc::c_int * 2 as libc::c_int {
            expected_0[t as usize] = opus_decoder_get_nb_samples(
                dec[t as usize],
                packet as *const libc::c_uchar,
                1 as libc::c_int,
            );
            t += 1;
        }
        j_0 = 2 as libc::c_int + skip;
        while j_0 < 1275 as libc::c_int {
            let mut jj: libc::c_int = 0;
            jj = 0 as libc::c_int;
            while jj < j_0 {
                *packet.offset((jj + 1 as libc::c_int) as isize) =
                    (fast_rand() & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
                jj += 1;
            }
            t = 0 as libc::c_int;
            while t < 5 as libc::c_int * 2 as libc::c_int {
                out_samples = opus_decode(
                    dec[t as usize],
                    packet,
                    j_0 + 1 as libc::c_int,
                    outbuf,
                    5760 as libc::c_int,
                    0 as libc::c_int,
                );
                if out_samples != expected_0[t as usize] {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        299 as libc::c_int,
                    );
                }
                opus_decoder_ctl(
                    dec[t as usize],
                    4031 as libc::c_int,
                    (&mut dec_final_range1 as *mut opus_uint32).offset(
                        (&mut dec_final_range1 as *mut opus_uint32)
                            .offset_from(&mut dec_final_range1 as *mut opus_uint32)
                            as libc::c_long as isize,
                    ),
                );
                if t == 0 as libc::c_int {
                    dec_final_range2 = dec_final_range1;
                } else if dec_final_range1 != dec_final_range2 {
                    _test_failed(
                        b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                        302 as libc::c_int,
                    );
                }
                t += 1;
            }
            j_0 += 4 as libc::c_int;
        }
        i += 1;
    }
    fprintf(
        stdout,
        b"  dec[all] random packets, all modes (64), every 8th size from from %d bytes to maximum OK.\n\0"
            as *const u8 as *const libc::c_char,
        2 as libc::c_int + skip,
    );
    debruijn2(64 as libc::c_int, modes.as_mut_ptr());
    plen = (fast_rand())
        .wrapping_rem(18 as libc::c_int as libc::c_uint)
        .wrapping_add(3 as libc::c_int as libc::c_uint)
        .wrapping_mul(8 as libc::c_int as libc::c_uint)
        .wrapping_add(skip as libc::c_uint)
        .wrapping_add(3 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        let mut j_1: libc::c_int = 0;
        let mut expected_1: [libc::c_int; 10] = [0; 10];
        *packet.offset(0 as libc::c_int as isize) =
            ((modes[i as usize] as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
        t = 0 as libc::c_int;
        while t < 5 as libc::c_int * 2 as libc::c_int {
            expected_1[t as usize] =
                opus_decoder_get_nb_samples(dec[t as usize], packet as *const libc::c_uchar, plen);
            t += 1;
        }
        j_1 = 0 as libc::c_int;
        while j_1 < plen {
            *packet.offset((j_1 + 1 as libc::c_int) as isize) =
                ((fast_rand() | fast_rand()) & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
            j_1 += 1;
        }
        memcpy(
            decbak as *mut libc::c_void,
            dec[0 as libc::c_int as usize] as *const libc::c_void,
            decsize as libc::c_ulong,
        );
        if opus_decode(
            decbak,
            packet,
            plen + 1 as libc::c_int,
            outbuf,
            expected_1[0 as libc::c_int as usize],
            1 as libc::c_int,
        ) != expected_1[0 as libc::c_int as usize]
        {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                317 as libc::c_int,
            );
        }
        memcpy(
            decbak as *mut libc::c_void,
            dec[0 as libc::c_int as usize] as *const libc::c_void,
            decsize as libc::c_ulong,
        );
        if opus_decode(
            decbak,
            0 as *const libc::c_uchar,
            0 as libc::c_int,
            outbuf,
            5760 as libc::c_int,
            1 as libc::c_int,
        ) < 20 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                319 as libc::c_int,
            );
        }
        memcpy(
            decbak as *mut libc::c_void,
            dec[0 as libc::c_int as usize] as *const libc::c_void,
            decsize as libc::c_ulong,
        );
        if opus_decode(
            decbak,
            0 as *const libc::c_uchar,
            0 as libc::c_int,
            outbuf,
            5760 as libc::c_int,
            0 as libc::c_int,
        ) < 20 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int,
            );
        }
        t = 0 as libc::c_int;
        while t < 5 as libc::c_int * 2 as libc::c_int {
            let mut dur_1: opus_int32 = 0;
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                plen + 1 as libc::c_int,
                outbuf,
                5760 as libc::c_int,
                0 as libc::c_int,
            );
            if out_samples != expected_1[t as usize] {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    326 as libc::c_int,
                );
            }
            if t == 0 as libc::c_int {
                dec_final_range2 = dec_final_range1;
            } else if dec_final_range1 != dec_final_range2 {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    328 as libc::c_int,
                );
            }
            if opus_decoder_ctl(
                dec[t as usize],
                4039 as libc::c_int,
                (&mut dur_1 as *mut opus_int32).offset(
                    (&mut dur_1 as *mut opus_int32).offset_from(&mut dur_1 as *mut opus_int32)
                        as libc::c_long as isize,
                ),
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    329 as libc::c_int,
                );
            }
            if dur_1 != out_samples {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    330 as libc::c_int,
                );
            }
            t += 1;
        }
        i += 1;
    }
    fprintf(
        stdout,
        b"  dec[all] random packets, all mode pairs (4096), %d bytes/frame OK.\n\0" as *const u8
            as *const libc::c_char,
        plen + 1 as libc::c_int,
    );
    plen = (fast_rand())
        .wrapping_rem(18 as libc::c_int as libc::c_uint)
        .wrapping_add(3 as libc::c_int as libc::c_uint)
        .wrapping_mul(8 as libc::c_int as libc::c_uint)
        .wrapping_add(skip as libc::c_uint)
        .wrapping_add(3 as libc::c_int as libc::c_uint) as libc::c_int;
    t = rand() & 3 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        let mut count: libc::c_int = 0;
        let mut j_2: libc::c_int = 0;
        let mut expected_2: libc::c_int = 0;
        *packet.offset(0 as libc::c_int as isize) =
            ((modes[i as usize] as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
        expected_2 =
            opus_decoder_get_nb_samples(dec[t as usize], packet as *const libc::c_uchar, plen);
        count = 0 as libc::c_int;
        while count < 10 as libc::c_int {
            j_2 = 0 as libc::c_int;
            while j_2 < plen {
                *packet.offset((j_2 + 1 as libc::c_int) as isize) = ((fast_rand() | fast_rand())
                    & 255 as libc::c_int as libc::c_uint)
                    as libc::c_uchar;
                j_2 += 1;
            }
            out_samples = opus_decode(
                dec[t as usize],
                packet,
                plen + 1 as libc::c_int,
                outbuf,
                5760 as libc::c_int,
                0 as libc::c_int,
            );
            if out_samples != expected_2 {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    346 as libc::c_int,
                );
            }
            count += 1;
        }
        i += 1;
    }
    fprintf(
        stdout,
        b"  dec[%3d] random packets, all mode pairs (4096)*10, %d bytes/frame OK.\n\0" as *const u8
            as *const libc::c_char,
        t,
        plen + 1 as libc::c_int,
    );
    let mut tmodes: [libc::c_int; 1] = [(25 as libc::c_int) << 2 as libc::c_int];
    let mut tseeds: [opus_uint32; 1] = [140441 as libc::c_int as opus_uint32];
    let mut tlen: [libc::c_int; 1] = [157 as libc::c_int];
    let mut tret: [opus_int32; 1] = [480 as libc::c_int];
    t = (fast_rand() & 1 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        let mut j_3: libc::c_int = 0;
        *packet.offset(0 as libc::c_int as isize) = tmodes[i as usize] as libc::c_uchar;
        Rz = tseeds[i as usize];
        Rw = Rz;
        j_3 = 1 as libc::c_int;
        while j_3 < tlen[i as usize] {
            *packet.offset(j_3 as isize) =
                (fast_rand() & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
            j_3 += 1;
        }
        out_samples = opus_decode(
            dec[t as usize],
            packet,
            tlen[i as usize],
            outbuf,
            5760 as libc::c_int,
            0 as libc::c_int,
        );
        if out_samples != tret[i as usize] {
            _test_failed(
                b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                364 as libc::c_int,
            );
        }
        i += 1;
    }
    fprintf(
        stdout,
        b"  dec[%3d] pre-selected random packets OK.\n\0" as *const u8 as *const libc::c_char,
        t,
    );
    free(decbak as *mut libc::c_void);
    t = 0 as libc::c_int;
    while t < 5 as libc::c_int * 2 as libc::c_int {
        opus_decoder_destroy(dec[t as usize]);
        t += 1;
    }
    printf(b"  Decoders stopped.\n\0" as *const u8 as *const libc::c_char);
    err = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int * 2 as libc::c_int {
        err |=
            (*outbuf_int.offset(i as isize) as libc::c_int != 32749 as libc::c_int) as libc::c_int;
        i += 1;
    }
    i = 5760 as libc::c_int * 2 as libc::c_int;
    while i < (5760 as libc::c_int + 8 as libc::c_int) * 2 as libc::c_int {
        err |= (*outbuf.offset(i as isize) as libc::c_int != 32749 as libc::c_int) as libc::c_int;
        i += 1;
    }
    if err != 0 {
        _test_failed(
            b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int,
        );
    }
    free(outbuf_int as *mut libc::c_void);
    free(packet as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "384:1"]
pub unsafe extern "C" fn test_soft_clip() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: [libc::c_float; 1024] = [0.; 1024];
    let mut s: [libc::c_float; 8] = [
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    fprintf(
        stdout,
        b"  Testing opus_pcm_soft_clip... \0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 1024 as libc::c_int {
            x[j as usize] = (j & 255 as libc::c_int) as libc::c_float
                * (1 as libc::c_int as libc::c_float / 32.0f32)
                - 4.0f32;
            j += 1;
        }
        opus_pcm_soft_clip(
            &mut *x.as_mut_ptr().offset(i as isize),
            1024 as libc::c_int - i,
            1 as libc::c_int,
            s.as_mut_ptr(),
        );
        j = i;
        while j < 1024 as libc::c_int {
            if x[j as usize] > 1.0f32 {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    399 as libc::c_int,
                );
            }
            if x[j as usize] < -1.0f32 {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    400 as libc::c_int,
                );
            }
            j += 1;
        }
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < 9 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 1024 as libc::c_int {
            x[j as usize] = (j & 255 as libc::c_int) as libc::c_float
                * (1 as libc::c_int as libc::c_float / 32.0f32)
                - 4.0f32;
            j += 1;
        }
        opus_pcm_soft_clip(x.as_mut_ptr(), 1024 as libc::c_int / i, i, s.as_mut_ptr());
        j = 0 as libc::c_int;
        while j < 1024 as libc::c_int / i * i {
            if x[j as usize] > 1.0f32 {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    412 as libc::c_int,
                );
            }
            if x[j as usize] < -1.0f32 {
                _test_failed(
                    b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
                    413 as libc::c_int,
                );
            }
            j += 1;
        }
        i += 1;
    }
    opus_pcm_soft_clip(
        x.as_mut_ptr(),
        0 as libc::c_int,
        1 as libc::c_int,
        s.as_mut_ptr(),
    );
    opus_pcm_soft_clip(
        x.as_mut_ptr(),
        1 as libc::c_int,
        0 as libc::c_int,
        s.as_mut_ptr(),
    );
    opus_pcm_soft_clip(
        x.as_mut_ptr(),
        1 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_float,
    );
    opus_pcm_soft_clip(
        x.as_mut_ptr(),
        1 as libc::c_int,
        -(1 as libc::c_int),
        s.as_mut_ptr(),
    );
    opus_pcm_soft_clip(
        x.as_mut_ptr(),
        -(1 as libc::c_int),
        1 as libc::c_int,
        s.as_mut_ptr(),
    );
    opus_pcm_soft_clip(
        0 as *mut libc::c_float,
        1 as libc::c_int,
        1 as libc::c_int,
        s.as_mut_ptr(),
    );
    printf(b"OK.\n\0" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "426:1"]
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut oversion: *const libc::c_char = 0 as *const libc::c_char;
    let mut env_seed: *const libc::c_char = 0 as *const libc::c_char;
    let mut env_used: libc::c_int = 0;
    if _argc > 2 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: %s [<seed>]\n\0" as *const u8 as *const libc::c_char,
            *_argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    env_used = 0 as libc::c_int;
    env_seed = getenv(b"SEED\0" as *const u8 as *const libc::c_char);
    if _argc > 1 as libc::c_int {
        iseed = atoi(*_argv.offset(1 as libc::c_int as isize)) as opus_uint32;
    } else if !env_seed.is_null() {
        iseed = atoi(env_seed) as opus_uint32;
        env_used = 1 as libc::c_int;
    } else {
        iseed = time(0 as *mut time_t) as opus_uint32
            ^ (getpid() as opus_uint32 & 65535 as libc::c_int as libc::c_uint) << 16 as libc::c_int;
    }
    Rz = iseed;
    Rw = Rz;
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_decode.c\0" as *const u8 as *const libc::c_char,
            450 as libc::c_int,
        );
    }
    fprintf(
        stderr,
        b"Testing %s decoder. Random seed: %u (%.4X)\n\0" as *const u8 as *const libc::c_char,
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
    test_decoder_code0(
        (getenv(b"TEST_OPUS_NOFUZZ\0" as *const u8 as *const libc::c_char)
            != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int,
    );
    test_soft_clip();
    return 0 as libc::c_int;
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
