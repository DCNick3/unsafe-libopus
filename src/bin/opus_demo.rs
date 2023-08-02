#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

type size_t = u64;
use unsafe_libopus::externs::{calloc, malloc};
use unsafe_libopus::opus_private::{MODE_CELT_ONLY, MODE_SILK_ONLY};
use unsafe_libopus::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_encode, opus_encoder_create,
    opus_encoder_ctl, opus_get_version_string, opus_packet_get_nb_frames,
    opus_packet_get_samples_per_frame, opus_strerror, OpusDecoder, OpusEncoder,
    OPUS_APPLICATION_AUDIO, OPUS_APPLICATION_RESTRICTED_LOWDELAY, OPUS_APPLICATION_VOIP, OPUS_AUTO,
    OPUS_BANDWIDTH_FULLBAND, OPUS_BANDWIDTH_MEDIUMBAND, OPUS_BANDWIDTH_NARROWBAND,
    OPUS_BANDWIDTH_SUPERWIDEBAND, OPUS_BANDWIDTH_WIDEBAND, OPUS_FRAMESIZE_100_MS,
    OPUS_FRAMESIZE_10_MS, OPUS_FRAMESIZE_120_MS, OPUS_FRAMESIZE_20_MS, OPUS_FRAMESIZE_2_5_MS,
    OPUS_FRAMESIZE_40_MS, OPUS_FRAMESIZE_5_MS, OPUS_FRAMESIZE_60_MS, OPUS_FRAMESIZE_80_MS,
    OPUS_SET_EXPERT_FRAME_DURATION_REQUEST,
};

#[rustfmt::skip]
pub fn print_usage(argv0: &str) {
    eprintln!("Usage: {} [-e] <application> <sampling rate (Hz)> <channels (1/2)> <bits per second>  [options] <input> <output>", argv0);
    eprintln!("       {} -d <sampling rate (Hz)> <channels (1/2)> [options] <input> <output>", argv0);
    eprintln!("application: voip | audio | restricted-lowdelay");
    eprintln!("options:");
    eprintln!("-e                   : only runs the encoder (output the bit-stream)");
    eprintln!("-d                   : only runs the decoder (reads the bit-stream as input)");
    eprintln!("-cbr                 : use constant bit-rate; default: variable bitrate");
    eprintln!("-cvbr                : enable constrained variable bitrate; default: unconstrained");
    eprintln!("-delayed-decision    : use look-ahead for speech/music detection (experts only); default: disabled");
    eprintln!("-bandwidth <NB|MB|WB|SWB|FB> : audio bandwidth (from narrowband to fullband); default: sampling rate");
    eprintln!("-framesize <2.5|5|10|20|40|60|80|100|120> : frame size in ms; default: 20 ");
    eprintln!("-max_payload <bytes> : maximum payload size in bytes, default: 1024");
    eprintln!("-complexity <comp>   : complexity, 0 (lowest) ... 10 (highest); default: 10");
    eprintln!("-inbandfec           : enable SILK inband FEC");
    eprintln!("-forcemono           : force mono encoding, even for stereo input");
    eprintln!("-dtx                 : enable SILK DTX");
    eprintln!("-loss <perc>         : simulate packet loss, in percent (0-100); default: 0");
}

static silk8_test: [[i32; 4]; 8] = [
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960 * 3, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960 * 2, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 480, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960 * 3, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960 * 2, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_NARROWBAND, 480, 2],
];
static silk12_test: [[i32; 4]; 8] = [
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 960 * 3, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 960 * 2, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 960, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 480, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 960 * 3, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 960 * 2, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 960, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_MEDIUMBAND, 480, 2],
];
static silk16_test: [[i32; 4]; 8] = [
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960 * 3, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960 * 2, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 480, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960 * 3, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960 * 2, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_WIDEBAND, 480, 2],
];
static hybrid24_test: [[i32; 4]; 4] = [
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 960, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 480, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 960, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 480, 2],
];
static hybrid48_test: [[i32; 4]; 4] = [
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_FULLBAND, 960, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_FULLBAND, 480, 1],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_FULLBAND, 960, 2],
    [MODE_SILK_ONLY, OPUS_BANDWIDTH_FULLBAND, 480, 2],
];
static celt_test: [[i32; 4]; 32] = [
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 960, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 960, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 480, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 480, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 480, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 480, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 240, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 240, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 240, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 240, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 120, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 120, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 120, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 120, 1],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 960, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 960, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 960, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 960, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 480, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 480, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 480, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 480, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 240, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 240, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 240, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 240, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 120, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_SUPERWIDEBAND, 120, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_WIDEBAND, 120, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_NARROWBAND, 120, 2],
];
static celt_hq_test: [[i32; 4]; 4] = [
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 960, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 480, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 240, 2],
    [MODE_CELT_ONLY, OPUS_BANDWIDTH_FULLBAND, 120, 2],
];

fn rand() -> i32 {
    4 // chosen by fair dice roll
      // guaranteed to be random
}
// TODO: random is used in some specific test cases which are not really clear how to run them I think?
// if needed - we can implement it using the `rand` crate
// it would make sense then to make the binaries separate crates though, as the library itself does not need random

const MAX_PACKET: i32 = 1500;

unsafe fn main_0() -> i32 {
    let mut err: i32 = 0;
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut len: [i32; 2] = [0; 2];
    let mut frame_size: i32 = 0;
    let mut bitrate_bps: i32 = 0 as i32;
    let mut data: [*mut u8; 2] = [std::ptr::null_mut::<u8>(), std::ptr::null_mut::<u8>()];
    let mut fbytes: *mut u8 = std::ptr::null_mut::<u8>();
    let mut cvbr: i32 = 0 as i32;
    let mut count: i32 = 0 as i32;
    let mut count_act: i32 = 0 as i32;
    let mut k: i32 = 0;
    let mut skip: i32 = 0 as i32;
    let mut stop: i32 = 0 as i32;
    let mut in_0: *mut i16 = std::ptr::null_mut::<i16>();
    let mut out: *mut i16 = std::ptr::null_mut::<i16>();
    let mut bits: f64 = 0.0f64;
    let mut bits_max: f64 = 0.0f64;
    let mut bits_act: f64 = 0.0f64;
    let mut bits2: f64 = 0.0f64;
    let mut nrg: f64 = 0.;
    let mut tot_samples: f64 = 0 as i32 as f64;
    let mut tot_in: u64 = 0;
    let mut tot_out: u64 = 0;
    let mut bandwidth: i32 = OPUS_AUTO;
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

    let mut args = std::env::args();
    let argc = args.len();
    let argv0 = args.next().unwrap();

    if argc < 5 {
        print_usage(&argv0);
        return 1;
    }

    tot_out = 0 as i32 as u64;
    tot_in = tot_out;
    eprintln!(
        "{}",
        std::ffi::CStr::from_ptr(opus_get_version_string() as _)
            .to_str()
            .unwrap()
    );

    let arg = args.next().unwrap();
    if arg == "-e" {
        encode_only = 1 as i32;
    } else if arg == "-d" {
        decode_only = 1 as i32;
    }
    if decode_only == 0 && argc < 7 {
        print_usage(&argv0);
        return 1;
    }

    let application = if decode_only == 0 {
        let arg = args.next().unwrap();

        if arg == "voip" {
            OPUS_APPLICATION_VOIP
        } else if arg == "restricted-lowdelay" {
            OPUS_APPLICATION_RESTRICTED_LOWDELAY
        } else if arg == "audio" {
            OPUS_APPLICATION_AUDIO
        } else {
            eprintln!("unknown application: {}", arg);
            print_usage(&argv0);
            return 1;
        }
    } else {
        OPUS_APPLICATION_AUDIO
    };

    let sampling_rate: i32 = args.next().unwrap().parse().expect("sampling rate");
    if sampling_rate != 8000 as i32
        && sampling_rate != 12000 as i32
        && sampling_rate != 16000 as i32
        && sampling_rate != 24000 as i32
        && sampling_rate != 48000 as i32
    {
        eprintln!("Supported sampling rates are 8000, 12000, 16000, 24000 and 48000.");
        return 1;
    }

    frame_size = sampling_rate / 50 as i32;
    let channels = args.next().unwrap().parse().expect("channels");
    if channels < 1 as i32 || channels > 2 as i32 {
        eprintln!("Opus_demo supports only 1 or 2 channels.");
        return 1;
    }

    if decode_only == 0 {
        bitrate_bps = args.next().unwrap().parse().expect("bitrate");
    }
    let mut use_vbr = 1 as i32;
    let mut max_payload_bytes: i32 = MAX_PACKET;
    let mut complexity = 10 as i32;
    let mut use_inbandfec = 0 as i32;
    let mut forcechannels = OPUS_AUTO;
    let mut use_dtx = 0 as i32;
    let mut packet_loss_perc = 0 as i32;
    loop {
        // the last two args are filenames
        if args.len() <= 2 {
            break;
        }
        let arg = args.next().unwrap();

        if arg == "-cbr" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-cbr");
                return 1;
            }

            use_vbr = 0 as i32;
        } else if arg == "-bandwidth" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-bandwidth");
                return 1;
            }

            let bandwidth_str = args.next().unwrap();
            bandwidth = if bandwidth_str == "NB" {
                OPUS_BANDWIDTH_NARROWBAND
            } else if bandwidth_str == "MB" {
                OPUS_BANDWIDTH_MEDIUMBAND
            } else if bandwidth_str == "WB" {
                OPUS_BANDWIDTH_WIDEBAND
            } else if bandwidth_str == "SWB" {
                OPUS_BANDWIDTH_SUPERWIDEBAND
            } else if bandwidth_str == "FB" {
                OPUS_BANDWIDTH_FULLBAND
            } else {
                eprintln!(
                    "Unknown bandwidth {}. Supported are NB, MB, WB, SWB, FB.",
                    bandwidth_str
                );
                return 1;
            };
        } else if arg == "-framesize" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-framesize");
                return 1;
            }

            let framesize_str = args.next().unwrap();

            frame_size = if framesize_str == "2.5" {
                sampling_rate / 400
            } else if framesize_str == "5" {
                sampling_rate / 200
            } else if framesize_str == "10" {
                sampling_rate / 100
            } else if framesize_str == "20" {
                sampling_rate / 50
            } else if framesize_str == "40" {
                sampling_rate / 25
            } else if framesize_str == "60" {
                3 * sampling_rate / 50
            } else if framesize_str == "80" {
                4 * sampling_rate / 50
            } else if framesize_str == "100" {
                5 * sampling_rate / 50
            } else if framesize_str == "120" {
                6 * sampling_rate / 50
            } else {
                eprintln!(
                    "Unsupported frame size: {} ms. Supported are 2.5, 5, 10, 20, 40, 60, 80, 100, 120.",
                    framesize_str
                );
                return 1;
            };
        } else if arg == "-max_payload" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-max_payload");
                return 1;
            }
            max_payload_bytes = args.next().unwrap().parse().expect("max_payload");
        } else if arg == "-complexity" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-complexity");
                return 1;
            }
            complexity = args.next().unwrap().parse().expect("complexity");
        } else if arg == "-inbandfec" {
            use_inbandfec = 1 as i32;
        } else if arg == "-forcemono" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-forcemono");

                return 1;
            }
            forcechannels = 1 as i32;
        } else if arg == "-cvbr" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-cvbr");
                return 1;
            }
            cvbr = 1 as i32;
        } else if arg == "-delayed-decision" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-delayed-decision");
                return 1;
            }
            delayed_decision = 1 as i32;
        } else if arg == "-dtx" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-dtx");
                return 1;
            }
            use_dtx = 1 as i32;
        } else if arg == "-loss" {
            packet_loss_perc = args.next().unwrap().parse().expect("loss");
        } else if arg == "-sweep" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-sweep");
                return 1;
            }
            sweep_bps = args.next().unwrap().parse().expect("sweep");
        } else if arg == "-random_framesize" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-random_framesize");
                return 1;
            }
            random_framesize = 1 as i32;
        } else if arg == "-sweep_max" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-sweep_max");
                return 1;
            }
            sweep_max = args.next().unwrap().parse().expect("sweep_max");
        } else if arg == "-random_fec" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-random_fec");
                return 1;
            }
            random_fec = 1 as i32;
        } else if arg == "-silk8k_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-silk8k_test");
                return 1;
            }
            mode_list = silk8_test.as_ptr();
            nb_modes_in_list = 8 as i32;
        } else if arg == "-silk12k_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-silk12k_test");
                return 1;
            }
            mode_list = silk12_test.as_ptr();
            nb_modes_in_list = 8 as i32;
        } else if arg == "-silk16k_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-silk16k_test");
                return 1;
            }
            mode_list = silk16_test.as_ptr();
            nb_modes_in_list = 8 as i32;
        } else if arg == "-hybrid24k_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-hybrid24k_test");
                return 1;
            }
            mode_list = hybrid24_test.as_ptr();
            nb_modes_in_list = 4 as i32;
        } else if arg == "-hybrid48k_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-hybrid48k_test");
                return 1;
            }
            mode_list = hybrid48_test.as_ptr();
            nb_modes_in_list = 4 as i32;
        } else if arg == "-celt_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-celt_test");
                return 1;
            }
            mode_list = celt_test.as_ptr();
            nb_modes_in_list = 32 as i32;
        } else if arg == "-celt_hq_test" {
            if decode_only != 0 {
                eprintln!("option {} is only for encoding", "-celt_hq_test");
                return 1;
            }
            mode_list = celt_hq_test.as_ptr();
            nb_modes_in_list = 4 as i32;
        } else {
            eprintln!("Error: unrecognized setting: {}", arg);
            print_usage(&argv0);
            return 1;
        }
    }

    if sweep_max != 0 {
        sweep_min = bitrate_bps;
    }
    if max_payload_bytes < 0 as i32 || max_payload_bytes > MAX_PACKET as i32 {
        eprintln!("max_payload_bytes must be between 0 and {}", MAX_PACKET);
        return 1;
    }

    let fin = args.next().expect("input file");
    eprintln!("Opening input file {}", fin);
    let mut fin = File::open(fin).expect("opening input file");
    if !mode_list.is_null() {
        let mut size: i32 = fin.seek(SeekFrom::End(0)).expect("seeking to end of file") as i32;
        eprintln!("File size is {} bytes", size);
        fin.seek(SeekFrom::Start(0))
            .expect("seeking back to start of file");

        mode_switch_time = (size as u64)
            .wrapping_div(::core::mem::size_of::<i16>() as u64)
            .wrapping_div(channels as u64)
            .wrapping_div(nb_modes_in_list as u64) as i32;
        eprintln!("Switching mode every {} samples", mode_switch_time);
    }

    let fout = args.next().expect("output file");
    let mut fout = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(fout)
        .expect("opening output file");

    if decode_only == 0 {
        enc = opus_encoder_create(sampling_rate, channels, application, &mut err);
        if err != 0 as i32 {
            let err = std::ffi::CStr::from_ptr(opus_strerror(err) as _)
                .to_str()
                .unwrap();
            eprintln!("Cannot create encoder: {}", err);
            return 1;
        } else {
            opus_encoder_ctl!(enc, 4002 as i32, bitrate_bps,);
            opus_encoder_ctl!(enc, 4008 as i32, bandwidth,);
            opus_encoder_ctl!(enc, 4006 as i32, use_vbr,);
            opus_encoder_ctl!(enc, 4020 as i32, cvbr,);
            opus_encoder_ctl!(enc, 4010 as i32, complexity,);
            opus_encoder_ctl!(enc, 4012 as i32, use_inbandfec,);
            opus_encoder_ctl!(enc, 4022 as i32, forcechannels,);
            opus_encoder_ctl!(enc, 4016 as i32, use_dtx,);
            opus_encoder_ctl!(enc, 4014 as i32, packet_loss_perc,);
            opus_encoder_ctl!(enc, 4027 as i32, &mut skip);
            opus_encoder_ctl!(enc, 4036 as i32, 16 as i32,);
            opus_encoder_ctl!(enc, 4040 as i32, variable_duration,);
        }
    }

    if encode_only == 0 {
        dec = opus_decoder_create(sampling_rate, channels, &mut err);
        if err != 0 as i32 {
            let err = std::ffi::CStr::from_ptr(opus_strerror(err) as _)
                .to_str()
                .unwrap();
            eprintln!("Cannot create decoder: {}", err);
            return 1;
        }
    }

    let bandwidth_string = match bandwidth {
        OPUS_BANDWIDTH_NARROWBAND => "narrowband",
        OPUS_BANDWIDTH_MEDIUMBAND => "mediumband",
        OPUS_BANDWIDTH_WIDEBAND => "wideband",
        OPUS_BANDWIDTH_SUPERWIDEBAND => "superwideband",
        OPUS_BANDWIDTH_FULLBAND => "fullband",
        OPUS_AUTO => "auto bandwidth",
        _ => "unknown",
    };

    if decode_only != 0 {
        eprintln!(
            "Decoding with {} Hz output ({} channels)",
            sampling_rate, channels
        );
    } else {
        eprintln!(
            "Encoding {} Hz input at {:.03} kb/s in {} with {}-sample frames.",
            sampling_rate,
            bitrate_bps as f64 * 0.001f64,
            bandwidth_string,
            frame_size,
        );
    }
    in_0 = malloc((max_frame_size * channels * 2) as u64) as *mut i16;
    out = malloc((max_frame_size * channels * 2) as u64) as *mut i16;
    fbytes = malloc((max_frame_size * channels * 2) as u64) as *mut u8;
    data[0 as i32 as usize] = calloc(max_payload_bytes as u64, 1) as *mut u8;
    if use_inbandfec != 0 {
        data[1 as i32 as usize] = calloc(max_payload_bytes as u64, 1) as *mut u8;
    }
    if delayed_decision != 0 {
        if frame_size == sampling_rate / 400 as i32 {
            variable_duration = OPUS_FRAMESIZE_2_5_MS;
        } else if frame_size == sampling_rate / 200 as i32 {
            variable_duration = OPUS_FRAMESIZE_5_MS;
        } else if frame_size == sampling_rate / 100 as i32 {
            variable_duration = OPUS_FRAMESIZE_10_MS;
        } else if frame_size == sampling_rate / 50 as i32 {
            variable_duration = OPUS_FRAMESIZE_20_MS;
        } else if frame_size == sampling_rate / 25 as i32 {
            variable_duration = OPUS_FRAMESIZE_40_MS;
        } else if frame_size == 3 as i32 * sampling_rate / 50 as i32 {
            variable_duration = OPUS_FRAMESIZE_60_MS;
        } else if frame_size == 4 as i32 * sampling_rate / 50 as i32 {
            variable_duration = OPUS_FRAMESIZE_80_MS;
        } else if frame_size == 5 as i32 * sampling_rate / 50 as i32 {
            variable_duration = OPUS_FRAMESIZE_100_MS;
        } else {
            variable_duration = OPUS_FRAMESIZE_120_MS;
        }
        opus_encoder_ctl!(
            enc,
            OPUS_SET_EXPERT_FRAME_DURATION_REQUEST,
            variable_duration,
        );
        frame_size = 2 * 48000;
    }

    loop {
        if stop != 0 {
            break;
        }
        if delayed_celt != 0 {
            frame_size = newsize;
            delayed_celt = 0 as i32;
        } else if random_framesize != 0 && rand() % 20 as i32 == 0 as i32 {
            newsize = match rand() % 6 as i32 {
                0 => sampling_rate / 400 as i32,
                1 => sampling_rate / 200 as i32,
                2 => sampling_rate / 100 as i32,
                3 => sampling_rate / 50 as i32,
                4 => sampling_rate / 25 as i32,
                5 => 3 as i32 * sampling_rate / 50 as i32,
                _ => unreachable!(),
            };
            while newsize < sampling_rate / 25 as i32
                && bitrate_bps - sweep_bps.abs() <= 3 as i32 * 12 as i32 * sampling_rate / newsize
            {
                newsize *= 2 as i32;
            }
            if newsize < sampling_rate / 100 as i32 && frame_size >= sampling_rate / 100 as i32 {
                opus_encoder_ctl!(enc, 11002 as i32, 1002 as i32,);
                delayed_celt = 1 as i32;
            } else {
                frame_size = newsize;
            }
        }
        if random_fec != 0 && rand() % 30 as i32 == 0 as i32 {
            rand();
            opus_encoder_ctl!(enc, 4012 as i32, (rand() % 4 as i32 == 0 as i32) as i32,);
        }
        if decode_only != 0 {
            let mut ch: [u8; 4] = [0; 4];
            match fin.read_exact(&mut ch) {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => {
                    panic!("Error reading from input: {}", e);
                }
            }
            len[toggle as usize] = i32::from_be_bytes(ch);
            if len[toggle as usize] > max_payload_bytes || len[toggle as usize] < 0 as i32 {
                eprintln!("Invalid payload length: {}", len[toggle as usize]);
                break;
            } else {
                fin.read_exact(&mut ch).unwrap();
                enc_final_range[toggle as usize] = u32::from_be_bytes(ch);
                fin.read_exact(std::slice::from_raw_parts_mut(
                    data[toggle as usize],
                    len[toggle as usize] as usize,
                ))
                .expect("Failed to read payload");
            }
        } else {
            let mut i: i32 = 0;
            if !mode_list.is_null() {
                opus_encoder_ctl!(
                    enc,
                    4008 as i32,
                    (*mode_list.offset(curr_mode as isize))[1 as i32 as usize],
                );
                opus_encoder_ctl!(
                    enc,
                    11002 as i32,
                    (*mode_list.offset(curr_mode as isize))[0 as i32 as usize],
                );
                opus_encoder_ctl!(
                    enc,
                    4022 as i32,
                    (*mode_list.offset(curr_mode as isize))[3 as i32 as usize],
                );
                frame_size = (*mode_list.offset(curr_mode as isize))[2 as i32 as usize];
            }
            num_read = fin
                .read(std::slice::from_raw_parts_mut(
                    fbytes,
                    ((frame_size - remaining) * channels * 2) as usize,
                ))
                .unwrap() as u64;
            curr_read = num_read as i32;
            tot_in = (tot_in as u64).wrapping_add(curr_read as u64) as u64 as u64;
            i = 0 as i32;
            while i < curr_read * channels {
                let mut s: i32 = 0;
                s = (*fbytes.offset((2 as i32 * i + 1 as i32) as isize) as i32) << 8 as i32
                    | *fbytes.offset((2 as i32 * i) as isize) as i32;
                s = (s & 0xffff as i32 ^ 0x8000 as i32) - 0x8000 as i32;
                *in_0.offset((i + remaining * channels) as isize) = s as i16;
                i += 1;
            }
            if curr_read + remaining < frame_size {
                i = (curr_read + remaining) * channels;
                while i < frame_size * channels {
                    *in_0.offset(i as isize) = 0 as i32 as i16;
                    i += 1;
                }
                if encode_only != 0 || decode_only != 0 {
                    stop = 1 as i32;
                }
            }
            len[toggle as usize] = opus_encode(
                enc,
                in_0,
                frame_size,
                data[toggle as usize],
                max_payload_bytes,
            );
            nb_encoded = opus_packet_get_samples_per_frame(data[toggle as usize], sampling_rate)
                * opus_packet_get_nb_frames(
                    data[toggle as usize] as *const u8,
                    len[toggle as usize],
                );
            remaining = frame_size - nb_encoded;
            i = 0 as i32;
            while i < remaining * channels {
                *in_0.offset(i as isize) = *in_0.offset((nb_encoded * channels + i) as isize);
                i += 1;
            }
            if sweep_bps != 0 as i32 {
                bitrate_bps += sweep_bps;
                if sweep_max != 0 {
                    if bitrate_bps > sweep_max {
                        sweep_bps = -sweep_bps;
                    } else if bitrate_bps < sweep_min {
                        sweep_bps = -sweep_bps;
                    }
                }
                if bitrate_bps < 1000 as i32 {
                    bitrate_bps = 1000 as i32;
                }
                opus_encoder_ctl!(enc, 4002 as i32, bitrate_bps,);
            }
            opus_encoder_ctl!(
                enc,
                4031 as i32,
                &mut *enc_final_range.as_mut_ptr().offset(toggle as isize)
            );
            if len[toggle as usize] < 0 as i32 {
                eprintln!("opus_encode() returned {}", len[toggle as usize]);
                return 1;
            } else {
                curr_mode_count += frame_size;
                if curr_mode_count > mode_switch_time && curr_mode < nb_modes_in_list - 1 as i32 {
                    curr_mode += 1;
                    curr_mode_count = 0 as i32;
                }
            }
        }
        if encode_only != 0 {
            let mut int_field: [u8; 4] = len[toggle as usize].to_be_bytes();
            fout.write_all(&int_field).unwrap();

            int_field = enc_final_range[toggle as usize].to_be_bytes();
            fout.write_all(&int_field).unwrap();

            fout.write_all(std::slice::from_raw_parts(
                data[toggle as usize],
                len[toggle as usize] as usize,
            ))
            .expect("Failed to write payload");

            tot_samples += nb_encoded as f64;
        } else {
            let mut output_samples: i32 = 0;
            lost = (len[toggle as usize] == 0 as i32
                || packet_loss_perc > 0 as i32 && (rand() % 100 as i32) < packet_loss_perc)
                as i32;
            if lost != 0 {
                opus_decoder_ctl!(dec, 4039 as i32, &mut output_samples);
            } else {
                output_samples = max_frame_size;
            }
            if count >= use_inbandfec {
                if use_inbandfec != 0 {
                    if lost_prev != 0 {
                        opus_decoder_ctl!(dec, 4039 as i32, &mut output_samples);
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
                if output_samples > 0 as i32 {
                    if decode_only == 0 && tot_out.wrapping_add(output_samples as u64) > tot_in {
                        stop = 1 as i32;
                        output_samples = tot_in.wrapping_sub(tot_out) as i32;
                    }
                    if output_samples > skip {
                        let mut i_0: i32 = 0;
                        i_0 = 0 as i32;
                        while i_0 < (output_samples - skip) * channels {
                            let mut s_0: i16 = 0;
                            s_0 = *out.offset((i_0 + skip * channels) as isize);
                            *fbytes.offset((2 as i32 * i_0) as isize) =
                                (s_0 as i32 & 0xff as i32) as u8;
                            *fbytes.offset((2 as i32 * i_0 + 1 as i32) as isize) =
                                (s_0 as i32 >> 8 as i32 & 0xff as i32) as u8;
                            i_0 += 1;
                        }
                        fout.write_all(std::slice::from_raw_parts(
                            fbytes,
                            (2 * (output_samples - skip) * channels) as usize,
                        ))
                        .expect("Failed to write decoded samples");

                        tot_out += (output_samples - skip) as u64;
                    }
                    if output_samples < skip {
                        skip -= output_samples;
                    } else {
                        skip = 0 as i32;
                    }
                } else {
                    let err = std::ffi::CStr::from_ptr(opus_strerror(output_samples) as _)
                        .to_str()
                        .unwrap();
                    eprintln!("opus_decode() returned {}", err);
                }
                tot_samples += output_samples as f64;
            }
        }
        if encode_only == 0 {
            opus_decoder_ctl!(dec, 4031 as i32, &mut dec_final_range);
        }
        if enc_final_range[(toggle ^ use_inbandfec) as usize] != 0 as i32 as u32
            && encode_only == 0
            && lost == 0
            && lost_prev == 0
            && dec_final_range != enc_final_range[(toggle ^ use_inbandfec) as usize]
        {
            eprintln!(
                "Error: Range coder state mismatch between encoder and decoder in frame {}: 0x{:08x} vs 0x{:08x}",
                count,
                enc_final_range[(toggle ^ use_inbandfec) as usize],
                dec_final_range
            );
            return 1;
        } else {
            lost_prev = lost;
            if count >= use_inbandfec {
                bits += (len[toggle as usize] * 8 as i32) as f64;
                bits_max = if (len[toggle as usize] * 8 as i32) as f64 > bits_max {
                    (len[toggle as usize] * 8 as i32) as f64
                } else {
                    bits_max
                };
                bits2 += (len[toggle as usize] * len[toggle as usize] * 64 as i32) as f64;
                if decode_only == 0 {
                    nrg = 0.0f64;
                    k = 0 as i32;
                    while k < frame_size * channels {
                        nrg += *in_0.offset(k as isize) as i32 as f64
                            * *in_0.offset(k as isize) as f64;
                        k += 1;
                    }
                    nrg /= (frame_size * channels) as f64;
                    if nrg > 1e5f64 {
                        bits_act += (len[toggle as usize] * 8 as i32) as f64;
                        count_act += 1;
                    }
                }
            }
            count += 1;
            toggle = (toggle + use_inbandfec) & 1 as i32;
        }
    }

    if decode_only != 0 && count > 0 as i32 {
        frame_size = (tot_samples / count as f64) as i32;
    }
    count -= use_inbandfec;
    if tot_samples >= 1 as i32 as f64 && count > 0 as i32 && frame_size != 0 {
        let mut var: f64 = 0.;
        eprintln!(
            "average bitrate:             {:7.3} kb/s",
            1e-3f64 * bits * sampling_rate as f64 / tot_samples
        );
        eprintln!(
            "maximum bitrate:             {:7.3} kb/s",
            1e-3f64 * bits_max * sampling_rate as f64 / frame_size as f64
        );
        if decode_only == 0 {
            eprintln!(
                "active bitrate:              {:7.3} kb/s",
                1e-3f64 * bits_act * sampling_rate as f64
                    / (1e-15f64 + frame_size as f64 * count_act as f64)
            );
        }
        var = bits2 / count as f64 - bits * bits / (count as f64 * count as f64);
        if var < 0 as i32 as f64 {
            var = 0 as i32 as f64;
        }
        eprintln!(
            "bitrate standard deviation:  {:7.3} kb/s",
            1e-3f64 * var.sqrt() * sampling_rate as f64 / frame_size as f64
        );
    } else {
        eprintln!("bitrate statistics are undefined");
    }
    ret = 0 as i32;

    // let's simplify stuff by not freeing memory ;)
    // opus_encoder_destroy(enc);
    // opus_decoder_destroy(dec);
    // free(data[0 as i32 as usize] as *mut core::ffi::c_void);
    // free(data[1 as i32 as usize] as *mut core::ffi::c_void);
    // if !fin.is_null() {
    //     fclose(fin);
    // }
    // if !fout.is_null() {
    //     fclose(fout);
    // }
    // free(in_0 as *mut core::ffi::c_void);
    // free(out as *mut core::ffi::c_void);
    // free(fbytes as *mut core::ffi::c_void);
    ret
}

pub fn main() {
    unsafe { std::process::exit(main_0()) }
}
