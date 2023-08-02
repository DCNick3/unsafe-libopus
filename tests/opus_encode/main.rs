#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use libc::{getpid, time};

pub mod test_opus_common_h {
    #[inline]
    pub unsafe fn deb2_impl(
        mut _t: *mut u8,
        mut _p: *mut *mut u8,
        mut _k: i32,
        mut _x: i32,
        mut _y: i32,
    ) {
        let mut i: i32 = 0;
        if _x > 2 as i32 {
            if _y < 3 as i32 {
                i = 0 as i32;
                while i < _y {
                    *_p = (*_p).offset(-1);
                    **_p = *_t.offset((i + 1 as i32) as isize);
                    i += 1;
                }
            }
        } else {
            *_t.offset(_x as isize) = *_t.offset((_x - _y) as isize);
            deb2_impl(_t, _p, _k, _x + 1 as i32, _y);
            i = *_t.offset((_x - _y) as isize) as i32 + 1 as i32;
            while i < _k {
                *_t.offset(_x as isize) = i as u8;
                deb2_impl(_t, _p, _k, _x + 1 as i32, _x);
                i += 1;
            }
        };
    }
    #[inline]
    pub unsafe fn debruijn2(mut _k: i32, mut _res: *mut u8) {
        let mut p: *mut u8 = std::ptr::null_mut::<u8>();
        let mut t: *mut u8 = std::ptr::null_mut::<u8>();
        t = malloc(
            (::core::mem::size_of::<u8>() as u64)
                .wrapping_mul(_k as u64)
                .wrapping_mul(2 as i32 as u64),
        ) as *mut u8;
        memset(
            t as *mut core::ffi::c_void,
            0 as i32,
            (::core::mem::size_of::<u8>() as u64)
                .wrapping_mul(_k as u64)
                .wrapping_mul(2 as i32 as u64),
        );
        p = &mut *_res.offset((_k * _k) as isize) as *mut u8;
        deb2_impl(t, &mut p, _k, 1 as i32, 1 as i32);
        free(t as *mut core::ffi::c_void);
    }
    pub static mut Rz: u32 = 0;
    pub static mut Rw: u32 = 0;
    #[inline]
    pub unsafe fn fast_rand() -> u32 {
        Rz = (36969 as i32 as u32)
            .wrapping_mul(Rz & 65535 as i32 as u32)
            .wrapping_add(Rz >> 16 as i32);
        Rw = (18000 as i32 as u32)
            .wrapping_mul(Rw & 65535 as i32 as u32)
            .wrapping_add(Rw >> 16 as i32);
        (Rz << 16 as i32).wrapping_add(Rw)
    }
    pub static mut iseed: u32 = 0;
    pub unsafe fn _test_failed(mut file: *const i8, mut line: i32) -> ! {
        eprintln!();
        eprintln!(" ***************************************************");
        eprintln!(" ***         A fatal error was detected.         ***");
        eprintln!(" ***************************************************");
        eprintln!("Please report this failure and include");
        eprintln!(
            "'make check SEED={} fails {} at line {} for {}'",
            iseed,
            std::ffi::CStr::from_ptr(file as _).to_str().unwrap(),
            line,
            std::ffi::CStr::from_ptr(opus_get_version_string() as _)
                .to_str()
                .unwrap()
        );
        eprintln!("and any relevant details about your system.");
        panic!("test failed");
    }

    use unsafe_libopus::externs::memset;
    use unsafe_libopus::externs::{free, malloc};
    use unsafe_libopus::opus_get_version_string;
}
pub use self::test_opus_common_h::{debruijn2, Rw, Rz, _test_failed, fast_rand, iseed};
use unsafe_libopus::externs::{free, malloc};
use unsafe_libopus::externs::{memcpy, memset};
use unsafe_libopus::{
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

pub unsafe fn generate_music(mut buf: *mut libc::c_short, mut len: i32) {
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
    b2 = 0 as i32;
    a2 = b2;
    b1 = a2;
    a1 = b1;
    d2 = 0 as i32;
    d1 = d2;
    c2 = d1;
    c1 = c2;
    j = 0 as i32;
    i = 0 as i32;
    while i < 2880 as i32 {
        let fresh0 = &mut (*buf.offset((i * 2 as i32 + 1 as i32) as isize));
        *fresh0 = 0 as i32 as libc::c_short;
        *buf.offset((i * 2 as i32) as isize) = *fresh0;
        i += 1;
    }
    i = 2880 as i32;
    while i < len {
        let mut r: u32 = 0;
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        v2 = (((j
            * (j >> 12 as i32 ^ (j >> 10 as i32 | j >> 12 as i32) & 26 as i32 & j >> 7 as i32))
            & 128 as i32)
            + 128 as i32)
            << 15 as i32;
        v1 = v2;
        r = fast_rand();
        v1 = (v1 as u32).wrapping_add(r & 65535 as i32 as u32) as i32 as i32;
        v1 = (v1 as u32).wrapping_sub(r >> 16 as i32) as i32 as i32;
        r = fast_rand();
        v2 = (v2 as u32).wrapping_add(r & 65535 as i32 as u32) as i32 as i32;
        v2 = (v2 as u32).wrapping_sub(r >> 16 as i32) as i32 as i32;
        b1 = v1 - a1 + ((b1 * 61 as i32 + 32 as i32) >> 6 as i32);
        a1 = v1;
        b2 = v2 - a2 + ((b2 * 61 as i32 + 32 as i32) >> 6 as i32);
        a2 = v2;
        c1 = (30 as i32 * (c1 + b1 + d1) + 32 as i32) >> 6 as i32;
        d1 = b1;
        c2 = (30 as i32 * (c2 + b2 + d2) + 32 as i32) >> 6 as i32;
        d2 = b2;
        v1 = (c1 + 128 as i32) >> 8 as i32;
        v2 = (c2 + 128 as i32) >> 8 as i32;
        *buf.offset((i * 2 as i32) as isize) = (if v1 > 32767 as i32 {
            32767 as i32
        } else if v1 < -(32768 as i32) {
            -(32768 as i32)
        } else {
            v1
        }) as libc::c_short;
        *buf.offset((i * 2 as i32 + 1 as i32) as isize) = (if v2 > 32767 as i32 {
            32767 as i32
        } else if v2 < -(32768 as i32) {
            -(32768 as i32)
        } else {
            v2
        }) as libc::c_short;
        if i % 6 as i32 == 0 as i32 {
            j += 1;
        }
        i += 1;
    }
}
pub unsafe fn get_frame_size_enum(mut frame_size: i32, mut sampling_rate: i32) -> i32 {
    let mut frame_size_enum: i32 = 0;
    if frame_size == sampling_rate / 400 as i32 {
        frame_size_enum = 5001 as i32;
    } else if frame_size == sampling_rate / 200 as i32 {
        frame_size_enum = 5002 as i32;
    } else if frame_size == sampling_rate / 100 as i32 {
        frame_size_enum = 5003 as i32;
    } else if frame_size == sampling_rate / 50 as i32 {
        frame_size_enum = 5004 as i32;
    } else if frame_size == sampling_rate / 25 as i32 {
        frame_size_enum = 5005 as i32;
    } else if frame_size == 3 as i32 * sampling_rate / 50 as i32 {
        frame_size_enum = 5006 as i32;
    } else if frame_size == 4 as i32 * sampling_rate / 50 as i32 {
        frame_size_enum = 5007 as i32;
    } else if frame_size == 5 as i32 * sampling_rate / 50 as i32 {
        frame_size_enum = 5008 as i32;
    } else if frame_size == 6 as i32 * sampling_rate / 50 as i32 {
        frame_size_enum = 5009 as i32;
    } else {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            138 as i32,
        );
    }
    frame_size_enum
}
pub unsafe fn test_encode(
    mut enc: *mut OpusEncoder,
    mut channels: i32,
    mut frame_size: i32,
    mut dec: *mut OpusDecoder,
) -> i32 {
    let mut samp_count: i32 = 0 as i32;
    let mut inbuf: *mut i16 = std::ptr::null_mut::<i16>();
    let mut packet: [u8; 1757] = [0; 1757];
    let mut len: i32 = 0;
    let mut outbuf: *mut i16 = std::ptr::null_mut::<i16>();
    let mut out_samples: i32 = 0;
    let mut ret: i32 = 0 as i32;
    inbuf = malloc(
        (::core::mem::size_of::<i16>() as u64)
            .wrapping_mul((48000 as i32 * 30 as i32 / 3 as i32) as u64),
    ) as *mut i16;
    generate_music(inbuf, 48000 as i32 * 30 as i32 / 3 as i32 / 2 as i32);
    outbuf = malloc(
        (::core::mem::size_of::<i16>() as u64)
            .wrapping_mul(5760 as i32 as u64)
            .wrapping_mul(3 as i32 as u64),
    ) as *mut i16;
    loop {
        len = opus_encode(
            enc,
            &mut *inbuf.offset((samp_count * channels) as isize),
            frame_size,
            packet.as_mut_ptr(),
            1500 as i32,
        );
        if len < 0 as i32 || len > 1500 as i32 {
            eprintln!("opus_encode() returned {}", len);
            ret = -(1 as i32);
            break;
        } else {
            out_samples = opus_decode(dec, packet.as_mut_ptr(), len, outbuf, 5760 as i32, 0 as i32);
            if out_samples != frame_size {
                eprintln!("opus_decode() returned {}", out_samples);
                ret = -(1 as i32);
                break;
            } else {
                samp_count += frame_size;
                if samp_count >= 48000 as i32 * 30 as i32 / 3 as i32 / 2 as i32 - 5760 as i32 {
                    break;
                }
            }
        }
    }
    free(inbuf as *mut core::ffi::c_void);
    free(outbuf as *mut core::ffi::c_void);
    ret
}
pub unsafe fn fuzz_encoder_settings(num_encoders: i32, num_setting_changes: i32) {
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut err: i32 = 0;
    let mut sampling_rates: [i32; 5] = [
        8000 as i32,
        12000 as i32,
        16000 as i32,
        24000 as i32,
        48000 as i32,
    ];
    let mut channels: [i32; 2] = [1 as i32, 2 as i32];
    let mut applications: [i32; 3] = [2049 as i32, 2048 as i32, 2051 as i32];
    let mut bitrates: [i32; 11] = [
        6000 as i32,
        12000 as i32,
        16000 as i32,
        24000 as i32,
        32000 as i32,
        48000 as i32,
        64000 as i32,
        96000 as i32,
        510000 as i32,
        -(1000 as i32),
        -(1 as i32),
    ];
    let mut force_channels: [i32; 4] = [-(1000 as i32), -(1000 as i32), 1 as i32, 2 as i32];
    let mut use_vbr: [i32; 3] = [0 as i32, 1 as i32, 1 as i32];
    let mut vbr_constraints: [i32; 3] = [0 as i32, 1 as i32, 1 as i32];
    let mut complexities: [i32; 11] = [
        0 as i32, 1 as i32, 2 as i32, 3 as i32, 4 as i32, 5 as i32, 6 as i32, 7 as i32, 8 as i32,
        9 as i32, 10 as i32,
    ];
    let mut max_bandwidths: [i32; 6] = [
        1101 as i32,
        1102 as i32,
        1103 as i32,
        1104 as i32,
        1105 as i32,
        1105 as i32,
    ];
    let mut signals: [i32; 4] = [-(1000 as i32), -(1000 as i32), 3001 as i32, 3002 as i32];
    let mut inband_fecs: [i32; 3] = [0 as i32, 0 as i32, 1 as i32];
    let mut packet_loss_perc: [i32; 4] = [0 as i32, 1 as i32, 2 as i32, 5 as i32];
    let mut lsb_depths: [i32; 2] = [8 as i32, 24 as i32];
    let mut prediction_disabled: [i32; 3] = [0 as i32, 0 as i32, 1 as i32];
    let mut use_dtx: [i32; 2] = [0 as i32, 1 as i32];
    let mut frame_sizes_ms_x2: [i32; 9] = [
        5 as i32, 10 as i32, 20 as i32, 40 as i32, 80 as i32, 120 as i32, 160 as i32, 200 as i32,
        240 as i32,
    ];
    i = 0 as i32;
    while i < num_encoders {
        let mut sampling_rate: i32 = sampling_rates[(fast_rand() as u64)
            .wrapping_rem(::core::mem::size_of::<[i32; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            as usize];
        let mut num_channels: i32 = channels[(fast_rand() as u64)
            .wrapping_rem(::core::mem::size_of::<[i32; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            as usize];
        let mut application: i32 = applications[(fast_rand() as u64)
            .wrapping_rem(::core::mem::size_of::<[i32; 3]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            as usize];
        dec = opus_decoder_create(sampling_rate, num_channels, &mut err);
        if err != 0 as i32 || dec.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                217 as i32,
            );
        }
        enc = opus_encoder_create(sampling_rate, num_channels, application, &mut err);
        if err != 0 as i32 || enc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                220 as i32,
            );
        }
        j = 0 as i32;
        while j < num_setting_changes {
            let mut bitrate: i32 = bitrates[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 11]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut force_channel: i32 = force_channels[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut vbr: i32 = use_vbr[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut vbr_constraint: i32 = vbr_constraints[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut complexity: i32 = complexities[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 11]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut max_bw: i32 = max_bandwidths[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut sig: i32 = signals[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut inband_fec: i32 = inband_fecs[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut pkt_loss: i32 = packet_loss_perc[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut lsb_depth: i32 = lsb_depths[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 2]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut pred_disabled: i32 = prediction_disabled[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut dtx: i32 = use_dtx[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 2]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut frame_size_ms_x2: i32 = frame_sizes_ms_x2[(fast_rand() as u64)
                .wrapping_rem(::core::mem::size_of::<[i32; 9]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64)
                as usize];
            let mut frame_size: i32 = frame_size_ms_x2 * sampling_rate / 2000 as i32;
            let mut frame_size_enum: i32 = get_frame_size_enum(frame_size, sampling_rate);
            force_channel = if force_channel < num_channels {
                force_channel
            } else {
                num_channels
            };
            if opus_encoder_ctl!(enc, 4002 as i32, bitrate) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    240 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4022 as i32, force_channel) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    241 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4006 as i32, vbr) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    242 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4020 as i32, vbr_constraint) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    243 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4010 as i32, complexity) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    244 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4004 as i32, max_bw) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    245 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4024 as i32, sig) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    246 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4012 as i32, inband_fec) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    247 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4014 as i32, pkt_loss) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    248 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4036 as i32, lsb_depth) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    249 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4042 as i32, pred_disabled) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    250 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4016 as i32, dtx) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    251 as i32,
                );
            }
            if opus_encoder_ctl!(enc, 4040 as i32, frame_size_enum) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    252 as i32,
                );
            }
            if test_encode(enc, num_channels, frame_size, dec) != 0 {
                eprintln!("fuzz_encoder_settings: {} kHz, {} ch, application: {}, {} bps, force ch: {}, vbr: {}, vbr constraint: {}, complexity: {}, max bw: {}, signal: {}, inband fec: {}, pkt loss: {}%, lsb depth: {}, pred disabled: {}, dtx: {}, ({}/2) ms", 
                    sampling_rate / 1000 as i32,
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
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    263 as i32,
                );
            }
            j += 1;
        }
        opus_encoder_destroy(enc);
        opus_decoder_destroy(dec);
        i += 1;
    }
}
pub unsafe fn run_test1(no_fuzz: bool) -> i32 {
    static mut fsizes: [i32; 6] = [
        960 as i32 * 3 as i32,
        960 as i32 * 2 as i32,
        120 as i32,
        240 as i32,
        480 as i32,
        960 as i32,
    ];
    static mut mstrings: [&str; 3] = ["    LP", "Hybrid", "  MDCT"];
    let mut mapping: [u8; 256] = [
        0, 1, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut db62: [u8; 36] = [0; 36];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut rc: i32 = 0;
    let mut err: i32 = 0;
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
    let mut packet: [u8; 1757] = [0; 1757];
    let mut enc_final_range: u32 = 0;
    let mut dec_final_range: u32 = 0;
    let mut fswitch: i32 = 0;
    let mut fsize: i32 = 0;
    let mut count: i32 = 0;
    println!("  Encode+Decode tests.");
    enc = opus_encoder_create(48000 as i32, 2 as i32, 2048 as i32, &mut err);
    if err != 0 as i32 || enc.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            302 as i32,
        );
    }
    i = 0 as i32;
    while i < 2 as i32 {
        let mut ret_err: *mut i32 = std::ptr::null_mut::<i32>();
        ret_err = if i != 0 {
            std::ptr::null_mut::<i32>()
        } else {
            &mut err
        };
        MSenc = opus_multistream_encoder_create(
            8000 as i32,
            2 as i32,
            2 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            -(5 as i32),
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                309 as i32,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as i32,
            0 as i32,
            1 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            2048 as i32,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                312 as i32,
            );
        }
        MSenc = opus_multistream_encoder_create(
            44100 as i32,
            2 as i32,
            2 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            2048 as i32,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                315 as i32,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as i32,
            2 as i32,
            2 as i32,
            3 as i32,
            mapping.as_mut_ptr(),
            2048 as i32,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                318 as i32,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as i32,
            2 as i32,
            -(1 as i32),
            0 as i32,
            mapping.as_mut_ptr(),
            2048 as i32,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                321 as i32,
            );
        }
        MSenc = opus_multistream_encoder_create(
            8000 as i32,
            256 as i32,
            2 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            2048 as i32,
            ret_err,
        );
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !MSenc.is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                324 as i32,
            );
        }
        i += 1;
    }
    MSenc = opus_multistream_encoder_create(
        8000 as i32,
        2 as i32,
        2 as i32,
        0 as i32,
        mapping.as_mut_ptr(),
        2049 as i32,
        &mut err,
    );
    if err != 0 as i32 || MSenc.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            328 as i32,
        );
    }
    if opus_multistream_encoder_ctl!(MSenc, 4003 as i32, &mut i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            331 as i32,
        );
    }
    if opus_multistream_encoder_ctl!(MSenc, 4037 as i32, &mut i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            332 as i32,
        );
    }
    if i < 16 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            333 as i32,
        );
    }
    let mut tmp_enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    if opus_multistream_encoder_ctl!(MSenc, 5120 as i32, 1 as i32, &mut tmp_enc) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            337 as i32,
        );
    }
    if opus_encoder_ctl!(tmp_enc, 4037 as i32, &mut j) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            338 as i32,
        );
    }
    if i != j {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            339 as i32,
        );
    }
    if opus_multistream_encoder_ctl!(MSenc, 5120 as i32, 2 as i32, &mut tmp_enc) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            340 as i32,
        );
    }
    dec = opus_decoder_create(48000 as i32, 2 as i32, &mut err);
    if err != 0 as i32 || dec.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            344 as i32,
        );
    }
    MSdec = opus_multistream_decoder_create(
        48000 as i32,
        2 as i32,
        2 as i32,
        0 as i32,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as i32 || MSdec.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            347 as i32,
        );
    }
    MSdec_err = opus_multistream_decoder_create(
        48000 as i32,
        3 as i32,
        2 as i32,
        0 as i32,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as i32 || MSdec_err.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            350 as i32,
        );
    }
    dec_err[0 as i32 as usize] = malloc(opus_decoder_get_size(2 as i32) as u64) as *mut OpusDecoder;
    memcpy(
        dec_err[0 as i32 as usize] as *mut core::ffi::c_void,
        dec as *const core::ffi::c_void,
        opus_decoder_get_size(2 as i32) as u64,
    );
    dec_err[1 as i32 as usize] = opus_decoder_create(48000 as i32, 1 as i32, &mut err);
    dec_err[2 as i32 as usize] = opus_decoder_create(24000 as i32, 2 as i32, &mut err);
    dec_err[3 as i32 as usize] = opus_decoder_create(24000 as i32, 1 as i32, &mut err);
    dec_err[4 as i32 as usize] = opus_decoder_create(16000 as i32, 2 as i32, &mut err);
    dec_err[5 as i32 as usize] = opus_decoder_create(16000 as i32, 1 as i32, &mut err);
    dec_err[6 as i32 as usize] = opus_decoder_create(12000 as i32, 2 as i32, &mut err);
    dec_err[7 as i32 as usize] = opus_decoder_create(12000 as i32, 1 as i32, &mut err);
    dec_err[8 as i32 as usize] = opus_decoder_create(8000 as i32, 2 as i32, &mut err);
    dec_err[9 as i32 as usize] = opus_decoder_create(8000 as i32, 1 as i32, &mut err);
    i = 0 as i32;
    while i < 10 as i32 {
        if (dec_err[i as usize]).is_null() {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                363 as i32,
            );
        }
        i += 1;
    }
    let mut enccpy: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    enccpy = malloc(opus_encoder_get_size(2 as i32) as u64) as *mut OpusEncoder;
    memcpy(
        enccpy as *mut core::ffi::c_void,
        enc as *const core::ffi::c_void,
        opus_encoder_get_size(2 as i32) as u64,
    );
    memset(
        enc as *mut core::ffi::c_void,
        255 as i32,
        opus_encoder_get_size(2 as i32) as u64,
    );
    opus_encoder_destroy(enc);
    enc = enccpy;
    inbuf = malloc(
        (::core::mem::size_of::<libc::c_short>() as u64)
            .wrapping_mul((48000 as i32 * 30 as i32) as u64)
            .wrapping_mul(2 as i32 as u64),
    ) as *mut libc::c_short;
    outbuf = malloc(
        (::core::mem::size_of::<libc::c_short>() as u64)
            .wrapping_mul((48000 as i32 * 30 as i32) as u64)
            .wrapping_mul(2 as i32 as u64),
    ) as *mut libc::c_short;
    out2buf = malloc(
        (::core::mem::size_of::<libc::c_short>() as u64)
            .wrapping_mul(5760 as i32 as u64)
            .wrapping_mul(3 as i32 as u64),
    ) as *mut libc::c_short;
    if inbuf.is_null() || outbuf.is_null() || out2buf.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            378 as i32,
        );
    }
    generate_music(inbuf, 48000 as i32 * 30 as i32);
    if opus_encoder_ctl!(enc, 4008 as i32, -(1000 as i32)) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            387 as i32,
        );
    }
    if opus_encoder_ctl!(enc, 11002 as i32, -(2 as i32)) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            388 as i32,
        );
    }
    if opus_encode(enc, inbuf, 500 as i32, packet.as_mut_ptr(), 1500 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            389 as i32,
        );
    }
    rc = 0 as i32;
    while rc < 3 as i32 {
        if opus_encoder_ctl!(enc, 4006 as i32, (rc < 2 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                393 as i32,
            );
        }
        if opus_encoder_ctl!(enc, 4020 as i32, (rc == 1 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                394 as i32,
            );
        }
        if opus_encoder_ctl!(enc, 4020 as i32, (rc == 1 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                395 as i32,
            );
        }
        if opus_encoder_ctl!(enc, 4012 as i32, (rc == 0 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                396 as i32,
            );
        }
        j = 0 as i32;
        while j < 13 as i32 {
            let mut rate: i32 = 0;
            let mut modes: [i32; 13] = [
                0 as i32, 0 as i32, 0 as i32, 1 as i32, 1 as i32, 1 as i32, 1 as i32, 2 as i32,
                2 as i32, 2 as i32, 2 as i32, 2 as i32, 2 as i32,
            ];
            let mut rates: [i32; 13] = [
                6000 as i32,
                12000 as i32,
                48000 as i32,
                16000 as i32,
                32000 as i32,
                48000 as i32,
                64000 as i32,
                512000 as i32,
                13000 as i32,
                24000 as i32,
                48000 as i32,
                64000 as i32,
                96000 as i32,
            ];
            let mut frame: [i32; 13] = [
                960 as i32 * 2 as i32,
                960 as i32,
                480 as i32,
                960 as i32,
                960 as i32,
                960 as i32,
                480 as i32,
                960 as i32 * 3 as i32,
                960 as i32 * 3 as i32,
                960 as i32,
                480 as i32,
                240 as i32,
                120 as i32,
            ];
            rate = (rates[j as usize] as u32)
                .wrapping_add((fast_rand()).wrapping_rem(rates[j as usize] as u32))
                as i32;
            i = 0 as i32;
            count = i;
            loop {
                let mut bw: i32 = 0;
                let mut len: i32 = 0;
                let mut out_samples: i32 = 0;
                let mut frame_size: i32 = 0;
                frame_size = frame[j as usize];
                if fast_rand() & 255 as i32 as u32 == 0 as i32 as u32 {
                    if opus_encoder_ctl!(enc, 4028 as i32) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            410 as i32,
                        );
                    }
                    if opus_decoder_ctl!(dec, 4028 as i32) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            411 as i32,
                        );
                    }
                    if fast_rand() & 1 as i32 as u32 != 0 as i32 as u32
                        && opus_decoder_ctl!(
                            dec_err[(fast_rand() & 1 as i32 as u32) as usize],
                            4028 as i32,
                        ) != 0 as i32
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            414 as i32,
                        );
                    }
                }
                if fast_rand() & 127 as i32 as u32 == 0 as i32 as u32
                    && opus_decoder_ctl!(
                        dec_err[(fast_rand() & 1 as i32 as u32) as usize],
                        4028 as i32,
                    ) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        419 as i32,
                    );
                }
                if (fast_rand()).wrapping_rem(10 as i32 as u32) == 0 as i32 as u32 {
                    let mut complex: i32 = (fast_rand()).wrapping_rem(11 as i32 as u32) as i32;
                    if opus_encoder_ctl!(enc, 4010 as i32, complex) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            423 as i32,
                        );
                    }
                }
                if (fast_rand()).wrapping_rem(50 as i32 as u32) == 0 as i32 as u32 {
                    opus_decoder_ctl!(dec, 4028 as i32);
                }
                if opus_encoder_ctl!(enc, 4012 as i32, (rc == 0 as i32) as i32,) != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        426 as i32,
                    );
                }
                if opus_encoder_ctl!(enc, 11002 as i32, 1000 as i32 + modes[j as usize],)
                    != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        427 as i32,
                    );
                }
                fast_rand();
                if opus_encoder_ctl!(enc, 4016 as i32, (fast_rand() & 1 as i32 as u32) as i32,)
                    != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        428 as i32,
                    );
                }
                if opus_encoder_ctl!(enc, 4002 as i32, rate) != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        429 as i32,
                    );
                }
                if rates[j as usize] >= 64000 as i32 {
                } else {
                };
                if opus_encoder_ctl!(
                    enc,
                    4022 as i32,
                    if rates[j as usize] >= 64000 as i32 {
                        2 as i32
                    } else {
                        1 as i32
                    },
                ) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        430 as i32,
                    );
                }
                if opus_encoder_ctl!(enc, 4010 as i32, (count >> 2 as i32) % 11 as i32,) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        431 as i32,
                    );
                }
                fast_rand();
                fast_rand();
                if opus_encoder_ctl!(
                    enc,
                    4014 as i32,
                    (fast_rand() & 15 as i32 as u32 & (fast_rand()).wrapping_rem(15 as i32 as u32))
                        as i32,
                ) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        432 as i32,
                    );
                }
                bw = (if modes[j as usize] == 0 as i32 {
                    (1101 as i32 as u32).wrapping_add((fast_rand()).wrapping_rem(3 as i32 as u32))
                } else if modes[j as usize] == 1 as i32 {
                    (1104 as i32 as u32).wrapping_add(fast_rand() & 1 as i32 as u32)
                } else {
                    (1101 as i32 as u32).wrapping_add((fast_rand()).wrapping_rem(5 as i32 as u32))
                }) as i32;
                if modes[j as usize] == 2 as i32 && bw == 1102 as i32 {
                    bw += 3 as i32;
                }
                if opus_encoder_ctl!(enc, 4008 as i32, bw) != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        437 as i32,
                    );
                }
                len = opus_encode(
                    enc,
                    &mut *inbuf.offset((i << 1 as i32) as isize),
                    frame_size,
                    packet.as_mut_ptr(),
                    1500 as i32,
                );
                if len < 0 as i32 || len > 1500 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        439 as i32,
                    );
                }
                if opus_encoder_ctl!(enc, 4031 as i32, &mut enc_final_range) != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        440 as i32,
                    );
                }
                if fast_rand() & 3 as i32 as u32 == 0 as i32 as u32 {
                    if opus_packet_pad(packet.as_mut_ptr(), len, len + 1 as i32) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            443 as i32,
                        );
                    }
                    len += 1;
                }
                if fast_rand() & 7 as i32 as u32 == 0 as i32 as u32 {
                    if opus_packet_pad(packet.as_mut_ptr(), len, len + 256 as i32) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            448 as i32,
                        );
                    }
                    len += 256 as i32;
                }
                if fast_rand() & 3 as i32 as u32 == 0 as i32 as u32 {
                    len = opus_packet_unpad(packet.as_mut_ptr(), len);
                    if len < 1 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            454 as i32,
                        );
                    }
                }
                out_samples = opus_decode(
                    dec,
                    packet.as_mut_ptr(),
                    len,
                    &mut *outbuf.offset((i << 1 as i32) as isize),
                    5760 as i32,
                    0 as i32,
                );
                if out_samples != frame_size {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        457 as i32,
                    );
                }
                if opus_decoder_ctl!(dec, 4031 as i32, &mut dec_final_range) != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        458 as i32,
                    );
                }
                if enc_final_range != dec_final_range {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        459 as i32,
                    );
                }
                out_samples = opus_decode(
                    dec_err[0 as i32 as usize],
                    packet.as_mut_ptr(),
                    len,
                    out2buf,
                    frame_size,
                    (fast_rand() & 3 as i32 as u32 != 0 as i32 as u32) as i32,
                );
                if out_samples != frame_size {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        462 as i32,
                    );
                }
                out_samples = opus_decode(
                    dec_err[1 as i32 as usize],
                    packet.as_mut_ptr(),
                    if fast_rand() & 3 as i32 as u32 == 0 as i32 as u32 {
                        0 as i32
                    } else {
                        len
                    },
                    out2buf,
                    5760 as i32,
                    (fast_rand() & 7 as i32 as u32 != 0 as i32 as u32) as i32,
                );
                if out_samples < 120 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        464 as i32,
                    );
                }
                i += frame_size;
                count += 1;
                if i >= 48000 as i32 * 30 as i32 / 3 as i32 - 5760 as i32 {
                    break;
                }
            }
            println!(
                "    Mode {} FB encode {}, {:6} bps OK.",
                mstrings[modes[j as usize] as usize],
                if rc == 0 as i32 {
                    " VBR"
                } else if rc == 1 as i32 {
                    "CVBR"
                } else {
                    " CBR"
                },
                rate,
            );
            j += 1;
        }
        rc += 1;
    }
    if opus_encoder_ctl!(enc, 11002 as i32, -(1000 as i32)) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            472 as i32,
        );
    }
    if opus_encoder_ctl!(enc, 4022 as i32, -(1000 as i32)) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            473 as i32,
        );
    }
    if opus_encoder_ctl!(enc, 4012 as i32, 0 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            474 as i32,
        );
    }
    if opus_encoder_ctl!(enc, 4016 as i32, 0 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            475 as i32,
        );
    }
    rc = 0 as i32;
    while rc < 3 as i32 {
        if opus_multistream_encoder_ctl!(MSenc, 4006 as i32, (rc < 2 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                479 as i32,
            );
        }
        if opus_multistream_encoder_ctl!(MSenc, 4020 as i32, (rc == 1 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                480 as i32,
            );
        }
        if opus_multistream_encoder_ctl!(MSenc, 4020 as i32, (rc == 1 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                481 as i32,
            );
        }
        if opus_multistream_encoder_ctl!(MSenc, 4012 as i32, (rc == 0 as i32) as i32,) != 0 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                482 as i32,
            );
        }
        j = 0 as i32;
        while j < 16 as i32 {
            let mut rate_0: i32 = 0;
            let mut modes_0: [i32; 16] = [
                0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32,
                2 as i32, 2 as i32, 2 as i32, 2 as i32, 2 as i32, 2 as i32, 2 as i32, 2 as i32,
            ];
            let mut rates_0: [i32; 16] = [
                4000 as i32,
                12000 as i32,
                32000 as i32,
                8000 as i32,
                16000 as i32,
                32000 as i32,
                48000 as i32,
                88000 as i32,
                4000 as i32,
                12000 as i32,
                32000 as i32,
                8000 as i32,
                16000 as i32,
                32000 as i32,
                48000 as i32,
                88000 as i32,
            ];
            let mut frame_0: [i32; 16] = [
                160 as i32 * 1 as i32,
                160 as i32,
                80 as i32,
                160 as i32,
                160 as i32,
                80 as i32,
                40 as i32,
                20 as i32,
                160 as i32 * 1 as i32,
                160 as i32,
                80 as i32,
                160 as i32,
                160 as i32,
                80 as i32,
                40 as i32,
                20 as i32,
            ];
            (rc == 0 as i32 && j == 1 as i32) as i32;
            if opus_multistream_encoder_ctl!(
                MSenc,
                4012 as i32,
                (rc == 0 as i32 && j == 1 as i32) as i32,
            ) != 0 as i32
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    489 as i32,
                );
            }
            if opus_multistream_encoder_ctl!(MSenc, 11002 as i32, 1000 as i32 + modes_0[j as usize],)
                != 0 as i32
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    490 as i32,
                );
            }
            rate_0 = (rates_0[j as usize] as u32)
                .wrapping_add((fast_rand()).wrapping_rem(rates_0[j as usize] as u32))
                as i32;
            fast_rand();
            if opus_multistream_encoder_ctl!(
                MSenc,
                4016 as i32,
                (fast_rand() & 1 as i32 as u32) as i32,
            ) != 0 as i32
            {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    492 as i32,
                );
            }
            if opus_multistream_encoder_ctl!(MSenc, 4002 as i32, rate_0) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                    493 as i32,
                );
            }
            i = 0 as i32;
            count = i;
            loop {
                let mut len_0: i32 = 0;
                let mut out_samples_0: i32 = 0;
                let mut frame_size_0: i32 = 0;
                let mut loss: i32 = 0;
                let mut pred: i32 = 0;
                if opus_multistream_encoder_ctl!(MSenc, 4043 as i32, &mut pred) != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        498 as i32,
                    );
                }
                fast_rand();
                if pred != 0 {
                } else {
                };
                if opus_multistream_encoder_ctl!(
                    MSenc,
                    4042 as i32,
                    (((fast_rand() & 15 as i32 as u32) as i32)
                        < (if pred != 0 { 11 as i32 } else { 4 as i32 }))
                        as i32,
                ) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        499 as i32,
                    );
                }
                frame_size_0 = frame_0[j as usize];
                if opus_multistream_encoder_ctl!(
                    MSenc,
                    4010 as i32,
                    (count >> 2 as i32) % 11 as i32,
                ) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        501 as i32,
                    );
                }
                fast_rand();
                fast_rand();
                if opus_multistream_encoder_ctl!(
                    MSenc,
                    4014 as i32,
                    (fast_rand() & 15 as i32 as u32 & (fast_rand()).wrapping_rem(15 as i32 as u32))
                        as i32,
                ) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        502 as i32,
                    );
                }
                if fast_rand() & 255 as i32 as u32 == 0 as i32 as u32 {
                    if opus_multistream_encoder_ctl!(MSenc, 4028 as i32) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            505 as i32,
                        );
                    }
                    if opus_multistream_decoder_ctl!(MSdec, 4028 as i32) != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            506 as i32,
                        );
                    }
                    if fast_rand() & 3 as i32 as u32 != 0 as i32 as u32
                        && opus_multistream_decoder_ctl!(MSdec_err, 4028 as i32) != 0 as i32
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            509 as i32,
                        );
                    }
                }
                if fast_rand() & 255 as i32 as u32 == 0 as i32 as u32
                    && opus_multistream_decoder_ctl!(MSdec_err, 4028 as i32) != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        514 as i32,
                    );
                }
                len_0 = opus_multistream_encode(
                    MSenc,
                    &mut *inbuf.offset((i << 1 as i32) as isize),
                    frame_size_0,
                    packet.as_mut_ptr(),
                    1500 as i32,
                );
                if len_0 < 0 as i32 || len_0 > 1500 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        517 as i32,
                    );
                }
                if opus_multistream_encoder_ctl!(MSenc, 4031 as i32, &mut enc_final_range)
                    != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        518 as i32,
                    );
                }
                if fast_rand() & 3 as i32 as u32 == 0 as i32 as u32 {
                    if opus_multistream_packet_pad(
                        packet.as_mut_ptr(),
                        len_0,
                        len_0 + 1 as i32,
                        2 as i32,
                    ) != 0 as i32
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            521 as i32,
                        );
                    }
                    len_0 += 1;
                }
                if fast_rand() & 7 as i32 as u32 == 0 as i32 as u32 {
                    if opus_multistream_packet_pad(
                        packet.as_mut_ptr(),
                        len_0,
                        len_0 + 256 as i32,
                        2 as i32,
                    ) != 0 as i32
                    {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            526 as i32,
                        );
                    }
                    len_0 += 256 as i32;
                }
                if fast_rand() & 3 as i32 as u32 == 0 as i32 as u32 {
                    len_0 = opus_multistream_packet_unpad(packet.as_mut_ptr(), len_0, 2 as i32);
                    if len_0 < 1 as i32 {
                        _test_failed(
                            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                            532 as i32,
                        );
                    }
                }
                out_samples_0 = opus_multistream_decode(
                    MSdec,
                    packet.as_mut_ptr(),
                    len_0,
                    out2buf,
                    5760 as i32,
                    0 as i32,
                );
                if out_samples_0 != frame_size_0 * 6 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        535 as i32,
                    );
                }
                if opus_multistream_decoder_ctl!(MSdec, 4031 as i32, &mut dec_final_range)
                    != 0 as i32
                {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        536 as i32,
                    );
                }
                if enc_final_range != dec_final_range {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        537 as i32,
                    );
                }
                loss = (fast_rand() & 63 as i32 as u32 == 0 as i32 as u32) as i32;
                out_samples_0 = opus_multistream_decode(
                    MSdec_err,
                    packet.as_mut_ptr(),
                    if loss != 0 { 0 as i32 } else { len_0 },
                    out2buf,
                    frame_size_0 * 6 as i32,
                    (fast_rand() & 3 as i32 as u32 != 0 as i32 as u32) as i32,
                );
                if out_samples_0 != frame_size_0 * 6 as i32 {
                    _test_failed(
                        b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                        541 as i32,
                    );
                }
                i += frame_size_0;
                count += 1;
                if i >= 48000 as i32 * 30 as i32 / 3 as i32 / 12 as i32 - 5760 as i32 {
                    break;
                }
            }
            println!(
                "    Mode {} NB dual-mono MS encode {}, {:6} bps OK.",
                mstrings[modes_0[j as usize] as usize],
                if rc == 0 as i32 {
                    " VBR"
                } else if rc == 1 as i32 {
                    "CVBR"
                } else {
                    " CBR"
                },
                rate_0,
            );
            j += 1;
        }
        rc += 1;
    }
    bitrate_bps = 512000 as i32;
    fsize = (fast_rand()).wrapping_rem(31 as i32 as u32) as i32;
    fswitch = 100 as i32;
    debruijn2(6 as i32, db62.as_mut_ptr());
    i = 0 as i32;
    count = i;
    loop {
        let mut toc: u8 = 0;
        let mut frames: [*const u8; 48] = [std::ptr::null::<u8>(); 48];
        let mut size: [libc::c_short; 48] = [0; 48];
        let mut payload_offset: i32 = 0;
        let mut dec_final_range2: u32 = 0;
        let mut jj: i32 = 0;
        let mut dec2: i32 = 0;
        let mut len_1: i32 = 0;
        let mut out_samples_1: i32 = 0;
        let mut frame_size_1: i32 = fsizes[db62[fsize as usize] as usize];
        let mut offset: i32 = i % (48000 as i32 * 30 as i32 - 5760 as i32);
        opus_encoder_ctl!(enc, 4002 as i32, bitrate_bps);
        len_1 = opus_encode(
            enc,
            &mut *inbuf.offset((offset << 1 as i32) as isize),
            frame_size_1,
            packet.as_mut_ptr(),
            1500 as i32,
        );
        if len_1 < 0 as i32 || len_1 > 1500 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                569 as i32,
            );
        }
        count += 1;
        opus_encoder_ctl!(enc, 4031 as i32, &mut enc_final_range);
        out_samples_1 = opus_decode(
            dec,
            packet.as_mut_ptr(),
            len_1,
            &mut *outbuf.offset((offset << 1 as i32) as isize),
            5760 as i32,
            0 as i32,
        );
        if out_samples_1 != frame_size_1 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                575 as i32,
            );
        }
        opus_decoder_ctl!(dec, 4031 as i32, &mut dec_final_range);
        if dec_final_range != enc_final_range {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                580 as i32,
            );
        }
        if opus_packet_parse(
            packet.as_mut_ptr(),
            len_1,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        ) <= 0 as i32
        {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                585 as i32,
            );
        }
        if fast_rand() & 1023 as i32 as u32 == 0 as i32 as u32 {
            len_1 = 0 as i32;
        }
        j = (frames[0 as i32 as usize]).offset_from(packet.as_mut_ptr()) as i64 as i32;
        while j < len_1 {
            jj = 0 as i32;
            while jj < 8 as i32 {
                packet[j as usize] = (packet[j as usize] as i32
                    ^ ((!no_fuzz && fast_rand() & 1023 as i32 as u32 == 0 as i32 as u32) as i32)
                        << jj) as u8;
                jj += 1;
            }
            j += 1;
        }
        out_samples_1 = opus_decode(
            dec_err[0 as i32 as usize],
            if len_1 > 0 as i32 {
                packet.as_mut_ptr()
            } else {
                std::ptr::null_mut::<u8>()
            },
            len_1,
            out2buf,
            5760 as i32,
            0 as i32,
        );
        if out_samples_1 < 0 as i32 || out_samples_1 > 5760 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                589 as i32,
            );
        }
        if len_1 > 0 as i32 && out_samples_1 != frame_size_1 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                590 as i32,
            );
        }
        opus_decoder_ctl!(
            dec_err[0 as i32 as usize],
            4031 as i32,
            &mut dec_final_range
        );
        dec2 = (fast_rand())
            .wrapping_rem(9 as i32 as u32)
            .wrapping_add(1 as i32 as u32) as i32;
        out_samples_1 = opus_decode(
            dec_err[dec2 as usize],
            if len_1 > 0 as i32 {
                packet.as_mut_ptr()
            } else {
                std::ptr::null_mut::<u8>()
            },
            len_1,
            out2buf,
            5760 as i32,
            0 as i32,
        );
        if out_samples_1 < 0 as i32 || out_samples_1 > 5760 as i32 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                597 as i32,
            );
        }
        opus_decoder_ctl!(dec_err[dec2 as usize], 4031 as i32, &mut dec_final_range2);
        if len_1 > 0 as i32 && dec_final_range != dec_final_range2 {
            _test_failed(
                b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
                600 as i32,
            );
        }
        fswitch -= 1;
        if fswitch < 1 as i32 {
            let mut new_size: i32 = 0;
            fsize = (fsize + 1 as i32) % 36 as i32;
            new_size = fsizes[db62[fsize as usize] as usize];
            if new_size == 960 as i32 || new_size == 480 as i32 {
                fswitch = ((2880 as i32 / new_size) as u32).wrapping_mul(
                    (fast_rand())
                        .wrapping_rem(19 as i32 as u32)
                        .wrapping_add(1 as i32 as u32),
                ) as i32;
            } else {
                fswitch = (fast_rand())
                    .wrapping_rem((2880 as i32 / new_size) as u32)
                    .wrapping_add(1 as i32 as u32) as i32;
            }
        }
        bitrate_bps = ((fast_rand())
            .wrapping_rem(508000 as i32 as u32)
            .wrapping_add(4000 as i32 as u32)
            .wrapping_add(bitrate_bps as u32)
            >> 1 as i32) as i32;
        i += frame_size_1;
        if i >= 48000 as i32 * 30 as i32 * 4 as i32 {
            break;
        }
    }
    println!(
        "    All framesize pairs switching encode, {} frames OK.",
        count
    );
    if opus_encoder_ctl!(enc, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            616 as i32,
        );
    }
    opus_encoder_destroy(enc);
    if opus_multistream_encoder_ctl!(MSenc, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            618 as i32,
        );
    }
    opus_multistream_encoder_destroy(MSenc);
    if opus_decoder_ctl!(dec, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            620 as i32,
        );
    }
    opus_decoder_destroy(dec);
    if opus_multistream_decoder_ctl!(MSdec, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            622 as i32,
        );
    }
    opus_multistream_decoder_destroy(MSdec);
    opus_multistream_decoder_destroy(MSdec_err);
    i = 0 as i32;
    while i < 10 as i32 {
        opus_decoder_destroy(dec_err[i as usize]);
        i += 1;
    }
    free(inbuf as *mut core::ffi::c_void);
    free(outbuf as *mut core::ffi::c_void);
    free(out2buf as *mut core::ffi::c_void);
    0 as i32
}

pub fn print_usage(argv0: &str) {
    eprintln!(
        "Usage: {} [<seed>] [-fuzz <num_encoders> <num_settings_per_encoder>]",
        argv0,
    );
}

// make dummy arguments
// rust's test harness has its own arguments and will handle them itself
// not sure of the best way to pass arguments except modifying the code rn...
// provide a fixed seed 42
const DUMMY_ARGS: &[&str] = &["test_opus_encode", "42"];

unsafe fn main_0() -> i32 {
    let mut args = DUMMY_ARGS.into_iter().map(|v| v.to_string()); // std::env::args();
    let argv0 = args.next().unwrap();

    iseed = match args
        .next()
        .map(|v| v.parse().expect("Failed to parse seed from command line"))
    {
        Some(v) => {
            eprintln!("Using seed from (dummy) arguments: {}", v);
            v
        }
        None => match std::env::var("SEED")
            .ok()
            .as_ref()
            .map(|v| v.parse().expect("Failed to parse seed from environment"))
        {
            Some(v) => {
                eprintln!("Using seed from environment: {}", v);
                v
            }
            None => {
                let v = time(std::ptr::null_mut()) as u32
                    ^ (getpid() as u32 & 65535 as i32 as u32) << 16 as i32;
                eprintln!("Using time-based seed: {}", v);
                v
            }
        },
    };

    Rz = iseed;
    Rw = Rz;

    let mut num_encoders_to_fuzz: i32 = 5 as i32;
    let mut num_setting_changes: i32 = 40 as i32;

    while let Some(arg) = args.next() {
        if arg == "-fuzz" {
            num_encoders_to_fuzz = match args.next().and_then(|v| v.parse().ok()) {
                Some(v) => v,
                None => {
                    print_usage(&argv0);
                    return 1;
                }
            };
            num_setting_changes = match args.next().and_then(|v| v.parse().ok()) {
                Some(v) => v,
                None => {
                    print_usage(&argv0);
                    return 1;
                }
            };
        } else {
            print_usage(&argv0);
            return 1 as i32;
        }
    }

    let oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_encode.c\0" as *const u8 as *const i8,
            682 as i32,
        );
    }
    eprintln!(
        "Testing {} encoder. Random seed: {} ({:4X})",
        std::ffi::CStr::from_ptr(oversion as _).to_str().unwrap(),
        iseed,
        fast_rand() % 65535
    );
    regression_test();
    run_test1(std::env::var("TEST_OPUS_NOFUZZ").is_ok());
    if std::env::var("TEST_OPUS_NOFUZZ").is_err() {
        eprintln!(
            "Running fuzz_encoder_settings with {} encoder(s) and {} setting change(s) each.",
            num_encoders_to_fuzz, num_setting_changes,
        );
        fuzz_encoder_settings(num_encoders_to_fuzz, num_setting_changes);
    }
    eprintln!("Tests completed successfully.");
    0 as i32
}

#[test]
fn test_opus_encode() {
    assert_eq!(unsafe { main_0() }, 0, "Test returned a non-zero exit code");
}
