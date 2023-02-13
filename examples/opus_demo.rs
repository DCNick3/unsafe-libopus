#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(register_tool)]
#![register_tool(c2rust)]

use libc::{
    abs, atoi, atol, fclose, fopen, fprintf, fread, fseek, ftell, fwrite, printf, rand, FILE,
};
use libc_stdhandle::stderr;

type size_t = u64;
use unsafe_libopus::externs::strcmp;
use unsafe_libopus::externs::{calloc, free, malloc};
use unsafe_libopus::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy, opus_encode,
    opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, opus_get_version_string,
    opus_packet_get_nb_frames, opus_packet_get_samples_per_frame, opus_strerror, OpusDecoder,
    OpusEncoder,
};

#[c2rust::src_loc = "45:1"]
pub unsafe fn print_usage(mut argv: *mut *mut i8) {
    fprintf(
        stderr(),
        b"Usage: %s [-e] <application> <sampling rate (Hz)> <channels (1/2)> <bits per second>  [options] <input> <output>\n\0"
            as *const u8 as *const i8,
        *argv.offset(0 as i32 as isize),
    );
    fprintf(
        stderr(),
        b"       %s -d <sampling rate (Hz)> <channels (1/2)> [options] <input> <output>\n\n\0"
            as *const u8 as *const i8,
        *argv.offset(0 as i32 as isize),
    );
    fprintf(
        stderr(),
        b"application: voip | audio | restricted-lowdelay\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr(), b"options:\n\0" as *const u8 as *const i8);
    fprintf(
        stderr(),
        b"-e                   : only runs the encoder (output the bit-stream)\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr(),
        b"-d                   : only runs the decoder (reads the bit-stream as input)\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-cbr                 : enable constant bitrate; default: variable bitrate\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-cvbr                : enable constrained variable bitrate; default: unconstrained\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-delayed-decision    : use look-ahead for speech/music detection (experts only); default: disabled\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-bandwidth <NB|MB|WB|SWB|FB> : audio bandwidth (from narrowband to fullband); default: sampling rate\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-framesize <2.5|5|10|20|40|60|80|100|120> : frame size in ms; default: 20 \n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-max_payload <bytes> : maximum payload size in bytes, default: 1024\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr(),
        b"-complexity <comp>   : complexity, 0 (lowest) ... 10 (highest); default: 10\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-inbandfec           : enable SILK inband FEC\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-forcemono           : force mono encoding, even for stereo input\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr(),
        b"-dtx                 : enable SILK DTX\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr(),
        b"-loss <perc>         : simulate packet loss, in percent (0-100); default: 0\n\0"
            as *const u8 as *const i8,
    );
}
#[c2rust::src_loc = "68:1"]
unsafe fn int_to_char(mut i: u32, mut ch: *mut u8) {
    *ch.offset(0 as i32 as isize) = (i >> 24 as i32) as u8;
    *ch.offset(1 as i32 as isize) = (i >> 16 as i32 & 0xff as i32 as u32) as u8;
    *ch.offset(2 as i32 as isize) = (i >> 8 as i32 & 0xff as i32 as u32) as u8;
    *ch.offset(3 as i32 as isize) = (i & 0xff as i32 as u32) as u8;
}
#[c2rust::src_loc = "76:1"]
unsafe fn char_to_int(mut ch: *mut u8) -> u32 {
    (*ch.offset(0 as i32 as isize) as u32) << 24 as i32
        | (*ch.offset(1 as i32 as isize) as u32) << 16 as i32
        | (*ch.offset(2 as i32 as isize) as u32) << 8 as i32
        | *ch.offset(3 as i32 as isize) as u32
}
#[c2rust::src_loc = "84:18"]
static mut silk8_test: [[i32; 4]; 8] = [
    [1000 as i32, 1101 as i32, 960 as i32 * 3 as i32, 1 as i32],
    [1000 as i32, 1101 as i32, 960 as i32 * 2 as i32, 1 as i32],
    [1000 as i32, 1101 as i32, 960 as i32, 1 as i32],
    [1000 as i32, 1101 as i32, 480 as i32, 1 as i32],
    [1000 as i32, 1101 as i32, 960 as i32 * 3 as i32, 2 as i32],
    [1000 as i32, 1101 as i32, 960 as i32 * 2 as i32, 2 as i32],
    [1000 as i32, 1101 as i32, 960 as i32, 2 as i32],
    [1000 as i32, 1101 as i32, 480 as i32, 2 as i32],
];
#[c2rust::src_loc = "95:18"]
static mut silk12_test: [[i32; 4]; 8] = [
    [1000 as i32, 1102 as i32, 960 as i32 * 3 as i32, 1 as i32],
    [1000 as i32, 1102 as i32, 960 as i32 * 2 as i32, 1 as i32],
    [1000 as i32, 1102 as i32, 960 as i32, 1 as i32],
    [1000 as i32, 1102 as i32, 480 as i32, 1 as i32],
    [1000 as i32, 1102 as i32, 960 as i32 * 3 as i32, 2 as i32],
    [1000 as i32, 1102 as i32, 960 as i32 * 2 as i32, 2 as i32],
    [1000 as i32, 1102 as i32, 960 as i32, 2 as i32],
    [1000 as i32, 1102 as i32, 480 as i32, 2 as i32],
];
#[c2rust::src_loc = "106:18"]
static mut silk16_test: [[i32; 4]; 8] = [
    [1000 as i32, 1103 as i32, 960 as i32 * 3 as i32, 1 as i32],
    [1000 as i32, 1103 as i32, 960 as i32 * 2 as i32, 1 as i32],
    [1000 as i32, 1103 as i32, 960 as i32, 1 as i32],
    [1000 as i32, 1103 as i32, 480 as i32, 1 as i32],
    [1000 as i32, 1103 as i32, 960 as i32 * 3 as i32, 2 as i32],
    [1000 as i32, 1103 as i32, 960 as i32 * 2 as i32, 2 as i32],
    [1000 as i32, 1103 as i32, 960 as i32, 2 as i32],
    [1000 as i32, 1103 as i32, 480 as i32, 2 as i32],
];
#[c2rust::src_loc = "117:18"]
static mut hybrid24_test: [[i32; 4]; 4] = [
    [1000 as i32, 1104 as i32, 960 as i32, 1 as i32],
    [1000 as i32, 1104 as i32, 480 as i32, 1 as i32],
    [1000 as i32, 1104 as i32, 960 as i32, 2 as i32],
    [1000 as i32, 1104 as i32, 480 as i32, 2 as i32],
];
#[c2rust::src_loc = "124:18"]
static mut hybrid48_test: [[i32; 4]; 4] = [
    [1000 as i32, 1105 as i32, 960 as i32, 1 as i32],
    [1000 as i32, 1105 as i32, 480 as i32, 1 as i32],
    [1000 as i32, 1105 as i32, 960 as i32, 2 as i32],
    [1000 as i32, 1105 as i32, 480 as i32, 2 as i32],
];
#[c2rust::src_loc = "131:18"]
static mut celt_test: [[i32; 4]; 32] = [
    [1002 as i32, 1105 as i32, 960 as i32, 1 as i32],
    [1002 as i32, 1104 as i32, 960 as i32, 1 as i32],
    [1002 as i32, 1103 as i32, 960 as i32, 1 as i32],
    [1002 as i32, 1101 as i32, 960 as i32, 1 as i32],
    [1002 as i32, 1105 as i32, 480 as i32, 1 as i32],
    [1002 as i32, 1104 as i32, 480 as i32, 1 as i32],
    [1002 as i32, 1103 as i32, 480 as i32, 1 as i32],
    [1002 as i32, 1101 as i32, 480 as i32, 1 as i32],
    [1002 as i32, 1105 as i32, 240 as i32, 1 as i32],
    [1002 as i32, 1104 as i32, 240 as i32, 1 as i32],
    [1002 as i32, 1103 as i32, 240 as i32, 1 as i32],
    [1002 as i32, 1101 as i32, 240 as i32, 1 as i32],
    [1002 as i32, 1105 as i32, 120 as i32, 1 as i32],
    [1002 as i32, 1104 as i32, 120 as i32, 1 as i32],
    [1002 as i32, 1103 as i32, 120 as i32, 1 as i32],
    [1002 as i32, 1101 as i32, 120 as i32, 1 as i32],
    [1002 as i32, 1105 as i32, 960 as i32, 2 as i32],
    [1002 as i32, 1104 as i32, 960 as i32, 2 as i32],
    [1002 as i32, 1103 as i32, 960 as i32, 2 as i32],
    [1002 as i32, 1101 as i32, 960 as i32, 2 as i32],
    [1002 as i32, 1105 as i32, 480 as i32, 2 as i32],
    [1002 as i32, 1104 as i32, 480 as i32, 2 as i32],
    [1002 as i32, 1103 as i32, 480 as i32, 2 as i32],
    [1002 as i32, 1101 as i32, 480 as i32, 2 as i32],
    [1002 as i32, 1105 as i32, 240 as i32, 2 as i32],
    [1002 as i32, 1104 as i32, 240 as i32, 2 as i32],
    [1002 as i32, 1103 as i32, 240 as i32, 2 as i32],
    [1002 as i32, 1101 as i32, 240 as i32, 2 as i32],
    [1002 as i32, 1105 as i32, 120 as i32, 2 as i32],
    [1002 as i32, 1104 as i32, 120 as i32, 2 as i32],
    [1002 as i32, 1103 as i32, 120 as i32, 2 as i32],
    [1002 as i32, 1101 as i32, 120 as i32, 2 as i32],
];
#[c2rust::src_loc = "174:18"]
static mut celt_hq_test: [[i32; 4]; 4] = [
    [1002 as i32, 1105 as i32, 960 as i32, 2 as i32],
    [1002 as i32, 1105 as i32, 480 as i32, 2 as i32],
    [1002 as i32, 1105 as i32, 240 as i32, 2 as i32],
    [1002 as i32, 1105 as i32, 120 as i32, 2 as i32],
];
#[c2rust::src_loc = "210:1"]
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut current_block: u64;
    let mut err: i32 = 0;
    let mut inFile: *mut i8 = std::ptr::null_mut::<i8>();
    let mut outFile: *mut i8 = std::ptr::null_mut::<i8>();
    let mut fin: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut fout: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut args: i32 = 0;
    let mut len: [i32; 2] = [0; 2];
    let mut frame_size: i32 = 0;
    let mut channels: i32 = 0;
    let mut bitrate_bps: i32 = 0 as i32;
    let mut data: [*mut u8; 2] = [std::ptr::null_mut::<u8>(), std::ptr::null_mut::<u8>()];
    let mut fbytes: *mut u8 = std::ptr::null_mut::<u8>();
    let mut sampling_rate: i32 = 0;
    let mut use_vbr: i32 = 0;
    let mut max_payload_bytes: i32 = 0;
    let mut complexity: i32 = 0;
    let mut use_inbandfec: i32 = 0;
    let mut use_dtx: i32 = 0;
    let mut forcechannels: i32 = 0;
    let mut cvbr: i32 = 0 as i32;
    let mut packet_loss_perc: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut count_act: i32 = 0 as i32;
    let mut k: i32 = 0;
    let mut skip: i32 = 0 as i32;
    let mut stop: i32 = 0 as i32;
    let mut in_0: *mut libc::c_short = std::ptr::null_mut::<libc::c_short>();
    let mut out: *mut libc::c_short = std::ptr::null_mut::<libc::c_short>();
    let mut application: i32 = 2049 as i32;
    let mut bits: f64 = 0.0f64;
    let mut bits_max: f64 = 0.0f64;
    let mut bits_act: f64 = 0.0f64;
    let mut bits2: f64 = 0.0f64;
    let mut nrg: f64 = 0.;
    let mut tot_samples: f64 = 0 as i32 as f64;
    let mut tot_in: u64 = 0;
    let mut tot_out: u64 = 0;
    let mut bandwidth: i32 = -(1000 as i32);
    let mut bandwidth_string: *const i8 = std::ptr::null::<i8>();
    let mut lost: i32 = 0 as i32;
    let mut lost_prev: i32 = 1 as i32;
    let mut toggle: i32 = 0 as i32;
    let mut enc_final_range: [u32; 2] = [0; 2];
    let mut dec_final_range: u32 = 0;
    let mut encode_only: i32 = 0 as i32;
    let mut decode_only: i32 = 0 as i32;
    let mut max_frame_size: i32 = 48000 as i32 * 2 as i32;
    let mut num_read: size_t = 0;
    let mut curr_read: i32 = 0 as i32;
    let mut sweep_bps: i32 = 0 as i32;
    let mut random_framesize: i32 = 0 as i32;
    let mut newsize: i32 = 0 as i32;
    let mut delayed_celt: i32 = 0 as i32;
    let mut sweep_max: i32 = 0 as i32;
    let mut sweep_min: i32 = 0 as i32;
    let mut random_fec: i32 = 0 as i32;
    let mut mode_list: *const [i32; 4] = std::ptr::null::<[i32; 4]>();
    let mut nb_modes_in_list: i32 = 0 as i32;
    let mut curr_mode: i32 = 0 as i32;
    let mut curr_mode_count: i32 = 0 as i32;
    let mut mode_switch_time: i32 = 48000 as i32;
    let mut nb_encoded: i32 = 0 as i32;
    let mut remaining: i32 = 0 as i32;
    let mut variable_duration: i32 = 5000 as i32;
    let mut delayed_decision: i32 = 0 as i32;
    let mut ret: i32 = 1 as i32;
    if argc < 5 as i32 {
        print_usage(argv);
    } else {
        tot_out = 0 as i32 as u64;
        tot_in = tot_out;
        fprintf(
            stderr(),
            b"%s\n\0" as *const u8 as *const i8,
            opus_get_version_string(),
        );
        args = 1 as i32;
        if strcmp(
            *argv.offset(args as isize),
            b"-e\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            encode_only = 1 as i32;
            args += 1;
        } else if strcmp(
            *argv.offset(args as isize),
            b"-d\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            decode_only = 1 as i32;
            args += 1;
        }
        if decode_only == 0 && argc < 7 as i32 {
            print_usage(argv);
        } else {
            if decode_only == 0 {
                if strcmp(
                    *argv.offset(args as isize),
                    b"voip\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    application = 2048 as i32;
                    current_block = 6450597802325118133;
                } else if strcmp(
                    *argv.offset(args as isize),
                    b"restricted-lowdelay\0" as *const u8 as *const i8,
                ) == 0 as i32
                {
                    application = 2051 as i32;
                    current_block = 6450597802325118133;
                } else if strcmp(
                    *argv.offset(args as isize),
                    b"audio\0" as *const u8 as *const i8,
                ) != 0 as i32
                {
                    fprintf(
                        stderr(),
                        b"unknown application: %s\n\0" as *const u8 as *const i8,
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
                    sampling_rate = atol(*argv.offset(args as isize)) as i32;
                    args += 1;
                    if sampling_rate != 8000 as i32
                        && sampling_rate != 12000 as i32
                        && sampling_rate != 16000 as i32
                        && sampling_rate != 24000 as i32
                        && sampling_rate != 48000 as i32
                    {
                        fprintf(
                            stderr(),
                            b"Supported sampling rates are 8000, 12000, 16000, 24000 and 48000.\n\0"
                                as *const u8 as *const i8,
                        );
                    } else {
                        frame_size = sampling_rate / 50 as i32;
                        channels = atoi(*argv.offset(args as isize));
                        args += 1;
                        if channels < 1 as i32 || channels > 2 as i32 {
                            fprintf(
                                stderr(),
                                b"Opus_demo supports only 1 or 2 channels.\n\0" as *const u8
                                    as *const i8,
                            );
                        } else {
                            if decode_only == 0 {
                                bitrate_bps = atol(*argv.offset(args as isize)) as i32;
                                args += 1;
                            }
                            use_vbr = 1 as i32;
                            max_payload_bytes = 1500 as i32;
                            complexity = 10 as i32;
                            use_inbandfec = 0 as i32;
                            forcechannels = -(1000 as i32);
                            use_dtx = 0 as i32;
                            packet_loss_perc = 0 as i32;
                            loop {
                                if args >= argc - 2 as i32 {
                                    current_block = 10024259685434459487;
                                    break;
                                }
                                if strcmp(
                                    *argv.offset(args as isize),
                                    b"-cbr\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-cbr\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        use_vbr = 0 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-bandwidth\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-bandwidth\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"NB\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            bandwidth = 1101 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"MB\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            bandwidth = 1102 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"WB\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            bandwidth = 1103 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"SWB\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            bandwidth = 1104 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"FB\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            bandwidth = 1105 as i32;
                                        } else {
                                            fprintf(
                                                stderr(),
                                                b"Unknown bandwidth %s. Supported are NB, MB, WB, SWB, FB.\n\0"
                                                    as *const u8 as *const i8,
                                                *argv.offset((args + 1 as i32) as isize),
                                            );
                                            current_block = 14460699602747363466;
                                            break;
                                        }
                                        args += 2 as i32;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-framesize\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-framesize\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"2.5\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = sampling_rate / 400 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"5\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = sampling_rate / 200 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"10\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = sampling_rate / 100 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"20\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = sampling_rate / 50 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"40\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = sampling_rate / 25 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"60\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = 3 as i32 * sampling_rate / 50 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"80\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = 4 as i32 * sampling_rate / 50 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"100\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = 5 as i32 * sampling_rate / 50 as i32;
                                        } else if strcmp(
                                            *argv.offset((args + 1 as i32) as isize),
                                            b"120\0" as *const u8 as *const i8,
                                        ) == 0 as i32
                                        {
                                            frame_size = 6 as i32 * sampling_rate / 50 as i32;
                                        } else {
                                            fprintf(
                                                stderr(),
                                                b"Unsupported frame size: %s ms. Supported are 2.5, 5, 10, 20, 40, 60, 80, 100, 120.\n\0"
                                                    as *const u8 as *const i8,
                                                *argv.offset((args + 1 as i32) as isize),
                                            );
                                            current_block = 14460699602747363466;
                                            break;
                                        }
                                        args += 2 as i32;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-max_payload\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-max_payload\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        max_payload_bytes =
                                            atoi(*argv.offset((args + 1 as i32) as isize));
                                        args += 2 as i32;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-complexity\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-complexity\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        complexity = atoi(*argv.offset((args + 1 as i32) as isize));
                                        args += 2 as i32;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-inbandfec\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    use_inbandfec = 1 as i32;
                                    args += 1;
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-forcemono\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-forcemono\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        forcechannels = 1 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-cvbr\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-cvbr\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        cvbr = 1 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-delayed-decision\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-delayed-decision\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        delayed_decision = 1 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-dtx\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-dtx\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        use_dtx = 1 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-loss\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    packet_loss_perc =
                                        atoi(*argv.offset((args + 1 as i32) as isize));
                                    args += 2 as i32;
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-sweep\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-sweep\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        sweep_bps = atoi(*argv.offset((args + 1 as i32) as isize));
                                        args += 2 as i32;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-random_framesize\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-random_framesize\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        random_framesize = 1 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-sweep_max\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-sweep_max\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        sweep_max = atoi(*argv.offset((args + 1 as i32) as isize));
                                        args += 2 as i32;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-random_fec\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-random_fec\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        random_fec = 1 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-silk8k_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-silk8k_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = silk8_test.as_ptr();
                                        nb_modes_in_list = 8 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-silk12k_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-silk12k_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = silk12_test.as_ptr();
                                        nb_modes_in_list = 8 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-silk16k_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-silk16k_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = silk16_test.as_ptr();
                                        nb_modes_in_list = 8 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-hybrid24k_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-hybrid24k_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = hybrid24_test.as_ptr();
                                        nb_modes_in_list = 4 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-hybrid48k_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-hybrid48k_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = hybrid48_test.as_ptr();
                                        nb_modes_in_list = 4 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-celt_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-celt_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = celt_test.as_ptr();
                                        nb_modes_in_list = 32 as i32;
                                        args += 1;
                                    }
                                } else if strcmp(
                                    *argv.offset(args as isize),
                                    b"-celt_hq_test\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                {
                                    if decode_only != 0 {
                                        fprintf(
                                            stderr(),
                                            b"option %s is only for encoding\n\0" as *const u8
                                                as *const i8,
                                            b"-celt_hq_test\0" as *const u8 as *const i8,
                                        );
                                        current_block = 14460699602747363466;
                                        break;
                                    } else {
                                        mode_list = celt_hq_test.as_ptr();
                                        nb_modes_in_list = 4 as i32;
                                        args += 1;
                                    }
                                } else {
                                    printf(
                                        b"Error: unrecognized setting: %s\n\n\0" as *const u8
                                            as *const i8,
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
                                    if max_payload_bytes < 0 as i32
                                        || max_payload_bytes > 1500 as i32
                                    {
                                        fprintf(
                                            stderr(),
                                            b"max_payload_bytes must be between 0 and %d\n\0"
                                                as *const u8
                                                as *const i8,
                                            1500 as i32,
                                        );
                                    } else {
                                        inFile = *argv.offset((argc - 2 as i32) as isize);
                                        fin = fopen(inFile, b"rb\0" as *const u8 as *const i8);
                                        if fin.is_null() {
                                            fprintf(
                                                stderr(),
                                                b"Could not open input file %s\n\0" as *const u8
                                                    as *const i8,
                                                *argv.offset((argc - 2 as i32) as isize),
                                            );
                                        } else {
                                            if !mode_list.is_null() {
                                                let mut size: i32 = 0;
                                                fseek(fin, 0 as i32 as i64, 2 as i32);
                                                size = ftell(fin) as i32;
                                                fprintf(
                                                    stderr(),
                                                    b"File size is %d bytes\n\0" as *const u8
                                                        as *const i8,
                                                    size,
                                                );
                                                fseek(fin, 0 as i32 as i64, 0 as i32);
                                                mode_switch_time = (size as u64)
                                                    .wrapping_div(::core::mem::size_of::<
                                                        libc::c_short,
                                                    >(
                                                    )
                                                        as u64)
                                                    .wrapping_div(channels as u64)
                                                    .wrapping_div(nb_modes_in_list as u64)
                                                    as i32;
                                                fprintf(
                                                    stderr(),
                                                    b"Switching mode every %d samples\n\0"
                                                        as *const u8
                                                        as *const i8,
                                                    mode_switch_time,
                                                );
                                            }
                                            outFile = *argv.offset((argc - 1 as i32) as isize);
                                            fout =
                                                fopen(outFile, b"wb+\0" as *const u8 as *const i8);
                                            if fout.is_null() {
                                                fprintf(
                                                    stderr(),
                                                    b"Could not open output file %s\n\0"
                                                        as *const u8
                                                        as *const i8,
                                                    *argv.offset((argc - 1 as i32) as isize),
                                                );
                                            } else {
                                                if decode_only == 0 {
                                                    enc = opus_encoder_create(
                                                        sampling_rate,
                                                        channels,
                                                        application,
                                                        &mut err,
                                                    );
                                                    if err != 0 as i32 {
                                                        fprintf(
                                                            stderr(),
                                                            b"Cannot create encoder: %s\n\0"
                                                                as *const u8
                                                                as *const i8,
                                                            opus_strerror(err),
                                                        );
                                                        current_block = 14460699602747363466;
                                                    } else {
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4002 as i32,
                                                            bitrate_bps,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4008 as i32,
                                                            bandwidth,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4006 as i32,
                                                            use_vbr,
                                                        );
                                                        opus_encoder_ctl!(enc, 4020 as i32, cvbr,);
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4010 as i32,
                                                            complexity,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4012 as i32,
                                                            use_inbandfec,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4022 as i32,
                                                            forcechannels,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4016 as i32,
                                                            use_dtx,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4014 as i32,
                                                            packet_loss_perc,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4027 as i32,
                                                            (&mut skip as *mut i32).offset(
                                                                (&mut skip as *mut i32).offset_from(
                                                                    &mut skip as *mut i32,
                                                                )
                                                                    as i64
                                                                    as isize,
                                                            ),
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4036 as i32,
                                                            16 as i32,
                                                        );
                                                        opus_encoder_ctl!(
                                                            enc,
                                                            4040 as i32,
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
                                                            if err != 0 as i32 {
                                                                fprintf(
                                                                    stderr(),
                                                                    b"Cannot create decoder: %s\n\0"
                                                                        as *const u8
                                                                        as *const i8,
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
                                                                        bandwidth_string =
                                                                            b"narrowband\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                    1102 => {
                                                                        bandwidth_string =
                                                                            b"mediumband\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                    1103 => {
                                                                        bandwidth_string =
                                                                            b"wideband\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                    1104 => {
                                                                        bandwidth_string =
                                                                            b"superwideband\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                    1105 => {
                                                                        bandwidth_string =
                                                                            b"fullband\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                    -1000 => {
                                                                        bandwidth_string =
                                                                            b"auto bandwidth\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                    _ => {
                                                                        bandwidth_string =
                                                                            b"unknown\0"
                                                                                as *const u8
                                                                                as *const i8;
                                                                    }
                                                                }
                                                                if decode_only != 0 {
                                                                    fprintf(
                                                                        stderr(),
                                                                        b"Decoding with %ld Hz output (%d channels)\n\0"
                                                                            as *const u8 as *const i8,
                                                                        sampling_rate as i64,
                                                                        channels,
                                                                    );
                                                                } else {
                                                                    fprintf(
                                                                        stderr(),
                                                                        b"Encoding %ld Hz input at %.3f kb/s in %s with %d-sample frames.\n\0"
                                                                            as *const u8 as *const i8,
                                                                        sampling_rate as i64,
                                                                        bitrate_bps as f64 * 0.001f64,
                                                                        bandwidth_string,
                                                                        frame_size,
                                                                    );
                                                                }
                                                                in_0 = malloc(
                                                                    ((max_frame_size * channels)
                                                                        as u64)
                                                                        .wrapping_mul(
                                                                            ::core::mem::size_of::<
                                                                                libc::c_short,
                                                                            >(
                                                                            )
                                                                                as u64,
                                                                        ),
                                                                )
                                                                    as *mut libc::c_short;
                                                                out = malloc(
                                                                    ((max_frame_size * channels)
                                                                        as u64)
                                                                        .wrapping_mul(
                                                                            ::core::mem::size_of::<
                                                                                libc::c_short,
                                                                            >(
                                                                            )
                                                                                as u64,
                                                                        ),
                                                                )
                                                                    as *mut libc::c_short;
                                                                fbytes = malloc(
                                                                    ((max_frame_size * channels)
                                                                        as u64)
                                                                        .wrapping_mul(
                                                                            ::core::mem::size_of::<
                                                                                libc::c_short,
                                                                            >(
                                                                            )
                                                                                as u64,
                                                                        ),
                                                                )
                                                                    as *mut u8;
                                                                data[0 as i32 as usize] = calloc(
                                                                    max_payload_bytes as u64,
                                                                    ::core::mem::size_of::<u8>()
                                                                        as u64,
                                                                )
                                                                    as *mut u8;
                                                                if use_inbandfec != 0 {
                                                                    data[1 as i32 as usize] = calloc(
                                                                        max_payload_bytes as u64,
                                                                        ::core::mem::size_of::<u8>()
                                                                            as u64,
                                                                    )
                                                                        as *mut u8;
                                                                }
                                                                if delayed_decision != 0 {
                                                                    if frame_size
                                                                        == sampling_rate
                                                                            / 400 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5001 as i32;
                                                                    } else if frame_size
                                                                        == sampling_rate
                                                                            / 200 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5002 as i32;
                                                                    } else if frame_size
                                                                        == sampling_rate
                                                                            / 100 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5003 as i32;
                                                                    } else if frame_size
                                                                        == sampling_rate / 50 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5004 as i32;
                                                                    } else if frame_size
                                                                        == sampling_rate / 25 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5005 as i32;
                                                                    } else if frame_size
                                                                        == 3 as i32 * sampling_rate
                                                                            / 50 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5006 as i32;
                                                                    } else if frame_size
                                                                        == 4 as i32 * sampling_rate
                                                                            / 50 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5007 as i32;
                                                                    } else if frame_size
                                                                        == 5 as i32 * sampling_rate
                                                                            / 50 as i32
                                                                    {
                                                                        variable_duration =
                                                                            5008 as i32;
                                                                    } else {
                                                                        variable_duration =
                                                                            5009 as i32;
                                                                    }
                                                                    opus_encoder_ctl!(
                                                                        enc,
                                                                        4040 as i32,
                                                                        variable_duration,
                                                                    );
                                                                    frame_size =
                                                                        2 as i32 * 48000 as i32;
                                                                }
                                                                loop {
                                                                    if stop != 0 {
                                                                        current_block =
                                                                            15240930316249348783;
                                                                        break;
                                                                    }
                                                                    if delayed_celt != 0 {
                                                                        frame_size = newsize;
                                                                        delayed_celt = 0 as i32;
                                                                    } else if random_framesize != 0
                                                                        && rand() % 20 as i32
                                                                            == 0 as i32
                                                                    {
                                                                        newsize = rand() % 6 as i32;
                                                                        match newsize {
                                                                            0 => {
                                                                                newsize =
                                                                                    sampling_rate
                                                                                        / 400
                                                                                            as i32;
                                                                            }
                                                                            1 => {
                                                                                newsize =
                                                                                    sampling_rate
                                                                                        / 200
                                                                                            as i32;
                                                                            }
                                                                            2 => {
                                                                                newsize =
                                                                                    sampling_rate
                                                                                        / 100
                                                                                            as i32;
                                                                            }
                                                                            3 => {
                                                                                newsize =
                                                                                    sampling_rate
                                                                                        / 50 as i32;
                                                                            }
                                                                            4 => {
                                                                                newsize =
                                                                                    sampling_rate
                                                                                        / 25 as i32;
                                                                            }
                                                                            5 => {
                                                                                newsize = 3 as i32
                                                                                    * sampling_rate
                                                                                    / 50 as i32;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        while newsize
                                                                            < sampling_rate
                                                                                / 25 as i32
                                                                            && bitrate_bps
                                                                                - abs(sweep_bps)
                                                                                <= 3 as i32
                                                                                    * 12 as i32
                                                                                    * sampling_rate
                                                                                    / newsize
                                                                        {
                                                                            newsize *= 2 as i32;
                                                                        }
                                                                        if newsize
                                                                            < sampling_rate
                                                                                / 100 as i32
                                                                            && frame_size
                                                                                >= sampling_rate
                                                                                    / 100 as i32
                                                                        {
                                                                            opus_encoder_ctl!(
                                                                                enc,
                                                                                11002 as i32,
                                                                                1002 as i32,
                                                                            );
                                                                            delayed_celt = 1 as i32;
                                                                        } else {
                                                                            frame_size = newsize;
                                                                        }
                                                                    }
                                                                    if random_fec != 0
                                                                        && rand() % 30 as i32
                                                                            == 0 as i32
                                                                    {
                                                                        rand();
                                                                        opus_encoder_ctl!(
                                                                            enc,
                                                                            4012 as i32,
                                                                            (rand() % 4 as i32
                                                                                == 0 as i32)
                                                                                as i32,
                                                                        );
                                                                    }
                                                                    if decode_only != 0 {
                                                                        let mut ch: [u8; 4] =
                                                                            [0; 4];
                                                                        num_read = fread(
                                                                            ch.as_mut_ptr() as *mut core::ffi::c_void,
                                                                            1,
                                                                            4,
                                                                            fin,
                                                                        ) as _;
                                                                        if num_read
                                                                            != 4 as i32 as u64
                                                                        {
                                                                            current_block = 15240930316249348783;
                                                                            break;
                                                                        }
                                                                        len[toggle as usize] =
                                                                            char_to_int(
                                                                                ch.as_mut_ptr(),
                                                                            )
                                                                                as i32;
                                                                        if len[toggle as usize]
                                                                            > max_payload_bytes
                                                                            || len[toggle as usize]
                                                                                < 0 as i32
                                                                        {
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"Invalid payload length: %d\n\0" as *const u8
                                                                                    as *const i8,
                                                                                len[toggle as usize],
                                                                            );
                                                                            current_block = 15240930316249348783;
                                                                            break;
                                                                        } else {
                                                                            num_read = fread(
                                                                                ch.as_mut_ptr() as *mut core::ffi::c_void,
                                                                                1,
                                                                                4,
                                                                                fin,
                                                                            ) as _;
                                                                            if num_read
                                                                                != 4 as i32 as u64
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
                                                                                data[toggle as usize] as *mut core::ffi::c_void,
                                                                                1,
                                                                                len[toggle as usize] as _,
                                                                                fin,
                                                                            ) as _;
                                                                            if num_read
                                                                                != len[toggle
                                                                                    as usize]
                                                                                    as size_t
                                                                            {
                                                                                fprintf(
                                                                                    stderr(),
                                                                                    b"Ran out of input, expecting %d bytes got %d\n\0"
                                                                                        as *const u8 as *const i8,
                                                                                    len[toggle as usize],
                                                                                    num_read as i32,
                                                                                );
                                                                                current_block = 15240930316249348783;
                                                                                break;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let mut i: i32 = 0;
                                                                        if !mode_list.is_null() {
                                                                            opus_encoder_ctl!(
                                                                                enc,
                                                                                4008 as i32,
                                                                                (*mode_list
                                                                                    .offset(
                                                                                    curr_mode
                                                                                        as isize
                                                                                ))
                                                                                    [1 as i32
                                                                                        as usize],
                                                                            );
                                                                            opus_encoder_ctl!(
                                                                                enc,
                                                                                11002 as i32,
                                                                                (*mode_list
                                                                                    .offset(
                                                                                    curr_mode
                                                                                        as isize
                                                                                ))
                                                                                    [0 as i32
                                                                                        as usize],
                                                                            );
                                                                            opus_encoder_ctl!(
                                                                                enc,
                                                                                4022 as i32,
                                                                                (*mode_list
                                                                                    .offset(
                                                                                    curr_mode
                                                                                        as isize
                                                                                ))
                                                                                    [3 as i32
                                                                                        as usize],
                                                                            );
                                                                            frame_size =
                                                                                (*mode_list
                                                                                    .offset(
                                                                                    curr_mode
                                                                                        as isize,
                                                                                ))
                                                                                    [2 as i32
                                                                                        as usize];
                                                                        }
                                                                        num_read = fread(
                                                                            fbytes as *mut core::ffi::c_void,
                                                                            (::core::mem::size_of::<libc::c_short>())
                                                                                .wrapping_mul(channels as _),
                                                                            (frame_size - remaining) as _,
                                                                            fin,
                                                                        ) as _;
                                                                        curr_read = num_read as i32;
                                                                        tot_in = (tot_in as u64)
                                                                            .wrapping_add(
                                                                                curr_read as u64,
                                                                            )
                                                                            as u64
                                                                            as u64;
                                                                        i = 0 as i32;
                                                                        while i < curr_read
                                                                            * channels
                                                                        {
                                                                            let mut s: i32 = 0;
                                                                            s = (*fbytes.offset(
                                                                                (2 as i32 * i
                                                                                    + 1 as i32)
                                                                                    as isize,
                                                                            )
                                                                                as i32)
                                                                                << 8 as i32
                                                                                | *fbytes.offset(
                                                                                    (2 as i32 * i)
                                                                                        as isize,
                                                                                )
                                                                                    as i32;
                                                                            s = (s & 0xffff as i32
                                                                                ^ 0x8000 as i32)
                                                                                - 0x8000 as i32;
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
                                                                                    .offset(i as isize) = 0 as i32 as libc::c_short;
                                                                                i += 1;
                                                                            }
                                                                            if encode_only != 0
                                                                                || decode_only != 0
                                                                            {
                                                                                stop = 1 as i32;
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
                                                                                data[toggle as usize] as *const u8,
                                                                                len[toggle as usize],
                                                                            );
                                                                        remaining =
                                                                            frame_size - nb_encoded;
                                                                        i = 0 as i32;
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
                                                                        if sweep_bps != 0 as i32 {
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
                                                                                < 1000 as i32
                                                                            {
                                                                                bitrate_bps =
                                                                                    1000 as i32;
                                                                            }
                                                                            opus_encoder_ctl!(
                                                                                enc,
                                                                                4002 as i32,
                                                                                bitrate_bps,
                                                                            );
                                                                        }
                                                                        opus_encoder_ctl!(
                                                                            enc,
                                                                            4031 as i32,
                                                                            (&mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
                                                                                as *mut u32)
                                                                                .offset(
                                                                                    (&mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
                                                                                        as *mut u32)
                                                                                        .offset_from(
                                                                                            &mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
                                                                                                as *mut u32,
                                                                                        ) as i64 as isize,
                                                                                ),
                                                                        );
                                                                        if len[toggle as usize]
                                                                            < 0 as i32
                                                                        {
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"opus_encode() returned %d\n\0" as *const u8
                                                                                    as *const i8,
                                                                                len[toggle as usize],
                                                                            );
                                                                            current_block = 14460699602747363466;
                                                                            break;
                                                                        } else {
                                                                            curr_mode_count +=
                                                                                frame_size;
                                                                            if curr_mode_count > mode_switch_time
                                                                                && curr_mode < nb_modes_in_list - 1 as i32
                                                                            {
                                                                                curr_mode += 1;
                                                                                curr_mode_count = 0 as i32;
                                                                            }
                                                                        }
                                                                    }
                                                                    if encode_only != 0 {
                                                                        let mut int_field: [u8; 4] =
                                                                            [0; 4];
                                                                        int_to_char(
                                                                            len[toggle as usize]
                                                                                as u32,
                                                                            int_field.as_mut_ptr(),
                                                                        );
                                                                        if fwrite(
                                                                            int_field.as_mut_ptr() as *const core::ffi::c_void,
                                                                            1,
                                                                            4,
                                                                            fout,
                                                                        ) != 4
                                                                        {
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"Error writing.\n\0" as *const u8 as *const i8,
                                                                            );
                                                                            current_block = 14460699602747363466;
                                                                            break;
                                                                        } else {
                                                                            int_to_char(
                                                                                enc_final_range[toggle as usize],
                                                                                int_field.as_mut_ptr(),
                                                                            );
                                                                            if fwrite(
                                                                                int_field.as_mut_ptr() as *const core::ffi::c_void,
                                                                                1,
                                                                                4,
                                                                                fout,
                                                                            ) != 4
                                                                            {
                                                                                fprintf(
                                                                                    stderr(),
                                                                                    b"Error writing.\n\0" as *const u8 as *const i8,
                                                                                );
                                                                                current_block = 14460699602747363466;
                                                                                break;
                                                                            } else if fwrite(
                                                                                data[toggle as usize] as *const core::ffi::c_void,
                                                                                1,
                                                                                len[toggle as usize] as _,
                                                                                fout,
                                                                            ) != len[toggle as usize] as _
                                                                            {
                                                                                fprintf(
                                                                                    stderr(),
                                                                                    b"Error writing.\n\0" as *const u8 as *const i8,
                                                                                );
                                                                                current_block = 14460699602747363466;
                                                                                break;
                                                                            } else {
                                                                                tot_samples += nb_encoded as f64;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let mut output_samples: i32 = 0;
                                                                        lost = (len[toggle as usize] == 0 as i32
                                                                            || packet_loss_perc > 0 as i32
                                                                                && (rand() % 100 as i32) < packet_loss_perc)
                                                                            as i32;
                                                                        if lost != 0 {
                                                                            opus_decoder_ctl!(
                                                                                dec,
                                                                                4039 as i32,
                                                                                (&mut output_samples as *mut i32)
                                                                                    .offset(
                                                                                        (&mut output_samples as *mut i32)
                                                                                            .offset_from(&mut output_samples as *mut i32)
                                                                                            as i64 as isize,
                                                                                    ),
                                                                            );
                                                                        } else {
                                                                            output_samples =
                                                                                max_frame_size;
                                                                        }
                                                                        if count >= use_inbandfec {
                                                                            if use_inbandfec != 0 {
                                                                                if lost_prev != 0 {
                                                                                    opus_decoder_ctl!(
                                                                                        dec,
                                                                                        4039 as i32,
                                                                                        (&mut output_samples as *mut i32)
                                                                                            .offset(
                                                                                                (&mut output_samples as *mut i32)
                                                                                                    .offset_from(&mut output_samples as *mut i32)
                                                                                                    as i64 as isize,
                                                                                            ),
                                                                                    );
                                                                                    output_samples = opus_decode(
                                                                                        dec,
                                                                                        if lost != 0 {
                                                                                            std::ptr::null_mut::<u8>()
                                                                                        } else {
                                                                                            data[toggle as usize]
                                                                                        },
                                                                                        len[toggle as usize],
                                                                                        out,
                                                                                        output_samples,
                                                                                        1 as i32,
                                                                                    );
                                                                                } else {
                                                                                    output_samples = max_frame_size;
                                                                                    output_samples = opus_decode(
                                                                                        dec,
                                                                                        data[(1 as i32 - toggle) as usize],
                                                                                        len[(1 as i32 - toggle) as usize],
                                                                                        out,
                                                                                        output_samples,
                                                                                        0 as i32,
                                                                                    );
                                                                                }
                                                                            } else {
                                                                                output_samples = opus_decode(
                                                                                    dec,
                                                                                    if lost != 0 {
                                                                                        std::ptr::null_mut::<u8>()
                                                                                    } else {
                                                                                        data[toggle as usize]
                                                                                    },
                                                                                    len[toggle as usize],
                                                                                    out,
                                                                                    output_samples,
                                                                                    0 as i32,
                                                                                );
                                                                            }
                                                                            if output_samples
                                                                                > 0 as i32
                                                                            {
                                                                                if decode_only == 0
                                                                                    && tot_out.wrapping_add(output_samples as u64)
                                                                                        > tot_in
                                                                                {
                                                                                    stop = 1 as i32;
                                                                                    output_samples = tot_in.wrapping_sub(tot_out) as i32;
                                                                                }
                                                                                if output_samples
                                                                                    > skip
                                                                                {
                                                                                    let mut i_0: i32 = 0;
                                                                                    i_0 = 0 as i32;
                                                                                    while i_0 < (output_samples - skip) * channels {
                                                                                        let mut s_0: libc::c_short = 0;
                                                                                        s_0 = *out.offset((i_0 + skip * channels) as isize);
                                                                                        *fbytes
                                                                                            .offset(
                                                                                                (2 as i32 * i_0) as isize,
                                                                                            ) = (s_0 as i32 & 0xff as i32)
                                                                                            as u8;
                                                                                        *fbytes
                                                                                            .offset(
                                                                                                (2 as i32 * i_0 + 1 as i32) as isize,
                                                                                            ) = (s_0 as i32 >> 8 as i32
                                                                                            & 0xff as i32) as u8;
                                                                                        i_0 += 1;
                                                                                    }
                                                                                    if fwrite(
                                                                                        fbytes as *const core::ffi::c_void,
                                                                                        (::core::mem::size_of::<libc::c_short>())
                                                                                            .wrapping_mul(channels as _),
                                                                                        (output_samples - skip) as _,
                                                                                        fout,
                                                                                    )
                                                                                        != (output_samples - skip) as _
                                                                                    {
                                                                                        fprintf(
                                                                                            stderr(),
                                                                                            b"Error writing.\n\0" as *const u8 as *const i8,
                                                                                        );
                                                                                        current_block = 14460699602747363466;
                                                                                        break;
                                                                                    } else {
                                                                                        tot_out = (tot_out as u64)
                                                                                            .wrapping_add((output_samples - skip) as u64)
                                                                                            as u64 as u64;
                                                                                    }
                                                                                }
                                                                                if output_samples
                                                                                    < skip
                                                                                {
                                                                                    skip -= output_samples;
                                                                                } else {
                                                                                    skip = 0 as i32;
                                                                                }
                                                                            } else {
                                                                                fprintf(
                                                                                    stderr(),
                                                                                    b"error decoding frame: %s\n\0" as *const u8
                                                                                        as *const i8,
                                                                                    opus_strerror(output_samples),
                                                                                );
                                                                            }
                                                                            tot_samples +=
                                                                                output_samples
                                                                                    as f64;
                                                                        }
                                                                    }
                                                                    if encode_only == 0 {
                                                                        opus_decoder_ctl!(
                                                                            dec,
                                                                            4031 as i32,
                                                                            (&mut dec_final_range as *mut u32)
                                                                                .offset(
                                                                                    (&mut dec_final_range as *mut u32)
                                                                                        .offset_from(&mut dec_final_range as *mut u32)
                                                                                        as i64 as isize,
                                                                                ),
                                                                        );
                                                                    }
                                                                    if enc_final_range[(toggle
                                                                        ^ use_inbandfec)
                                                                        as usize]
                                                                        != 0 as i32 as u32
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
                                                                            stderr(),
                                                                            b"Error: Range coder state mismatch between encoder and decoder in frame %ld: 0x%8lx vs 0x%8lx\n\0"
                                                                                as *const u8 as *const i8,
                                                                            count as i64,
                                                                            enc_final_range[(toggle ^ use_inbandfec) as usize]
                                                                                as u64,
                                                                            dec_final_range as u64,
                                                                        );
                                                                        current_block =
                                                                            14460699602747363466;
                                                                        break;
                                                                    } else {
                                                                        lost_prev = lost;
                                                                        if count >= use_inbandfec {
                                                                            bits += (len
                                                                                [toggle as usize]
                                                                                * 8 as i32)
                                                                                as f64;
                                                                            bits_max = if (len
                                                                                [toggle as usize]
                                                                                * 8 as i32)
                                                                                as f64
                                                                                > bits_max
                                                                            {
                                                                                (len[toggle
                                                                                    as usize]
                                                                                    * 8 as i32)
                                                                                    as f64
                                                                            } else {
                                                                                bits_max
                                                                            };
                                                                            bits2 += (len
                                                                                [toggle as usize]
                                                                                * len[toggle
                                                                                    as usize]
                                                                                * 64 as i32)
                                                                                as f64;
                                                                            if decode_only == 0 {
                                                                                nrg = 0.0f64;
                                                                                k = 0 as i32;
                                                                                while k < frame_size
                                                                                    * channels
                                                                                {
                                                                                    nrg
                                                                                        += *in_0.offset(k as isize) as i32 as f64
                                                                                            * *in_0.offset(k as isize) as f64;
                                                                                    k += 1;
                                                                                }
                                                                                nrg /= (frame_size
                                                                                    * channels)
                                                                                    as f64;
                                                                                if nrg > 1e5f64 {
                                                                                    bits_act
                                                                                        += (len[toggle as usize] * 8 as i32)
                                                                                            as f64;
                                                                                    count_act += 1;
                                                                                }
                                                                            }
                                                                        }
                                                                        count += 1;
                                                                        toggle = (toggle
                                                                            + use_inbandfec)
                                                                            & 1 as i32;
                                                                    }
                                                                }
                                                                match current_block {
                                                                    14460699602747363466 => {}
                                                                    _ => {
                                                                        if decode_only != 0
                                                                            && count > 0 as i32
                                                                        {
                                                                            frame_size =
                                                                                (tot_samples
                                                                                    / count as f64)
                                                                                    as i32;
                                                                        }
                                                                        count -= use_inbandfec;
                                                                        if tot_samples
                                                                            >= 1 as i32 as f64
                                                                            && count > 0 as i32
                                                                            && frame_size != 0
                                                                        {
                                                                            let mut var: f64 = 0.;
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"average bitrate:             %7.3f kb/s\n\0" as *const u8
                                                                                    as *const i8,
                                                                                1e-3f64 * bits * sampling_rate as f64
                                                                                    / tot_samples,
                                                                            );
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"maximum bitrate:             %7.3f kb/s\n\0" as *const u8
                                                                                    as *const i8,
                                                                                1e-3f64 * bits_max * sampling_rate as f64
                                                                                    / frame_size as f64,
                                                                            );
                                                                            if decode_only == 0 {
                                                                                fprintf(
                                                                                    stderr(),
                                                                                    b"active bitrate:              %7.3f kb/s\n\0" as *const u8
                                                                                        as *const i8,
                                                                                    1e-3f64 * bits_act * sampling_rate as f64
                                                                                        / (1e-15f64
                                                                                            + frame_size as f64
                                                                                                * count_act as f64),
                                                                                );
                                                                            }
                                                                            var = bits2
                                                                                / count as f64
                                                                                - bits * bits
                                                                                    / (count
                                                                                        as f64
                                                                                        * count
                                                                                            as f64);
                                                                            if var < 0 as i32 as f64
                                                                            {
                                                                                var =
                                                                                    0 as i32 as f64;
                                                                            }
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"bitrate standard deviation:  %7.3f kb/s\n\0" as *const u8
                                                                                    as *const i8,
                                                                                1e-3f64 * var.sqrt() * sampling_rate as f64
                                                                                    / frame_size as f64,
                                                                            );
                                                                        } else {
                                                                            fprintf(
                                                                                stderr(),
                                                                                b"bitrate statistics are undefined\n\0" as *const u8
                                                                                    as *const i8,
                                                                            );
                                                                        }
                                                                        ret = 0 as i32;
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
    free(data[0 as i32 as usize] as *mut core::ffi::c_void);
    free(data[1 as i32 as usize] as *mut core::ffi::c_void);
    if !fin.is_null() {
        fclose(fin);
    }
    if !fout.is_null() {
        fclose(fout);
    }
    free(in_0 as *mut core::ffi::c_void);
    free(out as *mut core::ffi::c_void);
    free(fbytes as *mut core::ffi::c_void);
    ret
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
