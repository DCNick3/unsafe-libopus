#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(register_tool)]
#![register_tool(c2rust)]

use ::libc;
use libc::{fprintf, printf};
use libc_stdhandle::{stderr, stdout};

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/tests/test_opus_common.h:55"]
pub mod test_opus_common_h {
    #[c2rust::src_loc = "63:20"]
    pub static mut iseed: u32 = 0;
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe fn _test_failed(mut file: *const libc::c_char, mut line: libc::c_int) -> ! {
        fprintf(
            stderr(),
            b"\n ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b" ***         A fatal error was detected.         ***\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b" ***************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"Please report this failure and include\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"'make check SEED=%u fails %s at line %d for %s'\n\0" as *const u8
                as *const libc::c_char,
            iseed,
            file,
            line,
            opus_get_version_string(),
        );
        fprintf(
            stderr(),
            b"and any relevant details about your system.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }

    use libc::{abort, fprintf};
    use libc_stdhandle::stderr;
    use libopus_unsafe::opus_get_version_string;
}

pub use self::test_opus_common_h::{_test_failed, iseed};

use libopus_unsafe::externs::{free, malloc};
use libopus_unsafe::externs::{memcmp, memcpy, memset, strlen};
use libopus_unsafe::{
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

#[c2rust::src_loc = "81:13"]
pub static mut null_int_ptr: *mut i32 = 0 as *const libc::c_void as *mut libc::c_void as *mut i32;
#[c2rust::src_loc = "82:14"]
pub static mut null_uint_ptr: *mut u32 = 0 as *const libc::c_void as *mut libc::c_void as *mut u32;
#[c2rust::src_loc = "84:25"]
static mut opus_rates: [i32; 5] = [
    48000 as libc::c_int,
    24000 as libc::c_int,
    16000 as libc::c_int,
    12000 as libc::c_int,
    8000 as libc::c_int,
];
#[c2rust::src_loc = "86:1"]
pub unsafe fn test_dec_api() -> i32 {
    let mut dec_final_range: u32 = 0;
    let mut dec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut dec2: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cfgs: i32 = 0;
    let mut packet: [libc::c_uchar; 1276] = [0; 1276];
    let mut fbuf: [libc::c_float; 1920] = [0.; 1920];
    let mut sbuf: [libc::c_short; 1920] = [0; 1920];
    let mut c: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    cfgs = 0 as libc::c_int;
    fprintf(
        stdout(),
        b"\n  Decoder basic API tests\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    c = 0 as libc::c_int;
    while c < 4 as libc::c_int {
        i = opus_decoder_get_size(c);
        if (c == 1 as libc::c_int || c == 2 as libc::c_int)
            && (i <= 2048 as libc::c_int || i > (1 as libc::c_int) << 16 as libc::c_int)
            || c != 1 as libc::c_int && c != 2 as libc::c_int && i != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int,
            );
        }
        fprintf(
            stdout(),
            b"    opus_decoder_get_size(%d)=%d ...............%s OK.\n\0" as *const u8
                as *const libc::c_char,
            c,
            i,
            if i > 0 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"....\0" as *const u8 as *const libc::c_char
            },
        );
        cfgs += 1;
        c += 1;
    }
    c = 0 as libc::c_int;
    while c < 4 as libc::c_int {
        i = -(7 as libc::c_int);
        while i <= 96000 as libc::c_int {
            let mut fs: libc::c_int = 0;
            if !((i == 8000 as libc::c_int
                || i == 12000 as libc::c_int
                || i == 16000 as libc::c_int
                || i == 24000 as libc::c_int
                || i == 48000 as libc::c_int)
                && (c == 1 as libc::c_int || c == 2 as libc::c_int))
            {
                match i {
                    -5 => {
                        fs = -(8000 as libc::c_int);
                    }
                    -6 => {
                        fs = 2147483647 as libc::c_int;
                    }
                    -7 => {
                        fs = -(2147483647 as libc::c_int) - 1 as libc::c_int;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0 as libc::c_int;
                dec = opus_decoder_create(fs, c, &mut err);
                if err != -(1 as libc::c_int) || !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        128 as libc::c_int,
                    );
                }
                cfgs += 1;
                dec = opus_decoder_create(fs, c, std::ptr::null_mut::<libc::c_int>());
                if !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        131 as libc::c_int,
                    );
                }
                cfgs += 1;
                dec = malloc(opus_decoder_get_size(2 as libc::c_int) as libc::c_ulong)
                    as *mut OpusDecoder;
                if dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        134 as libc::c_int,
                    );
                }
                err = opus_decoder_init(dec, fs, c);
                if err != -(1 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        136 as libc::c_int,
                    );
                }
                cfgs += 1;
                free(dec as *mut libc::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    dec = opus_decoder_create(48000 as libc::c_int, 2 as libc::c_int, &mut err);
    if err != 0 as libc::c_int || dec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_decoder_create() ........................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_decoder_init() .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_decoder_ctl!(dec, 4031 as libc::c_int, null_uint_ptr as *mut u32);
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4031 as libc::c_int,
        (&mut dec_final_range as *mut u32).offset(
            (&mut dec_final_range as *mut u32).offset_from(&mut dec_final_range as *mut u32)
                as libc::c_long as isize,
        )
    );
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_FINAL_RANGE ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(dec, -(5 as libc::c_int));
    if err != -(5 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_UNIMPLEMENTED ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4009 as libc::c_int, null_uint_ptr as *mut u32);
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4009 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_BANDWIDTH ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4029 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize)
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4029 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i != 48000 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_SAMPLE_RATE ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize)
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i > 0 as libc::c_int || i < -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
        );
    }
    cfgs += 1;
    packet[0 as libc::c_int as usize] = ((63 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
    packet[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    packet[1 as libc::c_int as usize] = packet[2 as libc::c_int as usize];
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i > 0 as libc::c_int || i < -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
        );
    }
    cfgs += 1;
    packet[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        1 as libc::c_int,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4033 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i > 0 as libc::c_int || i < -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_PITCH ............................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_decoder_ctl!(
        dec,
        4039 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize)
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
        );
    }
    err = opus_decoder_ctl!(
        dec,
        4039 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i != 960 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_LAST_PACKET_DURATION ................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_decoder_ctl!(
        dec,
        4045 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4045 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize)
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4034 as libc::c_int, -(32769 as libc::c_int));
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4034 as libc::c_int, 32768 as libc::c_int);
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(dec, 4034 as libc::c_int, -(15 as libc::c_int));
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_decoder_ctl!(
        dec,
        4045 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize)
    );
    if err != 0 as libc::c_int || i != -(15 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_SET_GAIN ................................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    OPUS_GET_GAIN ................................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    dec2 = malloc(opus_decoder_get_size(2 as libc::c_int) as libc::c_ulong) as *mut OpusDecoder;
    memcpy(
        dec2 as *mut libc::c_void,
        dec as *const libc::c_void,
        opus_decoder_get_size(2 as libc::c_int) as libc::c_ulong,
    );
    if opus_decoder_ctl!(dec, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
        );
    }
    if memcmp(
        dec2 as *const libc::c_void,
        dec as *const libc::c_void,
        opus_decoder_get_size(2 as libc::c_int) as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
        );
    }
    free(dec2 as *mut libc::c_void);
    fprintf(
        stdout(),
        b"    OPUS_RESET_STATE ............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    packet[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    if opus_decoder_get_nb_samples(
        dec,
        packet.as_mut_ptr() as *const libc::c_uchar,
        1 as libc::c_int,
    ) != 480 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int,
        );
    }
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        1 as libc::c_int,
        48000 as libc::c_int,
    ) != 480 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
        );
    }
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        1 as libc::c_int,
        96000 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
        );
    }
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        1 as libc::c_int,
        32000 as libc::c_int,
    ) != 320 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
        );
    }
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        1 as libc::c_int,
        8000 as libc::c_int,
    ) != 80 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
        );
    }
    packet[0 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        1 as libc::c_int,
        24000 as libc::c_int,
    ) != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
        );
    }
    packet[0 as libc::c_int as usize] =
        ((63 as libc::c_int) << 2 as libc::c_int | 3 as libc::c_int) as libc::c_uchar;
    packet[1 as libc::c_int as usize] = 63 as libc::c_int as libc::c_uchar;
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        0 as libc::c_int,
        24000 as libc::c_int,
    ) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            259 as libc::c_int,
        );
    }
    if opus_packet_get_nb_samples(
        packet.as_mut_ptr() as *const libc::c_uchar,
        2 as libc::c_int,
        48000 as libc::c_int,
    ) != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
        );
    }
    if opus_decoder_get_nb_samples(
        dec,
        packet.as_mut_ptr() as *const libc::c_uchar,
        2 as libc::c_int,
    ) != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    opus_{packet,decoder}_get_nb_samples() ....... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 9 as libc::c_int;
    if -(1 as libc::c_int)
        != opus_packet_get_nb_frames(
            packet.as_mut_ptr() as *const libc::c_uchar,
            0 as libc::c_int,
        )
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut l1res: [libc::c_int; 4] = [
            1 as libc::c_int,
            2 as libc::c_int,
            2 as libc::c_int,
            -(4 as libc::c_int),
        ];
        packet[0 as libc::c_int as usize] = i as libc::c_uchar;
        if l1res[(packet[0 as libc::c_int as usize] as libc::c_int & 3 as libc::c_int) as usize]
            != opus_packet_get_nb_frames(
                packet.as_mut_ptr() as *const libc::c_uchar,
                1 as libc::c_int,
            )
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                269 as libc::c_int,
            );
        }
        cfgs += 1;
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            packet[1 as libc::c_int as usize] = j as libc::c_uchar;
            if (if packet[0 as libc::c_int as usize] as libc::c_int & 3 as libc::c_int
                != 3 as libc::c_int
            {
                l1res
                    [(packet[0 as libc::c_int as usize] as libc::c_int & 3 as libc::c_int) as usize]
            } else {
                packet[1 as libc::c_int as usize] as libc::c_int & 63 as libc::c_int
            }) != opus_packet_get_nb_frames(
                packet.as_mut_ptr() as *const libc::c_uchar,
                2 as libc::c_int,
            ) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    273 as libc::c_int,
                );
            }
            cfgs += 1;
            j += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    opus_packet_get_nb_frames() .................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut bw: libc::c_int = 0;
        packet[0 as libc::c_int as usize] = i as libc::c_uchar;
        bw = packet[0 as libc::c_int as usize] as libc::c_int >> 4 as libc::c_int;
        bw = 1101 as libc::c_int
            + (((((bw & 7 as libc::c_int) * 9 as libc::c_int)
                & (63 as libc::c_int - (bw & 8 as libc::c_int)))
                + 2 as libc::c_int
                + 12 as libc::c_int * (bw & 8 as libc::c_int != 0 as libc::c_int) as libc::c_int)
                >> 4 as libc::c_int);
        if bw != opus_packet_get_bandwidth(packet.as_mut_ptr()) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                284 as libc::c_int,
            );
        }
        cfgs += 1;
        i += 1;
    }
    fprintf(
        stdout(),
        b"    opus_packet_get_bandwidth() .................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut fp3s: libc::c_int = 0;
        let mut rate: libc::c_int = 0;
        packet[0 as libc::c_int as usize] = i as libc::c_uchar;
        fp3s = packet[0 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int;
        fp3s = (((((3 as libc::c_int - (fp3s & 3 as libc::c_int)) * 13 as libc::c_int)
            & 119 as libc::c_int)
            + 9 as libc::c_int)
            >> 2 as libc::c_int)
            * ((fp3s > 13 as libc::c_int) as libc::c_int
                * (3 as libc::c_int
                    - (fp3s & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int)
                + 1 as libc::c_int)
            * 25 as libc::c_int;
        rate = 0 as libc::c_int;
        while rate < 5 as libc::c_int {
            if opus_rates[rate as usize] * 3 as libc::c_int / fp3s
                != opus_packet_get_samples_per_frame(packet.as_mut_ptr(), opus_rates[rate as usize])
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    295 as libc::c_int,
                );
            }
            cfgs += 1;
            rate += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    opus_packet_get_samples_per_frame() .......... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    packet[0 as libc::c_int as usize] =
        (((63 as libc::c_int) << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
    packet[1 as libc::c_int as usize] = 49 as libc::c_int as libc::c_uchar;
    j = 2 as libc::c_int;
    while j < 51 as libc::c_int {
        packet[j as usize] = 0 as libc::c_int as libc::c_uchar;
        j += 1;
    }
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        51 as libc::c_int,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int,
        );
    }
    cfgs += 1;
    packet[0 as libc::c_int as usize] = ((63 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
    packet[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    packet[1 as libc::c_int as usize] = packet[2 as libc::c_int as usize];
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        -(1 as libc::c_int),
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        60 as libc::c_int,
        0 as libc::c_int,
    ) != -(2 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        480 as libc::c_int,
        0 as libc::c_int,
    ) != -(2 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_decode() ................................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if opus_decode_float(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        fbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_decode_float() .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    opus_decoder_destroy(dec);
    cfgs += 1;
    fprintf(
        stdout(),
        b"                   All decoder interface tests passed\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"                             (%6d API invocations)\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs
}
#[c2rust::src_loc = "343:1"]
pub unsafe fn test_msdec_api() -> i32 {
    let mut dec_final_range: u32 = 0;
    let mut dec: *mut OpusMSDecoder = std::ptr::null_mut::<OpusMSDecoder>();
    let mut streamdec: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cfgs: i32 = 0;
    let mut packet: [libc::c_uchar; 1276] = [0; 1276];
    let mut mapping: [libc::c_uchar; 256] = [0; 256];
    let mut fbuf: [libc::c_float; 1920] = [0.; 1920];
    let mut sbuf: [libc::c_short; 1920] = [0; 1920];
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    mapping[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    mapping[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    i = 2 as libc::c_int;
    while i < 256 as libc::c_int {
        i += 1;
    }
    cfgs = 0 as libc::c_int;
    fprintf(
        stdout(),
        b"\n  Multistream decoder basic API tests\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    a = -(1 as libc::c_int);
    while a < 4 as libc::c_int {
        b = -(1 as libc::c_int);
        while b < 4 as libc::c_int {
            i = opus_multistream_decoder_get_size(a, b);
            if a > 0 as libc::c_int
                && b <= a
                && b >= 0 as libc::c_int
                && (i <= 2048 as libc::c_int || i > ((1 as libc::c_int) << 16 as libc::c_int) * a)
                || (a < 1 as libc::c_int || b > a || b < 0 as libc::c_int) && i != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    370 as libc::c_int,
                );
            }
            fprintf(
                stdout(),
                b"    opus_multistream_decoder_get_size(%2d,%2d)=%d %sOK.\n\0" as *const u8
                    as *const libc::c_char,
                a,
                b,
                i,
                if i > 0 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"... \0" as *const u8 as *const libc::c_char
                },
            );
            cfgs += 1;
            b += 1;
        }
        a += 1;
    }
    c = 1 as libc::c_int;
    while c < 3 as libc::c_int {
        i = -(7 as libc::c_int);
        while i <= 96000 as libc::c_int {
            let mut fs: libc::c_int = 0;
            if !((i == 8000 as libc::c_int
                || i == 12000 as libc::c_int
                || i == 16000 as libc::c_int
                || i == 24000 as libc::c_int
                || i == 48000 as libc::c_int)
                && (c == 1 as libc::c_int || c == 2 as libc::c_int))
            {
                match i {
                    -5 => {
                        fs = -(8000 as libc::c_int);
                    }
                    -6 => {
                        fs = 2147483647 as libc::c_int;
                    }
                    -7 => {
                        fs = -(2147483647 as libc::c_int) - 1 as libc::c_int;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0 as libc::c_int;
                dec = opus_multistream_decoder_create(
                    fs,
                    c,
                    1 as libc::c_int,
                    c - 1 as libc::c_int,
                    mapping.as_mut_ptr(),
                    &mut err,
                );
                if err != -(1 as libc::c_int) || !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        393 as libc::c_int,
                    );
                }
                cfgs += 1;
                dec = opus_multistream_decoder_create(
                    fs,
                    c,
                    1 as libc::c_int,
                    c - 1 as libc::c_int,
                    mapping.as_mut_ptr(),
                    std::ptr::null_mut::<libc::c_int>(),
                );
                if !dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        396 as libc::c_int,
                    );
                }
                cfgs += 1;
                dec = malloc(
                    opus_multistream_decoder_get_size(1 as libc::c_int, 1 as libc::c_int)
                        as libc::c_ulong,
                ) as *mut OpusMSDecoder;
                if dec.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        399 as libc::c_int,
                    );
                }
                err = opus_multistream_decoder_init(
                    dec,
                    fs,
                    c,
                    1 as libc::c_int,
                    c - 1 as libc::c_int,
                    mapping.as_mut_ptr(),
                );
                if err != -(1 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        401 as libc::c_int,
                    );
                }
                cfgs += 1;
                free(dec as *mut libc::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    c = 0 as libc::c_int;
    while c < 2 as libc::c_int {
        let mut ret_err: *mut libc::c_int = std::ptr::null_mut::<libc::c_int>();
        ret_err = if c != 0 {
            std::ptr::null_mut::<libc::c_int>()
        } else {
            &mut err
        };
        mapping[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        mapping[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        i = 2 as libc::c_int;
        while i < 256 as libc::c_int {
            i += 1;
        }
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                419 as libc::c_int,
            );
        }
        cfgs += 1;
        mapping[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        mapping[0 as libc::c_int as usize] = mapping[1 as libc::c_int as usize];
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as libc::c_int || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                426 as libc::c_int,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            1 as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as libc::c_int || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                434 as libc::c_int,
            );
        }
        cfgs += 1;
        err = opus_multistream_decoder_init(
            dec,
            48000 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
        );
        if err != -(1 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                438 as libc::c_int,
            );
        }
        cfgs += 1;
        err = opus_multistream_decoder_init(
            dec,
            48000 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            -(1 as libc::c_int),
            mapping.as_mut_ptr(),
        );
        if err != -(1 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as libc::c_int || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                451 as libc::c_int,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            255 as libc::c_int,
            255 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                459 as libc::c_int,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            -(1 as libc::c_int),
            1 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                465 as libc::c_int,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                471 as libc::c_int,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            1 as libc::c_int,
            -(1 as libc::c_int),
            2 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                477 as libc::c_int,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            1 as libc::c_int,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                483 as libc::c_int,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            256 as libc::c_int,
            255 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int,
            );
        }
        cfgs += 1;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            256 as libc::c_int,
            255 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                495 as libc::c_int,
            );
        }
        cfgs += 1;
        mapping[0 as libc::c_int as usize] = 255 as libc::c_int as libc::c_uchar;
        mapping[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        mapping[2 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            3 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                504 as libc::c_int,
            );
        }
        cfgs += 1;
        mapping[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        mapping[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        mapping[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        dec = opus_multistream_decoder_create(
            48000 as libc::c_int,
            3 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != 0 as libc::c_int || dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                513 as libc::c_int,
            );
        }
        cfgs += 1;
        opus_multistream_decoder_destroy(dec);
        cfgs += 1;
        mapping[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        mapping[1 as libc::c_int as usize] = 255 as libc::c_int as libc::c_uchar;
        mapping[2 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        mapping[3 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
        mapping[4 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
        dec = opus_multistream_decoder_create(
            48001 as libc::c_int,
            5 as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
            mapping.as_mut_ptr(),
            ret_err,
        );
        let _ = ret_err.is_null();
        if !ret_err.is_null() && *ret_err != -(1 as libc::c_int) || !dec.is_null() {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                526 as libc::c_int,
            );
        }
        cfgs += 1;
        c += 1;
    }
    mapping[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    mapping[1 as libc::c_int as usize] = 255 as libc::c_int as libc::c_uchar;
    mapping[2 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    mapping[3 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    dec = opus_multistream_decoder_create(
        48000 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as libc::c_int || dec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_multistream_decoder_create() ............ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_decoder_init() .............. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_multistream_decoder_ctl(
        dec,
        4031 as libc::c_int,
        (&mut dec_final_range as *mut u32).offset(
            (&mut dec_final_range as *mut u32).offset_from(&mut dec_final_range as *mut u32)
                as libc::c_long as isize,
        ),
    );
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_FINAL_RANGE ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    streamdec = std::ptr::null_mut::<OpusDecoder>();
    err = opus_multistream_decoder_ctl(
        dec,
        5122 as libc::c_int,
        -(1 as libc::c_int),
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as libc::c_long
                as isize,
        ),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            553 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl(
        dec,
        5122 as libc::c_int,
        1 as libc::c_int,
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as libc::c_long
                as isize,
        ),
    );
    if err != 0 as libc::c_int || streamdec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl(
        dec,
        5122 as libc::c_int,
        2 as libc::c_int,
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as libc::c_long
                as isize,
        ),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_multistream_decoder_ctl(
        dec,
        5122 as libc::c_int,
        0 as libc::c_int,
        (&mut streamdec as *mut *mut OpusDecoder).offset(
            (&mut streamdec as *mut *mut OpusDecoder)
                .offset_from(&mut streamdec as *mut *mut OpusDecoder) as libc::c_long
                as isize,
        ),
    );
    if err != 0 as libc::c_int || streamdec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            563 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_MULTISTREAM_GET_DECODER_STATE ........... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        let mut od: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        err = opus_multistream_decoder_ctl(
            dec,
            5122 as libc::c_int,
            j,
            (&mut od as *mut *mut OpusDecoder).offset(
                (&mut od as *mut *mut OpusDecoder).offset_from(&mut od as *mut *mut OpusDecoder)
                    as libc::c_long as isize,
            ),
        );
        if err != 0 as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                572 as libc::c_int,
            );
        }
        err = opus_decoder_ctl!(
            od,
            4045 as libc::c_int,
            (&mut i as *mut i32).offset(
                (&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize
            )
        );
        if err != 0 as libc::c_int || i != 0 as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                576 as libc::c_int,
            );
        }
        cfgs += 1;
        j += 1;
    }
    err = opus_multistream_decoder_ctl(dec, 4034 as libc::c_int, 15 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            580 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_GAIN ................................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        let mut od_0: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        err = opus_multistream_decoder_ctl(
            dec,
            5122 as libc::c_int,
            j,
            (&mut od_0 as *mut *mut OpusDecoder).offset(
                (&mut od_0 as *mut *mut OpusDecoder).offset_from(&mut od_0 as *mut *mut OpusDecoder)
                    as libc::c_long as isize,
            ),
        );
        if err != 0 as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                586 as libc::c_int,
            );
        }
        err = opus_decoder_ctl!(
            od_0,
            4045 as libc::c_int,
            (&mut i as *mut i32).offset(
                (&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize
            )
        );
        if err != 0 as libc::c_int || i != 15 as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                590 as libc::c_int,
            );
        }
        cfgs += 1;
        j += 1;
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_GAIN ................................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_multistream_decoder_ctl(
        dec,
        4009 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            597 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_BANDWIDTH ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    err = opus_multistream_decoder_ctl(dec, -(5 as libc::c_int));
    if err != -(5 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            602 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_UNIMPLEMENTED ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    if opus_multistream_decoder_ctl(dec, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            635 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_RESET_STATE ............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    opus_multistream_decoder_destroy(dec);
    cfgs += 1;
    dec = opus_multistream_decoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        mapping.as_mut_ptr(),
        &mut err,
    );
    if err != 0 as libc::c_int || dec.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            643 as libc::c_int,
        );
    }
    cfgs += 1;
    packet[0 as libc::c_int as usize] =
        (((63 as libc::c_int) << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
    packet[1 as libc::c_int as usize] = 49 as libc::c_int as libc::c_uchar;
    j = 2 as libc::c_int;
    while j < 51 as libc::c_int {
        packet[j as usize] = 0 as libc::c_int as libc::c_uchar;
        j += 1;
    }
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        51 as libc::c_int,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            650 as libc::c_int,
        );
    }
    cfgs += 1;
    packet[0 as libc::c_int as usize] = ((63 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
    packet[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    packet[1 as libc::c_int as usize] = packet[2 as libc::c_int as usize];
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        -(1 as libc::c_int),
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != -(1 as libc::c_int)
    {
        printf(
            b"%d\n\0" as *const u8 as *const libc::c_char,
            opus_multistream_decode(
                dec,
                packet.as_mut_ptr(),
                -(1 as libc::c_int),
                sbuf.as_mut_ptr(),
                960 as libc::c_int,
                0 as libc::c_int,
            ),
        );
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        -(960 as libc::c_int),
        0 as libc::c_int,
    ) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            656 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        60 as libc::c_int,
        0 as libc::c_int,
    ) != -(2 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            658 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        480 as libc::c_int,
        0 as libc::c_int,
    ) != -(2 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            660 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_decode(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            662 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_multistream_decode() .................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if opus_multistream_decode_float(
        dec,
        packet.as_mut_ptr(),
        3 as libc::c_int,
        fbuf.as_mut_ptr(),
        960 as libc::c_int,
        0 as libc::c_int,
    ) != 960 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            667 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_multistream_decode_float() .............. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    opus_multistream_decoder_destroy(dec);
    cfgs += 1;
    fprintf(
        stdout(),
        b"       All multistream decoder interface tests passed\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"                             (%6d API invocations)\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs
}
#[c2rust::src_loc = "707:1"]
pub unsafe fn test_parse() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut sz: i32 = 0;
    let mut packet: [libc::c_uchar; 1276] = [0; 1276];
    let mut cfgs: i32 = 0;
    let mut cfgs_total: i32 = 0;
    let mut toc: libc::c_uchar = 0;
    let mut frames: [*const libc::c_uchar; 48] = [std::ptr::null::<libc::c_uchar>(); 48];
    let mut size: [libc::c_short; 48] = [0; 48];
    let mut payload_offset: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    fprintf(
        stdout(),
        b"\n  Packet header parsing tests\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(1276 as libc::c_int as libc::c_ulong),
    );
    packet[0 as libc::c_int as usize] = ((63 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
    if opus_packet_parse(
        packet.as_mut_ptr(),
        1 as libc::c_int,
        &mut toc,
        frames.as_mut_ptr(),
        std::ptr::null_mut::<i16>(),
        &mut payload_offset,
    ) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            720 as libc::c_int,
        );
    }
    cfgs = 1 as libc::c_int;
    cfgs_total = cfgs;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] = (i << 2 as libc::c_int) as libc::c_uchar;
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            4 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != 1 as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                729 as libc::c_int,
            );
        }
        if size[0 as libc::c_int as usize] as libc::c_int != 3 as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                730 as libc::c_int,
            );
        }
        if frames[0 as libc::c_int as usize]
            != packet.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_uchar
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                731 as libc::c_int,
            );
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 0 (%2d cases) ............................ OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 1 as libc::c_int) as libc::c_uchar;
        jj = 0 as libc::c_int;
        while jj <= 1275 as libc::c_int * 2 as libc::c_int + 3 as libc::c_int {
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                jj,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if jj & 1 as libc::c_int == 1 as libc::c_int && jj <= 2551 as libc::c_int {
                if ret != 2 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        749 as libc::c_int,
                    );
                }
                if size[0 as libc::c_int as usize] as libc::c_int
                    != size[1 as libc::c_int as usize] as libc::c_int
                    || size[0 as libc::c_int as usize] as libc::c_int
                        != (jj - 1 as libc::c_int) >> 1 as libc::c_int
                {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        750 as libc::c_int,
                    );
                }
                if frames[0 as libc::c_int as usize]
                    != packet.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_uchar
                {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        751 as libc::c_int,
                    );
                }
                if frames[1 as libc::c_int as usize]
                    != (frames[0 as libc::c_int as usize])
                        .offset(size[0 as libc::c_int as usize] as libc::c_int as isize)
                {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        752 as libc::c_int,
                    );
                }
                if toc as libc::c_int >> 2 as libc::c_int != i {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        753 as libc::c_int,
                    );
                }
            } else if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    754 as libc::c_int,
                );
            }
            jj += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 1 (%6d cases) ........................ OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 2 as libc::c_int) as libc::c_uchar;
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                767 as libc::c_int,
            );
        }
        packet[1 as libc::c_int as usize] = 252 as libc::c_int as libc::c_uchar;
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            2 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                772 as libc::c_int,
            );
        }
        j = 0 as libc::c_int;
        while j < 1275 as libc::c_int {
            if j < 252 as libc::c_int {
                packet[1 as libc::c_int as usize] = j as libc::c_uchar;
            } else {
                packet[1 as libc::c_int as usize] =
                    (252 as libc::c_int + (j & 3 as libc::c_int)) as libc::c_uchar;
                packet[2 as libc::c_int as usize] =
                    ((j - 252 as libc::c_int) >> 2 as libc::c_int) as libc::c_uchar;
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 as libc::c_int {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                }) - 1 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    781 as libc::c_int,
                );
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 as libc::c_int {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                }) + 1276 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    786 as libc::c_int,
                );
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + (if j < 252 as libc::c_int {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                }),
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 2 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    791 as libc::c_int,
                );
            }
            if size[0 as libc::c_int as usize] as libc::c_int != j
                || size[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    792 as libc::c_int,
                );
            }
            if frames[1 as libc::c_int as usize]
                != (frames[0 as libc::c_int as usize])
                    .offset(size[0 as libc::c_int as usize] as libc::c_int as isize)
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    793 as libc::c_int,
                );
            }
            if toc as libc::c_int >> 2 as libc::c_int != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    794 as libc::c_int,
                );
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                (j << 1 as libc::c_int) + 4 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 2 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    799 as libc::c_int,
                );
            }
            if size[0 as libc::c_int as usize] as libc::c_int != j
                || size[1 as libc::c_int as usize] as libc::c_int
                    != (j << 1 as libc::c_int) + 3 as libc::c_int
                        - j
                        - (if j < 252 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            2 as libc::c_int
                        })
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    800 as libc::c_int,
                );
            }
            if frames[1 as libc::c_int as usize]
                != (frames[0 as libc::c_int as usize])
                    .offset(size[0 as libc::c_int as usize] as libc::c_int as isize)
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    801 as libc::c_int,
                );
            }
            if toc as libc::c_int >> 2 as libc::c_int != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    802 as libc::c_int,
                );
            }
            j += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 2 (%6d cases) ........................ OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                815 as libc::c_int,
            );
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m-truncation (%2d cases) ............... OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
        jj = 49 as libc::c_int;
        while jj <= 64 as libc::c_int {
            packet[1 as libc::c_int as usize] =
                (0 as libc::c_int + (jj & 63 as libc::c_int)) as libc::c_uchar;
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    830 as libc::c_int,
                );
            }
            packet[1 as libc::c_int as usize] =
                (128 as libc::c_int + (jj & 63 as libc::c_int)) as libc::c_uchar;
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    835 as libc::c_int,
                );
            }
            packet[1 as libc::c_int as usize] =
                (64 as libc::c_int + (jj & 63 as libc::c_int)) as libc::c_uchar;
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    840 as libc::c_int,
                );
            }
            packet[1 as libc::c_int as usize] =
                (128 as libc::c_int + 64 as libc::c_int + (jj & 63 as libc::c_int))
                    as libc::c_uchar;
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                1275 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    845 as libc::c_int,
                );
            }
            jj += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=0,49-64 (%2d cases) ................ OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
        packet[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int;
        while j < 1276 as libc::c_int {
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                j + 2 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 1 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    861 as libc::c_int,
                );
            }
            if size[0 as libc::c_int as usize] as libc::c_int != j {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    862 as libc::c_int,
                );
            }
            if toc as libc::c_int >> 2 as libc::c_int != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    863 as libc::c_int,
                );
            }
            j += 1;
        }
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1276 as libc::c_int + 2 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                868 as libc::c_int,
            );
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=1 CBR (%2d cases) ................. OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut frame_samp: libc::c_int = 0;
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
        frame_samp = opus_packet_get_samples_per_frame(packet.as_mut_ptr(), 48000 as libc::c_int);
        j = 2 as libc::c_int;
        while j < 49 as libc::c_int {
            packet[1 as libc::c_int as usize] = j as libc::c_uchar;
            sz = 2 as libc::c_int;
            while sz < (j + 2 as libc::c_int) * 1275 as libc::c_int {
                toc = -(1 as libc::c_int) as libc::c_uchar;
                frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                payload_offset = -(1 as libc::c_int);
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    sz,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if frame_samp * j <= 5760 as libc::c_int
                    && (sz - 2 as libc::c_int) % j == 0 as libc::c_int
                    && (sz - 2 as libc::c_int) / j < 1276 as libc::c_int
                {
                    if ret != j {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            890 as libc::c_int,
                        );
                    }
                    jj = 1 as libc::c_int;
                    while jj < ret {
                        if frames[jj as usize]
                            != (frames[(jj - 1 as libc::c_int) as usize]).offset(
                                size[(jj - 1 as libc::c_int) as usize] as libc::c_int as isize,
                            )
                        {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                891 as libc::c_int,
                            );
                        }
                        jj += 1;
                    }
                    if toc as libc::c_int >> 2 as libc::c_int != i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            892 as libc::c_int,
                        );
                    }
                } else if ret != -(4 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        893 as libc::c_int,
                    );
                }
                sz += 1;
            }
            j += 1;
        }
        packet[1 as libc::c_int as usize] = (5760 as libc::c_int / frame_samp) as libc::c_uchar;
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            1275 as libc::c_int * packet[1 as libc::c_int as usize] as libc::c_int
                + 2 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != packet[1 as libc::c_int as usize] as libc::c_int {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                901 as libc::c_int,
            );
        }
        jj = 0 as libc::c_int;
        while jj < ret {
            if size[jj as usize] as libc::c_int != 1275 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    902 as libc::c_int,
                );
            }
            jj += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 m=1-48 CBR (%2d cases) .......... OK.\n\0" as *const u8 as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut frame_samp_0: libc::c_int = 0;
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
        packet[1 as libc::c_int as usize] =
            (128 as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
        frame_samp_0 = opus_packet_get_samples_per_frame(packet.as_mut_ptr(), 48000 as libc::c_int);
        jj = 0 as libc::c_int;
        while jj < 1276 as libc::c_int {
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as libc::c_int + jj,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != 1 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    919 as libc::c_int,
                );
            }
            if size[0 as libc::c_int as usize] as libc::c_int != jj {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    920 as libc::c_int,
                );
            }
            if toc as libc::c_int >> 2 as libc::c_int != i {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    921 as libc::c_int,
                );
            }
            jj += 1;
        }
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            2 as libc::c_int + 1276 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                926 as libc::c_int,
            );
        }
        j = 2 as libc::c_int;
        while j < 49 as libc::c_int {
            packet[1 as libc::c_int as usize] = (128 as libc::c_int + j) as libc::c_uchar;
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as libc::c_int + j - 2 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    934 as libc::c_int,
                );
            }
            packet[2 as libc::c_int as usize] = 252 as libc::c_int as libc::c_uchar;
            packet[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            jj = 4 as libc::c_int;
            while jj < 2 as libc::c_int + j {
                packet[jj as usize] = 0 as libc::c_int as libc::c_uchar;
                jj += 1;
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as libc::c_int + j,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    941 as libc::c_int,
                );
            }
            jj = 2 as libc::c_int;
            while jj < 2 as libc::c_int + j {
                packet[jj as usize] = 0 as libc::c_int as libc::c_uchar;
                jj += 1;
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as libc::c_int + j - 2 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    947 as libc::c_int,
                );
            }
            packet[2 as libc::c_int as usize] = 252 as libc::c_int as libc::c_uchar;
            packet[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            jj = 4 as libc::c_int;
            while jj < 2 as libc::c_int + j {
                packet[jj as usize] = 0 as libc::c_int as libc::c_uchar;
                jj += 1;
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as libc::c_int + j + 252 as libc::c_int - 1 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    955 as libc::c_int,
                );
            }
            jj = 2 as libc::c_int;
            while jj < 2 as libc::c_int + j {
                packet[jj as usize] = 0 as libc::c_int as libc::c_uchar;
                jj += 1;
            }
            toc = -(1 as libc::c_int) as libc::c_uchar;
            frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
            payload_offset = -(1 as libc::c_int);
            ret = opus_packet_parse(
                packet.as_mut_ptr(),
                2 as libc::c_int + j - 1 as libc::c_int,
                &mut toc,
                frames.as_mut_ptr(),
                size.as_mut_ptr(),
                &mut payload_offset,
            );
            cfgs += 1;
            if frame_samp_0 * j <= 5760 as libc::c_int {
                if ret != j {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        962 as libc::c_int,
                    );
                }
                jj = 0 as libc::c_int;
                while jj < j {
                    if size[jj as usize] as libc::c_int != 0 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            963 as libc::c_int,
                        );
                    }
                    jj += 1;
                }
                if toc as libc::c_int >> 2 as libc::c_int != i {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        964 as libc::c_int,
                    );
                }
            } else if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    965 as libc::c_int,
                );
            }
            sz = 0 as libc::c_int;
            while sz < 8 as libc::c_int {
                let tsz: [libc::c_int; 8] = [
                    50 as libc::c_int,
                    201 as libc::c_int,
                    403 as libc::c_int,
                    700 as libc::c_int,
                    1472 as libc::c_int,
                    5110 as libc::c_int,
                    20400 as libc::c_int,
                    61298 as libc::c_int,
                ];
                let mut pos: libc::c_int = 0 as libc::c_int;
                let mut as_0: libc::c_int = (tsz[sz as usize] + i - j - 2 as libc::c_int) / j;
                jj = 0 as libc::c_int;
                while jj < j - 1 as libc::c_int {
                    if as_0 < 252 as libc::c_int {
                        packet[(2 as libc::c_int + pos) as usize] = as_0 as libc::c_uchar;
                        pos += 1;
                    } else {
                        packet[(2 as libc::c_int + pos) as usize] =
                            (252 as libc::c_int + (as_0 & 3 as libc::c_int)) as libc::c_uchar;
                        packet[(3 as libc::c_int + pos) as usize] =
                            ((as_0 - 252 as libc::c_int) >> 2 as libc::c_int) as libc::c_uchar;
                        pos += 2 as libc::c_int;
                    }
                    jj += 1;
                }
                toc = -(1 as libc::c_int) as libc::c_uchar;
                frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                payload_offset = -(1 as libc::c_int);
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    tsz[sz as usize] + i,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if frame_samp_0 * j <= 5760 as libc::c_int
                    && as_0 < 1276 as libc::c_int
                    && tsz[sz as usize] + i - 2 as libc::c_int - pos - as_0 * (j - 1 as libc::c_int)
                        < 1276 as libc::c_int
                {
                    if ret != j {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            981 as libc::c_int,
                        );
                    }
                    jj = 0 as libc::c_int;
                    while jj < j - 1 as libc::c_int {
                        if size[jj as usize] as libc::c_int != as_0 {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                982 as libc::c_int,
                            );
                        }
                        jj += 1;
                    }
                    if size[(j - 1 as libc::c_int) as usize] as libc::c_int
                        != tsz[sz as usize] + i
                            - 2 as libc::c_int
                            - pos
                            - as_0 * (j - 1 as libc::c_int)
                    {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            983 as libc::c_int,
                        );
                    }
                    if toc as libc::c_int >> 2 as libc::c_int != i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            984 as libc::c_int,
                        );
                    }
                } else if ret != -(4 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        985 as libc::c_int,
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
        b"    code 3 m=1-48 VBR (%2d cases) ............. OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    cfgs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        packet[0 as libc::c_int as usize] =
            ((i << 2 as libc::c_int) + 3 as libc::c_int) as libc::c_uchar;
        packet[1 as libc::c_int as usize] =
            (128 as libc::c_int + 1 as libc::c_int + 64 as libc::c_int) as libc::c_uchar;
        jj = 2 as libc::c_int;
        while jj < 127 as libc::c_int {
            packet[jj as usize] = 255 as libc::c_int as libc::c_uchar;
            jj += 1;
        }
        toc = -(1 as libc::c_int) as libc::c_uchar;
        frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
        payload_offset = -(1 as libc::c_int);
        ret = opus_packet_parse(
            packet.as_mut_ptr(),
            127 as libc::c_int,
            &mut toc,
            frames.as_mut_ptr(),
            size.as_mut_ptr(),
            &mut payload_offset,
        );
        cfgs += 1;
        if ret != -(4 as libc::c_int) {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                1002 as libc::c_int,
            );
        }
        sz = 0 as libc::c_int;
        while sz < 4 as libc::c_int {
            let tsz_0: [libc::c_int; 4] = [
                0 as libc::c_int,
                72 as libc::c_int,
                512 as libc::c_int,
                1275 as libc::c_int,
            ];
            jj = sz;
            while jj < 65025 as libc::c_int {
                let mut pos_0: libc::c_int = 0;
                pos_0 = 0 as libc::c_int;
                while pos_0 < jj / 254 as libc::c_int {
                    packet[(2 as libc::c_int + pos_0) as usize] =
                        255 as libc::c_int as libc::c_uchar;
                    pos_0 += 1;
                }
                packet[(2 as libc::c_int + pos_0) as usize] =
                    (jj % 254 as libc::c_int) as libc::c_uchar;
                pos_0 += 1;
                if sz == 0 as libc::c_int && i == 63 as libc::c_int {
                    toc = -(1 as libc::c_int) as libc::c_uchar;
                    frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                    frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                    payload_offset = -(1 as libc::c_int);
                    ret = opus_packet_parse(
                        packet.as_mut_ptr(),
                        2 as libc::c_int + jj + pos_0 - 1 as libc::c_int,
                        &mut toc,
                        frames.as_mut_ptr(),
                        size.as_mut_ptr(),
                        &mut payload_offset,
                    );
                    cfgs += 1;
                    if ret != -(4 as libc::c_int) {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            1019 as libc::c_int,
                        );
                    }
                }
                toc = -(1 as libc::c_int) as libc::c_uchar;
                frames[0 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                frames[1 as libc::c_int as usize] = std::ptr::null_mut::<libc::c_uchar>();
                payload_offset = -(1 as libc::c_int);
                ret = opus_packet_parse(
                    packet.as_mut_ptr(),
                    2 as libc::c_int + jj + tsz_0[sz as usize] + i + pos_0,
                    &mut toc,
                    frames.as_mut_ptr(),
                    size.as_mut_ptr(),
                    &mut payload_offset,
                );
                cfgs += 1;
                if tsz_0[sz as usize] + i < 1276 as libc::c_int {
                    if ret != 1 as libc::c_int {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            1026 as libc::c_int,
                        );
                    }
                    if size[0 as libc::c_int as usize] as libc::c_int != tsz_0[sz as usize] + i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            1027 as libc::c_int,
                        );
                    }
                    if toc as libc::c_int >> 2 as libc::c_int != i {
                        _test_failed(
                            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                            1028 as libc::c_int,
                        );
                    }
                } else if ret != -(4 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1029 as libc::c_int,
                    );
                }
                jj += 11 as libc::c_int;
            }
            sz += 1;
        }
        i += 1;
    }
    fprintf(
        stdout(),
        b"    code 3 padding (%2d cases) ............... OK.\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs_total += cfgs;
    fprintf(
        stdout(),
        b"    opus_packet_parse ............................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"                      All packet parsing tests passed\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"                          (%d API invocations)\n\0" as *const u8 as *const libc::c_char,
        cfgs_total,
    );
    cfgs_total
}
#[c2rust::src_loc = "1065:1"]
pub unsafe fn test_enc_api() -> i32 {
    let mut enc_final_range: u32 = 0;
    let mut enc: *mut OpusEncoder = std::ptr::null_mut::<OpusEncoder>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut packet: [libc::c_uchar; 1276] = [0; 1276];
    let mut fbuf: [libc::c_float; 1920] = [0.; 1920];
    let mut sbuf: [libc::c_short; 1920] = [0; 1920];
    let mut c: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut cfgs: libc::c_int = 0;
    cfgs = 0 as libc::c_int;
    fprintf(
        stdout(),
        b"\n  Encoder basic API tests\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    c = 0 as libc::c_int;
    while c < 4 as libc::c_int {
        i = opus_encoder_get_size(c);
        if (c == 1 as libc::c_int || c == 2 as libc::c_int)
            && (i <= 2048 as libc::c_int || i > (1 as libc::c_int) << 17 as libc::c_int)
            || c != 1 as libc::c_int && c != 2 as libc::c_int && i != 0 as libc::c_int
        {
            _test_failed(
                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                1084 as libc::c_int,
            );
        }
        fprintf(
            stdout(),
            b"    opus_encoder_get_size(%d)=%d ...............%s OK.\n\0" as *const u8
                as *const libc::c_char,
            c,
            i,
            if i > 0 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"....\0" as *const u8 as *const libc::c_char
            },
        );
        cfgs += 1;
        c += 1;
    }
    c = 0 as libc::c_int;
    while c < 4 as libc::c_int {
        i = -(7 as libc::c_int);
        while i <= 96000 as libc::c_int {
            let mut fs: libc::c_int = 0;
            if !((i == 8000 as libc::c_int
                || i == 12000 as libc::c_int
                || i == 16000 as libc::c_int
                || i == 24000 as libc::c_int
                || i == 48000 as libc::c_int)
                && (c == 1 as libc::c_int || c == 2 as libc::c_int))
            {
                match i {
                    -5 => {
                        fs = -(8000 as libc::c_int);
                    }
                    -6 => {
                        fs = 2147483647 as libc::c_int;
                    }
                    -7 => {
                        fs = -(2147483647 as libc::c_int) - 1 as libc::c_int;
                    }
                    _ => {
                        fs = i;
                    }
                }
                err = 0 as libc::c_int;
                enc = opus_encoder_create(fs, c, 2048 as libc::c_int, &mut err);
                if err != -(1 as libc::c_int) || !enc.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1106 as libc::c_int,
                    );
                }
                cfgs += 1;
                enc = opus_encoder_create(
                    fs,
                    c,
                    2048 as libc::c_int,
                    std::ptr::null_mut::<libc::c_int>(),
                );
                if !enc.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1109 as libc::c_int,
                    );
                }
                cfgs += 1;
                opus_encoder_destroy(enc);
                enc = malloc(opus_encoder_get_size(2 as libc::c_int) as libc::c_ulong)
                    as *mut OpusEncoder;
                if enc.is_null() {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1113 as libc::c_int,
                    );
                }
                err = opus_encoder_init(enc, fs, c, 2048 as libc::c_int);
                if err != -(1 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1115 as libc::c_int,
                    );
                }
                cfgs += 1;
                free(enc as *mut libc::c_void);
            }
            i += 1;
        }
        c += 1;
    }
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        -(1000 as libc::c_int),
        std::ptr::null_mut::<libc::c_int>(),
    );
    if !enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1122 as libc::c_int,
        );
    }
    cfgs += 1;
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        -(1000 as libc::c_int),
        &mut err,
    );
    if err != -(1 as libc::c_int) || !enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1127 as libc::c_int,
        );
    }
    cfgs += 1;
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        2048 as libc::c_int,
        std::ptr::null_mut::<libc::c_int>(),
    );
    if enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1132 as libc::c_int,
        );
    }
    opus_encoder_destroy(enc);
    cfgs += 1;
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        2051 as libc::c_int,
        &mut err,
    );
    if err != 0 as libc::c_int || enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1138 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4027 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i < 0 as libc::c_int || i > 32766 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1141 as libc::c_int,
        );
    }
    cfgs += 1;
    opus_encoder_destroy(enc);
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        2049 as libc::c_int,
        &mut err,
    );
    if err != 0 as libc::c_int || enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1147 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4027 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i < 0 as libc::c_int || i > 32766 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1150 as libc::c_int,
        );
    }
    opus_encoder_destroy(enc);
    cfgs += 1;
    enc = opus_encoder_create(
        48000 as libc::c_int,
        2 as libc::c_int,
        2048 as libc::c_int,
        &mut err,
    );
    if err != 0 as libc::c_int || enc.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1156 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_encoder_create() ........................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_encoder_init() .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4027 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i < 0 as libc::c_int || i > 32766 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1165 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4027 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1168 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_LOOKAHEAD ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_encoder_ctl(
        enc,
        4029 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != 48000 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1173 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4029 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1176 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_SAMPLE_RATE ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if opus_encoder_ctl(enc, -(5 as libc::c_int)) != -(5 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1180 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_UNIMPLEMENTED ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4001 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1185 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4000 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        );
    }
    i = -(1000 as libc::c_int);
    if opus_encoder_ctl(enc, 4000 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        );
    }
    i = 2049 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4000 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4001 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        );
    }
    i = 2051 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4000 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_APPLICATION ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4001 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_APPLICATION ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4003 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1193 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl(enc, 4002 as libc::c_int, 1073741832 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1195 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl(
        enc,
        4003 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    ) != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1198 as libc::c_int,
        );
    }
    if i > 700000 as libc::c_int || i < 256000 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1199 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(12345 as libc::c_int);
    if opus_encoder_ctl(enc, 4002 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    if opus_encoder_ctl(enc, 4002 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
        );
    }
    i = 500 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4002 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4003 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
        );
    }
    i = 256000 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4002 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_BITRATE ............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4003 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_BITRATE ............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4023 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4022 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
        );
    }
    i = 3 as libc::c_int;
    if opus_encoder_ctl(enc, 4022 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4022 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4023 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
        );
    }
    i = -(1000 as libc::c_int);
    j = i;
    if opus_encoder_ctl(enc, 4022 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_FORCE_CHANNELS ...................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4023 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_FORCE_CHANNELS ...................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    i = -(2 as libc::c_int);
    if opus_encoder_ctl(enc, 4008 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1215 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1105 as libc::c_int + 1 as libc::c_int;
    if opus_encoder_ctl(enc, 4008 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1218 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1101 as libc::c_int;
    if opus_encoder_ctl(enc, 4008 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1221 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1105 as libc::c_int;
    if opus_encoder_ctl(enc, 4008 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1224 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1103 as libc::c_int;
    if opus_encoder_ctl(enc, 4008 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1227 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1102 as libc::c_int;
    if opus_encoder_ctl(enc, 4008 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1230 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_SET_BANDWIDTH ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4009 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int
        || i != 1101 as libc::c_int
            && i != 1102 as libc::c_int
            && i != 1103 as libc::c_int
            && i != 1105 as libc::c_int
            && i != -(1000 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1240 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl(enc, 4008 as libc::c_int, -(1000 as libc::c_int)) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1242 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4009 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1245 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_BANDWIDTH ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(2 as libc::c_int);
    if opus_encoder_ctl(enc, 4004 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1250 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1105 as libc::c_int + 1 as libc::c_int;
    if opus_encoder_ctl(enc, 4004 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1253 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1101 as libc::c_int;
    if opus_encoder_ctl(enc, 4004 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1256 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1105 as libc::c_int;
    if opus_encoder_ctl(enc, 4004 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1259 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1103 as libc::c_int;
    if opus_encoder_ctl(enc, 4004 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1262 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 1102 as libc::c_int;
    if opus_encoder_ctl(enc, 4004 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1265 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_SET_MAX_BANDWIDTH ....................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4005 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int
        || i != 1101 as libc::c_int
            && i != 1102 as libc::c_int
            && i != 1103 as libc::c_int
            && i != 1105 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1275 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4005 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1278 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_MAX_BANDWIDTH ....................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    err = opus_encoder_ctl(
        enc,
        4017 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1283 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4016 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
        );
    }
    i = 2 as libc::c_int;
    if opus_encoder_ctl(enc, 4016 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4016 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4017 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4016 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_DTX ................................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4017 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_DTX ................................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4011 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1291 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4010 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
        );
    }
    i = 11 as libc::c_int;
    if opus_encoder_ctl(enc, 4010 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4010 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4011 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
        );
    }
    i = 10 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4010 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_COMPLEXITY .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4011 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_COMPLEXITY .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4013 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1299 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4012 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
        );
    }
    i = 2 as libc::c_int;
    if opus_encoder_ctl(enc, 4012 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4012 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4013 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4012 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_INBAND_FEC .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4013 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_INBAND_FEC .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4015 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1307 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4014 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
        );
    }
    i = 101 as libc::c_int;
    if opus_encoder_ctl(enc, 4014 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
        );
    }
    i = 100 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4014 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4015 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4014 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_PACKET_LOSS_PERC .................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4015 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_PACKET_LOSS_PERC .................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4007 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1315 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4006 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
        );
    }
    i = 2 as libc::c_int;
    if opus_encoder_ctl(enc, 4006 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4006 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4007 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4006 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_VBR ................................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4007 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_VBR ................................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4021 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1331 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4020 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
        );
    }
    i = 2 as libc::c_int;
    if opus_encoder_ctl(enc, 4020 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4020 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4021 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4020 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_VBR_CONSTRAINT ...................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4021 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_VBR_CONSTRAINT ...................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4025 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1339 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(12345 as libc::c_int);
    if opus_encoder_ctl(enc, 4024 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
        );
    }
    i = 0x7fffffff as libc::c_int;
    if opus_encoder_ctl(enc, 4024 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
        );
    }
    i = 3002 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4024 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4025 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
        );
    }
    i = -(1000 as libc::c_int);
    j = i;
    if opus_encoder_ctl(enc, 4024 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_SIGNAL .............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4025 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_SIGNAL .............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4037 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1347 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 7 as libc::c_int;
    if opus_encoder_ctl(enc, 4036 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
        );
    }
    i = 25 as libc::c_int;
    if opus_encoder_ctl(enc, 4036 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
        );
    }
    i = 16 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4036 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4037 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
        );
    }
    i = 24 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4036 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_LSB_DEPTH ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4037 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_LSB_DEPTH ........................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4043 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if i != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1354 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(
        enc,
        4043 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1357 as libc::c_int,
        );
    }
    cfgs += 1;
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4042 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
        );
    }
    i = 2 as libc::c_int;
    if opus_encoder_ctl(enc, 4042 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
        );
    }
    i = 1 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4042 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4043 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4042 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_PREDICTION_DISABLED ................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4043 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_PREDICTION_DISABLED ................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(
        enc,
        4041 as libc::c_int,
        null_int_ptr.offset(null_int_ptr.offset_from(null_int_ptr) as libc::c_long as isize),
    );
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1364 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5001 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1367 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5002 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1370 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5003 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1373 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5004 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1376 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5005 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1379 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5006 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1382 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5007 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1385 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5008 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1388 as libc::c_int,
        );
    }
    cfgs += 1;
    err = opus_encoder_ctl(enc, 4040 as libc::c_int, 5009 as libc::c_int);
    if err != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1391 as libc::c_int,
        );
    }
    cfgs += 1;
    i = 0 as libc::c_int;
    if opus_encoder_ctl(enc, 4040 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
        );
    }
    i = -(1 as libc::c_int);
    if opus_encoder_ctl(enc, 4040 as libc::c_int, i) == 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
        );
    }
    i = 5006 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4040 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
        );
    }
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4041 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
        );
    }
    i = 5000 as libc::c_int;
    j = i;
    if opus_encoder_ctl(enc, 4040 as libc::c_int, i) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_SET_EXPERT_FRAME_DURATION ............... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    i = -(12345 as libc::c_int);
    err = opus_encoder_ctl(
        enc,
        4041 as libc::c_int,
        (&mut i as *mut i32)
            .offset((&mut i as *mut i32).offset_from(&mut i as *mut i32) as libc::c_long as isize),
    );
    if err != 0 as libc::c_int || i != j {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
        );
    }
    fprintf(
        stdout(),
        b"    OPUS_GET_EXPERT_FRAME_DURATION ............... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    cfgs += 6 as libc::c_int;
    err = opus_encoder_ctl(enc, 4031 as libc::c_int, null_uint_ptr as *mut u32);
    if err != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1401 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_encoder_ctl(
        enc,
        4031 as libc::c_int,
        (&mut enc_final_range as *mut u32).offset(
            (&mut enc_final_range as *mut u32).offset_from(&mut enc_final_range as *mut u32)
                as libc::c_long as isize,
        ),
    ) != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1403 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_GET_FINAL_RANGE ......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if opus_encoder_ctl(enc, 4028 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1408 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    OPUS_RESET_STATE ............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        sbuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(960 as libc::c_int as libc::c_ulong),
    );
    i = opus_encode(
        enc,
        sbuf.as_mut_ptr(),
        960 as libc::c_int,
        packet.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 1276]>() as libc::c_ulong as i32,
    );
    if i < 1 as libc::c_int
        || i > ::core::mem::size_of::<[libc::c_uchar; 1276]>() as libc::c_ulong as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1415 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_encode() ................................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        fbuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(960 as libc::c_int as libc::c_ulong),
    );
    i = opus_encode_float(
        enc,
        fbuf.as_mut_ptr(),
        960 as libc::c_int,
        packet.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 1276]>() as libc::c_ulong as i32,
    );
    if i < 1 as libc::c_int
        || i > ::core::mem::size_of::<[libc::c_uchar; 1276]>() as libc::c_ulong as i32
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1423 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_encode_float() .......................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    opus_encoder_destroy(enc);
    cfgs += 1;
    fprintf(
        stdout(),
        b"                   All encoder interface tests passed\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"                             (%d API invocations)\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs
}
#[c2rust::src_loc = "1444:1"]
pub unsafe fn test_repacketizer_api() -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut cfgs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut rp: *mut OpusRepacketizer = std::ptr::null_mut::<OpusRepacketizer>();
    let mut packet: *mut libc::c_uchar = std::ptr::null_mut::<libc::c_uchar>();
    let mut po: *mut libc::c_uchar = std::ptr::null_mut::<libc::c_uchar>();
    cfgs = 0 as libc::c_int;
    fprintf(
        stdout(),
        b"\n  Repacketizer tests\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    packet = malloc(
        (1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if packet.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1455 as libc::c_int,
        );
    }
    memset(
        packet as *mut libc::c_void,
        0 as libc::c_int,
        (1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int) as libc::c_ulong,
    );
    po = malloc(
        (1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int
            + 256 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if po.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1458 as libc::c_int,
        );
    }
    i = opus_repacketizer_get_size();
    if i <= 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1461 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_get_size()=%d ............. OK.\n\0" as *const u8
            as *const libc::c_char,
        i,
    );
    rp = malloc(i as libc::c_ulong) as *mut OpusRepacketizer;
    rp = opus_repacketizer_init(rp);
    if rp.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1467 as libc::c_int,
        );
    }
    cfgs += 1;
    free(rp as *mut libc::c_void);
    fprintf(
        stdout(),
        b"    opus_repacketizer_init ....................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    rp = opus_repacketizer_create();
    if rp.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1473 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_create ..................... OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if opus_repacketizer_get_nb_frames(rp) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1477 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_get_nb_frames .............. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    if opus_repacketizer_cat(rp, packet, 0 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1483 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 2 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1486 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 1 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1489 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 3 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 1 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1492 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_uchar;
    *packet.offset(1 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 2 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1496 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_uchar;
    *packet.offset(1 as libc::c_int as isize) = 250 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 251 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1500 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 3 as libc::c_int as libc::c_uchar;
    *packet.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 2 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1504 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(1 as libc::c_int as isize) = 49 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 100 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1507 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 3 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1510 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) =
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 3 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1513 as libc::c_int,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    j = 0 as libc::c_int;
    while j < 32 as libc::c_int {
        let mut maxi: libc::c_int = 0;
        *packet.offset(0 as libc::c_int as isize) =
            (((j << 1 as libc::c_int) + (j & 1 as libc::c_int)) << 2 as libc::c_int)
                as libc::c_uchar;
        maxi = 960 as libc::c_int / opus_packet_get_samples_per_frame(packet, 8000 as libc::c_int);
        i = 1 as libc::c_int;
        while i <= maxi {
            let mut maxp: libc::c_int = 0;
            *packet.offset(0 as libc::c_int as isize) =
                (((j << 1 as libc::c_int) + (j & 1 as libc::c_int)) << 2 as libc::c_int)
                    as libc::c_uchar;
            if i > 1 as libc::c_int {
                let fresh0 = &mut (*packet.offset(0 as libc::c_int as isize));
                *fresh0 = (*fresh0 as libc::c_int
                    + if i == 2 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        3 as libc::c_int
                    }) as libc::c_uchar;
            }
            *packet.offset(1 as libc::c_int as isize) = (if i > 2 as libc::c_int {
                i
            } else {
                0 as libc::c_int
            }) as libc::c_uchar;
            maxp = 960 as libc::c_int
                / (i * opus_packet_get_samples_per_frame(packet, 8000 as libc::c_int));
            k = 0 as libc::c_int;
            while k <= 1275 as libc::c_int + 75 as libc::c_int {
                let mut cnt: i32 = 0;
                let mut rcnt: i32 = 0;
                if k % i == 0 as libc::c_int {
                    cnt = 0 as libc::c_int;
                    while cnt < maxp + 2 as libc::c_int {
                        if cnt > 0 as libc::c_int {
                            ret = opus_repacketizer_cat(
                                rp,
                                packet,
                                k + (if i > 2 as libc::c_int {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }),
                            );
                            if if cnt <= maxp && k <= 1275 as libc::c_int * i {
                                (ret != 0 as libc::c_int) as libc::c_int
                            } else {
                                (ret != -(4 as libc::c_int)) as libc::c_int
                            } != 0
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1542 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                        }
                        rcnt = if k <= 1275 as libc::c_int * i {
                            if cnt < maxp {
                                cnt
                            } else {
                                maxp
                            }
                        } else {
                            0 as libc::c_int
                        };
                        if opus_repacketizer_get_nb_frames(rp) != rcnt * i {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                1546 as libc::c_int,
                            );
                        }
                        cfgs += 1;
                        ret = opus_repacketizer_out_range(
                            rp,
                            0 as libc::c_int,
                            rcnt * i,
                            po,
                            1276 as libc::c_int * 48 as libc::c_int
                                + 48 as libc::c_int * 2 as libc::c_int
                                + 2 as libc::c_int,
                        );
                        if rcnt > 0 as libc::c_int {
                            let mut len: libc::c_int = 0;
                            len = k * rcnt
                                + (if rcnt * i > 2 as libc::c_int {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                });
                            if ret != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1553 as libc::c_int,
                                );
                            }
                            if rcnt * i < 2 as libc::c_int
                                && *po.offset(0 as libc::c_int as isize) as libc::c_int
                                    & 3 as libc::c_int
                                    != 0 as libc::c_int
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1554 as libc::c_int,
                                );
                            }
                            if rcnt * i == 2 as libc::c_int
                                && *po.offset(0 as libc::c_int as isize) as libc::c_int
                                    & 3 as libc::c_int
                                    != 1 as libc::c_int
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1555 as libc::c_int,
                                );
                            }
                            if rcnt * i > 2 as libc::c_int
                                && (*po.offset(0 as libc::c_int as isize) as libc::c_int
                                    & 3 as libc::c_int
                                    != 3 as libc::c_int
                                    || *po.offset(1 as libc::c_int as isize) as libc::c_int
                                        != rcnt * i)
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1556 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_repacketizer_out(rp, po, len) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1558 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_unpad(po, len) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1560 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_pad(po, len, len + 1 as libc::c_int) != 0 as libc::c_int
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1562 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_pad(po, len + 1 as libc::c_int, len + 256 as libc::c_int)
                                != 0 as libc::c_int
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1564 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_packet_unpad(po, len + 256 as libc::c_int) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1566 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_unpad(po, len, 1 as libc::c_int) != len {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1568 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_pad(
                                po,
                                len,
                                len + 1 as libc::c_int,
                                1 as libc::c_int,
                            ) != 0 as libc::c_int
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1570 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_pad(
                                po,
                                len + 1 as libc::c_int,
                                len + 256 as libc::c_int,
                                1 as libc::c_int,
                            ) != 0 as libc::c_int
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1572 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_multistream_packet_unpad(
                                po,
                                len + 256 as libc::c_int,
                                1 as libc::c_int,
                            ) != len
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1574 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if opus_repacketizer_out(rp, po, len - 1 as libc::c_int)
                                != -(2 as libc::c_int)
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1576 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                            if len > 1 as libc::c_int {
                                if opus_repacketizer_out(rp, po, 1 as libc::c_int)
                                    != -(2 as libc::c_int)
                                {
                                    _test_failed(
                                        b"tests/test_opus_api.c\0" as *const u8
                                            as *const libc::c_char,
                                        1580 as libc::c_int,
                                    );
                                }
                                cfgs += 1;
                            }
                            if opus_repacketizer_out(rp, po, 0 as libc::c_int)
                                != -(2 as libc::c_int)
                            {
                                _test_failed(
                                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                    1583 as libc::c_int,
                                );
                            }
                            cfgs += 1;
                        } else if ret != -(1 as libc::c_int) {
                            _test_failed(
                                b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                                1585 as libc::c_int,
                            );
                        }
                        cnt += 1;
                    }
                    opus_repacketizer_init(rp);
                }
                k += 3 as libc::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    opus_repacketizer_init(rp);
    *packet.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 5 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1595 as libc::c_int,
        );
    }
    cfgs += 1;
    let fresh1 = &mut (*packet.offset(0 as libc::c_int as isize));
    *fresh1 = (*fresh1 as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 9 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1598 as libc::c_int,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int,
    );
    if i != 4 as libc::c_int + 8 as libc::c_int + 2 as libc::c_int
        || *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
            != 3 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int & 63 as libc::c_int
            != 3 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int
            != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1601 as libc::c_int,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out_range(
        rp,
        0 as libc::c_int,
        1 as libc::c_int,
        po,
        1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int,
    );
    if i != 5 as libc::c_int
        || *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
            != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1604 as libc::c_int,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out_range(
        rp,
        1 as libc::c_int,
        2 as libc::c_int,
        po,
        1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int,
    );
    if i != 5 as libc::c_int
        || *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
            != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1607 as libc::c_int,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 9 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1613 as libc::c_int,
        );
    }
    cfgs += 1;
    *packet.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 3 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1616 as libc::c_int,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int,
    );
    if i != 2 as libc::c_int + 8 as libc::c_int + 2 as libc::c_int + 2 as libc::c_int
        || *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
            != 3 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int & 63 as libc::c_int
            != 3 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int
            != 1 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1619 as libc::c_int,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_uchar;
    *packet.offset(1 as libc::c_int as isize) = 4 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 8 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1626 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_repacketizer_cat(rp, packet, 8 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1628 as libc::c_int,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int,
    );
    if i != 2 as libc::c_int
        + 1 as libc::c_int
        + 1 as libc::c_int
        + 1 as libc::c_int
        + 4 as libc::c_int
        + 2 as libc::c_int
        + 4 as libc::c_int
        + 2 as libc::c_int
        || *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
            != 3 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int & 63 as libc::c_int
            != 4 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int
            != 1 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1631 as libc::c_int,
        );
    }
    cfgs += 1;
    opus_repacketizer_init(rp);
    *packet.offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_uchar;
    *packet.offset(1 as libc::c_int as isize) = 4 as libc::c_int as libc::c_uchar;
    if opus_repacketizer_cat(rp, packet, 10 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1638 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_repacketizer_cat(rp, packet, 10 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1640 as libc::c_int,
        );
    }
    cfgs += 1;
    i = opus_repacketizer_out(
        rp,
        po,
        1276 as libc::c_int * 48 as libc::c_int
            + 48 as libc::c_int * 2 as libc::c_int
            + 2 as libc::c_int,
    );
    if i != 2 as libc::c_int
        + 4 as libc::c_int
        + 4 as libc::c_int
        + 4 as libc::c_int
        + 4 as libc::c_int
        || *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
            != 3 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int & 63 as libc::c_int
            != 4 as libc::c_int
        || *po.offset(1 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int
            != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1643 as libc::c_int,
        );
    }
    cfgs += 1;
    j = 0 as libc::c_int;
    while j < 32 as libc::c_int {
        let mut maxi_0: libc::c_int = 0;
        let mut sum: libc::c_int = 0;
        let mut rcnt_0: libc::c_int = 0;
        *packet.offset(0 as libc::c_int as isize) =
            (((j << 1 as libc::c_int) + (j & 1 as libc::c_int)) << 2 as libc::c_int)
                as libc::c_uchar;
        maxi_0 =
            960 as libc::c_int / opus_packet_get_samples_per_frame(packet, 8000 as libc::c_int);
        sum = 0 as libc::c_int;
        rcnt_0 = 0 as libc::c_int;
        opus_repacketizer_init(rp);
        i = 1 as libc::c_int;
        while i <= maxi_0 + 2 as libc::c_int {
            let mut len_0: libc::c_int = 0;
            ret = opus_repacketizer_cat(rp, packet, i);
            if rcnt_0 < maxi_0 {
                if ret != 0 as libc::c_int {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1662 as libc::c_int,
                    );
                }
                rcnt_0 += 1;
                sum += i - 1 as libc::c_int;
            } else if ret != -(4 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1665 as libc::c_int,
                );
            }
            cfgs += 1;
            len_0 = sum
                + (if rcnt_0 < 2 as libc::c_int {
                    1 as libc::c_int
                } else if rcnt_0 < 3 as libc::c_int {
                    2 as libc::c_int
                } else {
                    2 as libc::c_int + rcnt_0 - 1 as libc::c_int
                });
            if opus_repacketizer_out(
                rp,
                po,
                1276 as libc::c_int * 48 as libc::c_int
                    + 48 as libc::c_int * 2 as libc::c_int
                    + 2 as libc::c_int,
            ) != len_0
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1668 as libc::c_int,
                );
            }
            if rcnt_0 > 2 as libc::c_int
                && *po.offset(1 as libc::c_int as isize) as libc::c_int & 63 as libc::c_int
                    != rcnt_0
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1669 as libc::c_int,
                );
            }
            if rcnt_0 == 2 as libc::c_int
                && *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
                    != 2 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1670 as libc::c_int,
                );
            }
            if rcnt_0 == 1 as libc::c_int
                && *po.offset(0 as libc::c_int as isize) as libc::c_int & 3 as libc::c_int
                    != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1671 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_repacketizer_out(rp, po, len_0) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1673 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_packet_unpad(po, len_0) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1675 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_packet_pad(po, len_0, len_0 + 1 as libc::c_int) != 0 as libc::c_int {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1677 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_packet_pad(po, len_0 + 1 as libc::c_int, len_0 + 256 as libc::c_int)
                != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1679 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_packet_unpad(po, len_0 + 256 as libc::c_int) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1681 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_unpad(po, len_0, 1 as libc::c_int) != len_0 {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1683 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_pad(po, len_0, len_0 + 1 as libc::c_int, 1 as libc::c_int)
                != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1685 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_pad(
                po,
                len_0 + 1 as libc::c_int,
                len_0 + 256 as libc::c_int,
                1 as libc::c_int,
            ) != 0 as libc::c_int
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1687 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_multistream_packet_unpad(po, len_0 + 256 as libc::c_int, 1 as libc::c_int)
                != len_0
            {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1689 as libc::c_int,
                );
            }
            cfgs += 1;
            if opus_repacketizer_out(rp, po, len_0 - 1 as libc::c_int) != -(2 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1691 as libc::c_int,
                );
            }
            cfgs += 1;
            if len_0 > 1 as libc::c_int {
                if opus_repacketizer_out(rp, po, 1 as libc::c_int) != -(2 as libc::c_int) {
                    _test_failed(
                        b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                        1695 as libc::c_int,
                    );
                }
                cfgs += 1;
            }
            if opus_repacketizer_out(rp, po, 0 as libc::c_int) != -(2 as libc::c_int) {
                _test_failed(
                    b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
                    1698 as libc::c_int,
                );
            }
            cfgs += 1;
            i += 1;
        }
        j += 1;
    }
    *po.offset(0 as libc::c_int as isize) = 'O' as i32 as libc::c_uchar;
    *po.offset(1 as libc::c_int as isize) = 'p' as i32 as libc::c_uchar;
    if opus_packet_pad(po, 4 as libc::c_int, 4 as libc::c_int) != 0 as libc::c_int {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1705 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int)
        != 0 as libc::c_int
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1707 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_packet_pad(po, 4 as libc::c_int, 5 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1709 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 4 as libc::c_int, 5 as libc::c_int, 1 as libc::c_int)
        != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1711 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_packet_pad(po, 0 as libc::c_int, 5 as libc::c_int) != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1713 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 0 as libc::c_int, 5 as libc::c_int, 1 as libc::c_int)
        != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1715 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_packet_unpad(po, 0 as libc::c_int) != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1717 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_unpad(po, 0 as libc::c_int, 1 as libc::c_int) != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1719 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_packet_unpad(po, 4 as libc::c_int) != -(4 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1721 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_unpad(po, 4 as libc::c_int, 1 as libc::c_int) != -(4 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1723 as libc::c_int,
        );
    }
    cfgs += 1;
    *po.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    *po.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    *po.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    if opus_packet_pad(po, 5 as libc::c_int, 4 as libc::c_int) != -(1 as libc::c_int) {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1728 as libc::c_int,
        );
    }
    cfgs += 1;
    if opus_multistream_packet_pad(po, 5 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int)
        != -(1 as libc::c_int)
    {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1730 as libc::c_int,
        );
    }
    cfgs += 1;
    fprintf(
        stdout(),
        b"    opus_repacketizer_cat ........................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_repacketizer_out ........................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_repacketizer_out_range .................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_packet_pad .............................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_packet_unpad ............................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_packet_pad .................. OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_packet_unpad ................ OK.\n\0" as *const u8
            as *const libc::c_char,
    );
    opus_repacketizer_destroy(rp);
    cfgs += 1;
    free(packet as *mut libc::c_void);
    free(po as *mut libc::c_void);
    fprintf(
        stdout(),
        b"                        All repacketizer tests passed\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"                            (%7d API invocations)\n\0" as *const u8
            as *const libc::c_char,
        cfgs,
    );
    cfgs
}
#[c2rust::src_loc = "1766:1"]
pub unsafe fn test_malloc_fail() -> libc::c_int {
    fprintf(
        stdout(),
        b"\n  malloc() failure tests\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"  ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_decoder_create() ................... SKIPPED.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_encoder_create() ................... SKIPPED.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_repacketizer_create() .............. SKIPPED.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_decoder_create() ....... SKIPPED.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"    opus_multistream_encoder_create() ....... SKIPPED.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stdout(),
        b"(Test only supported with GLIBC and without valgrind)\n\0" as *const u8
            as *const libc::c_char,
    );
    0 as libc::c_int
}
#[c2rust::src_loc = "1875:1"]
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut total: i32 = 0;
    let mut oversion: *const libc::c_char = std::ptr::null::<libc::c_char>();
    if _argc > 1 as libc::c_int {
        fprintf(
            stderr(),
            b"Usage: %s\n\0" as *const u8 as *const libc::c_char,
            *_argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    iseed = 0 as libc::c_int as u32;
    oversion = opus_get_version_string();
    if oversion.is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1887 as libc::c_int,
        );
    }
    fprintf(
        stderr(),
        b"Testing the %s API deterministically\n\0" as *const u8 as *const libc::c_char,
        oversion,
    );
    if (opus_strerror(-(32768 as libc::c_int))).is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1889 as libc::c_int,
        );
    }
    if (opus_strerror(32767 as libc::c_int)).is_null() {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1890 as libc::c_int,
        );
    }
    if strlen(opus_strerror(0 as libc::c_int)) < 1 as libc::c_int as libc::c_ulong {
        _test_failed(
            b"tests/test_opus_api.c\0" as *const u8 as *const libc::c_char,
            1891 as libc::c_int,
        );
    }
    total = 4 as libc::c_int;
    total += test_dec_api();
    total += test_msdec_api();
    total += test_parse();
    total += test_enc_api();
    total += test_repacketizer_api();
    total += test_malloc_fail();
    fprintf(
        stderr(),
        b"\nAll API tests passed.\nThe libopus API was invoked %d times.\n\0" as *const u8
            as *const libc::c_char,
        total,
    );
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
