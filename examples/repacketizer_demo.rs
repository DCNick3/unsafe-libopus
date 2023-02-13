#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(register_tool)]
#![register_tool(c2rust)]

use libc::{atoi, fclose, feof, fopen, fprintf, fread, fwrite, FILE};
use libc_stdhandle::stderr;

use unsafe_libopus::externs::strcmp;
use unsafe_libopus::{
    opus_repacketizer_cat, opus_repacketizer_create, opus_repacketizer_get_nb_frames,
    opus_repacketizer_init, opus_repacketizer_out, opus_repacketizer_out_range, opus_strerror,
    OpusRepacketizer,
};
#[c2rust::src_loc = "39:1"]
pub unsafe fn usage(mut argv0: *mut libc::c_char) {
    fprintf(
        stderr(),
        b"usage: %s [options] input_file output_file\n\0" as *const u8 as *const libc::c_char,
        argv0,
    );
}
#[c2rust::src_loc = "44:1"]
unsafe fn int_to_char(mut i: u32, mut ch: *mut libc::c_uchar) {
    *ch.offset(0 as libc::c_int as isize) = (i >> 24 as libc::c_int) as libc::c_uchar;
    *ch.offset(1 as libc::c_int as isize) =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *ch.offset(2 as libc::c_int as isize) =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *ch.offset(3 as libc::c_int as isize) =
        (i & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[c2rust::src_loc = "52:1"]
unsafe fn char_to_int(mut ch: *mut libc::c_uchar) -> u32 {
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
                    stderr(),
                    b"-merge parameter must be at least 1.\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if merge > 48 as libc::c_int {
                fprintf(
                    stderr(),
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
                stderr(),
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
            stderr(),
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
            stderr(),
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
            err = fread(ch.as_mut_ptr() as *mut libc::c_void, 1, 4, fin) as libc::c_int;
            len[i as usize] = char_to_int(ch.as_mut_ptr()) as libc::c_int;
            if len[i as usize] > 1500 as libc::c_int || len[i as usize] < 0 as libc::c_int {
                if feof(fin) != 0 {
                    eof = 1 as libc::c_int;
                } else {
                    fprintf(
                        stderr(),
                        b"Invalid payload length\n\0" as *const u8 as *const libc::c_char,
                    );
                    fclose(fin);
                    fclose(fout);
                    return 1 as libc::c_int;
                }
                break;
            } else {
                err = fread(ch.as_mut_ptr() as *mut libc::c_void, 1, 4, fin) as libc::c_int;
                rng[i as usize] = char_to_int(ch.as_mut_ptr()) as libc::c_int;
                err = fread(
                    (packets[i as usize]).as_mut_ptr() as *mut libc::c_void,
                    1,
                    len[i as usize] as _,
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
                            stderr(),
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
                if fwrite(int_field.as_mut_ptr() as *const libc::c_void, 1, 4, fout) != 4 {
                    fprintf(
                        stderr(),
                        b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                int_to_char(
                    rng[(nb_packets - 1 as libc::c_int) as usize] as u32,
                    int_field.as_mut_ptr(),
                );
                if fwrite(int_field.as_mut_ptr() as *const libc::c_void, 1, 4, fout) != 4 {
                    fprintf(
                        stderr(),
                        b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                if fwrite(
                    output_packet.as_mut_ptr() as *const libc::c_void,
                    1,
                    err as _,
                    fout,
                ) != err as _
                {
                    fprintf(
                        stderr(),
                        b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            } else {
                fprintf(
                    stderr(),
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
                    if fwrite(int_field_0.as_mut_ptr() as *const libc::c_void, 1, 4, fout) != 4 {
                        fprintf(
                            stderr(),
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
                    if fwrite(int_field_0.as_mut_ptr() as *const libc::c_void, 1, 4, fout) != 4 {
                        fprintf(
                            stderr(),
                            b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    if fwrite(
                        output_packet.as_mut_ptr() as *const libc::c_void,
                        1,
                        err as _,
                        fout,
                    ) != err as _
                    {
                        fprintf(
                            stderr(),
                            b"Error writing.\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                } else {
                    fprintf(
                        stderr(),
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
