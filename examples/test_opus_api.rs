#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use libc::{fprintf, printf};
use libc_stdhandle::{stderr, stdout};

pub mod test_opus_common_h {
    pub static mut iseed: u32 = 0;
    #[inline]
    pub unsafe fn _test_failed(mut file: *const i8, mut line: i32) -> ! {
        fprintf(
            stderr(),
            b"\n ***************************************************\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b" ***         A fatal error was detected.         ***\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b" ***************************************************\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b"Please report this failure and include\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr(),
            b"'make check SEED=%u fails %s at line %d for %s'\n\0" as *const u8 as *const i8,
            iseed,
            file,
            line,
            opus_get_version_string(),
        );
        fprintf(
            stderr(),
            b"and any relevant details about your system.\n\n\0" as *const u8 as *const i8,
        );
        abort();
    }

    use libc::{abort, fprintf};
    use libc_stdhandle::stderr;
    use unsafe_libopus::opus_get_version_string;
}

pub use self::test_opus_common_h::{_test_failed, iseed};

use unsafe_libopus::externs::{free, malloc};
use unsafe_libopus::externs::{memcmp, memcpy, memset, strlen};
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
static mut opus_rates: [i32; 5] = [
    48000 as i32,
    24000 as i32,
    16000 as i32,
    12000 as i32,
    8000 as i32,
];
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
    cfgs = 0 as i32;
    fprintf(
        stdout(),
        b"\n  Decoder basic API tests\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8 as *const i8,
    );
    c = 0 as i32;
    while c < 4 as i32 {
        i = opus_decoder_get_size(c);
        if (c == 1 as i32 || c == 2 as i32) && (i <= 2048 as i32 || i > (1 as i32) << 16 as i32)
            || c != 1 as i32 && c != 2 as i32 && i != 0 as i32
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                106 as i32,
            );
        }
        fprintf(
            stdout(),
            b"    opus_decoder_get_size(%d)=%d ...............%s OK.\n\0" as *const u8 as *const i8,
            c,
            i,
            if i > 0 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"....\0" as *const u8 as *const i8
            },
        );
        cfgs += 1;
        c += 1;
    }
    c = 0 as i32;
    while c < 4 as i32 {
        i = -(7 as i32);
        while i <= 96000 as i32 {
            let mut fs: i32 = 0;
            if !((i == 8000 as i32
                || i == 12000 as i32
                || i == 16000 as i32
                || i == 24000 as i32
                || i == 48000 as i32)
                && (c == 1 as i32 || c == 2 as i32))
            {
                match i {
                    -5 => {
                        fs = -(8000 as i32);
                    }
                    -6 => {
                        fs = 2147483647 as i32;
                    }
                    -7 => {
                        fs = -(2147483647 as i32) - 1 as i32;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0 as i32;
                dec = opus_decoder_create(fs, c, &mut err);
                if err != -(1 as i32) || !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        128 as i32,
                    );
                }
                cfgs += 1;
                dec = opus_decoder_create(fs, c, std::ptr::null_mut::<i32>());
                if !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        131 as i32,
                    );
                }
                cfgs += 1;
                dec = malloc(opus_decoder_get_size(2 as i32) as u64) as *mut OpusDecoder;
                if dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        134 as i32,
                    );
                }
                err = opus_decoder_init(dec, fs, c);
                if err != -(1 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        136 as i32,
                    );
                }
                cfgs += 1;
                free(dec as *mut core::ffi::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    dec = opus_decoder_create(48000 as i32, 2 as i32, &mut err);
    if err != 0 as i32 || dec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            144 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_decoder_create() ........................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_decoder_init() .......................... OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_decoder_ctl!(dec, 4031 as i32, null_uint_ptr as *mut u32);
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            152 as i32,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4031 as i32,
        (&mut dec_final_range as *mut u32).offset(
            (&mut dec_final_range as *mut u32).offset_from(&mut dec_final_range as *mut u32) as i64
                as isize,
        )
    );
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            155 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_FINAL_RANGE ......................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(dec, -(5 as i32));
    if err != -(5 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            161 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_UNIMPLEMENTED ........................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4009 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize)
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            166 as i32,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4009 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            169 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_BANDWIDTH ........................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4029 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize)
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            174 as i32,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4029 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i != 48000 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            177 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_SAMPLE_RATE ......................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize)
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            183 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i > 0 as i32 || i < -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            187 as i32,
        );
    }
    cfgs += 1;
    packet[0 as i32 as usize] = ((63 as i32) << 2 as i32) as u8;
    packet[2 as i32 as usize] = 0 as i32 as u8;
    packet[1 as i32 as usize] = packet[2 as i32 as usize];
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            191 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i > 0 as i32 || i < -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            195 as i32,
        );
    }
    cfgs += 1;
    packet[0 as i32 as usize] = 1 as i32 as u8;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        1 as i32,
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            198 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i > 0 as i32 || i < -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            202 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_PITCH ............................... OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_decoder_ctl!(
        dec,
        4039 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize)
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            207 as i32,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4039 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i != 960 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            210 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_LAST_PACKET_DURATION ................ OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_decoder_ctl!(
        dec,
        4045 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            217 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4045 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize)
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            220 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4034 as i32, -(32769 as i32));
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            223 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4034 as i32, 32768 as i32);
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            226 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4034 as i32, -(15 as i32));
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            229 as i32,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4045 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
    );
    if err != 0 as i32 || i != -(15 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            234 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_SET_GAIN ................................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    OPUS_GET_GAIN ................................ OK.\n\0" as *const u8 as *const i8,
    );
    dec2 = malloc(opus_decoder_get_size(2 as i32) as u64) as *mut OpusDecoder;
    memcpy(
        dec2 as *mut core::ffi::c_void,
        dec as *const core::ffi::c_void,
        opus_decoder_get_size(2 as i32) as u64,
    );
    if opus_decoder_ctl!(dec, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            242 as i32,
        );
    }
    if memcmp(
        dec2 as *const core::ffi::c_void,
        dec as *const core::ffi::c_void,
        opus_decoder_get_size(2 as i32) as u64,
    ) == 0 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            243 as i32,
        );
    }
    free(dec2 as *mut core::ffi::c_void);
    fprintf(
        stdout(),
        b"    OPUS_RESET_STATE ............................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    packet[0 as i32 as usize] = 0 as i32 as u8;
    if opus_decoder_get_nb_samples(dec, packet.as_mut_ptr() as *const u8, 1 as i32) != 480 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            250 as i32,
        );
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1 as i32, 48000 as i32)
        != 480 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            251 as i32,
        );
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1 as i32, 96000 as i32)
        != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            252 as i32,
        );
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1 as i32, 32000 as i32)
        != 320 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            253 as i32,
        );
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1 as i32, 8000 as i32)
        != 80 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            254 as i32,
        );
    }
    packet[0 as i32 as usize] = 3 as i32 as u8;
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 1 as i32, 24000 as i32)
        != -(4 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            256 as i32,
        );
    }
    packet[0 as i32 as usize] = ((63 as i32) << 2 as i32 | 3 as i32) as u8;
    packet[1 as i32 as usize] = 63 as i32 as u8;
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 0 as i32, 24000 as i32)
        != -(1 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            259 as i32,
        );
    }
    if opus_packet_get_nb_samples(packet.as_mut_ptr() as *const u8, 2 as i32, 48000 as i32)
        != -(4 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            260 as i32,
        );
    }
    if opus_decoder_get_nb_samples(dec, packet.as_mut_ptr() as *const u8, 2 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            261 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    opus_{packet,decoder}_get_nb_samples() ....... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 9 as i32;
    if -(1 as i32) != opus_packet_get_nb_frames(packet.as_mut_ptr() as *const u8, 0 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            265 as i32,
        );
    }
    i = 0 as i32;
    while i < 256 as i32 {
        let mut l1res: [i32; 4] = [1 as i32, 2 as i32, 2 as i32, -(4 as i32)];
        packet[0 as i32 as usize] = i as u8;
        if l1res[(packet[0 as i32 as usize] as i32 & 3 as i32) as usize]
            != opus_packet_get_nb_frames(packet.as_mut_ptr() as *const u8, 1 as i32)
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                269 as i32,
            );
        }
        cfgs += 1;
        j = 0 as i32;
        while j < 256 as i32 {
            packet[1 as i32 as usize] = j as u8;
            if (if packet[0 as i32 as usize] as i32 & 3 as i32 != 3 as i32 {
                l1res[(packet[0 as i32 as usize] as i32 & 3 as i32) as usize]
            } else {
                packet[1 as i32 as usize] as i32 & 63 as i32
            }) != opus_packet_get_nb_frames(packet.as_mut_ptr() as *const u8, 2 as i32)
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    273 as i32,
                );
            }
            cfgs += 1;
            j += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    opus_packet_get_nb_frames() .................. OK.\n\0" as *const u8 as *const i8,
    );
    i = 0 as i32;
    while i < 256 as i32 {
        let mut bw: i32 = 0;
        packet[0 as i32 as usize] = i as u8;
        bw = packet[0 as i32 as usize] as i32 >> 4 as i32;
        bw = 1101 as i32
            + (((((bw & 7 as i32) * 9 as i32) & (63 as i32 - (bw & 8 as i32)))
                + 2 as i32
                + 12 as i32 * (bw & 8 as i32 != 0 as i32) as i32)
                >> 4 as i32);
        if bw != opus_packet_get_bandwidth(packet.as_mut_ptr()) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                284 as i32,
            );
        }
        cfgs += 1;
        i += 1;
    }
    fprintf(
        stdout(),
        b"    opus_packet_get_bandwidth() .................. OK.\n\0" as *const u8 as *const i8,
    );
    i = 0 as i32;
    while i < 256 as i32 {
        let mut fp3s: i32 = 0;
        let mut rate: i32 = 0;
        packet[0 as i32 as usize] = i as u8;
        fp3s = packet[0 as i32 as usize] as i32 >> 3 as i32;
        fp3s = (((((3 as i32 - (fp3s & 3 as i32)) * 13 as i32) & 119 as i32) + 9 as i32)
            >> 2 as i32)
            * ((fp3s > 13 as i32) as i32 * (3 as i32 - (fp3s & 3 as i32 == 3 as i32) as i32)
                + 1 as i32)
            * 25 as i32;
        rate = 0 as i32;
        while rate < 5 as i32 {
            if opus_rates[rate as usize] * 3 as i32 / fp3s
                != opus_packet_get_samples_per_frame(packet.as_mut_ptr(), opus_rates[rate as usize])
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    295 as i32,
                );
            }
            cfgs += 1;
            rate += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    opus_packet_get_samples_per_frame() .......... OK.\n\0" as *const u8 as *const i8,
    );
    packet[0 as i32 as usize] = (((63 as i32) << 2 as i32) + 3 as i32) as u8;
    packet[1 as i32 as usize] = 49 as i32 as u8;
    j = 2 as i32;
    while j < 51 as i32 {
        packet[j as usize] = 0 as i32 as u8;
        j += 1;
    }
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        51 as i32,
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != -(4 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            305 as i32,
        );
    }
    cfgs += 1;
    packet[0 as i32 as usize] = ((63 as i32) << 2 as i32) as u8;
    packet[2 as i32 as usize] = 0 as i32 as u8;
    packet[1 as i32 as usize] = packet[2 as i32 as usize];
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        -(1 as i32),
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != -(1 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            309 as i32,
        );
    }
    cfgs += 1;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        60 as i32,
        0 as i32,
    ) != -(2 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            311 as i32,
        );
    }
    cfgs += 1;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        480 as i32,
        0 as i32,
    ) != -(2 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            313 as i32,
        );
    }
    cfgs += 1;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            315 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_decode() ................................ OK.\n\0" as *const u8 as *const i8,
    );
    if opus_decode_float(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        fbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            320 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_decode_float() .......................... OK.\n\0" as *const u8 as *const i8,
    );
    opus_decoder_destroy(dec);
    cfgs += 1;
    fprintf(
        stdout(),
        b"                   All decoder interface tests passed\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"                             (%6d API invocations)\n\0" as *const u8 as *const i8,
        cfgs,
    );
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
    mapping[0 as i32 as usize] = 0 as i32 as u8;
    mapping[1 as i32 as usize] = 1 as i32 as u8;
    i = 2 as i32;
    while i < 256 as i32 {
        i += 1;
    }
    cfgs = 0 as i32;
    fprintf(
        stdout(),
        b"\n  Multistream decoder basic API tests\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8 as *const i8,
    );
    a = -(1 as i32);
    while a < 4 as i32 {
        b = -(1 as i32);
        while b < 4 as i32 {
            i = opus_multistream_decoder_get_size(a, b);
            if a > 0 as i32
                && b <= a
                && b >= 0 as i32
                && (i <= 2048 as i32 || i > ((1 as i32) << 16 as i32) * a)
                || (a < 1 as i32 || b > a || b < 0 as i32) && i != 0 as i32
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    370 as i32,
                );
            }
            fprintf(
                stdout(),
                b"    opus_multistream_decoder_get_size(%2d,%2d)=%d %sOK.\n\0" as *const u8
                    as *const i8,
                a,
                b,
                i,
                if i > 0 as i32 {
                    b"\0" as *const u8 as *const i8
                } else {
                    b"... \0" as *const u8 as *const i8
                },
            );
            cfgs += 1;
            b += 1;
        }
        a += 1;
    }
    c = 1 as i32;
    while c < 3 as i32 {
        i = -(7 as i32);
        while i <= 96000 as i32 {
            let mut fs: i32 = 0;
            if !((i == 8000 as i32
                || i == 12000 as i32
                || i == 16000 as i32
                || i == 24000 as i32
                || i == 48000 as i32)
                && (c == 1 as i32 || c == 2 as i32))
            {
                match i {
                    -5 => {
                        fs = -(8000 as i32);
                    }
                    -6 => {
                        fs = 2147483647 as i32;
                    }
                    -7 => {
                        fs = -(2147483647 as i32) - 1 as i32;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0 as i32;
                dec = opus_multistream_decoder_create(
                    fs,
                    c,
                    1 as i32,
                    c - 1 as i32,
                    mapping.as_mut_ptr(),
                    &mut err,
                );
                if err != -(1 as i32) || !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        393 as i32,
                    );
                }
                cfgs += 1;
                dec = opus_multistream_decoder_create(
                    fs,
                    c,
                    1 as i32,
                    c - 1 as i32,
                    mapping.as_mut_ptr(),
                    std::ptr::null_mut::<i32>(),
                );
                if !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        396 as i32,
                    );
                }
                cfgs += 1;
                dec = malloc(opus_multistream_decoder_get_size(1 as i32, 1 as i32) as u64)
                    as *mut OpusMSDecoder;
                if dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        399 as i32,
                    );
                }
                err = opus_multistream_decoder_init(
                    dec,
                    fs,
                    c,
                    1 as i32,
                    c - 1 as i32,
                    mapping.as_mut_ptr(),
                );
                if err != -(1 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        401 as i32,
                    );
                }
                cfgs += 1;
                free(dec as *mut core::ffi::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    c = 0 as i32;
    while c < 2 as i32 {
        let mut ret_err: *mut i32 = std::ptr::null_mut::<i32>();
        ret_err = if c != 0 {
            std::ptr::null_mut::<i32>()
        } else {
            &mut err
        };
        mapping[0 as i32 as usize] = 0 as i32 as u8;
        mapping[1 as i32 as usize] = 1 as i32 as u8;
        i = 2 as i32;
        while i < 256 as i32 {
            i += 1;
        }
        dec = opus_multistream_decoder_create(
            48000 as i32,
            2 as i32,
            1 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                419 as i32,
            );
        }
        cfgs += 1;
        mapping[1 as i32 as usize] = 0 as i32 as u8;
        mapping[0 as i32 as usize] = mapping[1 as i32 as usize];
        dec = opus_multistream_decoder_create(
            48000 as i32,
            2 as i32,
            1 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as i32 || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                426 as i32,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            1 as i32,
            4 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as i32 || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                434 as i32,
            );
        }
        cfgs += 1;
        err = opus_multistream_decoder_init(
            dec,
            48000 as i32,
            1 as i32,
            0 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
        );
        if err != -(1 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                438 as i32,
            );
        }
        cfgs += 1;
        err = opus_multistream_decoder_init(
            dec,
            48000 as i32,
            1 as i32,
            1 as i32,
            -(1 as i32),
            mapping.as_mut_ptr(),
        );
        if err != -(1 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                442 as i32,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            2 as i32,
            1 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as i32 || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                451 as i32,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            255 as i32,
            255 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                459 as i32,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            -(1 as i32),
            1 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                465 as i32,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            0 as i32,
            1 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                471 as i32,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            1 as i32,
            -(1 as i32),
            2 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                477 as i32,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            1 as i32,
            -(1 as i32),
            -(1 as i32),
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                483 as i32,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            256 as i32,
            255 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                489 as i32,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            256 as i32,
            255 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                495 as i32,
            );
        }
        cfgs += 1;
        mapping[0 as i32 as usize] = 255 as i32 as u8;
        mapping[1 as i32 as usize] = 1 as i32 as u8;
        mapping[2 as i32 as usize] = 2 as i32 as u8;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            3 as i32,
            2 as i32,
            0 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                504 as i32,
            );
        }
        cfgs += 1;
        mapping[0 as i32 as usize] = 0 as i32 as u8;
        mapping[1 as i32 as usize] = 0 as i32 as u8;
        mapping[2 as i32 as usize] = 0 as i32 as u8;
        dec = opus_multistream_decoder_create(
            48000 as i32,
            3 as i32,
            2 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as i32 || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                513 as i32,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        mapping[0 as i32 as usize] = 0 as i32 as u8;
        mapping[1 as i32 as usize] = 255 as i32 as u8;
        mapping[2 as i32 as usize] = 1 as i32 as u8;
        mapping[3 as i32 as usize] = 2 as i32 as u8;
        mapping[4 as i32 as usize] = 3 as i32 as u8;
        dec = opus_multistream_decoder_create(
            48001 as i32,
            5 as i32,
            4 as i32,
            1 as i32,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as i32) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                526 as i32,
            );
        }
        cfgs += 1;
        c += 1;
    }
    mapping[0 as i32 as usize] = 0 as i32 as u8;
    mapping[1 as i32 as usize] = 255 as i32 as u8;
    mapping[2 as i32 as usize] = 1 as i32 as u8;
    mapping[3 as i32 as usize] = 2 as i32 as u8;
    dec = opus_multistream_decoder_create(
        48000 as i32,
        4 as i32,
        2 as i32,
        1 as i32,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as i32 || dec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            537 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_multistream_decoder_create() ............ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_decoder_init() .............. OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_multistream_decoder_ctl!(
        dec,
        4031 as i32,
        (&mut dec_final_range as *mut u32).offset(
            (&mut dec_final_range as *mut u32).offset_from(&mut dec_final_range as *mut u32) as i64
                as isize,
        ),
    );
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            545 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_FINAL_RANGE ......................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    streamdec = std::ptr::null_mut::<OpusDecoder>();
    err = opus_multistream_decoder_ctl!(
        dec,
        5122 as i32,
        -(1 as i32),
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as i64 as isize,
        ),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            553 as i32,
        );
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(
        dec,
        5122 as i32,
        1 as i32,
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as i64 as isize,
        ),
    );
    if err != 0 as i32 || streamdec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            556 as i32,
        );
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(
        dec,
        5122 as i32,
        2 as i32,
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as i64 as isize,
        ),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            560 as i32,
        );
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(
        dec,
        5122 as i32,
        0 as i32,
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as i64 as isize,
        ),
    );
    if err != 0 as i32 || streamdec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            563 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_MULTISTREAM_GET_DECODER_STATE ........... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    j = 0 as i32;
    while j < 2 as i32 {
        let mut od: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        err = opus_multistream_decoder_ctl!(
            dec,
            5122 as i32,
            j,
            (&mut od as *mut *mut OpusDecoder).offset(
                (&mut od as *mut *mut OpusDecoder).offset_from(&mut od as *mut *mut OpusDecoder)
                    as i64 as isize,
            ),
        );
        if err != 0 as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                572 as i32,
            );
        }
        err = opus_decoder_ctl!(
            od,
            4045 as i32,
            (&mut i as *mut i32)
                .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
        );
        if err != 0 as i32 || i != 0 as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                576 as i32,
            );
        }
        cfgs += 1;
        j += 1;
    }
    err = opus_multistream_decoder_ctl!(dec, 4034 as i32, 15 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            580 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_GAIN ................................ OK.\n\0" as *const u8 as *const i8,
    );
    j = 0 as i32;
    while j < 2 as i32 {
        let mut od_0: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        err = opus_multistream_decoder_ctl!(
            dec,
            5122 as i32,
            j,
            (&mut od_0 as *mut *mut OpusDecoder).offset(
                (&mut od_0 as *mut *mut OpusDecoder).offset_from(&mut od_0 as *mut *mut OpusDecoder)
                    as i64 as isize,
            ),
        );
        if err != 0 as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                586 as i32,
            );
        }
        err = opus_decoder_ctl!(
            od_0,
            4045 as i32,
            (&mut i as *mut i32)
                .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize)
        );
        if err != 0 as i32 || i != 15 as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                590 as i32,
            );
        }
        cfgs += 1;
        j += 1;
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_GAIN ................................ OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_multistream_decoder_ctl!(
        dec,
        4009 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            597 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_BANDWIDTH ........................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    err = opus_multistream_decoder_ctl!(dec, -(5 as i32));
    if err != -(5 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            602 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_UNIMPLEMENTED ........................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    if opus_multistream_decoder_ctl!(dec, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            635 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_RESET_STATE ............................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    opus_multistream_decoder_destroy(dec);
    cfgs += 1;
    dec = opus_multistream_decoder_create(
        48000 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as i32 || dec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            643 as i32,
        );
    }
    cfgs += 1;
    packet[0 as i32 as usize] = (((63 as i32) << 2 as i32) + 3 as i32) as u8;
    packet[1 as i32 as usize] = 49 as i32 as u8;
    j = 2 as i32;
    while j < 51 as i32 {
        packet[j as usize] = 0 as i32 as u8;
        j += 1;
    }
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        51 as i32,
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != -(4 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            650 as i32,
        );
    }
    cfgs += 1;
    packet[0 as i32 as usize] = ((63 as i32) << 2 as i32) as u8;
    packet[2 as i32 as usize] = 0 as i32 as u8;
    packet[1 as i32 as usize] = packet[2 as i32 as usize];
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        -(1 as i32),
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != -(1 as i32)
    {
        printf(
            b"%d\n\0" as *const u8 as *const i8,
            opus_multistream_decode(
                dec,
                packet.as_mut_ptr(),
                -(1 as i32),
                sbuf.as_mut_ptr(),
                960 as i32,
                0 as i32,
            ),
        );
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            654 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        -(960 as i32),
        0 as i32,
    ) != -(1 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            656 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        60 as i32,
        0 as i32,
    ) != -(2 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            658 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        480 as i32,
        0 as i32,
    ) != -(2 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            660 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        sbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            662 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_multistream_decode() .................... OK.\n\0" as *const u8 as *const i8,
    );
    if opus_multistream_decode_float(
        dec,
        packet.as_mut_ptr(),
        3 as i32,
        fbuf.as_mut_ptr(),
        960 as i32,
        0 as i32,
    ) != 960 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            667 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_multistream_decode_float() .............. OK.\n\0" as *const u8 as *const i8,
    );
    opus_multistream_decoder_destroy(dec);
    cfgs += 1;
    fprintf(
        stdout(),
        b"       All multistream decoder interface tests passed\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"                             (%6d API invocations)\n\0" as *const u8 as *const i8,
        cfgs,
    );
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
    fprintf(
        stdout(),
        b"\n  Packet header parsing tests\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8 as *const i8,
    );
    memset(
        packet.as_mut_ptr() as *mut core::ffi::c_void,
        0 as i32,
        (::core::mem::size_of::<i8>() as u64).wrapping_mul(1276 as i32 as u64),
    );
    packet[0 as i32 as usize] = ((63 as i32) << 2 as i32) as u8;
    if opus_packet_parse(
        packet.as_mut_ptr(),
        1 as i32,
        &mut toc,
        frames.as_mut_ptr(),
        std::ptr::null_mut::<i16>(),
        &mut payload_offset,
    ) != -(1 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            720 as i32,
        );
    }
    cfgs = 1 as i32;
    cfgs_total = cfgs;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = (i << 2 as i32) as u8;
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            4 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != 1 as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                729 as i32,
            );
        }
        if size[0 as i32 as usize] as i32 != 3 as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                730 as i32,
            );
        }
        if frames[0 as i32 as usize] != packet.as_mut_ptr().offset(1 as i32 as isize) as *const u8 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                731 as i32,
            );
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 0 (%2d cases) ............................ OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = ((i << 2 as i32) + 1 as i32) as u8;
        jj = 0 as i32;
        while jj <= 1275 as i32 * 2 as i32 + 3 as i32 {
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                jj,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if jj & 1 as i32 == 1 as i32 && jj <= 2551 as i32 {
                if ret != 2 as i32 {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        749 as i32,
                    );
                }
                if size[0 as i32 as usize] as i32 != size[1 as i32 as usize] as i32
                    || size[0 as i32 as usize] as i32 != (jj - 1 as i32) >> 1 as i32
                {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        750 as i32,
                    );
                }
                if frames[0 as i32 as usize]
                    != packet.as_mut_ptr().offset(1 as i32 as isize) as *const u8
                {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        751 as i32,
                    );
                }
                if frames[1 as i32 as usize]
                    != (frames[0 as i32 as usize]).offset(size[0 as i32 as usize] as i32 as isize)
                {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        752 as i32,
                    );
                }
                if toc as i32 >> 2 as i32 != i {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        753 as i32,
                    );
                }
            } else if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    754 as i32,
                );
            }
            jj += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 1 (%6d cases) ........................ OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = ((i << 2 as i32) + 2 as i32) as u8;
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                767 as i32,
            );
        }
        packet[1 as i32 as usize] = 252 as i32 as u8;
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            2 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                772 as i32,
            );
        }
        j = 0 as i32;
        while j < 1275 as i32 {
            if j < 252 as i32 {
                packet[1 as i32 as usize] = j as u8;
            } else {
                packet[1 as i32 as usize] = (252 as i32 + (j & 3 as i32)) as u8;
                packet[2 as i32 as usize] = ((j - 252 as i32) >> 2 as i32) as u8;
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 as i32 { 2 as i32 } else { 3 as i32 }) - 1 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    781 as i32,
                );
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 as i32 { 2 as i32 } else { 3 as i32 }) + 1276 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    786 as i32,
                );
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 as i32 { 2 as i32 } else { 3 as i32 }),
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 2 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    791 as i32,
                );
            }
            if size[0 as i32 as usize] as i32 != j || size[1 as i32 as usize] as i32 != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    792 as i32,
                );
            }
            if frames[1 as i32 as usize]
                != (frames[0 as i32 as usize]).offset(size[0 as i32 as usize] as i32 as isize)
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    793 as i32,
                );
            }
            if toc as i32 >> 2 as i32 != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    794 as i32,
                );
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                (j << 1 as i32) + 4 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 2 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    799 as i32,
                );
            }
            if size[0 as i32 as usize] as i32 != j
                || size[1 as i32 as usize] as i32
                    != (j << 1 as i32) + 3 as i32
                        - j
                        - (if j < 252 as i32 { 1 as i32 } else { 2 as i32 })
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    800 as i32,
                );
            }
            if frames[1 as i32 as usize]
                != (frames[0 as i32 as usize]).offset(size[0 as i32 as usize] as i32 as isize)
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    801 as i32,
                );
            }
            if toc as i32 >> 2 as i32 != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    802 as i32,
                );
            }
            j += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 2 (%6d cases) ........................ OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = ((i << 2 as i32) + 3 as i32) as u8;
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                815 as i32,
            );
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m-truncation (%2d cases) ............... OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = ((i << 2 as i32) + 3 as i32) as u8;
        jj = 49 as i32;
        while jj <= 64 as i32 {
            packet[1 as i32 as usize] = (0 as i32 + (jj & 63 as i32)) as u8;
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    830 as i32,
                );
            }
            packet[1 as i32 as usize] = (128 as i32 + (jj & 63 as i32)) as u8;
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    835 as i32,
                );
            }
            packet[1 as i32 as usize] = (64 as i32 + (jj & 63 as i32)) as u8;
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    840 as i32,
                );
            }
            packet[1 as i32 as usize] = (128 as i32 + 64 as i32 + (jj & 63 as i32)) as u8;
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    845 as i32,
                );
            }
            jj += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=0,49-64 (%2d cases) ................ OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = ((i << 2 as i32) + 3 as i32) as u8;
        packet[1 as i32 as usize] = 1 as i32 as u8;
        j = 0 as i32;
        while j < 1276 as i32 {
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + 2 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 1 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    861 as i32,
                );
            }
            if size[0 as i32 as usize] as i32 != j {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    862 as i32,
                );
            }
            if toc as i32 >> 2 as i32 != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    863 as i32,
                );
            }
            j += 1;
        }
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1276 as i32 + 2 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                868 as i32,
            );
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=1 CBR (%2d cases) ................. OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        let mut frame_samp: i32 = 0;
        packet[0 as i32 as usize] = ((i << 2 as i32) + 3 as i32) as u8;
        frame_samp = opus_packet_get_samples_per_frame(packet.as_mut_ptr(), 48000 as i32);
        j = 2 as i32;
        while j < 49 as i32 {
            packet[1 as i32 as usize] = j as u8;
            sz = 2 as i32;
            while sz < (j + 2 as i32) * 1275 as i32 {
                toc = -(1 as i32) as u8;
                frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
                frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
                payload_offset = -(1 as i32);
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    sz,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if frame_samp * j <= 5760 as i32
                    && (sz - 2 as i32) % j == 0 as i32
                    && (sz - 2 as i32) / j < 1276 as i32
                {
                    if ret != j {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            890 as i32,
                        );
                    }
                    jj = 1 as i32;
                    while jj < ret {
                        if frames[jj as usize]
                            != (frames[(jj - 1 as i32) as usize])
                                .offset(size[(jj - 1 as i32) as usize] as i32 as isize)
                        {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                891 as i32,
                            );
                        }
                        jj += 1;
                    }
                    if toc as i32 >> 2 as i32 != i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            892 as i32,
                        );
                    }
                } else if ret != -(4 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        893 as i32,
                    );
                }
                sz += 1;
            }
            j += 1;
        }
        packet[1 as i32 as usize] = (5760 as i32 / frame_samp) as u8;
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1275 as i32 * packet[1 as i32 as usize] as i32 + 2 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != packet[1 as i32 as usize] as i32 {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                901 as i32,
            );
        }
        jj = 0 as i32;
        while jj < ret {
            if size[jj as usize] as i32 != 1275 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    902 as i32,
                );
            }
            jj += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=1-48 CBR (%2d cases) .......... OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        let mut frame_samp_0: i32 = 0;
        packet[0 as i32 as usize] = ((i << 2 as i32) + 3 as i32) as u8;
        packet[1 as i32 as usize] = (128 as i32 + 1 as i32) as u8;
        frame_samp_0 = opus_packet_get_samples_per_frame(packet.as_mut_ptr(), 48000 as i32);
        jj = 0 as i32;
        while jj < 1276 as i32 {
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as i32 + jj,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 1 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    919 as i32,
                );
            }
            if size[0 as i32 as usize] as i32 != jj {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    920 as i32,
                );
            }
            if toc as i32 >> 2 as i32 != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    921 as i32,
                );
            }
            jj += 1;
        }
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            2 as i32 + 1276 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                926 as i32,
            );
        }
        j = 2 as i32;
        while j < 49 as i32 {
            packet[1 as i32 as usize] = (128 as i32 + j) as u8;
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as i32 + j - 2 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    934 as i32,
                );
            }
            packet[2 as i32 as usize] = 252 as i32 as u8;
            packet[3 as i32 as usize] = 0 as i32 as u8;
            jj = 4 as i32;
            while jj < 2 as i32 + j {
                packet[jj as usize] = 0 as i32 as u8;
                jj += 1;
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as i32 + j,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    941 as i32,
                );
            }
            jj = 2 as i32;
            while jj < 2 as i32 + j {
                packet[jj as usize] = 0 as i32 as u8;
                jj += 1;
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as i32 + j - 2 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    947 as i32,
                );
            }
            packet[2 as i32 as usize] = 252 as i32 as u8;
            packet[3 as i32 as usize] = 0 as i32 as u8;
            jj = 4 as i32;
            while jj < 2 as i32 + j {
                packet[jj as usize] = 0 as i32 as u8;
                jj += 1;
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as i32 + j + 252 as i32 - 1 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    955 as i32,
                );
            }
            jj = 2 as i32;
            while jj < 2 as i32 + j {
                packet[jj as usize] = 0 as i32 as u8;
                jj += 1;
            }
            toc = -(1 as i32) as u8;
            frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
            frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
            payload_offset = -(1 as i32);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as i32 + j - 1 as i32,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if frame_samp_0 * j <= 5760 as i32 {
                if ret != j {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        962 as i32,
                    );
                }
                jj = 0 as i32;
                while jj < j {
                    if size[jj as usize] as i32 != 0 as i32 {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            963 as i32,
                        );
                    }
                    jj += 1;
                }
                if toc as i32 >> 2 as i32 != i {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        964 as i32,
                    );
                }
            } else if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    965 as i32,
                );
            }
            sz = 0 as i32;
            while sz < 8 as i32 {
                let tsz: [i32; 8] = [
                    50 as i32,
                    201 as i32,
                    403 as i32,
                    700 as i32,
                    1472 as i32,
                    5110 as i32,
                    20400 as i32,
                    61298 as i32,
                ];
                let mut pos: i32 = 0 as i32;
                let mut as_0: i32 = (tsz[sz as usize] + i - j - 2 as i32) / j;
                jj = 0 as i32;
                while jj < j - 1 as i32 {
                    if as_0 < 252 as i32 {
                        packet[(2 as i32 + pos) as usize] = as_0 as u8;
                        pos += 1;
                    } else {
                        packet[(2 as i32 + pos) as usize] = (252 as i32 + (as_0 & 3 as i32)) as u8;
                        packet[(3 as i32 + pos) as usize] = ((as_0 - 252 as i32) >> 2 as i32) as u8;
                        pos += 2 as i32;
                    }
                    jj += 1;
                }
                toc = -(1 as i32) as u8;
                frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
                frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
                payload_offset = -(1 as i32);
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    tsz[sz as usize] + i,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if frame_samp_0 * j <= 5760 as i32
                    && as_0 < 1276 as i32
                    && tsz[sz as usize] + i - 2 as i32 - pos - as_0 * (j - 1 as i32) < 1276 as i32
                {
                    if ret != j {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            981 as i32,
                        );
                    }
                    jj = 0 as i32;
                    while jj < j - 1 as i32 {
                        if size[jj as usize] as i32 != as_0 {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                982 as i32,
                            );
                        }
                        jj += 1;
                    }
                    if size[(j - 1 as i32) as usize] as i32
                        != tsz[sz as usize] + i - 2 as i32 - pos - as_0 * (j - 1 as i32)
                    {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            983 as i32,
                        );
                    }
                    if toc as i32 >> 2 as i32 != i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            984 as i32,
                        );
                    }
                } else if ret != -(4 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        985 as i32,
                    );
                }
                sz += 1;
            }
            j += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=1-48 VBR (%2d cases) ............. OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        packet[0 as i32 as usize] = ((i << 2 as i32) + 3 as i32) as u8;
        packet[1 as i32 as usize] = (128 as i32 + 1 as i32 + 64 as i32) as u8;
        jj = 2 as i32;
        while jj < 127 as i32 {
            packet[jj as usize] = 255 as i32 as u8;
            jj += 1;
        }
        toc = -(1 as i32) as u8;
        frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
        frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
        payload_offset = -(1 as i32);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            127 as i32,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as i32) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                1002 as i32,
            );
        }
        sz = 0 as i32;
        while sz < 4 as i32 {
            let tsz_0: [i32; 4] = [0 as i32, 72 as i32, 512 as i32, 1275 as i32];
            jj = sz;
            while jj < 65025 as i32 {
                let mut pos_0: i32 = 0;
                pos_0 = 0 as i32;
                while pos_0 < jj / 254 as i32 {
                    packet[(2 as i32 + pos_0) as usize] = 255 as i32 as u8;
                    pos_0 += 1;
                }
                packet[(2 as i32 + pos_0) as usize] = (jj % 254 as i32) as u8;
                pos_0 += 1;
                if sz == 0 as i32 && i == 63 as i32 {
                    toc = -(1 as i32) as u8;
                    frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
                    frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
                    payload_offset = -(1 as i32);
                    ret = opus_packet_parse(
                        packet.as_mut_ptr(),
                        2 as i32 + jj + pos_0 - 1 as i32,
                        &mut toc,
                        frames.as_mut_ptr(),
                        size.as_mut_ptr(),
                        &mut payload_offset,
                    );
                    cfgs += 1;
                    if ret != -(4 as i32) {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            1019 as i32,
                        );
                    }
                }
                toc = -(1 as i32) as u8;
                frames[0 as i32 as usize] = std::ptr::null_mut::<u8>();
                frames[1 as i32 as usize] = std::ptr::null_mut::<u8>();
                payload_offset = -(1 as i32);
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    2 as i32 + jj + tsz_0[sz as usize] + i + pos_0,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if tsz_0[sz as usize] + i < 1276 as i32 {
                    if ret != 1 as i32 {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            1026 as i32,
                        );
                    }
                    if size[0 as i32 as usize] as i32 != tsz_0[sz as usize] + i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            1027 as i32,
                        );
                    }
                    if toc as i32 >> 2 as i32 != i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                            1028 as i32,
                        );
                    }
                } else if ret != -(4 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1029 as i32,
                    );
                }
                jj += 11 as i32;
            }
            sz += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 padding (%2d cases) ............... OK.\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs_total += cfgs;
    fprintf(
        stdout(),
        b"    opus_packet_parse ............................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"                      All packet parsing tests passed\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"                          (%d API invocations)\n\0" as *const u8 as *const i8,
        cfgs_total,
    );
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
    cfgs = 0 as i32;
    fprintf(
        stdout(),
        b"\n  Encoder basic API tests\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8 as *const i8,
    );
    c = 0 as i32;
    while c < 4 as i32 {
        i = opus_encoder_get_size(c);
        if (c == 1 as i32 || c == 2 as i32) && (i <= 2048 as i32 || i > (1 as i32) << 17 as i32)
            || c != 1 as i32 && c != 2 as i32 && i != 0 as i32
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                1084 as i32,
            );
        }
        fprintf(
            stdout(),
            b"    opus_encoder_get_size(%d)=%d ...............%s OK.\n\0" as *const u8 as *const i8,
            c,
            i,
            if i > 0 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"....\0" as *const u8 as *const i8
            },
        );
        cfgs += 1;
        c += 1;
    }
    c = 0 as i32;
    while c < 4 as i32 {
        i = -(7 as i32);
        while i <= 96000 as i32 {
            let mut fs: i32 = 0;
            if !((i == 8000 as i32
                || i == 12000 as i32
                || i == 16000 as i32
                || i == 24000 as i32
                || i == 48000 as i32)
                && (c == 1 as i32 || c == 2 as i32))
            {
                match i {
                    -5 => {
                        fs = -(8000 as i32);
                    }
                    -6 => {
                        fs = 2147483647 as i32;
                    }
                    -7 => {
                        fs = -(2147483647 as i32) - 1 as i32;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0 as i32;
                enc = opus_encoder_create(fs, c, 2048 as i32, &mut err);
                if err != -(1 as i32) || !enc.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1106 as i32,
                    );
                }
                cfgs += 1;
                enc = opus_encoder_create(fs, c, 2048 as i32, std::ptr::null_mut::<i32>());
                if !enc.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1109 as i32,
                    );
                }
                cfgs += 1;
                opus_encoder_destroy(enc);
                enc = malloc(opus_encoder_get_size(2 as i32) as u64) as *mut OpusEncoder;
                if enc.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1113 as i32,
                    );
                }
                err = opus_encoder_init(enc, fs, c, 2048 as i32);
                if err != -(1 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1115 as i32,
                    );
                }
                cfgs += 1;
                free(enc as *mut core::ffi::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    enc = opus_encoder_create(
        48000 as i32,
        2 as i32,
        -(1000 as i32),
        std::ptr::null_mut::<i32>(),
    );
    if !enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1122 as i32,
        );
    }
    cfgs += 1;
    enc = opus_encoder_create(48000 as i32, 2 as i32, -(1000 as i32), &mut err);
    if err != -(1 as i32) || !enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1127 as i32,
        );
    }
    cfgs += 1;
    enc = opus_encoder_create(
        48000 as i32,
        2 as i32,
        2048 as i32,
        std::ptr::null_mut::<i32>(),
    );
    if enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1132 as i32,
        );
    }
    opus_encoder_destroy(enc);
    cfgs += 1;
    enc = opus_encoder_create(48000 as i32, 2 as i32, 2051 as i32, &mut err);
    if err != 0 as i32 || enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1138 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4027 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i < 0 as i32 || i > 32766 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1141 as i32,
        );
    }
    cfgs += 1;
    opus_encoder_destroy(enc);
    enc = opus_encoder_create(48000 as i32, 2 as i32, 2049 as i32, &mut err);
    if err != 0 as i32 || enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1147 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4027 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i < 0 as i32 || i > 32766 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1150 as i32,
        );
    }
    opus_encoder_destroy(enc);
    cfgs += 1;
    enc = opus_encoder_create(48000 as i32, 2 as i32, 2048 as i32, &mut err);
    if err != 0 as i32 || enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1156 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_encoder_create() ........................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_encoder_init() .......................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4027 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i < 0 as i32 || i > 32766 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1165 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4027 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1168 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_LOOKAHEAD ........................... OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_encoder_ctl!(
        enc,
        4029 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != 48000 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1173 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4029 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1176 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_SAMPLE_RATE ......................... OK.\n\0" as *const u8 as *const i8,
    );
    if opus_encoder_ctl!(enc, -(5 as i32)) != -(5 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1180 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_UNIMPLEMENTED ........................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4001 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1185 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4000 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1190 as i32,
        );
    }
    i = -(1000 as i32);
    if opus_encoder_ctl!(enc, 4000 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1190 as i32,
        );
    }
    i = 2049 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4000 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1190 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4001 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1190 as i32,
        );
    }
    i = 2051 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4000 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1190 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_APPLICATION ......................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4001 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1190 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_APPLICATION ......................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4003 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1193 as i32,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl!(enc, 4002 as i32, 1073741832 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1195 as i32,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl!(
        enc,
        4003 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    ) != 0 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1198 as i32,
        );
    }
    if i > 700000 as i32 || i < 256000 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1199 as i32,
        );
    }
    cfgs += 1;
    i = -(12345 as i32);
    if opus_encoder_ctl!(enc, 4002 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1204 as i32,
        );
    }
    i = 0 as i32;
    if opus_encoder_ctl!(enc, 4002 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1204 as i32,
        );
    }
    i = 500 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4002 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1204 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4003 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1204 as i32,
        );
    }
    i = 256000 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4002 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1204 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_BITRATE ............................. OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4003 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1204 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_BITRATE ............................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4023 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1207 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4022 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1212 as i32,
        );
    }
    i = 3 as i32;
    if opus_encoder_ctl!(enc, 4022 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1212 as i32,
        );
    }
    i = 1 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4022 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1212 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4023 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1212 as i32,
        );
    }
    i = -(1000 as i32);
    j = i;
    if opus_encoder_ctl!(enc, 4022 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1212 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_FORCE_CHANNELS ...................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4023 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1212 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_FORCE_CHANNELS ...................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    i = -(2 as i32);
    if opus_encoder_ctl!(enc, 4008 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1215 as i32,
        );
    }
    cfgs += 1;
    i = 1105 as i32 + 1 as i32;
    if opus_encoder_ctl!(enc, 4008 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1218 as i32,
        );
    }
    cfgs += 1;
    i = 1101 as i32;
    if opus_encoder_ctl!(enc, 4008 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1221 as i32,
        );
    }
    cfgs += 1;
    i = 1105 as i32;
    if opus_encoder_ctl!(enc, 4008 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1224 as i32,
        );
    }
    cfgs += 1;
    i = 1103 as i32;
    if opus_encoder_ctl!(enc, 4008 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1227 as i32,
        );
    }
    cfgs += 1;
    i = 1102 as i32;
    if opus_encoder_ctl!(enc, 4008 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1230 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_SET_BANDWIDTH ........................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4009 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32
        || i != 1101 as i32
            && i != 1102 as i32
            && i != 1103 as i32
            && i != 1105 as i32
            && i != -(1000 as i32)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1240 as i32,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl!(enc, 4008 as i32, -(1000 as i32)) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1242 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4009 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1245 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_BANDWIDTH ........................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(2 as i32);
    if opus_encoder_ctl!(enc, 4004 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1250 as i32,
        );
    }
    cfgs += 1;
    i = 1105 as i32 + 1 as i32;
    if opus_encoder_ctl!(enc, 4004 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1253 as i32,
        );
    }
    cfgs += 1;
    i = 1101 as i32;
    if opus_encoder_ctl!(enc, 4004 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1256 as i32,
        );
    }
    cfgs += 1;
    i = 1105 as i32;
    if opus_encoder_ctl!(enc, 4004 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1259 as i32,
        );
    }
    cfgs += 1;
    i = 1103 as i32;
    if opus_encoder_ctl!(enc, 4004 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1262 as i32,
        );
    }
    cfgs += 1;
    i = 1102 as i32;
    if opus_encoder_ctl!(enc, 4004 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1265 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_SET_MAX_BANDWIDTH ....................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4005 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32
        || i != 1101 as i32 && i != 1102 as i32 && i != 1103 as i32 && i != 1105 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1275 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4005 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1278 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_MAX_BANDWIDTH ....................... OK.\n\0" as *const u8 as *const i8,
    );
    err = opus_encoder_ctl!(
        enc,
        4017 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1283 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4016 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1288 as i32,
        );
    }
    i = 2 as i32;
    if opus_encoder_ctl!(enc, 4016 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1288 as i32,
        );
    }
    i = 1 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4016 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1288 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4017 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1288 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4016 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1288 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_DTX ................................. OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4017 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1288 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_DTX ................................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4011 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1291 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4010 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1296 as i32,
        );
    }
    i = 11 as i32;
    if opus_encoder_ctl!(enc, 4010 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1296 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4010 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1296 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4011 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1296 as i32,
        );
    }
    i = 10 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4010 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1296 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_COMPLEXITY .......................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4011 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1296 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_COMPLEXITY .......................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4013 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1299 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4012 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1304 as i32,
        );
    }
    i = 2 as i32;
    if opus_encoder_ctl!(enc, 4012 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1304 as i32,
        );
    }
    i = 1 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4012 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1304 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4013 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1304 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4012 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1304 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_INBAND_FEC .......................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4013 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1304 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_INBAND_FEC .......................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4015 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1307 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4014 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1312 as i32,
        );
    }
    i = 101 as i32;
    if opus_encoder_ctl!(enc, 4014 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1312 as i32,
        );
    }
    i = 100 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4014 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1312 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4015 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1312 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4014 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1312 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_PACKET_LOSS_PERC .................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4015 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1312 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_PACKET_LOSS_PERC .................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4007 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1315 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4006 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1320 as i32,
        );
    }
    i = 2 as i32;
    if opus_encoder_ctl!(enc, 4006 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1320 as i32,
        );
    }
    i = 1 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4006 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1320 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4007 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1320 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4006 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1320 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_VBR ................................. OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4007 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1320 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_VBR ................................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4021 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1331 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4020 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1336 as i32,
        );
    }
    i = 2 as i32;
    if opus_encoder_ctl!(enc, 4020 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1336 as i32,
        );
    }
    i = 1 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4020 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1336 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4021 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1336 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4020 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1336 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_VBR_CONSTRAINT ...................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4021 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1336 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_VBR_CONSTRAINT ...................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4025 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1339 as i32,
        );
    }
    cfgs += 1;
    i = -(12345 as i32);
    if opus_encoder_ctl!(enc, 4024 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1344 as i32,
        );
    }
    i = 0x7fffffff as i32;
    if opus_encoder_ctl!(enc, 4024 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1344 as i32,
        );
    }
    i = 3002 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4024 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1344 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4025 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1344 as i32,
        );
    }
    i = -(1000 as i32);
    j = i;
    if opus_encoder_ctl!(enc, 4024 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1344 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_SIGNAL .............................. OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4025 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1344 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_SIGNAL .............................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4037 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1347 as i32,
        );
    }
    cfgs += 1;
    i = 7 as i32;
    if opus_encoder_ctl!(enc, 4036 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1351 as i32,
        );
    }
    i = 25 as i32;
    if opus_encoder_ctl!(enc, 4036 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1351 as i32,
        );
    }
    i = 16 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4036 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1351 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4037 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1351 as i32,
        );
    }
    i = 24 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4036 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1351 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_LSB_DEPTH ........................... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4037 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1351 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_LSB_DEPTH ........................... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4043 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if i != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1354 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(
        enc,
        4043 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1357 as i32,
        );
    }
    cfgs += 1;
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4042 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1361 as i32,
        );
    }
    i = 2 as i32;
    if opus_encoder_ctl!(enc, 4042 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1361 as i32,
        );
    }
    i = 1 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4042 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1361 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4043 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1361 as i32,
        );
    }
    i = 0 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4042 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1361 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_PREDICTION_DISABLED ................. OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4043 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1361 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_PREDICTION_DISABLED ................. OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(
        enc,
        4041 as i32,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as i64 as isize),
    );
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1364 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5001 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1367 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5002 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1370 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5003 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1373 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5004 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1376 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5005 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1379 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5006 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1382 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5007 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1385 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5008 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1388 as i32,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl!(enc, 4040 as i32, 5009 as i32);
    if err != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1391 as i32,
        );
    }
    cfgs += 1;
    i = 0 as i32;
    if opus_encoder_ctl!(enc, 4040 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1396 as i32,
        );
    }
    i = -(1 as i32);
    if opus_encoder_ctl!(enc, 4040 as i32, i) == 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1396 as i32,
        );
    }
    i = 5006 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4040 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1396 as i32,
        );
    }
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4041 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1396 as i32,
        );
    }
    i = 5000 as i32;
    j = i;
    if opus_encoder_ctl!(enc, 4040 as i32, i) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1396 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_EXPERT_FRAME_DURATION ............... OK.\n\0" as *const u8 as *const i8,
    );
    i = -(12345 as i32);
    err = opus_encoder_ctl!(
        enc,
        4041 as i32,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as i64 as isize),
    );
    if err != 0 as i32 || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1396 as i32,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_EXPERT_FRAME_DURATION ............... OK.\n\0" as *const u8 as *const i8,
    );
    cfgs += 6 as i32;
    err = opus_encoder_ctl!(enc, 4031 as i32, null_uint_ptr as *mut u32);
    if err != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1401 as i32,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl!(
        enc,
        4031 as i32,
        (&mut enc_final_range as *mut u32).offset(
            (&mut enc_final_range as *mut u32).offset_from(&mut enc_final_range as *mut u32) as i64
                as isize,
        ),
    ) != 0 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1403 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_FINAL_RANGE ......................... OK.\n\0" as *const u8 as *const i8,
    );
    if opus_encoder_ctl!(enc, 4028 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1408 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_RESET_STATE ............................. OK.\n\0" as *const u8 as *const i8,
    );
    memset(
        sbuf.as_mut_ptr() as *mut core::ffi::c_void,
        0 as i32,
        (::core::mem::size_of::<libc::c_short>() as u64)
            .wrapping_mul(2 as i32 as u64)
            .wrapping_mul(960 as i32 as u64),
    );
    i = opus_encode(
        enc,
        sbuf.as_mut_ptr(),
        960 as i32,
        packet.as_mut_ptr(),
        ::core::mem::size_of::<[u8; 1276]>() as u64 as i32,
    );
    if i < 1 as i32 || i > ::core::mem::size_of::<[u8; 1276]>() as u64 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1415 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_encode() ................................ OK.\n\0" as *const u8 as *const i8,
    );
    memset(
        fbuf.as_mut_ptr() as *mut core::ffi::c_void,
        0 as i32,
        (::core::mem::size_of::<f32>() as u64)
            .wrapping_mul(2 as i32 as u64)
            .wrapping_mul(960 as i32 as u64),
    );
    i = opus_encode_float(
        enc,
        fbuf.as_mut_ptr(),
        960 as i32,
        packet.as_mut_ptr(),
        ::core::mem::size_of::<[u8; 1276]>() as u64 as i32,
    );
    if i < 1 as i32 || i > ::core::mem::size_of::<[u8; 1276]>() as u64 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1423 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_encode_float() .......................... OK.\n\0" as *const u8 as *const i8,
    );
    opus_encoder_destroy(enc);
    cfgs += 1;
    fprintf(
        stdout(),
        b"                   All encoder interface tests passed\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"                             (%d API invocations)\n\0" as *const u8 as *const i8,
        cfgs,
    );
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
    cfgs = 0 as i32;
    fprintf(
        stdout(),
        b"\n  Repacketizer tests\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8 as *const i8,
    );
    packet = malloc((1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32) as u64) as *mut u8;
    if packet.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1455 as i32,
        );
    }
    memset(
        packet as *mut core::ffi::c_void,
        0 as i32,
        (1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32) as u64,
    );
    po = malloc((1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32 + 256 as i32) as u64)
        as *mut u8;
    if po.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1458 as i32,
        );
    }
    i = opus_repacketizer_get_size();
    if i <= 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1461 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_get_size()=%d ............. OK.\n\0" as *const u8 as *const i8,
        i,
    );
    rp = malloc(i as u64) as *mut OpusRepacketizer;
    rp = opus_repacketizer_init(rp);
    if rp.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1467 as i32,
        );
    }
    cfgs += 1;
    free(rp as *mut core::ffi::c_void);
    fprintf(
        stdout(),
        b"    opus_repacketizer_init ....................... OK.\n\0" as *const u8 as *const i8,
    );
    rp = opus_repacketizer_create();
    if rp.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1473 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_create ..................... OK.\n\0" as *const u8 as *const i8,
    );
    if opus_repacketizer_get_nb_frames(rp) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1477 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_get_nb_frames .............. OK.\n\0" as *const u8 as *const i8,
    );
    if opus_repacketizer_cat(rp, packet, 0 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1483 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 1 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 2 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1486 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 2 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 1 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1489 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 3 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 1 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1492 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 2 as i32 as u8;
    *packet.offset(1 as i32 as isize) = 255 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 2 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1496 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 2 as i32 as u8;
    *packet.offset(1 as i32 as isize) = 250 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 251 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1500 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 3 as i32 as u8;
    *packet.offset(1 as i32 as isize) = 0 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 2 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1504 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(1 as i32 as isize) = 49 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 100 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1507 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 0 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 3 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1510 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = ((1 as i32) << 2 as i32) as u8;
    if opus_repacketizer_cat(rp, packet, 3 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1513 as i32,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    j = 0 as i32;
    while j < 32 as i32 {
        let mut maxi: i32 = 0;
        *packet.offset(0 as i32 as isize) = (((j << 1 as i32) + (j & 1 as i32)) << 2 as i32) as u8;
        maxi = 960 as i32 / opus_packet_get_samples_per_frame(packet, 8000 as i32);
        i = 1 as i32;
        while i <= maxi {
            let mut maxp: i32 = 0;
            *packet.offset(0 as i32 as isize) =
                (((j << 1 as i32) + (j & 1 as i32)) << 2 as i32) as u8;
            if i > 1 as i32 {
                let fresh0 = &mut (*packet.offset(0 as i32 as isize));
                *fresh0 = (*fresh0 as i32 + if i == 2 as i32 { 1 as i32 } else { 3 as i32 }) as u8;
            }
            *packet.offset(1 as i32 as isize) = (if i > 2 as i32 { i } else { 0 as i32 }) as u8;
            maxp = 960 as i32 / (i * opus_packet_get_samples_per_frame(packet, 8000 as i32));
            k = 0 as i32;
            while k <= 1275 as i32 + 75 as i32 {
                let mut cnt: i32 = 0;
                let mut rcnt: i32 = 0;
                if k % i == 0 as i32 {
                    cnt = 0 as i32;
                    while cnt < maxp + 2 as i32 {
                        if cnt > 0 as i32 {
                            ret = opus_repacketizer_cat(
                                rp,
                                packet,
                                k + (if i > 2 as i32 { 2 as i32 } else { 1 as i32 }),
                            );
                            if if cnt <= maxp && k <= 1275 as i32 * i {
                                (ret != 0 as i32) as i32
                            } else {
                                (ret != -(4 as i32)) as i32
                            } != 0
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1542 as i32,
                                );
                            }
                            cfgs += 1;
                        }
                        rcnt = if k <= 1275 as i32 * i {
                            if cnt < maxp {
                                cnt
                            } else {
                                maxp
                            }
                        } else {
                            0 as i32
                        };
                        if opus_repacketizer_get_nb_frames(rp) != rcnt * i {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                1546 as i32,
                            );
                        }
                        cfgs += 1;
                        ret = opus_repacketizer_out_range(
                            rp,
                            0 as i32,
                            rcnt * i,
                            po,
                            1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
                        );
                        if rcnt > 0 as i32 {
                            let mut len: i32 = 0;
                            len = k * rcnt
                                + (if rcnt * i > 2 as i32 {
                                    2 as i32
                                } else {
                                    1 as i32
                                });
                            if ret != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1553 as i32,
                                );
                            }
                            if rcnt * i < 2 as i32
                                && *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 0 as i32
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1554 as i32,
                                );
                            }
                            if rcnt * i == 2 as i32
                                && *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 1 as i32
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1555 as i32,
                                );
                            }
                            if rcnt * i > 2 as i32
                                && (*po.offset(0 as i32 as isize) as i32 & 3 as i32 != 3 as i32
                                    || *po.offset(1 as i32 as isize) as i32 != rcnt * i)
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1556 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_repacketizer_out(rp, po, len) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1558 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_unpad(po, len) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1560 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_pad(po, len, len + 1 as i32) != 0 as i32 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1562 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_pad(po, len + 1 as i32, len + 256 as i32) != 0 as i32 {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1564 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_unpad(po, len + 256 as i32) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1566 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_unpad(po, len, 1 as i32) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1568 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_pad(po, len, len + 1 as i32, 1 as i32)
                                != 0 as i32
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1570 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_pad(
                                po,
                                len + 1 as i32,
                                len + 256 as i32,
                                1 as i32,
                            ) != 0 as i32
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1572 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_unpad(po, len + 256 as i32, 1 as i32) != len
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1574 as i32,
                                );
                            }
                            cfgs += 1;
                            if opus_repacketizer_out(rp, po, len - 1 as i32) != -(2 as i32) {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1576 as i32,
                                );
                            }
                            cfgs += 1;
                            if len > 1 as i32 {
                                if opus_repacketizer_out(rp, po, 1 as i32) != -(2 as i32) {
                                    _test_failed(
                                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                        1580 as i32,
                                    );
                                }
                                cfgs += 1;
                            }
                            if opus_repacketizer_out(rp, po, 0 as i32) != -(2 as i32) {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                    1583 as i32,
                                );
                            }
                            cfgs += 1;
                        } else if ret != -(1 as i32) {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                                1585 as i32,
                            );
                        }
                        cnt += 1;
                    }
                    opus_repacketizer_init(rp);
                }
                k += 3 as i32;
            }
            i += 1;
        }
        j += 1;
    }
    opus_repacketizer_init(rp);
    *packet.offset(0 as i32 as isize) = 0 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 5 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1595 as i32,
        );
    }
    cfgs += 1;
    let fresh1 = &mut (*packet.offset(0 as i32 as isize));
    *fresh1 = (*fresh1 as i32 + 1 as i32) as u8;
    if opus_repacketizer_cat(rp, packet, 9 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1598 as i32,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
    );
    if i != 4 as i32 + 8 as i32 + 2 as i32
        || *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 3 as i32
        || *po.offset(1 as i32 as isize) as i32 & 63 as i32 != 3 as i32
        || *po.offset(1 as i32 as isize) as i32 >> 7 as i32 != 0 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1601 as i32,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out_range(
        rp,
        0 as i32,
        1 as i32,
        po,
        1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
    );
    if i != 5 as i32 || *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1604 as i32,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out_range(
        rp,
        1 as i32,
        2 as i32,
        po,
        1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
    );
    if i != 5 as i32 || *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1607 as i32,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as i32 as isize) = 1 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 9 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1613 as i32,
        );
    }
    cfgs += 1;
    *packet.offset(0 as i32 as isize) = 0 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 3 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1616 as i32,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
    );
    if i != 2 as i32 + 8 as i32 + 2 as i32 + 2 as i32
        || *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 3 as i32
        || *po.offset(1 as i32 as isize) as i32 & 63 as i32 != 3 as i32
        || *po.offset(1 as i32 as isize) as i32 >> 7 as i32 != 1 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1619 as i32,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as i32 as isize) = 2 as i32 as u8;
    *packet.offset(1 as i32 as isize) = 4 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 8 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1626 as i32,
        );
    }
    cfgs += 1;
    if opus_repacketizer_cat(rp, packet, 8 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1628 as i32,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
    );
    if i != 2 as i32 + 1 as i32 + 1 as i32 + 1 as i32 + 4 as i32 + 2 as i32 + 4 as i32 + 2 as i32
        || *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 3 as i32
        || *po.offset(1 as i32 as isize) as i32 & 63 as i32 != 4 as i32
        || *po.offset(1 as i32 as isize) as i32 >> 7 as i32 != 1 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1631 as i32,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as i32 as isize) = 2 as i32 as u8;
    *packet.offset(1 as i32 as isize) = 4 as i32 as u8;
    if opus_repacketizer_cat(rp, packet, 10 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1638 as i32,
        );
    }
    cfgs += 1;
    if opus_repacketizer_cat(rp, packet, 10 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1640 as i32,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
    );
    if i != 2 as i32 + 4 as i32 + 4 as i32 + 4 as i32 + 4 as i32
        || *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 3 as i32
        || *po.offset(1 as i32 as isize) as i32 & 63 as i32 != 4 as i32
        || *po.offset(1 as i32 as isize) as i32 >> 7 as i32 != 0 as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1643 as i32,
        );
    }
    cfgs += 1;
    j = 0 as i32;
    while j < 32 as i32 {
        let mut maxi_0: i32 = 0;
        let mut sum: i32 = 0;
        let mut rcnt_0: i32 = 0;
        *packet.offset(0 as i32 as isize) = (((j << 1 as i32) + (j & 1 as i32)) << 2 as i32) as u8;
        maxi_0 = 960 as i32 / opus_packet_get_samples_per_frame(packet, 8000 as i32);
        sum = 0 as i32;
        rcnt_0 = 0 as i32;
        opus_repacketizer_init(rp);
        i = 1 as i32;
        while i <= maxi_0 + 2 as i32 {
            let mut len_0: i32 = 0;
            ret = opus_repacketizer_cat(rp, packet, i);
            if rcnt_0 < maxi_0 {
                if ret != 0 as i32 {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1662 as i32,
                    );
                }
                rcnt_0 += 1;
                sum += i - 1 as i32;
            } else if ret != -(4 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1665 as i32,
                );
            }
            cfgs += 1;
            len_0 = sum
                + (if rcnt_0 < 2 as i32 {
                    1 as i32
                } else if rcnt_0 < 3 as i32 {
                    2 as i32
                } else {
                    2 as i32 + rcnt_0 - 1 as i32
                });
            if opus_repacketizer_out(
                rp,
                po,
                1276 as i32 * 48 as i32 + 48 as i32 * 2 as i32 + 2 as i32,
            ) != len_0
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1668 as i32,
                );
            }
            if rcnt_0 > 2 as i32 && *po.offset(1 as i32 as isize) as i32 & 63 as i32 != rcnt_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1669 as i32,
                );
            }
            if rcnt_0 == 2 as i32 && *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 2 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1670 as i32,
                );
            }
            if rcnt_0 == 1 as i32 && *po.offset(0 as i32 as isize) as i32 & 3 as i32 != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1671 as i32,
                );
            }
            cfgs += 1;
            if opus_repacketizer_out(rp, po, len_0) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1673 as i32,
                );
            }
            cfgs += 1;
            if opus_packet_unpad(po, len_0) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1675 as i32,
                );
            }
            cfgs += 1;
            if opus_packet_pad(po, len_0, len_0 + 1 as i32) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1677 as i32,
                );
            }
            cfgs += 1;
            if opus_packet_pad(po, len_0 + 1 as i32, len_0 + 256 as i32) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1679 as i32,
                );
            }
            cfgs += 1;
            if opus_packet_unpad(po, len_0 + 256 as i32) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1681 as i32,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_unpad(po, len_0, 1 as i32) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1683 as i32,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_pad(po, len_0, len_0 + 1 as i32, 1 as i32) != 0 as i32 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1685 as i32,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_pad(po, len_0 + 1 as i32, len_0 + 256 as i32, 1 as i32)
                != 0 as i32
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1687 as i32,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_unpad(po, len_0 + 256 as i32, 1 as i32) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1689 as i32,
                );
            }
            cfgs += 1;
            if opus_repacketizer_out(rp, po, len_0 - 1 as i32) != -(2 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1691 as i32,
                );
            }
            cfgs += 1;
            if len_0 > 1 as i32 {
                if opus_repacketizer_out(rp, po, 1 as i32) != -(2 as i32) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                        1695 as i32,
                    );
                }
                cfgs += 1;
            }
            if opus_repacketizer_out(rp, po, 0 as i32) != -(2 as i32) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const i8,
                    1698 as i32,
                );
            }
            cfgs += 1;
            i += 1;
        }
        j += 1;
    }
    *po.offset(0 as i32 as isize) = 'O' as i32 as u8;
    *po.offset(1 as i32 as isize) = 'p' as i32 as u8;
    if opus_packet_pad(po, 4 as i32, 4 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1705 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 4 as i32, 4 as i32, 1 as i32) != 0 as i32 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1707 as i32,
        );
    }
    cfgs += 1;
    if opus_packet_pad(po, 4 as i32, 5 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1709 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 4 as i32, 5 as i32, 1 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1711 as i32,
        );
    }
    cfgs += 1;
    if opus_packet_pad(po, 0 as i32, 5 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1713 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 0 as i32, 5 as i32, 1 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1715 as i32,
        );
    }
    cfgs += 1;
    if opus_packet_unpad(po, 0 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1717 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_unpad(po, 0 as i32, 1 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1719 as i32,
        );
    }
    cfgs += 1;
    if opus_packet_unpad(po, 4 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1721 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_unpad(po, 4 as i32, 1 as i32) != -(4 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1723 as i32,
        );
    }
    cfgs += 1;
    *po.offset(0 as i32 as isize) = 0 as i32 as u8;
    *po.offset(1 as i32 as isize) = 0 as i32 as u8;
    *po.offset(2 as i32 as isize) = 0 as i32 as u8;
    if opus_packet_pad(po, 5 as i32, 4 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1728 as i32,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 5 as i32, 4 as i32, 1 as i32) != -(1 as i32) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1730 as i32,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_cat ........................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_repacketizer_out ........................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_repacketizer_out_range .................. OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_packet_pad .............................. OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_packet_unpad ............................ OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_packet_pad .................. OK.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_packet_unpad ................ OK.\n\0" as *const u8 as *const i8,
    );
    opus_repacketizer_destroy(rp);
    cfgs += 1;
    free(packet as *mut core::ffi::c_void);
    free(po as *mut core::ffi::c_void);
    fprintf(
        stdout(),
        b"                        All repacketizer tests passed\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"                            (%7d API invocations)\n\0" as *const u8 as *const i8,
        cfgs,
    );
    cfgs
}
pub unsafe fn test_malloc_fail() -> i32 {
    fprintf(
        stdout(),
        b"\n  malloc() failure tests\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_decoder_create() ................... SKIPPED.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_encoder_create() ................... SKIPPED.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_repacketizer_create() .............. SKIPPED.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_decoder_create() ....... SKIPPED.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_encoder_create() ....... SKIPPED.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stdout(),
        b"(Test only supported with GLIBC and without valgrind)\n\0" as *const u8 as *const i8,
    );
    0 as i32
}
unsafe fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
    let mut total: i32 = 0;
    let mut oversion: *const i8 = std::ptr::null::<i8>();
    if _argc > 1 as i32 {
        fprintf(
            stderr(),
            b"Usage: %s\n\0" as *const u8 as *const i8,
            *_argv.offset(0 as i32 as isize),
        );
        return 1 as i32;
    }
    iseed = 0 as i32 as u32;
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1887 as i32,
        );
    }
    fprintf(
        stderr(),
        b"Testing the %s API deterministically\n\0" as *const u8 as *const i8,
        oversion,
    );
    if (opus_strerror(-(32768 as i32))).is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1889 as i32,
        );
    }
    if (opus_strerror(32767 as i32)).is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1890 as i32,
        );
    }
    if strlen(opus_strerror(0 as i32)) < 1 as i32 as u64 {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const i8,
            1891 as i32,
        );
    }
    total = 4 as i32;
    total += test_dec_api();
    total += test_msdec_api();
    total += test_parse();
    total += test_enc_api();
    total += test_repacketizer_api();
    total += test_malloc_fail();
    fprintf(
        stderr(),
        b"\nAll API tests passed.\nThe libopus API was invoked %d times.\n\0" as *const u8
            as *const i8,
        total,
    );
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
