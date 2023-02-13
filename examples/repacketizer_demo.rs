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
pub unsafe fn usage(mut argv0: *mut i8) {
    fprintf(
        stderr(),
        b"usage: %s [options] input_file output_file\n\0" as *const u8 as *const i8,
        argv0,
    );
}
#[c2rust::src_loc = "44:1"]
unsafe fn int_to_char(mut i: u32, mut ch: *mut u8) {
    *ch.offset(0 as i32 as isize) = (i >> 24 as i32) as u8;
    *ch.offset(1 as i32 as isize) = (i >> 16 as i32 & 0xff as i32 as u32) as u8;
    *ch.offset(2 as i32 as isize) = (i >> 8 as i32 & 0xff as i32 as u32) as u8;
    *ch.offset(3 as i32 as isize) = (i & 0xff as i32 as u32) as u8;
}
#[c2rust::src_loc = "52:1"]
unsafe fn char_to_int(mut ch: *mut u8) -> u32 {
    (*ch.offset(0 as i32 as isize) as u32) << 24 as i32
        | (*ch.offset(1 as i32 as isize) as u32) << 16 as i32
        | (*ch.offset(2 as i32 as isize) as u32) << 8 as i32
        | *ch.offset(3 as i32 as isize) as u32
}
#[c2rust::src_loc = "58:1"]
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut eof: i32 = 0 as i32;
    let mut fin: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut fout: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut packets: [[u8; 1500]; 48] = [[0; 1500]; 48];
    let mut len: [i32; 48] = [0; 48];
    let mut rng: [i32; 48] = [0; 48];
    let mut rp: *mut OpusRepacketizer = std::ptr::null_mut::<OpusRepacketizer>();
    let mut output_packet: [u8; 32000] = [0; 32000];
    let mut merge: i32 = 1 as i32;
    let mut split: i32 = 0 as i32;
    if argc < 3 as i32 {
        usage(*argv.offset(0 as i32 as isize));
        return 1 as i32;
    }
    i = 1 as i32;
    while i < argc - 2 as i32 {
        if strcmp(
            *argv.offset(i as isize),
            b"-merge\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            merge = atoi(*argv.offset((i + 1 as i32) as isize));
            if merge < 1 as i32 {
                fprintf(
                    stderr(),
                    b"-merge parameter must be at least 1.\n\0" as *const u8 as *const i8,
                );
                return 1 as i32;
            }
            if merge > 48 as i32 {
                fprintf(
                    stderr(),
                    b"-merge parameter must be less than 48.\n\0" as *const u8 as *const i8,
                );
                return 1 as i32;
            }
            i += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-split\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            split = 1 as i32;
        } else {
            fprintf(
                stderr(),
                b"Unknown option: %s\n\0" as *const u8 as *const i8,
                *argv.offset(i as isize),
            );
            usage(*argv.offset(0 as i32 as isize));
            return 1 as i32;
        }
        i += 1;
    }
    fin = fopen(
        *argv.offset((argc - 2 as i32) as isize),
        b"r\0" as *const u8 as *const i8,
    );
    if fin.is_null() {
        fprintf(
            stderr(),
            b"Error opening input file: %s\n\0" as *const u8 as *const i8,
            *argv.offset((argc - 2 as i32) as isize),
        );
        return 1 as i32;
    }
    fout = fopen(
        *argv.offset((argc - 1 as i32) as isize),
        b"w\0" as *const u8 as *const i8,
    );
    if fout.is_null() {
        fprintf(
            stderr(),
            b"Error opening output file: %s\n\0" as *const u8 as *const i8,
            *argv.offset((argc - 1 as i32) as isize),
        );
        fclose(fin);
        return 1 as i32;
    }
    rp = opus_repacketizer_create();
    while eof == 0 {
        let mut err: i32 = 0;
        let mut nb_packets: i32 = merge;
        opus_repacketizer_init(rp);
        i = 0 as i32;
        while i < nb_packets {
            let mut ch: [u8; 4] = [0; 4];
            err = fread(ch.as_mut_ptr() as *mut core::ffi::c_void, 1, 4, fin) as i32;
            len[i as usize] = char_to_int(ch.as_mut_ptr()) as i32;
            if len[i as usize] > 1500 as i32 || len[i as usize] < 0 as i32 {
                if feof(fin) != 0 {
                    eof = 1 as i32;
                } else {
                    fprintf(
                        stderr(),
                        b"Invalid payload length\n\0" as *const u8 as *const i8,
                    );
                    fclose(fin);
                    fclose(fout);
                    return 1 as i32;
                }
                break;
            } else {
                err = fread(ch.as_mut_ptr() as *mut core::ffi::c_void, 1, 4, fin) as i32;
                rng[i as usize] = char_to_int(ch.as_mut_ptr()) as i32;
                err = fread(
                    (packets[i as usize]).as_mut_ptr() as *mut core::ffi::c_void,
                    1,
                    len[i as usize] as _,
                    fin,
                ) as i32;
                if feof(fin) != 0 {
                    eof = 1 as i32;
                    break;
                } else {
                    err = opus_repacketizer_cat(
                        rp,
                        (packets[i as usize]).as_mut_ptr(),
                        len[i as usize],
                    );
                    if err != 0 as i32 {
                        fprintf(
                            stderr(),
                            b"opus_repacketizer_cat() failed: %s\n\0" as *const u8 as *const i8,
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
            err = opus_repacketizer_out(rp, output_packet.as_mut_ptr(), 32000 as i32);
            if err > 0 as i32 {
                let mut int_field: [u8; 4] = [0; 4];
                int_to_char(err as u32, int_field.as_mut_ptr());
                if fwrite(
                    int_field.as_mut_ptr() as *const core::ffi::c_void,
                    1,
                    4,
                    fout,
                ) != 4
                {
                    fprintf(stderr(), b"Error writing.\n\0" as *const u8 as *const i8);
                    return 1 as i32;
                }
                int_to_char(
                    rng[(nb_packets - 1 as i32) as usize] as u32,
                    int_field.as_mut_ptr(),
                );
                if fwrite(
                    int_field.as_mut_ptr() as *const core::ffi::c_void,
                    1,
                    4,
                    fout,
                ) != 4
                {
                    fprintf(stderr(), b"Error writing.\n\0" as *const u8 as *const i8);
                    return 1 as i32;
                }
                if fwrite(
                    output_packet.as_mut_ptr() as *const core::ffi::c_void,
                    1,
                    err as _,
                    fout,
                ) != err as _
                {
                    fprintf(stderr(), b"Error writing.\n\0" as *const u8 as *const i8);
                    return 1 as i32;
                }
            } else {
                fprintf(
                    stderr(),
                    b"opus_repacketizer_out() failed: %s\n\0" as *const u8 as *const i8,
                    opus_strerror(err),
                );
            }
        } else {
            let mut nb_frames: i32 = opus_repacketizer_get_nb_frames(rp);
            i = 0 as i32;
            while i < nb_frames {
                err = opus_repacketizer_out_range(
                    rp,
                    i,
                    i + 1 as i32,
                    output_packet.as_mut_ptr(),
                    32000 as i32,
                );
                if err > 0 as i32 {
                    let mut int_field_0: [u8; 4] = [0; 4];
                    int_to_char(err as u32, int_field_0.as_mut_ptr());
                    if fwrite(
                        int_field_0.as_mut_ptr() as *const core::ffi::c_void,
                        1,
                        4,
                        fout,
                    ) != 4
                    {
                        fprintf(stderr(), b"Error writing.\n\0" as *const u8 as *const i8);
                        return 1 as i32;
                    }
                    if i == nb_frames - 1 as i32 {
                        int_to_char(
                            rng[(nb_packets - 1 as i32) as usize] as u32,
                            int_field_0.as_mut_ptr(),
                        );
                    } else {
                        int_to_char(0 as i32 as u32, int_field_0.as_mut_ptr());
                    }
                    if fwrite(
                        int_field_0.as_mut_ptr() as *const core::ffi::c_void,
                        1,
                        4,
                        fout,
                    ) != 4
                    {
                        fprintf(stderr(), b"Error writing.\n\0" as *const u8 as *const i8);
                        return 1 as i32;
                    }
                    if fwrite(
                        output_packet.as_mut_ptr() as *const core::ffi::c_void,
                        1,
                        err as _,
                        fout,
                    ) != err as _
                    {
                        fprintf(stderr(), b"Error writing.\n\0" as *const u8 as *const i8);
                        return 1 as i32;
                    }
                } else {
                    fprintf(
                        stderr(),
                        b"opus_repacketizer_out() failed: %s\n\0" as *const u8 as *const i8,
                        opus_strerror(err),
                    );
                }
                i += 1;
            }
        }
    }
    fclose(fin);
    fclose(fout);
    0 as i32
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}
