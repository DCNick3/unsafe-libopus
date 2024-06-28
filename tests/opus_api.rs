#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(deprecated)]

pub mod test_opus_common_h {
    pub unsafe fn _test_failed(mut file: *const i8, mut line: i32) -> ! {
        eprintln!();
        eprintln!(" ***************************************************");
        eprintln!(" ***         A fatal error was detected.         ***");
        eprintln!(" ***************************************************");
        eprintln!("Please report this failure and include");
        eprintln!(
            "'make check fails {} at line {} for {}'",
            std::ffi::CStr::from_ptr(file as _).to_str().unwrap(),
            line,
            opus_get_version_string()
        );
        eprintln!("and any relevant details about your system.");
        panic!("test failed");
    }

    use unsafe_libopus::opus_get_version_string;
}

pub use self::test_opus_common_h::_test_failed;

use unsafe_libopus::externs::{free, malloc};
use unsafe_libopus::externs::{memcmp, memcpy, memset};
use unsafe_libopus::{
    opus_decode, opus_decode_float, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy,
    opus_decoder_get_nb_samples, opus_decoder_get_size, opus_decoder_init, opus_encode,
    opus_encode_float, opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy,
    opus_encoder_get_size, opus_encoder_init, opus_get_version_string, opus_multistream_decode,
    opus_multistream_decode_float, opus_multistream_decoder_create, opus_multistream_decoder_ctl,
    opus_multistream_decoder_destroy, opus_multistream_decoder_get_size,
    opus_multistream_decoder_init, opus_multistream_packet_pad, opus_multistream_packet_unpad,
    opus_packet_get_bandwidth, opus_packet_get_nb_frames, opus_packet_get_nb_samples,
    opus_packet_get_samples_per_frame, opus_packet_pad, opus_packet_parse, opus_packet_unpad,
    opus_repacketizer_cat, opus_repacketizer_create, opus_repacketizer_destroy,
    opus_repacketizer_get_nb_frames, opus_repacketizer_get_size, opus_repacketizer_init,
    opus_repacketizer_out, opus_repacketizer_out_range, opus_strerror, OpusDecoder, OpusEncoder,
    OpusMSDecoder, OpusRepacketizer,
};

pub static mut null_int_ptr: *mut i32 =
    0 as *const core::ffi::c_void as *mut core::ffi::c_void as *mut i32;
pub static mut null_uint_ptr: *mut u32 =
    0 as *const core::ffi::c_void as *mut core::ffi::c_void as *mut u32;
static mut opus_rates: [i32; 5] = [48000, 24000, 16000, 12000, 8000];
pub unsafe fn test_dec_api() -> i32 {
    let mut dec_final_range: u32 = 0;
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut dec2: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cfgs: i32 = 0;
    let mut packet: [u8; 1276] = [0; 1276];
    let mut fbuf: [f32; 1920] = [0.; 1920];
    let mut sbuf: [libc::c_short; 1920] = [0; 1920];
    let mut c: i32 = 0;
    let mut err: i32 = 0;
    cfgs = 0;
    println!("\n  Decoder basic API tests");
    println!("  ---------------------------------------------------");
    c = 0;
    while c < 4 {
        i = opus_decoder_get_size(c);
        if (c == 1 || c == 2) && (i <= 2048 || i > (1) << 16) || c != 1 && c != 2 && i != 0 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 106);
        }
        println!(
            "    opus_decoder_get_size({})={} ...............{} OK.",
            c,
            i,
            if i > 0 { "" } else { "...." }
        );
        cfgs += 1;
        c += 1;
    }
    c = 0;
    while c < 4 {
        i = -(7);
        while i <= 96000 {
            let mut fs: i32 = 0;
            if !((i == 8000 || i == 12000 || i == 16000 || i == 24000 || i == 48000)
                && (c == 1 || c == 2))
            {
                match i {
                    -5 => {
                        fs = -(8000);
                    }
                    -6 => {
                        fs = 2147483647;
                    }
                    -7 => {
                        fs = -(2147483647) - 1;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0;
                dec = opus_decoder_create(fs, c, &mut err);
                if err != -1 || !dec.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 128);
                }
                cfgs += 1;
                dec = opus_decoder_create(fs, c, std::ptr::null_mut::<i32>());
                if !dec.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 131);
                }
                cfgs += 1;
                dec = malloc(opus_decoder_get_size(2) as u64) as *mut OpusDecoder;
                if dec.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 134);
                }
                err = opus_decoder_init(dec, fs, c);
                if err != -1 {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 136);
                }
                cfgs += 1;
                free(dec as *mut core::ffi::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    dec = opus_decoder_create(48000, 2, &mut err);
    if err != 0 || dec.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 144);
    }
    cfgs += 1;
    println!("    opus_decoder_create() ........................ OK.");
    println!("    opus_decoder_init() .......................... OK.");
    err = opus_decoder_ctl!(&mut *dec, 4031, &mut dec_final_range);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 155);
    }
    println!("    OPUS_GET_FINAL_RANGE ......................... OK.");
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, -(5));
    if err != -(5) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 161);
    }
    println!("    OPUS_UNIMPLEMENTED ........................... OK.");
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4009, &mut i);
    if err != 0 || i != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 169);
    }
    println!("    OPUS_GET_BANDWIDTH ........................... OK.");
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4029, &mut i);
    if err != 0 || i != 48000 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 177);
    }
    println!("    OPUS_GET_SAMPLE_RATE ......................... OK.");
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4033, &mut i);
    if err != 0 || i > 0 || i < -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 187);
    }
    cfgs += 1;
    packet[0 as usize] = ((63) << 2) as u8;
    packet[2 as usize] = 0;
    packet[1 as usize] = packet[2 as usize];
    if opus_decode(&mut *dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 960, 0) != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 191);
    }
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4033, &mut i);
    if err != 0 || i > 0 || i < -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 195);
    }
    cfgs += 1;
    packet[0 as usize] = 1;
    if opus_decode(&mut *dec, packet.as_mut_ptr(), 1, sbuf.as_mut_ptr(), 960, 0) != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 198);
    }
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4033, &mut i);
    if err != 0 || i > 0 || i < -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 202);
    }
    cfgs += 1;
    println!("    OPUS_GET_PITCH ............................... OK.");
    err = opus_decoder_ctl!(&mut *dec, 4039, &mut i);
    if err != 0 || i != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 210);
    }
    cfgs += 1;
    println!("    OPUS_GET_LAST_PACKET_DURATION ................ OK.");
    err = opus_decoder_ctl!(&mut *dec, 4045, &mut i);
    if err != 0 || i != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 217);
    }
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4034, -(32769));
    if err != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 223);
    }
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4034, 32768);
    if err != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 226);
    }
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4034, -(15));
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 229);
    }
    cfgs += 1;
    err = opus_decoder_ctl!(&mut *dec, 4045, &mut i);
    if err != 0 || i != -(15) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 234);
    }
    cfgs += 1;
    println!("    OPUS_SET_GAIN ................................ OK.");
    println!("    OPUS_GET_GAIN ................................ OK.");
    dec2 = malloc(opus_decoder_get_size(2) as u64) as *mut OpusDecoder;
    memcpy(
        dec2 as *mut core::ffi::c_void,
        dec as *const core::ffi::c_void,
        opus_decoder_get_size(2) as u64,
    );
    if opus_decoder_ctl!(&mut *dec, 4028) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 242);
    }
    if memcmp(
        dec2 as *const core::ffi::c_void,
        dec as *const core::ffi::c_void,
        opus_decoder_get_size(2) as u64,
    ) == 0
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 243);
    }
    free(dec2 as *mut core::ffi::c_void);
    println!("    OPUS_RESET_STATE ............................. OK.");
    cfgs += 1;
    packet[0 as usize] = 0;
    if opus_decoder_get_nb_samples(&mut *dec, packet.as_mut_ptr() as *const u8, 1) != 480 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 250);
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1, 48000) != 480 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 251);
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1, 96000) != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 252);
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1, 32000) != 320 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 253);
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1, 8000) != 80 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 254);
    }
    packet[0 as usize] = 3;
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1, 24000) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 256);
    }
    packet[0 as usize] = ((63) << 2 | 3) as u8;
    packet[1 as usize] = 63;
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 0, 24000) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 259);
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 2, 48000) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 260);
    }
    if opus_decoder_get_nb_samples(&mut *dec, packet.as_mut_ptr() as *const u8, 2) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 261);
    }
    println!("    opus_{{packet,decoder}}_get_nb_samples() ....... OK.");
    cfgs += 9;
    if -1 != opus_packet_get_nb_frames(packet.as_mut_ptr() as *const u8, 0) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 265);
    }
    i = 0;
    while i < 256 {
        let mut l1res: [i32; 4] = [1, 2, 2, -(4)];
        packet[0 as usize] = i as u8;
        if l1res[(packet[0 as usize] as i32 & 3) as usize]
            != opus_packet_get_nb_frames(packet.as_mut_ptr() as *const u8, 1)
        {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 269);
        }
        cfgs += 1;
        j = 0;
        while j < 256 {
            packet[1 as usize] = j as u8;
            if (if packet[0 as usize] as i32 & 3 != 3 {
                l1res[(packet[0 as usize] as i32 & 3) as usize]
            } else {
                packet[1 as usize] as i32 & 63
            }) != opus_packet_get_nb_frames(packet.as_mut_ptr() as *const u8, 2)
            {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 273);
            }
            cfgs += 1;
            j += 1;
        }
        i += 1;
    }
    println!("    opus_packet_get_nb_frames() .................. OK.");
    i = 0;
    while i < 256 {
        let mut bw: i32 = 0;
        packet[0 as usize] = i as u8;
        bw = packet[0 as usize] as i32 >> 4;
        bw = 1101 + (((((bw & 7) * 9) & (63 - (bw & 8))) + 2 + 12 * (bw & 8 != 0) as i32) >> 4);
        if bw != opus_packet_get_bandwidth(packet.as_mut_ptr()) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 284);
        }
        cfgs += 1;
        i += 1;
    }
    println!("    opus_packet_get_bandwidth() .................. OK.");
    i = 0;
    while i < 256 {
        let mut fp3s: i32 = 0;
        let mut rate: i32 = 0;
        packet[0 as usize] = i as u8;
        fp3s = packet[0 as usize] as i32 >> 3;
        fp3s = (((((3 - (fp3s & 3)) * 13) & 119) + 9) >> 2)
            * ((fp3s > 13) as i32 * (3 - (fp3s & 3 == 3) as i32) + 1)
            * 25;
        rate = 0;
        while rate < 5 {
            if opus_rates[rate as usize] * 3 / fp3s
                != opus_packet_get_samples_per_frame(packet.as_mut_ptr(), opus_rates[rate as usize])
            {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 295);
            }
            cfgs += 1;
            rate += 1;
        }
        i += 1;
    }
    println!("    opus_packet_get_samples_per_frame() .......... OK.");
    packet[0 as usize] = (((63) << 2) + 3) as u8;
    packet[1 as usize] = 49;
    j = 2;
    while j < 51 {
        packet[j as usize] = 0;
        j += 1;
    }
    if opus_decode(
        &mut *dec,
        packet.as_mut_ptr(),
        51,
        sbuf.as_mut_ptr(),
        960,
        0,
    ) != -(4)
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 305);
    }
    cfgs += 1;
    packet[0 as usize] = ((63) << 2) as u8;
    packet[2 as usize] = 0;
    packet[1 as usize] = packet[2 as usize];
    if opus_decode(
        &mut *dec,
        packet.as_mut_ptr(),
        -1,
        sbuf.as_mut_ptr(),
        960,
        0,
    ) != -1
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 309);
    }
    cfgs += 1;
    if opus_decode(&mut *dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 60, 0) != -(2) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 311);
    }
    cfgs += 1;
    if opus_decode(&mut *dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 480, 0) != -(2) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 313);
    }
    cfgs += 1;
    if opus_decode(&mut *dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 960, 0) != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 315);
    }
    cfgs += 1;
    println!("    opus_decode() ................................ OK.");
    if opus_decode_float(&mut *dec, packet.as_mut_ptr(), 3, fbuf.as_mut_ptr(), 960, 0) != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 320);
    }
    cfgs += 1;
    println!("    opus_decode_float() .......................... OK.");
    opus_decoder_destroy(dec);
    cfgs += 1;
    println!("                   All decoder interface tests passed");
    println!("                   ({:6} API invocations)", cfgs);
    cfgs
}
pub unsafe fn test_msdec_api() -> i32 {
    let mut dec_final_range: u32 = 0;
    let mut dec: *mut OpusMSDecoder = std::ptr::null_mut::<OpusMSDecoder>();
    let mut streamdec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cfgs: i32 = 0;
    let mut packet: [u8; 1276] = [0; 1276];
    let mut mapping: [u8; 256] = [0; 256];
    let mut fbuf: [f32; 1920] = [0.; 1920];
    let mut sbuf: [libc::c_short; 1920] = [0; 1920];
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut err: i32 = 0;
    mapping[0 as usize] = 0;
    mapping[1 as usize] = 1;
    i = 2;
    while i < 256 {
        i += 1;
    }
    cfgs = 0;
    println!("\n  Multistream decoder basic API tests");
    println!("  ---------------------------------------------------");
    a = -1;
    while a < 4 {
        b = -1;
        while b < 4 {
            i = opus_multistream_decoder_get_size(a, b);
            if a > 0 && b <= a && b >= 0 && (i <= 2048 || i > ((1) << 16) * a)
                || (a < 1 || b > a || b < 0) && i != 0
            {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 370);
            }
            println!(
                "    opus_multistream_decoder_get_size({:2},{:2})={:4} {}OK.",
                a,
                b,
                i,
                if i > 0 { "" } else { "... " }
            );
            cfgs += 1;
            b += 1;
        }
        a += 1;
    }
    c = 1;
    while c < 3 {
        i = -(7);
        while i <= 96000 {
            let mut fs: i32 = 0;
            if !((i == 8000 || i == 12000 || i == 16000 || i == 24000 || i == 48000)
                && (c == 1 || c == 2))
            {
                match i {
                    -5 => {
                        fs = -(8000);
                    }
                    -6 => {
                        fs = 2147483647;
                    }
                    -7 => {
                        fs = -(2147483647) - 1;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0;
                dec = opus_multistream_decoder_create(
                    fs,
                    c,
                    1,
                    c - 1,
                    mapping.as_mut_ptr(),
                    &mut err,
                );
                if err != -1 || !dec.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 393);
                }
                cfgs += 1;
                dec = opus_multistream_decoder_create(
                    fs,
                    c,
                    1,
                    c - 1,
                    mapping.as_mut_ptr(),
                    std::ptr::null_mut::<i32>(),
                );
                if !dec.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 396);
                }
                cfgs += 1;
                dec = malloc(opus_multistream_decoder_get_size(1, 1) as u64) as *mut OpusMSDecoder;
                if dec.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 399);
                }
                err = opus_multistream_decoder_init(dec, fs, c, 1, c - 1, mapping.as_mut_ptr());
                if err != -1 {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 401);
                }
                cfgs += 1;
                free(dec as *mut core::ffi::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    c = 0;
    while c < 2 {
        let mut ret_err: *mut i32 = std::ptr::null_mut::<i32>();
        ret_err = if c != 0 {
            std::ptr::null_mut::<i32>()
        } else {
            &mut err
        };
        mapping[0 as usize] = 0;
        mapping[1 as usize] = 1;
        i = 2;
        while i < 256 {
            i += 1;
        }
        dec = opus_multistream_decoder_create(48000, 2, 1, 0, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 419);
        }
        cfgs += 1;
        mapping[1 as usize] = 0;
        mapping[0 as usize] = mapping[1 as usize];
        dec = opus_multistream_decoder_create(48000, 2, 1, 0, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 || dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 426);
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 1, 4, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 || dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 434);
        }
        cfgs += 1;
        err = opus_multistream_decoder_init(dec, 48000, 1, 0, 0, mapping.as_mut_ptr());
        if err != -1 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 438);
        }
        cfgs += 1;
        err = opus_multistream_decoder_init(dec, 48000, 1, 1, -1, mapping.as_mut_ptr());
        if err != -1 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 442);
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 2, 1, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 || dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 451);
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 255, 255, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 459);
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, -1, 1, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 465);
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 0, 1, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 471);
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 1, -1, 2, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 477);
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 1, -1, -1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 483);
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 256, 255, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 489);
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(48000, 256, 255, 0, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 495);
        }
        cfgs += 1;
        mapping[0 as usize] = 255;
        mapping[1 as usize] = 1;
        mapping[2 as usize] = 2;
        dec = opus_multistream_decoder_create(48000, 3, 2, 0, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 504);
        }
        cfgs += 1;
        mapping[0 as usize] = 0;
        mapping[1 as usize] = 0;
        mapping[2 as usize] = 0;
        dec = opus_multistream_decoder_create(48000, 3, 2, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 || dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 513);
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        mapping[0 as usize] = 0;
        mapping[1 as usize] = 255;
        mapping[2 as usize] = 1;
        mapping[3 as usize] = 2;
        mapping[4 as usize] = 3;
        dec = opus_multistream_decoder_create(48001, 5, 4, 1, mapping.as_mut_ptr(), ret_err);
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -1 || !dec.is_null() {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 526);
        }
        cfgs += 1;
        c += 1;
    }
    mapping[0 as usize] = 0;
    mapping[1 as usize] = 255;
    mapping[2 as usize] = 1;
    mapping[3 as usize] = 2;
    dec = opus_multistream_decoder_create(48000, 4, 2, 1, mapping.as_mut_ptr(), &mut err);
    if err != 0 || dec.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 537);
    }
    cfgs += 1;
    println!("    opus_multistream_decoder_create() ............ OK.");
    println!("    opus_multistream_decoder_init() .............. OK.");
    err = opus_multistream_decoder_ctl!(dec, 4031, &mut dec_final_range);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 545);
    }
    println!("    OPUS_GET_FINAL_RANGE ......................... OK.");
    cfgs += 1;
    streamdec = std::ptr::null_mut::<OpusDecoder>();
    err = opus_multistream_decoder_ctl!(dec, 5122, -1, &mut streamdec);
    if err != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 553);
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(dec, 5122, 1, &mut streamdec);
    if err != 0 || streamdec.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 556);
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(dec, 5122, 2, &mut streamdec);
    if err != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 560);
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(dec, 5122, 0, &mut streamdec);
    if err != 0 || streamdec.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 563);
    }
    println!("    OPUS_MULTISTREAM_GET_DECODER_STATE ........... OK.");
    cfgs += 1;
    j = 0;
    while j < 2 {
        let mut od: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        err = opus_multistream_decoder_ctl!(dec, 5122, j, &mut od);
        if err != 0 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 572);
        }
        err = opus_decoder_ctl!(&mut *od, 4045, &mut i);
        if err != 0 || i != 0 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 576);
        }
        cfgs += 1;
        j += 1;
    }
    err = opus_multistream_decoder_ctl!(dec, 4034, 15);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 580);
    }
    println!("    OPUS_SET_GAIN ................................ OK.");
    j = 0;
    while j < 2 {
        let mut od_0: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        err = opus_multistream_decoder_ctl!(dec, 5122, j, &mut od_0);
        if err != 0 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 586);
        }
        err = opus_decoder_ctl!(&mut *od_0, 4045, &mut i);
        if err != 0 || i != 15 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 590);
        }
        cfgs += 1;
        j += 1;
    }
    println!("    OPUS_GET_GAIN ................................ OK.");
    err = opus_multistream_decoder_ctl!(dec, 4009, &mut i);
    if err != 0 || i != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 597);
    }
    println!("    OPUS_GET_BANDWIDTH ........................... OK.");
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(dec, -(5));
    if err != -(5) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 602);
    }
    println!("    OPUS_UNIMPLEMENTED ........................... OK.");
    cfgs += 1;
    if opus_multistream_decoder_ctl!(dec, 4028) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 635);
    }
    println!("    OPUS_RESET_STATE ............................. OK.");
    cfgs += 1;
    opus_multistream_decoder_destroy(dec);
    cfgs += 1;
    dec = opus_multistream_decoder_create(48000, 2, 1, 1, mapping.as_mut_ptr(), &mut err);
    if err != 0 || dec.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 643);
    }
    cfgs += 1;
    packet[0 as usize] = (((63) << 2) + 3) as u8;
    packet[1 as usize] = 49;
    j = 2;
    while j < 51 {
        packet[j as usize] = 0;
        j += 1;
    }
    if opus_multistream_decode(dec, packet.as_mut_ptr(), 51, sbuf.as_mut_ptr(), 960, 0) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 650);
    }
    cfgs += 1;
    packet[0 as usize] = ((63) << 2) as u8;
    packet[2 as usize] = 0;
    packet[1 as usize] = packet[2 as usize];
    if opus_multistream_decode(dec, packet.as_mut_ptr(), -1, sbuf.as_mut_ptr(), 960, 0) != -1 {
        println!(
            "{}",
            opus_multistream_decode(dec, packet.as_mut_ptr(), -1, sbuf.as_mut_ptr(), 960, 0,)
        );
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 654);
    }
    cfgs += 1;
    if opus_multistream_decode(dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), -(960), 0) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 656);
    }
    cfgs += 1;
    if opus_multistream_decode(dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 60, 0) != -(2) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 658);
    }
    cfgs += 1;
    if opus_multistream_decode(dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 480, 0) != -(2) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 660);
    }
    cfgs += 1;
    if opus_multistream_decode(dec, packet.as_mut_ptr(), 3, sbuf.as_mut_ptr(), 960, 0) != 960 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 662);
    }
    cfgs += 1;
    println!("    opus_multistream_decode() .................... OK.");
    if opus_multistream_decode_float(dec, packet.as_mut_ptr(), 3, fbuf.as_mut_ptr(), 960, 0) != 960
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 667);
    }
    cfgs += 1;
    println!("    opus_multistream_decode_float() .............. OK.");
    opus_multistream_decoder_destroy(dec);
    cfgs += 1;
    println!("       All multistream decoder interface tests passed");
    println!("                             ({:6} API invocations)", cfgs);
    cfgs
}
pub unsafe fn test_parse() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut sz: i32 = 0;
    let mut packet: [u8; 1276] = [0; 1276];
    let mut cfgs: i32 = 0;
    let mut cfgs_total: i32 = 0;
    let mut toc: u8 = 0;
    let mut frames: [*const u8; 48] = [std::ptr::null::<u8>(); 48];
    let mut size: [libc::c_short; 48] = [0; 48];
    let mut payload_offset: i32 = 0;
    let mut ret: i32 = 0;
    println!("\n  Packet header parsing tests");
    println!("  ---------------------------------------------------");
    memset(
        packet.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        (::core::mem::size_of::<i8>() as u64).wrapping_mul(1276),
    );
    packet[0 as usize] = ((63) << 2) as u8;
    if opus_packet_parse(
        packet.as_mut_ptr(),
        1,
        &mut toc,
        frames.as_mut_ptr(),
        std::ptr::null_mut::<i16>(),
        &mut payload_offset,
    ) != -1
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 720);
    }
    cfgs = 1;
    cfgs_total = cfgs;
    i = 0;
    while i < 64 {
        packet[0 as usize] = (i << 2) as u8;
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            4,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != 1 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 729);
        }
        if size[0 as usize] as i32 != 3 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 730);
        }
        if frames[0 as usize] != packet.as_mut_ptr().offset(1 as isize) as *const u8 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 731);
        }
        i += 1;
    }
    println!(
        "    code 0 ({:2} cases) ............................ OK.",
        cfgs
    );
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        packet[0 as usize] = ((i << 2) + 1) as u8;
        jj = 0;
        while jj <= 1275 * 2 + 3 {
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                jj,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if jj & 1 == 1 && jj <= 2551 {
                if ret != 2 {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 749);
                }
                if size[0 as usize] as i32 != size[1 as usize] as i32
                    || size[0 as usize] as i32 != (jj - 1) >> 1
                {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 750);
                }
                if frames[0 as usize] != packet.as_mut_ptr().offset(1 as isize) as *const u8 {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 751);
                }
                if frames[1 as usize]
                    != (frames[0 as usize]).offset(size[0 as usize] as i32 as isize)
                {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 752);
                }
                if toc as i32 >> 2 != i {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 753);
                }
            } else if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 754);
            }
            jj += 1;
        }
        i += 1;
    }
    println!("    code 1 ({:6} cases) ........................ OK.", cfgs);
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        packet[0 as usize] = ((i << 2) + 2) as u8;
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 767);
        }
        packet[1 as usize] = 252;
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            2,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 772);
        }
        j = 0;
        while j < 1275 {
            if j < 252 {
                packet[1 as usize] = j as u8;
            } else {
                packet[1 as usize] = (252 + (j & 3)) as u8;
                packet[2 as usize] = ((j - 252) >> 2) as u8;
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 { 2 } else { 3 }) - 1,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 781);
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 { 2 } else { 3 }) + 1276,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 786);
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 { 2 } else { 3 }),
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 2 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 791);
            }
            if size[0 as usize] as i32 != j || size[1 as usize] as i32 != 0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 792);
            }
            if frames[1 as usize] != (frames[0 as usize]).offset(size[0 as usize] as i32 as isize) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 793);
            }
            if toc as i32 >> 2 != i {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 794);
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                (j << 1) + 4,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 2 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 799);
            }
            if size[0 as usize] as i32 != j
                || size[1 as usize] as i32 != (j << 1) + 3 - j - (if j < 252 { 1 } else { 2 })
            {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 800);
            }
            if frames[1 as usize] != (frames[0 as usize]).offset(size[0 as usize] as i32 as isize) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 801);
            }
            if toc as i32 >> 2 != i {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 802);
            }
            j += 1;
        }
        i += 1;
    }
    println!(
        "    code 2 ({:6} cases) ........................ OK.",
        cfgs_total
    );
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        packet[0 as usize] = ((i << 2) + 3) as u8;
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 815);
        }
        i += 1;
    }
    println!(
        "    code 3 m-truncation ({:2} cases) ............... OK.",
        cfgs
    );
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        packet[0 as usize] = ((i << 2) + 3) as u8;
        jj = 49;
        while jj <= 64 {
            packet[1 as usize] = (0 + (jj & 63)) as u8;
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 830);
            }
            packet[1 as usize] = (128 + (jj & 63)) as u8;
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 835);
            }
            packet[1 as usize] = (64 + (jj & 63)) as u8;
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 840);
            }
            packet[1 as usize] = (128 + 64 + (jj & 63)) as u8;
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 845);
            }
            jj += 1;
        }
        i += 1;
    }
    println!(
        "    code 3 m=0,49-64 ({:2} cases) ................ OK.",
        cfgs
    );
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        packet[0 as usize] = ((i << 2) + 3) as u8;
        packet[1 as usize] = 1;
        j = 0;
        while j < 1276 {
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + 2,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 1 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 861);
            }
            if size[0 as usize] as i32 != j {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 862);
            }
            if toc as i32 >> 2 != i {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 863);
            }
            j += 1;
        }
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1276 + 2,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 868);
        }
        i += 1;
    }
    println!(
        "    code 3 m=1 CBR ({:2} cases) ................. OK.",
        cfgs
    );
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        let mut frame_samp: i32 = 0;
        packet[0 as usize] = ((i << 2) + 3) as u8;
        frame_samp = opus_packet_get_samples_per_frame(packet.as_mut_ptr(), 48000);
        j = 2;
        while j < 49 {
            packet[1 as usize] = j as u8;
            sz = 2;
            while sz < (j + 2) * 1275 {
                toc = u8::MAX;
                frames[0 as usize] = std::ptr::null_mut::<u8>();
                frames[1 as usize] = std::ptr::null_mut::<u8>();
                payload_offset = -1;
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    sz,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if frame_samp * j <= 5760 && (sz - 2) % j == 0 && (sz - 2) / j < 1276 {
                    if ret != j {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 890);
                    }
                    jj = 1;
                    while jj < ret {
                        if frames[jj as usize]
                            != (frames[(jj - 1) as usize])
                                .offset(size[(jj - 1) as usize] as i32 as isize)
                        {
                            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 891);
                        }
                        jj += 1;
                    }
                    if toc as i32 >> 2 != i {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 892);
                    }
                } else if ret != -(4) {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 893);
                }
                sz += 1;
            }
            j += 1;
        }
        packet[1 as usize] = (5760 / frame_samp) as u8;
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1275 * packet[1 as usize] as i32 + 2,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != packet[1 as usize] as i32 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 901);
        }
        jj = 0;
        while jj < ret {
            if size[jj as usize] as i32 != 1275 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 902);
            }
            jj += 1;
        }
        i += 1;
    }
    println!("    code 3 m=1-48 CBR ({:2} cases) .......... OK.", cfgs);
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        let mut frame_samp_0: i32 = 0;
        packet[0 as usize] = ((i << 2) + 3) as u8;
        packet[1 as usize] = (128 + 1) as u8;
        frame_samp_0 = opus_packet_get_samples_per_frame(packet.as_mut_ptr(), 48000);
        jj = 0;
        while jj < 1276 {
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 + jj,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 1 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 919);
            }
            if size[0 as usize] as i32 != jj {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 920);
            }
            if toc as i32 >> 2 != i {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 921);
            }
            jj += 1;
        }
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            2 + 1276,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 926);
        }
        j = 2;
        while j < 49 {
            packet[1 as usize] = (128 + j) as u8;
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 + j - 2,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 934);
            }
            packet[2 as usize] = 252;
            packet[3 as usize] = 0;
            jj = 4;
            while jj < 2 + j {
                packet[jj as usize] = 0;
                jj += 1;
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 + j,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 941);
            }
            jj = 2;
            while jj < 2 + j {
                packet[jj as usize] = 0;
                jj += 1;
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 + j - 2,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 947);
            }
            packet[2 as usize] = 252;
            packet[3 as usize] = 0;
            jj = 4;
            while jj < 2 + j {
                packet[jj as usize] = 0;
                jj += 1;
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 + j + 252 - 1,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 955);
            }
            jj = 2;
            while jj < 2 + j {
                packet[jj as usize] = 0;
                jj += 1;
            }
            toc = u8::MAX;
            frames[0 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -1;
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 + j - 1,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if frame_samp_0 * j <= 5760 {
                if ret != j {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 962);
                }
                jj = 0;
                while jj < j {
                    if size[jj as usize] as i32 != 0 {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 963);
                    }
                    jj += 1;
                }
                if toc as i32 >> 2 != i {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 964);
                }
            } else if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 965);
            }
            sz = 0;
            while sz < 8 {
                let tsz: [i32; 8] = [50, 201, 403, 700, 1472, 5110, 20400, 61298];
                let mut pos: i32 = 0;
                let mut as_0: i32 = (tsz[sz as usize] + i - j - 2) / j;
                jj = 0;
                while jj < j - 1 {
                    if as_0 < 252 {
                        packet[(2 + pos) as usize] = as_0 as u8;
                        pos += 1;
                    } else {
                        packet[(2 + pos) as usize] = (252 + (as_0 & 3)) as u8;
                        packet[(3 + pos) as usize] = ((as_0 - 252) >> 2) as u8;
                        pos += 2;
                    }
                    jj += 1;
                }
                toc = u8::MAX;
                frames[0 as usize] = std::ptr::null_mut::<u8>();
                frames[1 as usize] = std::ptr::null_mut::<u8>();
                payload_offset = -1;
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    tsz[sz as usize] + i,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if frame_samp_0 * j <= 5760
                    && as_0 < 1276
                    && tsz[sz as usize] + i - 2 - pos - as_0 * (j - 1) < 1276
                {
                    if ret != j {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 981);
                    }
                    jj = 0;
                    while jj < j - 1 {
                        if size[jj as usize] as i32 != as_0 {
                            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 982);
                        }
                        jj += 1;
                    }
                    if size[(j - 1) as usize] as i32
                        != tsz[sz as usize] + i - 2 - pos - as_0 * (j - 1)
                    {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 983);
                    }
                    if toc as i32 >> 2 != i {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 984);
                    }
                } else if ret != -(4) {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 985);
                }
                sz += 1;
            }
            j += 1;
        }
        i += 1;
    }
    println!("    code 3 m=1-48 VBR ({:2} cases) ............. OK.", cfgs);
    cfgs_total += cfgs;
    cfgs = 0;
    i = 0;
    while i < 64 {
        packet[0 as usize] = ((i << 2) + 3) as u8;
        packet[1 as usize] = (128 + 1 + 64) as u8;
        jj = 2;
        while jj < 127 {
            packet[jj as usize] = 255;
            jj += 1;
        }
        toc = u8::MAX;
        frames[0 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -1;
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            127,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4) {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1002);
        }
        sz = 0;
        while sz < 4 {
            let tsz_0: [i32; 4] = [0, 72, 512, 1275];
            jj = sz;
            while jj < 65025 {
                let mut pos_0: i32 = 0;
                pos_0 = 0;
                while pos_0 < jj / 254 {
                    packet[(2 + pos_0) as usize] = 255;
                    pos_0 += 1;
                }
                packet[(2 + pos_0) as usize] = (jj % 254) as u8;
                pos_0 += 1;
                if sz == 0 && i == 63 {
                    toc = u8::MAX;
                    frames[0 as usize] = std::ptr::null_mut::<u8>();
                    frames[1 as usize] = std::ptr::null_mut::<u8>();
                    payload_offset = -1;
                    ret = opus_packet_parse(
                        packet.as_mut_ptr(),
                        2 + jj + pos_0 - 1,
                        &mut toc,
                        frames.as_mut_ptr(),
                        size.as_mut_ptr(),
                        &mut payload_offset,
                    );
                    cfgs += 1;
                    if ret != -(4) {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1019);
                    }
                }
                toc = u8::MAX;
                frames[0 as usize] = std::ptr::null_mut::<u8>();
                frames[1 as usize] = std::ptr::null_mut::<u8>();
                payload_offset = -1;
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    2 + jj + tsz_0[sz as usize] + i + pos_0,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if tsz_0[sz as usize] + i < 1276 {
                    if ret != 1 {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1026);
                    }
                    if size[0 as usize] as i32 != tsz_0[sz as usize] + i {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1027);
                    }
                    if toc as i32 >> 2 != i {
                        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1028);
                    }
                } else if ret != -(4) {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1029);
                }
                jj += 11;
            }
            sz += 1;
        }
        i += 1;
    }
    println!("    code 3 padding ({:2} cases) ............... OK.", cfgs);
    cfgs_total += cfgs;
    println!("    opus_packet_parse ............................ OK.");
    println!("                      All packet parsing tests passed");
    println!("                   ({} API invocations)", cfgs_total);
    cfgs_total
}
pub unsafe fn test_enc_api() -> i32 {
    let mut enc_final_range: u32 = 0;
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut packet: [u8; 1276] = [0; 1276];
    let mut fbuf: [f32; 1920] = [0.; 1920];
    let mut sbuf: [libc::c_short; 1920] = [0; 1920];
    let mut c: i32 = 0;
    let mut err: i32 = 0;
    let mut cfgs: i32 = 0;
    cfgs = 0;
    println!("\n  Encoder basic API tests");
    println!("  ---------------------------------------------------");
    c = 0;
    while c < 4 {
        i = opus_encoder_get_size(c);
        if (c == 1 || c == 2) && (i <= 2048 || i > (1) << 17) || c != 1 && c != 2 && i != 0 {
            _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1084);
        }
        println!(
            "    opus_encoder_get_size({})={} ...............{} OK.",
            c,
            i,
            if i > 0 { "" } else { "...." }
        );
        cfgs += 1;
        c += 1;
    }
    c = 0;
    while c < 4 {
        i = -(7);
        while i <= 96000 {
            let mut fs: i32 = 0;
            if !((i == 8000 || i == 12000 || i == 16000 || i == 24000 || i == 48000)
                && (c == 1 || c == 2))
            {
                match i {
                    -5 => {
                        fs = -(8000);
                    }
                    -6 => {
                        fs = 2147483647;
                    }
                    -7 => {
                        fs = -(2147483647) - 1;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0;
                enc = opus_encoder_create(fs, c, 2048, &mut err);
                if err != -1 || !enc.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1106);
                }
                cfgs += 1;
                enc = opus_encoder_create(fs, c, 2048, std::ptr::null_mut::<i32>());
                if !enc.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1109);
                }
                cfgs += 1;
                opus_encoder_destroy(enc);
                enc = malloc(opus_encoder_get_size(2) as u64) as *mut OpusEncoder;
                if enc.is_null() {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1113);
                }
                err = opus_encoder_init(enc, fs, c, 2048);
                if err != -1 {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1115);
                }
                cfgs += 1;
                free(enc as *mut core::ffi::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    enc = opus_encoder_create(48000, 2, -(1000), std::ptr::null_mut::<i32>());
    if !enc.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1122);
    }
    cfgs += 1;
    enc = opus_encoder_create(48000, 2, -(1000), &mut err);
    if err != -1 || !enc.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1127);
    }
    cfgs += 1;
    enc = opus_encoder_create(48000, 2, 2048, std::ptr::null_mut::<i32>());
    if enc.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1132);
    }
    opus_encoder_destroy(enc);
    cfgs += 1;
    enc = opus_encoder_create(48000, 2, 2051, &mut err);
    if err != 0 || enc.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1138);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4027, &mut i);
    if err != 0 || i < 0 || i > 32766 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1141);
    }
    cfgs += 1;
    opus_encoder_destroy(enc);
    enc = opus_encoder_create(48000, 2, 2049, &mut err);
    if err != 0 || enc.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1147);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4027, &mut i);
    if err != 0 || i < 0 || i > 32766 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1150);
    }
    opus_encoder_destroy(enc);
    cfgs += 1;
    enc = opus_encoder_create(48000, 2, 2048, &mut err);
    if err != 0 || enc.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1156);
    }
    cfgs += 1;
    println!("    opus_encoder_create() ........................ OK.");
    println!("    opus_encoder_init() .......................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4027, &mut i);
    if err != 0 || i < 0 || i > 32766 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1165);
    }
    cfgs += 1;
    println!("    OPUS_GET_LOOKAHEAD ........................... OK.");
    err = opus_encoder_ctl!(enc, 4029, &mut i);
    if err != 0 || i != 48000 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1173);
    }
    cfgs += 1;
    println!("    OPUS_GET_SAMPLE_RATE ......................... OK.");
    if opus_encoder_ctl!(enc, -(5)) != -(5) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1180);
    }
    println!("    OPUS_UNIMPLEMENTED ........................... OK.");
    cfgs += 1;
    i = -1;
    if opus_encoder_ctl!(enc, 4000, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1190);
    }
    i = -(1000);
    if opus_encoder_ctl!(enc, 4000, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1190);
    }
    i = 2049;
    j = i;
    if opus_encoder_ctl!(enc, 4000, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1190);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4001, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1190);
    }
    i = 2051;
    j = i;
    if opus_encoder_ctl!(enc, 4000, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1190);
    }
    println!("    OPUS_SET_APPLICATION ......................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4001, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1190);
    }
    println!("    OPUS_GET_APPLICATION ......................... OK.");
    cfgs += 6;
    if opus_encoder_ctl!(enc, 4002, 1073741832) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1195);
    }
    cfgs += 1;
    if opus_encoder_ctl!(enc, 4003, &mut i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1198);
    }
    if i > 700000 || i < 256000 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1199);
    }
    cfgs += 1;
    i = -(12345);
    if opus_encoder_ctl!(enc, 4002, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1204);
    }
    i = 0;
    if opus_encoder_ctl!(enc, 4002, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1204);
    }
    i = 500;
    j = i;
    if opus_encoder_ctl!(enc, 4002, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1204);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4003, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1204);
    }
    i = 256000;
    j = i;
    if opus_encoder_ctl!(enc, 4002, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1204);
    }
    println!("    OPUS_SET_BITRATE ............................. OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4003, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1204);
    }
    println!("    OPUS_GET_BITRATE ............................. OK.");
    cfgs += 6;
    i = -1;
    if opus_encoder_ctl!(enc, 4022, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1212);
    }
    i = 3;
    if opus_encoder_ctl!(enc, 4022, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1212);
    }
    i = 1;
    j = i;
    if opus_encoder_ctl!(enc, 4022, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1212);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4023, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1212);
    }
    i = -(1000);
    j = i;
    if opus_encoder_ctl!(enc, 4022, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1212);
    }
    println!("    OPUS_SET_FORCE_CHANNELS ...................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4023, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1212);
    }
    println!("    OPUS_GET_FORCE_CHANNELS ...................... OK.");
    cfgs += 6;
    i = -(2);
    if opus_encoder_ctl!(enc, 4008, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1215);
    }
    cfgs += 1;
    i = 1105 + 1;
    if opus_encoder_ctl!(enc, 4008, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1218);
    }
    cfgs += 1;
    i = 1101;
    if opus_encoder_ctl!(enc, 4008, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1221);
    }
    cfgs += 1;
    i = 1105;
    if opus_encoder_ctl!(enc, 4008, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1224);
    }
    cfgs += 1;
    i = 1103;
    if opus_encoder_ctl!(enc, 4008, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1227);
    }
    cfgs += 1;
    i = 1102;
    if opus_encoder_ctl!(enc, 4008, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1230);
    }
    cfgs += 1;
    println!("    OPUS_SET_BANDWIDTH ........................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4009, &mut i);
    if err != 0 || i != 1101 && i != 1102 && i != 1103 && i != 1105 && i != -(1000) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1240);
    }
    cfgs += 1;
    if opus_encoder_ctl!(enc, 4008, -(1000)) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1242);
    }
    cfgs += 1;
    println!("    OPUS_GET_BANDWIDTH ........................... OK.");
    i = -(2);
    if opus_encoder_ctl!(enc, 4004, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1250);
    }
    cfgs += 1;
    i = 1105 + 1;
    if opus_encoder_ctl!(enc, 4004, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1253);
    }
    cfgs += 1;
    i = 1101;
    if opus_encoder_ctl!(enc, 4004, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1256);
    }
    cfgs += 1;
    i = 1105;
    if opus_encoder_ctl!(enc, 4004, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1259);
    }
    cfgs += 1;
    i = 1103;
    if opus_encoder_ctl!(enc, 4004, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1262);
    }
    cfgs += 1;
    i = 1102;
    if opus_encoder_ctl!(enc, 4004, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1265);
    }
    cfgs += 1;
    println!("    OPUS_SET_MAX_BANDWIDTH ....................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4005, &mut i);
    if err != 0 || i != 1101 && i != 1102 && i != 1103 && i != 1105 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1275);
    }
    cfgs += 1;
    println!("    OPUS_GET_MAX_BANDWIDTH ....................... OK.");
    i = -1;
    if opus_encoder_ctl!(enc, 4016, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1288);
    }
    i = 2;
    if opus_encoder_ctl!(enc, 4016, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1288);
    }
    i = 1;
    j = i;
    if opus_encoder_ctl!(enc, 4016, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1288);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4017, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1288);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4016, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1288);
    }
    println!("    OPUS_SET_DTX ................................. OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4017, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1288);
    }
    println!("    OPUS_GET_DTX ................................. OK.");
    cfgs += 6;
    i = -1;
    if opus_encoder_ctl!(enc, 4010, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1296);
    }
    i = 11;
    if opus_encoder_ctl!(enc, 4010, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1296);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4010, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1296);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4011, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1296);
    }
    i = 10;
    j = i;
    if opus_encoder_ctl!(enc, 4010, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1296);
    }
    println!("    OPUS_SET_COMPLEXITY .......................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4011, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1296);
    }
    println!("    OPUS_GET_COMPLEXITY .......................... OK.");
    cfgs += 6;
    i = -1;
    if opus_encoder_ctl!(enc, 4012, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1304);
    }
    i = 2;
    if opus_encoder_ctl!(enc, 4012, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1304);
    }
    i = 1;
    j = i;
    if opus_encoder_ctl!(enc, 4012, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1304);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4013, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1304);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4012, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1304);
    }
    println!("    OPUS_SET_INBAND_FEC .......................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4013, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1304);
    }
    println!("    OPUS_GET_INBAND_FEC .......................... OK.");
    cfgs += 6;
    i = -1;
    if opus_encoder_ctl!(enc, 4014, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1312);
    }
    i = 101;
    if opus_encoder_ctl!(enc, 4014, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1312);
    }
    i = 100;
    j = i;
    if opus_encoder_ctl!(enc, 4014, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1312);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4015, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1312);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4014, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1312);
    }
    println!("    OPUS_SET_PACKET_LOSS_PERC .................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4015, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1312);
    }
    println!("    OPUS_GET_PACKET_LOSS_PERC .................... OK.");
    cfgs += 6;
    i = -1;
    if opus_encoder_ctl!(enc, 4006, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1320);
    }
    i = 2;
    if opus_encoder_ctl!(enc, 4006, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1320);
    }
    i = 1;
    j = i;
    if opus_encoder_ctl!(enc, 4006, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1320);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4007, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1320);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4006, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1320);
    }
    println!("    OPUS_SET_VBR ................................. OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4007, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1320);
    }
    println!("    OPUS_GET_VBR ................................. OK.");
    cfgs += 6;
    i = -1;
    if opus_encoder_ctl!(enc, 4020, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1336);
    }
    i = 2;
    if opus_encoder_ctl!(enc, 4020, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1336);
    }
    i = 1;
    j = i;
    if opus_encoder_ctl!(enc, 4020, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1336);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4021, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1336);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4020, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1336);
    }
    println!("    OPUS_SET_VBR_CONSTRAINT ...................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4021, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1336);
    }
    println!("    OPUS_GET_VBR_CONSTRAINT ...................... OK.");
    cfgs += 6;
    i = -(12345);
    if opus_encoder_ctl!(enc, 4024, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1344);
    }
    i = 0x7fffffff;
    if opus_encoder_ctl!(enc, 4024, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1344);
    }
    i = 3002;
    j = i;
    if opus_encoder_ctl!(enc, 4024, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1344);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4025, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1344);
    }
    i = -(1000);
    j = i;
    if opus_encoder_ctl!(enc, 4024, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1344);
    }
    println!("    OPUS_SET_SIGNAL .............................. OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4025, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1344);
    }
    println!("    OPUS_GET_SIGNAL .............................. OK.");
    cfgs += 6;
    i = 7;
    if opus_encoder_ctl!(enc, 4036, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1351);
    }
    i = 25;
    if opus_encoder_ctl!(enc, 4036, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1351);
    }
    i = 16;
    j = i;
    if opus_encoder_ctl!(enc, 4036, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1351);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4037, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1351);
    }
    i = 24;
    j = i;
    if opus_encoder_ctl!(enc, 4036, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1351);
    }
    println!("    OPUS_SET_LSB_DEPTH ........................... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4037, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1351);
    }
    println!("    OPUS_GET_LSB_DEPTH ........................... OK.");
    cfgs += 6;
    err = opus_encoder_ctl!(enc, 4043, &mut i);
    if i != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1354);
    }
    cfgs += 1;
    i = -1;
    if opus_encoder_ctl!(enc, 4042, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1361);
    }
    i = 2;
    if opus_encoder_ctl!(enc, 4042, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1361);
    }
    i = 1;
    j = i;
    if opus_encoder_ctl!(enc, 4042, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1361);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4043, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1361);
    }
    i = 0;
    j = i;
    if opus_encoder_ctl!(enc, 4042, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1361);
    }
    println!("    OPUS_SET_PREDICTION_DISABLED ................. OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4043, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1361);
    }
    println!("    OPUS_GET_PREDICTION_DISABLED ................. OK.");
    cfgs += 6;
    err = opus_encoder_ctl!(enc, 4040, 5001);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1367);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5002);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1370);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5003);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1373);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5004);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1376);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5005);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1379);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5006);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1382);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5007);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1385);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5008);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1388);
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040, 5009);
    if err != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1391);
    }
    cfgs += 1;
    i = 0;
    if opus_encoder_ctl!(enc, 4040, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1396);
    }
    i = -1;
    if opus_encoder_ctl!(enc, 4040, i) == 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1396);
    }
    i = 5006;
    j = i;
    if opus_encoder_ctl!(enc, 4040, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1396);
    }
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4041, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1396);
    }
    i = 5000;
    j = i;
    if opus_encoder_ctl!(enc, 4040, i) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1396);
    }
    println!("    OPUS_SET_EXPERT_FRAME_DURATION ............... OK.");
    i = -(12345);
    err = opus_encoder_ctl!(enc, 4041, &mut i);
    if err != 0 || i != j {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1396);
    }
    println!("    OPUS_GET_EXPERT_FRAME_DURATION ............... OK.");
    cfgs += 6;
    if opus_encoder_ctl!(enc, 4031, &mut enc_final_range) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1403);
    }
    cfgs += 1;
    println!("    OPUS_GET_FINAL_RANGE ......................... OK.");
    if opus_encoder_ctl!(enc, 4028) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1408);
    }
    cfgs += 1;
    println!("    OPUS_RESET_STATE ............................. OK.");
    memset(
        sbuf.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        (::core::mem::size_of::<libc::c_short>() as u64)
            .wrapping_mul(2)
            .wrapping_mul(960),
    );
    i = opus_encode(
        enc,
        sbuf.as_mut_ptr(),
        960,
        packet.as_mut_ptr(),
        ::core::mem::size_of::<[u8; 1276]>() as u64 as i32,
    );
    if i < 1 || i > ::core::mem::size_of::<[u8; 1276]>() as u64 as i32 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1415);
    }
    cfgs += 1;
    println!("    opus_encode() ................................ OK.");
    memset(
        fbuf.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        (::core::mem::size_of::<f32>() as u64)
            .wrapping_mul(2)
            .wrapping_mul(960),
    );
    i = opus_encode_float(
        enc,
        fbuf.as_mut_ptr(),
        960,
        packet.as_mut_ptr(),
        ::core::mem::size_of::<[u8; 1276]>() as u64 as i32,
    );
    if i < 1 || i > ::core::mem::size_of::<[u8; 1276]>() as u64 as i32 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1423);
    }
    cfgs += 1;
    println!("    opus_encode_float() .......................... OK.");
    opus_encoder_destroy(enc);
    cfgs += 1;
    println!("                   All encoder interface tests passed");

    println!("                   ({} API invocations)", cfgs);
    cfgs
}
pub unsafe fn test_repacketizer_api() -> i32 {
    let mut ret: i32 = 0;
    let mut cfgs: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut rp: *mut OpusRepacketizer = std::ptr::null_mut::<OpusRepacketizer>();
    let mut packet: *mut u8 = std::ptr::null_mut::<u8>();
    let mut po: *mut u8 = std::ptr::null_mut::<u8>();
    cfgs = 0;
    println!("\n  Repacketizer tests");
    println!("  ---------------------------------------------------");
    packet = malloc((1276 * 48 + 48 * 2 + 2) as u64) as *mut u8;
    if packet.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1455);
    }
    memset(
        packet as *mut core::ffi::c_void,
        0,
        (1276 * 48 + 48 * 2 + 2) as u64,
    );
    po = malloc((1276 * 48 + 48 * 2 + 2 + 256) as u64) as *mut u8;
    if po.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1458);
    }
    i = opus_repacketizer_get_size();
    if i <= 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1461);
    }
    cfgs += 1;
    println!("    opus_repacketizer_get_size()={} ............. OK.", i);
    rp = malloc(i as u64) as *mut OpusRepacketizer;
    rp = opus_repacketizer_init(rp);
    if rp.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1467);
    }
    cfgs += 1;
    free(rp as *mut core::ffi::c_void);
    println!("    opus_repacketizer_init ....................... OK.");
    rp = opus_repacketizer_create();
    if rp.is_null() {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1473);
    }
    cfgs += 1;
    println!("    opus_repacketizer_create ..................... OK.");
    if opus_repacketizer_get_nb_frames(rp) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1477);
    }
    cfgs += 1;
    println!("    opus_repacketizer_get_nb_frames .............. OK.");
    if opus_repacketizer_cat(rp, packet, 0) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1483);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 1;
    if opus_repacketizer_cat(rp, packet, 2) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1486);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 2;
    if opus_repacketizer_cat(rp, packet, 1) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1489);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 3;
    if opus_repacketizer_cat(rp, packet, 1) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1492);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 2;
    *packet.offset(1 as isize) = 255;
    if opus_repacketizer_cat(rp, packet, 2) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1496);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 2;
    *packet.offset(1 as isize) = 250;
    if opus_repacketizer_cat(rp, packet, 251) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1500);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 3;
    *packet.offset(1 as isize) = 0;
    if opus_repacketizer_cat(rp, packet, 2) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1504);
    }
    cfgs += 1;
    *packet.offset(1 as isize) = 49;
    if opus_repacketizer_cat(rp, packet, 100) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1507);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 0;
    if opus_repacketizer_cat(rp, packet, 3) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1510);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = ((1) << 2) as u8;
    if opus_repacketizer_cat(rp, packet, 3) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1513);
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    j = 0;
    while j < 32 {
        let mut maxi: i32 = 0;
        *packet.offset(0 as isize) = (((j << 1) + (j & 1)) << 2) as u8;
        maxi = 960 / opus_packet_get_samples_per_frame(packet, 8000);
        i = 1;
        while i <= maxi {
            let mut maxp: i32 = 0;
            *packet.offset(0 as isize) = (((j << 1) + (j & 1)) << 2) as u8;
            if i > 1 {
                let fresh0 = &mut (*packet.offset(0 as isize));
                *fresh0 = (*fresh0 as i32 + if i == 2 { 1 } else { 3 }) as u8;
            }
            *packet.offset(1 as isize) = (if i > 2 { i } else { 0 }) as u8;
            maxp = 960 / (i * opus_packet_get_samples_per_frame(packet, 8000));
            k = 0;
            while k <= 1275 + 75 {
                let mut cnt: i32 = 0;
                let mut rcnt: i32 = 0;
                if k % i == 0 {
                    cnt = 0;
                    while cnt < maxp + 2 {
                        if cnt > 0 {
                            ret =
                                opus_repacketizer_cat(rp, packet, k + (if i > 2 { 2 } else { 1 }));
                            if if cnt <= maxp && k <= 1275 * i {
                                (ret != 0) as i32
                            } else {
                                (ret != -(4)) as i32
                            } != 0
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1542,
                                );
                            }
                            cfgs += 1;
                        }
                        rcnt = if k <= 1275 * i {
                            if cnt < maxp {
                                cnt
                            } else {
                                maxp
                            }
                        } else {
                            0
                        };
                        if opus_repacketizer_get_nb_frames(rp) != rcnt * i {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                1546,
                            );
                        }
                        cfgs += 1;
                        ret = opus_repacketizer_out_range(
                            rp,
                            0,
                            rcnt * i,
                            po,
                            1276 * 48 + 48 * 2 + 2,
                        );
                        if rcnt > 0 {
                            let mut len: i32 = 0;
                            len = k * rcnt + (if rcnt * i > 2 { 2 } else { 1 });
                            if ret != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1553,
                                );
                            }
                            if rcnt * i < 2 && *po.offset(0 as isize) as i32 & 3 != 0 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1554,
                                );
                            }
                            if rcnt * i == 2 && *po.offset(0 as isize) as i32 & 3 != 1 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1555,
                                );
                            }
                            if rcnt * i > 2
                                && (*po.offset(0 as isize) as i32 & 3 != 3
                                    || *po.offset(1 as isize) as i32 != rcnt * i)
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1556,
                                );
                            }
                            cfgs += 1;
                            if opus_repacketizer_out(rp, po, len) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1558,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_unpad(po, len) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1560,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_pad(po, len, len + 1) != 0 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1562,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_pad(po, len + 1, len + 256) != 0 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1564,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_unpad(po, len + 256) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1566,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_unpad(po, len, 1) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1568,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_pad(po, len, len + 1, 1) != 0 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1570,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_pad(po, len + 1, len + 256, 1) != 0 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1572,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_unpad(po, len + 256, 1) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1574,
                                );
                            }
                            cfgs += 1;
                            if opus_repacketizer_out(rp, po, len - 1) != -(2) {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1576,
                                );
                            }
                            cfgs += 1;
                            if len > 1 {
                                if opus_repacketizer_out(rp, po, 1) != -(2) {
                                    _test_failed(
                                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                        1580,
                                    );
                                }
                                cfgs += 1;
                            }
                            if opus_repacketizer_out(rp, po, 0) != -(2) {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1583,
                                );
                            }
                            cfgs += 1;
                        } else if ret != -1 {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                1585,
                            );
                        }
                        cnt += 1;
                    }
                    opus_repacketizer_init(rp);
                }
                k += 3;
            }
            i += 1;
        }
        j += 1;
    }
    opus_repacketizer_init(rp);
    *packet.offset(0 as isize) = 0;
    if opus_repacketizer_cat(rp, packet, 5) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1595);
    }
    cfgs += 1;
    let fresh1 = &mut (*packet.offset(0 as isize));
    *fresh1 = (*fresh1 as i32 + 1) as u8;
    if opus_repacketizer_cat(rp, packet, 9) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1598);
    }
    cfgs += 1;
    i = opus_repacketizer_out(rp, po, 1276 * 48 + 48 * 2 + 2);
    if i != 4 + 8 + 2
        || *po.offset(0 as isize) as i32 & 3 != 3
        || *po.offset(1 as isize) as i32 & 63 != 3
        || *po.offset(1 as isize) as i32 >> 7 != 0
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1601);
    }
    cfgs += 1;
    i = opus_repacketizer_out_range(rp, 0, 1, po, 1276 * 48 + 48 * 2 + 2);
    if i != 5 || *po.offset(0 as isize) as i32 & 3 != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1604);
    }
    cfgs += 1;
    i = opus_repacketizer_out_range(rp, 1, 2, po, 1276 * 48 + 48 * 2 + 2);
    if i != 5 || *po.offset(0 as isize) as i32 & 3 != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1607);
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as isize) = 1;
    if opus_repacketizer_cat(rp, packet, 9) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1613);
    }
    cfgs += 1;
    *packet.offset(0 as isize) = 0;
    if opus_repacketizer_cat(rp, packet, 3) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1616);
    }
    cfgs += 1;
    i = opus_repacketizer_out(rp, po, 1276 * 48 + 48 * 2 + 2);
    if i != 2 + 8 + 2 + 2
        || *po.offset(0 as isize) as i32 & 3 != 3
        || *po.offset(1 as isize) as i32 & 63 != 3
        || *po.offset(1 as isize) as i32 >> 7 != 1
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1619);
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as isize) = 2;
    *packet.offset(1 as isize) = 4;
    if opus_repacketizer_cat(rp, packet, 8) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1626);
    }
    cfgs += 1;
    if opus_repacketizer_cat(rp, packet, 8) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1628);
    }
    cfgs += 1;
    i = opus_repacketizer_out(rp, po, 1276 * 48 + 48 * 2 + 2);
    if i != 2 + 1 + 1 + 1 + 4 + 2 + 4 + 2
        || *po.offset(0 as isize) as i32 & 3 != 3
        || *po.offset(1 as isize) as i32 & 63 != 4
        || *po.offset(1 as isize) as i32 >> 7 != 1
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1631);
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as isize) = 2;
    *packet.offset(1 as isize) = 4;
    if opus_repacketizer_cat(rp, packet, 10) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1638);
    }
    cfgs += 1;
    if opus_repacketizer_cat(rp, packet, 10) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1640);
    }
    cfgs += 1;
    i = opus_repacketizer_out(rp, po, 1276 * 48 + 48 * 2 + 2);
    if i != 2 + 4 + 4 + 4 + 4
        || *po.offset(0 as isize) as i32 & 3 != 3
        || *po.offset(1 as isize) as i32 & 63 != 4
        || *po.offset(1 as isize) as i32 >> 7 != 0
    {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1643);
    }
    cfgs += 1;
    j = 0;
    while j < 32 {
        let mut maxi_0: i32 = 0;
        let mut sum: i32 = 0;
        let mut rcnt_0: i32 = 0;
        *packet.offset(0 as isize) = (((j << 1) + (j & 1)) << 2) as u8;
        maxi_0 = 960 / opus_packet_get_samples_per_frame(packet, 8000);
        sum = 0;
        rcnt_0 = 0;
        opus_repacketizer_init(rp);
        i = 1;
        while i <= maxi_0 + 2 {
            let mut len_0: i32 = 0;
            ret = opus_repacketizer_cat(rp, packet, i);
            if rcnt_0 < maxi_0 {
                if ret != 0 {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1662);
                }
                rcnt_0 += 1;
                sum += i - 1;
            } else if ret != -(4) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1665);
            }
            cfgs += 1;
            len_0 = sum
                + (if rcnt_0 < 2 {
                    1
                } else if rcnt_0 < 3 {
                    2
                } else {
                    2 + rcnt_0 - 1
                });
            if opus_repacketizer_out(rp, po, 1276 * 48 + 48 * 2 + 2) != len_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1668);
            }
            if rcnt_0 > 2 && *po.offset(1 as isize) as i32 & 63 != rcnt_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1669);
            }
            if rcnt_0 == 2 && *po.offset(0 as isize) as i32 & 3 != 2 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1670);
            }
            if rcnt_0 == 1 && *po.offset(0 as isize) as i32 & 3 != 0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1671);
            }
            cfgs += 1;
            if opus_repacketizer_out(rp, po, len_0) != len_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1673);
            }
            cfgs += 1;
            if opus_packet_unpad(po, len_0) != len_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1675);
            }
            cfgs += 1;
            if opus_packet_pad(po, len_0, len_0 + 1) != 0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1677);
            }
            cfgs += 1;
            if opus_packet_pad(po, len_0 + 1, len_0 + 256) != 0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1679);
            }
            cfgs += 1;
            if opus_packet_unpad(po, len_0 + 256) != len_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1681);
            }
            cfgs += 1;
            if opus_multistream_packet_unpad(po, len_0, 1) != len_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1683);
            }
            cfgs += 1;
            if opus_multistream_packet_pad(po, len_0, len_0 + 1, 1) != 0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1685);
            }
            cfgs += 1;
            if opus_multistream_packet_pad(po, len_0 + 1, len_0 + 256, 1) != 0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1687);
            }
            cfgs += 1;
            if opus_multistream_packet_unpad(po, len_0 + 256, 1) != len_0 {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1689);
            }
            cfgs += 1;
            if opus_repacketizer_out(rp, po, len_0 - 1) != -(2) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1691);
            }
            cfgs += 1;
            if len_0 > 1 {
                if opus_repacketizer_out(rp, po, 1) != -(2) {
                    _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1695);
                }
                cfgs += 1;
            }
            if opus_repacketizer_out(rp, po, 0) != -(2) {
                _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1698);
            }
            cfgs += 1;
            i += 1;
        }
        j += 1;
    }
    *po.offset(0 as isize) = 'O' as i32 as u8;
    *po.offset(1 as isize) = 'p' as i32 as u8;
    if opus_packet_pad(po, 4, 4) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1705);
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 4, 4, 1) != 0 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1707);
    }
    cfgs += 1;
    if opus_packet_pad(po, 4, 5) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1709);
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 4, 5, 1) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1711);
    }
    cfgs += 1;
    if opus_packet_pad(po, 0, 5) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1713);
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 0, 5, 1) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1715);
    }
    cfgs += 1;
    if opus_packet_unpad(po, 0) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1717);
    }
    cfgs += 1;
    if opus_multistream_packet_unpad(po, 0, 1) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1719);
    }
    cfgs += 1;
    if opus_packet_unpad(po, 4) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1721);
    }
    cfgs += 1;
    if opus_multistream_packet_unpad(po, 4, 1) != -(4) {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1723);
    }
    cfgs += 1;
    *po.offset(0 as isize) = 0;
    *po.offset(1 as isize) = 0;
    *po.offset(2 as isize) = 0;
    if opus_packet_pad(po, 5, 4) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1728);
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 5, 4, 1) != -1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1730);
    }
    cfgs += 1;
    println!("    opus_repacketizer_cat ........................ OK.");
    println!("    opus_repacketizer_out ........................ OK.");
    println!("    opus_repacketizer_out_range .................. OK.");
    println!("    opus_packet_pad .............................. OK.");
    println!("    opus_packet_unpad ............................ OK.");
    println!("    opus_multistream_packet_pad .................. OK.");
    println!("    opus_multistream_packet_unpad ................ OK.");
    opus_repacketizer_destroy(rp);
    cfgs += 1;
    free(packet as *mut core::ffi::c_void);
    free(po as *mut core::ffi::c_void);
    println!("                        All repacketizer tests passed");
    println!("                   ({:7} API invocations)", cfgs);
    cfgs
}
pub unsafe fn test_malloc_fail() -> i32 {
    println!("\n  malloc() failure tests");
    println!("  ---------------------------------------------------");
    println!("    opus_decoder_create() ................... SKIPPED.");
    println!("    opus_encoder_create() ................... SKIPPED.");
    println!("    opus_repacketizer_create() .............. SKIPPED.");
    println!("    opus_multistream_decoder_create() ....... SKIPPED.");
    println!("    opus_multistream_encoder_create() ....... SKIPPED.");
    println!("(Test only supported with GLIBC and without valgrind)");
    0
}

unsafe fn main_0() -> i32 {
    let mut total: i32 = 0;
    let oversion = opus_get_version_string();
    eprintln!("Testing the {} API deterministically", oversion);
    opus_strerror(-(32768));
    opus_strerror(32767);
    if opus_strerror(0).len() < 1 {
        _test_failed(b"tests/test_opus_api.c\0" as *const u8 as *const i8, 1891);
    }
    total = 4;
    total += test_dec_api();
    total += test_msdec_api();
    total += test_parse();
    total += test_enc_api();
    total += test_repacketizer_api();
    total += test_malloc_fail();
    eprintln!(
        "\nAll API tests passed.\nThe libopus API was invoked {} times.",
        total
    );
    0
}

#[test]
fn test_opus_encode() {
    assert_eq!(unsafe { main_0() }, 0, "Test returned a non-zero exit code");
}
