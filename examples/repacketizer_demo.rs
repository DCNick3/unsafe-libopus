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
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
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
        #[c2rust::src_loc = "675:15"]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
        #[c2rust::src_loc = "788:1"]
        pub fn feof(__stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "681:15"]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
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
    }
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
use self::stdio_h::{fclose, feof, fopen, fprintf, fread, fwrite, stderr};
pub use self::stdlib_h::atoi;
use self::string_h::strcmp;
pub use self::FILE_h::FILE;
use libopus_unsafe::{
    opus_repacketizer_cat, opus_repacketizer_create, opus_repacketizer_get_nb_frames,
    opus_repacketizer_init, opus_repacketizer_out, opus_repacketizer_out_range, opus_strerror,
    OpusRepacketizer,
};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn usage(mut argv0: *mut libc::c_char) {
    fprintf(
        stderr,
        b"usage: %s [options] input_file output_file\n\0" as *const u8 as *const libc::c_char,
        argv0,
    );
}
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn int_to_char(mut i: u32, mut ch: *mut libc::c_uchar) {
    *ch.offset(0 as libc::c_int as isize) = (i >> 24 as libc::c_int) as libc::c_uchar;
    *ch.offset(1 as libc::c_int as isize) =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *ch.offset(2 as libc::c_int as isize) =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *ch.offset(3 as libc::c_int as isize) =
        (i & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn char_to_int(mut ch: *mut libc::c_uchar) -> u32 {
    (*ch.offset(0 as libc::c_int as isize) as u32) << 24 as libc::c_int
        | (*ch.offset(1 as libc::c_int as isize) as u32) << 16 as libc::c_int
        | (*ch.offset(2 as libc::c_int as isize) as u32) << 8 as libc::c_int
        | *ch.offset(3 as libc::c_int as isize) as u32
}
#[c2rust::src_loc = "58:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut eof: libc::c_int = 0 as libc::c_int;
    let mut fin: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut fout: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut packets: [[libc::c_uchar; 1500]; 48] = [[0; 1500]; 48];
    let mut len: [libc::c_int; 48] = [0; 48];
    let mut rng: [libc::c_int; 48] = [0; 48];
    let mut rp: *mut OpusRepacketizer = std::ptr::null_mut::<OpusRepacketizer>();
    let mut output_packet: [libc::c_uchar; 32000] = [0; 32000];
    let mut merge: libc::c_int = 1 as libc::c_int;
    let mut split: libc::c_int = 0 as libc::c_int;
    if argc < 3 as libc::c_int {
        usage(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < argc - 2 as libc::c_int {
        if strcmp(
            *argv.offset(i as isize),
            b"-merge\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            merge = atoi(*argv.offset((i + 1 as libc::c_int) as isize));
            if merge < 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"-merge parameter must be at least 1.\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if merge > 48 as libc::c_int {
                fprintf(
                    stderr,
                    b"-merge parameter must be less than 48.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            i += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-split\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            split = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"Unknown option: %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
            usage(*argv.offset(0 as libc::c_int as isize));
            return 1 as libc::c_int;
        }
        i += 1;
    }
    fin = fopen(
        *argv.offset((argc - 2 as libc::c_int) as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fin.is_null() {
        fprintf(
            stderr,
            b"Error opening input file: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset((argc - 2 as libc::c_int) as isize),
        );
        return 1 as libc::c_int;
    }
    fout = fopen(
        *argv.offset((argc - 1 as libc::c_int) as isize),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if fout.is_null() {
        fprintf(
            stderr,
            b"Error opening output file: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset((argc - 1 as libc::c_int) as isize),
        );
        fclose(fin);
        return 1 as libc::c_int;
    }
    rp = opus_repacketizer_create();
    while eof == 0 {
        let mut err: libc::c_int = 0;
        let mut nb_packets: libc::c_int = merge;
        opus_repacketizer_init(rp);
        i = 0 as libc::c_int;
        while i < nb_packets {
            let mut ch: [libc::c_uchar; 4] = [0; 4];
            err = fread(
                ch.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                4 as libc::c_int as libc::c_ulong,
                fin,
            ) as libc::c_int;
            len[i as usize] = char_to_int(ch.as_mut_ptr()) as libc::c_int;
            if len[i as usize] > 1500 as libc::c_int || len[i as usize] < 0 as libc::c_int {
                if feof(fin) != 0 {
                    eof = 1 as libc::c_int;
                } else {
                    fprintf(
                        stderr,
                        b"Invalid payload length\n\0" as *const u8 as *const libc::c_char,
                    );
                    fclose(fin);
                    fclose(fout);
                    return 1 as libc::c_int;
                }
                break;
            } else {
                err = fread(
                    ch.as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    4 as libc::c_int as libc::c_ulong,
                    fin,
                ) as libc::c_int;
                rng[i as usize] = char_to_int(ch.as_mut_ptr()) as libc::c_int;
                err = fread(
                    (packets[i as usize]).as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    len[i as usize] as libc::c_ulong,
                    fin,
                ) as libc::c_int;
                if feof(fin) != 0 {
                    eof = 1 as libc::c_int;
                    break;
                } else {
                    err = opus_repacketizer_cat(
                        rp,
                        (packets[i as usize]).as_mut_ptr(),
                        len[i as usize],
                    );
                    if err != 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"opus_repacketizer_cat() failed: %s\n\0" as *const u8
                                as *const libc::c_char,
                            opus_strerror(err),
                        );
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        nb_packets = i;
        if eof != 0 {
            break;
        }
        if split == 0 {
            err = opus_repacketizer_out(rp, output_packet.as_mut_ptr(), 32000 as libc::c_int);
            if err > 0 as libc::c_int {
                let mut int_field: [libc::c_uchar; 4] = [0; 4];
                int_to_char(err as u32, int_field.as_mut_ptr());
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
                    return 1 as libc::c_int;
                }
                int_to_char(
                    rng[(nb_packets - 1 as libc::c_int) as usize] as u32,
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
                    return 1 as libc::c_int;
                }
                if fwrite(
                    output_packet.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    err as libc::c_ulong,
                    fout,
                ) != err as libc::c_uint as libc::c_ulong
                {
                    fprintf(
                        stderr,
                        b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            } else {
                fprintf(
                    stderr,
                    b"opus_repacketizer_out() failed: %s\n\0" as *const u8 as *const libc::c_char,
                    opus_strerror(err),
                );
            }
        } else {
            let mut nb_frames: libc::c_int = opus_repacketizer_get_nb_frames(rp);
            i = 0 as libc::c_int;
            while i < nb_frames {
                err = opus_repacketizer_out_range(
                    rp,
                    i,
                    i + 1 as libc::c_int,
                    output_packet.as_mut_ptr(),
                    32000 as libc::c_int,
                );
                if err > 0 as libc::c_int {
                    let mut int_field_0: [libc::c_uchar; 4] = [0; 4];
                    int_to_char(err as u32, int_field_0.as_mut_ptr());
                    if fwrite(
                        int_field_0.as_mut_ptr() as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        4 as libc::c_int as libc::c_ulong,
                        fout,
                    ) != 4 as libc::c_int as libc::c_ulong
                    {
                        fprintf(
                            stderr,
                            b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    if i == nb_frames - 1 as libc::c_int {
                        int_to_char(
                            rng[(nb_packets - 1 as libc::c_int) as usize] as u32,
                            int_field_0.as_mut_ptr(),
                        );
                    } else {
                        int_to_char(0 as libc::c_int as u32, int_field_0.as_mut_ptr());
                    }
                    if fwrite(
                        int_field_0.as_mut_ptr() as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        4 as libc::c_int as libc::c_ulong,
                        fout,
                    ) != 4 as libc::c_int as libc::c_ulong
                    {
                        fprintf(
                            stderr,
                            b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    if fwrite(
                        output_packet.as_mut_ptr() as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        err as libc::c_ulong,
                        fout,
                    ) != err as libc::c_uint as libc::c_ulong
                    {
                        fprintf(
                            stderr,
                            b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                } else {
                    fprintf(
                        stderr,
                        b"opus_repacketizer_out() failed: %s\n\0" as *const u8
                            as *const libc::c_char,
                        opus_strerror(err),
                    );
                }
                i += 1;
            }
        }
    }
    fclose(fin);
    fclose(fout);
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
