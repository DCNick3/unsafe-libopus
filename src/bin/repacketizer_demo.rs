#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::ffi::CStr;
use std::fs::File;
use std::io::{Read, Seek, Write};

use unsafe_libopus::{
    opus_repacketizer_cat, opus_repacketizer_create, opus_repacketizer_get_nb_frames,
    opus_repacketizer_init, opus_repacketizer_out, opus_repacketizer_out_range, opus_strerror,
    OpusRepacketizer,
};
fn usage(argv0: &str) {
    eprintln!("usage: {} [options] input_file output_file", argv0);
}

fn is_eof(file: &mut File) -> bool {
    let position = file.seek(std::io::SeekFrom::Current(0)).expect("seeking");
    let end = file.seek(std::io::SeekFrom::End(0)).expect("seeking");
    file.seek(std::io::SeekFrom::Start(position))
        .expect("seeking");
    position == end
}

unsafe fn main_0() -> i32 {
    let mut packets: [[u8; 1500]; 48] = [[0; 1500]; 48];
    let mut len: [i32; 48] = [0; 48];
    let mut rng: [i32; 48] = [0; 48];
    let mut output_packet: [u8; 32000] = [0; 32000];

    let mut merge: usize = 1;
    let mut split: bool = false;

    let mut args = std::env::args();
    let argc = args.len();
    let argv0 = args.next().unwrap();

    if argc < 3 {
        usage(&argv0);
        return 1;
    }

    while let Some(arg) = args.next() {
        if arg == "-merge" {
            let merge_str = args.next().expect("-merge requires a parameter");
            merge = merge_str
                .parse()
                .expect("-merge parameter must be an integer");
            if merge < 1 {
                eprintln!("-merge parameter must be at least 1.");
                return 1;
            }
            if merge > 48 {
                eprintln!("-merge parameter must be less than 48.");
                return 1;
            }
        } else if arg == "-split" {
            split = true;
        } else {
            eprintln!("Unknown option: {}", arg);
            usage(&argv0);
            return 1;
        }
    }

    let mut fin =
        File::open(args.next().expect("input file argument")).expect("opening input file");
    let mut fout =
        File::create(args.next().expect("output file argument")).expect("opening output file");

    // TODO: cleanup on exit
    let mut rp: *mut OpusRepacketizer = opus_repacketizer_create();
    let mut eof = false;
    while !eof {
        let mut err: i32 = 0;
        let mut nb_packets: usize = merge;
        opus_repacketizer_init(rp);
        let mut i = 0 as usize;
        while i < nb_packets {
            let mut ch: [u8; 4] = [0; 4];
            fin.read_exact(&mut ch).unwrap();
            len[i] = i32::from_be_bytes(ch);
            if len[i] > 1500 || len[i] < 0 {
                if is_eof(&mut fin) {
                    eof = true;
                } else {
                    eprintln!("Invalid payload length");
                    return 1;
                }
                break;
            } else {
                fin.read_exact(&mut ch).unwrap();
                rng[i] = i32::from_be_bytes(ch);
                fin.read_exact(&mut packets[i][..len[i] as usize]).unwrap();
                if is_eof(&mut fin) {
                    eof = true;
                    break;
                } else {
                    err = opus_repacketizer_cat(
                        rp,
                        (packets[i as usize]).as_mut_ptr(),
                        len[i as usize],
                    );
                    if err != 0 as i32 {
                        let err = CStr::from_ptr(opus_strerror(err) as _);
                        let err = err.to_str().unwrap();
                        eprintln!("opus_repacketizer_cat() failed: {}", err);
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        nb_packets = i;
        if eof {
            break;
        }
        if !split {
            err = opus_repacketizer_out(rp, output_packet.as_mut_ptr(), 32000);
            if err > 0 as i32 {
                let mut int_field: [u8; 4] = err.to_be_bytes();
                fout.write_all(&int_field).unwrap();

                int_field = rng[nb_packets - 1].to_be_bytes();
                fout.write_all(&int_field).unwrap();

                fout.write_all(&output_packet[..err as usize]).unwrap();
            } else {
                let err = CStr::from_ptr(opus_strerror(err) as _);
                let err = err.to_str().unwrap();
                eprintln!("opus_repacketizer_out() failed: {}", err);
            }
        } else {
            let mut nb_frames: i32 = opus_repacketizer_get_nb_frames(rp);
            let mut i = 0;
            while i < nb_frames {
                err = opus_repacketizer_out_range(
                    rp,
                    i,
                    i + 1 as i32,
                    output_packet.as_mut_ptr(),
                    32000 as i32,
                );
                if err > 0 as i32 {
                    let mut int_field_0: [u8; 4] = err.to_be_bytes();
                    fout.write_all(&int_field_0).unwrap();
                    int_field_0 = if i == nb_frames - 1 as i32 {
                        rng[nb_packets - 1].to_be_bytes()
                    } else {
                        0i32.to_be_bytes()
                    };
                    fout.write_all(&int_field_0).unwrap();

                    fout.write_all(&output_packet[..err as usize]).unwrap();
                } else {
                    let err = CStr::from_ptr(opus_strerror(err) as _);
                    let err = err.to_str().unwrap();
                    eprintln!("opus_repacketizer_out_range() failed: {}", err);
                }
                i += 1;
            }
        }
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0()) }
}
