#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(deprecated)]

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
        if _x > 2 {
            if _y < 3 {
                i = 0;
                while i < _y {
                    *_p = (*_p).offset(-1);
                    **_p = *_t.offset((i + 1) as isize);
                    i += 1;
                }
            }
        } else {
            *_t.offset(_x as isize) = *_t.offset((_x - _y) as isize);
            deb2_impl(_t, _p, _k, _x + 1, _y);
            i = *_t.offset((_x - _y) as isize) as i32 + 1;
            while i < _k {
                *_t.offset(_x as isize) = i as u8;
                deb2_impl(_t, _p, _k, _x + 1, _x);
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
                .wrapping_mul(2),
        ) as *mut u8;
        memset(
            t as *mut core::ffi::c_void,
            0,
            (::core::mem::size_of::<u8>() as u64)
                .wrapping_mul(_k as u64)
                .wrapping_mul(2),
        );
        p = &mut *_res.offset((_k * _k) as isize) as *mut u8;
        deb2_impl(t, &mut p, _k, 1, 1);
        free(t as *mut core::ffi::c_void);
    }
    pub static mut Rz: u32 = 0;
    pub static mut Rw: u32 = 0;
    #[inline]
    pub unsafe fn fast_rand() -> u32 {
        Rz = 36969_u32.wrapping_mul(Rz & 65535).wrapping_add(Rz >> 16);
        Rw = 18000_u32.wrapping_mul(Rw & 65535).wrapping_add(Rw >> 16);
        (Rz << 16).wrapping_add(Rw)
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
            opus_get_version_string()
        );
        eprintln!("and any relevant details about your system.");
        panic!("test failed");
    }

    use unsafe_libopus::externs::memset;
    use unsafe_libopus::externs::{free, malloc};
    use unsafe_libopus::opus_get_version_string;
}
pub use self::test_opus_common_h::{debruijn2, Rw, Rz, _test_failed, fast_rand, iseed};
use unsafe_libopus::externs::{memcpy, memset};

use unsafe_libopus::externs::{free, malloc};
use unsafe_libopus::{
    opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy,
    opus_decoder_get_nb_samples, opus_decoder_get_size, opus_get_version_string,
    opus_packet_get_nb_channels, opus_pcm_soft_clip, OpusDecoder,
};

pub unsafe fn test_decoder_code0(no_fuzz: bool) -> i32 {
    static mut fsv: [i32; 5] = [48000, 24000, 16000, 12000, 8000];
    let mut err: i32 = 0;
    let mut skip: i32 = 0;
    let mut plen: i32 = 0;
    let mut out_samples: i32 = 0;
    let mut fec: i32 = 0;
    let mut t: i32 = 0;
    let mut i: i32 = 0;
    let mut dec: [*mut OpusDecoder; 10] = [std::ptr::null_mut::<OpusDecoder>(); 10];
    let mut decsize: i32 = 0;
    let mut decbak: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
    let mut dec_final_range1: u32 = 0;
    let mut dec_final_range2: u32 = 0;
    let mut dec_final_acc: u32 = 0;
    let mut packet: *mut u8 = std::ptr::null_mut::<u8>();
    let mut modes: [u8; 4096] = [0; 4096];
    let mut outbuf_int: *mut i16 = std::ptr::null_mut::<i16>();
    let mut outbuf: *mut i16 = std::ptr::null_mut::<i16>();
    dec_final_range2 = 2;
    dec_final_range1 = dec_final_range2;
    packet = malloc((::core::mem::size_of::<u8>() as u64).wrapping_mul(1500)) as *mut u8;
    if packet.is_null() {
        _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 70);
    }
    outbuf_int = malloc(
        (::core::mem::size_of::<i16>() as u64)
            .wrapping_mul((5760 + 16) as u64)
            .wrapping_mul(2),
    ) as *mut i16;
    i = 0;
    while i < (5760 + 16) * 2 {
        *outbuf_int.offset(i as isize) = 32749 as i16;
        i += 1;
    }
    outbuf = &mut *outbuf_int.offset((8 * 2) as isize) as *mut i16;
    println!("  Starting {} decoders...", 5 * 2);
    t = 0;
    while t < 5 * 2 {
        let mut fs: i32 = fsv[(t >> 1) as usize];
        let mut c: i32 = (t & 1) + 1;
        err = -(3);
        dec[t as usize] = opus_decoder_create(fs, c, &mut err);
        if err != 0 || (dec[t as usize]).is_null() {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 83);
        }
        print!("    opus_decoder_create({:5},{}): OK. Copy ", fs, c);
        let mut dec2: *mut OpusDecoder = std::ptr::null_mut::<OpusDecoder>();
        dec2 = malloc(opus_decoder_get_size(c) as u64) as *mut OpusDecoder;
        if dec2.is_null() {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 89);
        }
        memcpy(
            dec2 as *mut core::ffi::c_void,
            dec[t as usize] as *const core::ffi::c_void,
            opus_decoder_get_size(c) as u64,
        );
        memset(
            dec[t as usize] as *mut core::ffi::c_void,
            255,
            opus_decoder_get_size(c) as u64,
        );
        opus_decoder_destroy(dec[t as usize]);
        println!("OK.");
        dec[t as usize] = dec2;
        t += 1;
    }
    decsize = opus_decoder_get_size(1);
    decbak = malloc(decsize as u64) as *mut OpusDecoder;
    if decbak.is_null() {
        _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 100);
    }
    t = 0;
    while t < 5 * 2 {
        let mut factor: i32 = 48000 / fsv[(t >> 1) as usize];
        fec = 0;
        while fec < 2 {
            let mut dur: i32 = 0;
            out_samples = opus_decode(
                &mut *dec[t as usize],
                std::ptr::null::<u8>(),
                0,
                outbuf,
                120 / factor,
                fec,
            );
            if out_samples != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 110);
            }
            if opus_decoder_ctl!(&mut *dec[t as usize], 4039, &mut dur) != 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 111);
            }
            if dur != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 112);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                std::ptr::null::<u8>(),
                0,
                outbuf,
                120 / factor + 2,
                fec,
            );
            if out_samples != -1 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 116);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                std::ptr::null::<u8>(),
                -1,
                outbuf,
                120 / factor,
                fec,
            );
            if out_samples != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 120);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                std::ptr::null::<u8>(),
                1,
                outbuf,
                120 / factor,
                fec,
            );
            if out_samples != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 122);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                std::ptr::null::<u8>(),
                10,
                outbuf,
                120 / factor,
                fec,
            );
            if out_samples != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 124);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                std::ptr::null::<u8>(),
                fast_rand() as i32,
                outbuf,
                120 / factor,
                fec,
            );
            if out_samples != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 126);
            }
            if opus_decoder_ctl!(&mut *dec[t as usize], 4039, &mut dur) != 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 127);
            }
            if dur != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 128);
            }
            out_samples = opus_decode(&mut *dec[t as usize], packet, 0, outbuf, 120 / factor, fec);
            if out_samples != 120 / factor {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 132);
            }
            *outbuf.offset(0 as isize) = 32749 as i16;
            out_samples = opus_decode(&mut *dec[t as usize], packet, 0, outbuf, 0, fec);
            if out_samples > 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 137);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                packet,
                0,
                std::ptr::null_mut::<i16>(),
                0,
                fec,
            );
            if out_samples > 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 146);
            }
            if *outbuf.offset(0 as isize) as i32 != 32749 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 147);
            }
            out_samples = opus_decode(&mut *dec[t as usize], packet, -1, outbuf, 5760, fec);
            if out_samples >= 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 151);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                packet,
                -(2147483647) - 1,
                outbuf,
                5760,
                fec,
            );
            if out_samples >= 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 153);
            }
            out_samples = opus_decode(&mut *dec[t as usize], packet, -1, outbuf, -1, fec);
            if out_samples >= 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 155);
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                packet,
                1,
                outbuf,
                5760,
                if fec != 0 { -1 } else { 2 },
            );
            if out_samples >= 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 159);
            }
            if opus_decoder_ctl!(&mut *dec[t as usize], 4028) != 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 162);
            }
            fec += 1;
        }
        t += 1;
    }
    println!("  dec[all] initial frame PLC OK.");
    i = 0;
    while i < 64 {
        let mut dur_0: i32 = 0;
        let mut j: i32 = 0;
        let mut expected: [i32; 10] = [0; 10];
        *packet.offset(0 as isize) = (i << 2) as u8;
        *packet.offset(1 as isize) = 255;
        *packet.offset(2 as isize) = 255;
        err = opus_packet_get_nb_channels(packet);
        if err != (i & 1) + 1 {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 176);
        }
        t = 0;
        while t < 5 * 2 {
            expected[t as usize] = opus_decoder_get_nb_samples(
                &mut *dec[t as usize],
                std::slice::from_raw_parts(packet, 1),
            );
            if expected[t as usize] > 2880 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 180);
            }
            t += 1;
        }
        j = 0;
        while j < 256 {
            *packet.offset(1 as isize) = j as u8;
            t = 0;
            while t < 5 * 2 {
                out_samples = opus_decode(&mut *dec[t as usize], packet, 3, outbuf, 5760, 0);
                if out_samples != expected[t as usize] {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 189);
                }
                if opus_decoder_ctl!(&mut *dec[t as usize], 4039, &mut dur_0) != 0 {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 190);
                }
                if dur_0 != out_samples {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 191);
                }
                opus_decoder_ctl!(&mut *dec[t as usize], 4031, &mut dec_final_range1);
                if t == 0 {
                    dec_final_range2 = dec_final_range1;
                } else if dec_final_range1 != dec_final_range2 {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 194);
                }
                t += 1;
            }
            j += 1;
        }
        t = 0;
        while t < 5 * 2 {
            let mut factor_0: i32 = 48000 / fsv[(t >> 1) as usize];
            j = 0;
            while j < 6 {
                out_samples = opus_decode(
                    &mut *dec[t as usize],
                    std::ptr::null::<u8>(),
                    0,
                    outbuf,
                    expected[t as usize],
                    0,
                );
                if out_samples != expected[t as usize] {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 204);
                }
                if opus_decoder_ctl!(&mut *dec[t as usize], 4039, &mut dur_0) != 0 {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 205);
                }
                if dur_0 != out_samples {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 206);
                }
                j += 1;
            }
            if expected[t as usize] != 120 / factor_0 {
                out_samples = opus_decode(
                    &mut *dec[t as usize],
                    std::ptr::null::<u8>(),
                    0,
                    outbuf,
                    120 / factor_0,
                    0,
                );
                if out_samples != 120 / factor_0 {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 213);
                }
                if opus_decoder_ctl!(&mut *dec[t as usize], 4039, &mut dur_0) != 0 {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 214);
                }
                if dur_0 != out_samples {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 215);
                }
            }
            out_samples = opus_decode(
                &mut *dec[t as usize],
                packet,
                2,
                outbuf,
                expected[t as usize] - 1,
                0,
            );
            if out_samples > 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 218);
            }
            t += 1;
        }
        i += 1;
    }
    println!("  dec[all] all 2-byte prefix for length 2 and PLC, all modes (64) OK.",);
    if no_fuzz {
        println!("  Skipping many tests which fuzz the decoder as requested.");
        free(decbak as *mut core::ffi::c_void);
        t = 0;
        while t < 5 * 2 {
            opus_decoder_destroy(dec[t as usize]);
            t += 1;
        }
        println!("  Decoders stopped.");
        err = 0;
        i = 0;
        while i < 8 * 2 {
            err |= (*outbuf_int.offset(i as isize) as i32 != 32749) as i32;
            i += 1;
        }
        i = 5760 * 2;
        while i < (5760 + 8) * 2 {
            err |= (*outbuf.offset(i as isize) as i32 != 32749) as i32;
            i += 1;
        }
        if err != 0 {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 233);
        }
        free(outbuf_int as *mut core::ffi::c_void);
        free(packet as *mut core::ffi::c_void);
        return 0;
    }
    static mut cmodes: [i32; 4] = [16, 20, 24, 28];
    static mut cres: [u32; 4] = [
        116290185,
        2172123586 as u32,
        2172123586 as u32,
        2172123586 as u32,
    ];
    static mut lres: [u32; 3] = [3285687739 as u32, 1481572662, 694350475];
    static mut lmodes: [i32; 3] = [0, 4, 8];
    let mut mode: i32 = (fast_rand()).wrapping_rem(4) as i32;
    *packet.offset(0 as isize) = (cmodes[mode as usize] << 3) as u8;
    dec_final_acc = 0;
    t = (fast_rand()).wrapping_rem(10) as i32;
    i = 0;
    while i < 65536 {
        let mut factor_1: i32 = 48000 / fsv[(t >> 1) as usize];
        *packet.offset(1 as isize) = (i >> 8) as u8;
        *packet.offset(2 as isize) = (i & 255) as u8;
        *packet.offset(3 as isize) = 255;
        out_samples = opus_decode(&mut *dec[t as usize], packet, 4, outbuf, 5760, 0);
        if out_samples != 120 / factor_1 {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 260);
        }
        opus_decoder_ctl!(&mut *dec[t as usize], 4031, &mut dec_final_range1);
        dec_final_acc = (dec_final_acc as u32).wrapping_add(dec_final_range1) as u32 as u32;
        i += 1;
    }
    if dec_final_acc != cres[mode as usize] {
        _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 264);
    }
    println!(
        "  dec[{:3}] all 3-byte prefix for length 4, mode {:2} OK.",
        t, cmodes[mode as usize]
    );
    mode = (fast_rand()).wrapping_rem(3) as i32;
    *packet.offset(0 as isize) = (lmodes[mode as usize] << 3) as u8;
    dec_final_acc = 0;
    t = (fast_rand()).wrapping_rem(10) as i32;
    i = 0;
    while i < 65536 {
        let mut factor_2: i32 = 48000 / fsv[(t >> 1) as usize];
        *packet.offset(1 as isize) = (i >> 8) as u8;
        *packet.offset(2 as isize) = (i & 255) as u8;
        *packet.offset(3 as isize) = 255;
        out_samples = opus_decode(&mut *dec[t as usize], packet, 4, outbuf, 5760, 0);
        if out_samples != 480 / factor_2 {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 278);
        }
        opus_decoder_ctl!(&mut *dec[t as usize], 4031, &mut dec_final_range1);
        dec_final_acc = (dec_final_acc as u32).wrapping_add(dec_final_range1) as u32 as u32;
        i += 1;
    }
    if dec_final_acc != lres[mode as usize] {
        _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 282);
    }
    println!(
        "  dec[{:3}] all 3-byte prefix for length 4, mode {:2} OK.",
        t, lmodes[mode as usize]
    );
    skip = (fast_rand()).wrapping_rem(7) as i32;
    i = 0;
    while i < 64 {
        let mut j_0: i32 = 0;
        let mut expected_0: [i32; 10] = [0; 10];
        *packet.offset(0 as isize) = (i << 2) as u8;
        t = 0;
        while t < 5 * 2 {
            expected_0[t as usize] = opus_decoder_get_nb_samples(
                &mut *dec[t as usize],
                std::slice::from_raw_parts(packet, 1),
            );
            t += 1;
        }
        j_0 = 2 + skip;
        while j_0 < 1275 {
            let mut jj: i32 = 0;
            jj = 0;
            while jj < j_0 {
                *packet.offset((jj + 1) as isize) = (fast_rand() & 255) as u8;
                jj += 1;
            }
            t = 0;
            while t < 5 * 2 {
                out_samples = opus_decode(&mut *dec[t as usize], packet, j_0 + 1, outbuf, 5760, 0);
                if out_samples != expected_0[t as usize] {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 299);
                }
                opus_decoder_ctl!(&mut *dec[t as usize], 4031, &mut dec_final_range1);
                if t == 0 {
                    dec_final_range2 = dec_final_range1;
                } else if dec_final_range1 != dec_final_range2 {
                    _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 302);
                }
                t += 1;
            }
            j_0 += 4;
        }
        i += 1;
    }
    println!(
        "  dec[all] random packets, all modes (64), every 4th size from from {} bytes to maximum OK.",
        2 + skip,
    );
    debruijn2(64, modes.as_mut_ptr());
    plen = (fast_rand())
        .wrapping_rem(18)
        .wrapping_add(3)
        .wrapping_mul(8)
        .wrapping_add(skip as u32)
        .wrapping_add(3) as i32;
    i = 0;
    while i < 4096 {
        let mut j_1: i32 = 0;
        let mut expected_1: [i32; 10] = [0; 10];
        *packet.offset(0 as isize) = ((modes[i as usize] as i32) << 2) as u8;
        t = 0;
        while t < 5 * 2 {
            expected_1[t as usize] = opus_decoder_get_nb_samples(
                &mut *dec[t as usize],
                std::slice::from_raw_parts(packet, plen as usize),
            );
            t += 1;
        }
        j_1 = 0;
        while j_1 < plen {
            *packet.offset((j_1 + 1) as isize) = ((fast_rand() | fast_rand()) & 255) as u8;
            j_1 += 1;
        }
        memcpy(
            decbak as *mut core::ffi::c_void,
            dec[0 as usize] as *const core::ffi::c_void,
            decsize as u64,
        );
        if opus_decode(
            &mut *decbak,
            packet,
            plen + 1,
            outbuf,
            expected_1[0 as usize],
            1,
        ) != expected_1[0 as usize]
        {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 317);
        }
        memcpy(
            decbak as *mut core::ffi::c_void,
            dec[0 as usize] as *const core::ffi::c_void,
            decsize as u64,
        );
        if opus_decode(&mut *decbak, std::ptr::null::<u8>(), 0, outbuf, 5760, 1) < 20 {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 319);
        }
        memcpy(
            decbak as *mut core::ffi::c_void,
            dec[0 as usize] as *const core::ffi::c_void,
            decsize as u64,
        );
        if opus_decode(&mut *decbak, std::ptr::null::<u8>(), 0, outbuf, 5760, 0) < 20 {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 321);
        }
        t = 0;
        while t < 5 * 2 {
            let mut dur_1: i32 = 0;
            out_samples = opus_decode(&mut *dec[t as usize], packet, plen + 1, outbuf, 5760, 0);
            if out_samples != expected_1[t as usize] {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 326);
            }
            if t == 0 {
                dec_final_range2 = dec_final_range1;
            } else if dec_final_range1 != dec_final_range2 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 328);
            }
            if opus_decoder_ctl!(&mut *dec[t as usize], 4039, &mut dur_1) != 0 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 329);
            }
            if dur_1 != out_samples {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 330);
            }
            t += 1;
        }
        i += 1;
    }
    println!(
        "  dec[all] random packets, all mode pairs (4096), {} bytes/frame OK.",
        plen + 1
    );
    plen = (fast_rand())
        .wrapping_rem(18)
        .wrapping_add(3)
        .wrapping_mul(8)
        .wrapping_add(skip as u32)
        .wrapping_add(3) as i32;

    t = {
        let mut t = [0];
        getrandom::getrandom(&mut t).unwrap();
        t[0] as i32 & 0x3
    };
    i = 0;
    while i < 4096 {
        let mut count: i32 = 0;
        let mut j_2: i32 = 0;
        let mut expected_2: i32 = 0;
        *packet.offset(0 as isize) = ((modes[i as usize] as i32) << 2) as u8;
        expected_2 = opus_decoder_get_nb_samples(
            &mut *dec[t as usize],
            std::slice::from_raw_parts(packet, plen as usize),
        );
        count = 0;
        while count < 10 {
            j_2 = 0;
            while j_2 < plen {
                *packet.offset((j_2 + 1) as isize) = ((fast_rand() | fast_rand()) & 255) as u8;
                j_2 += 1;
            }
            out_samples = opus_decode(&mut *dec[t as usize], packet, plen + 1, outbuf, 5760, 0);
            if out_samples != expected_2 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 346);
            }
            count += 1;
        }
        i += 1;
    }
    println!(
        "  dec[{}] random packets, all mode pairs (4096)*10, {} bytes/frame OK.",
        t,
        plen + 1
    );
    let mut tmodes: [i32; 1] = [(25) << 2];
    let mut tseeds: [u32; 1] = [140441];
    let mut tlen: [i32; 1] = [157];
    let mut tret: [i32; 1] = [480];
    t = (fast_rand() & 1) as i32;
    i = 0;
    while i < 1 {
        let mut j_3: i32 = 0;
        *packet.offset(0 as isize) = tmodes[i as usize] as u8;
        Rz = tseeds[i as usize];
        Rw = Rz;
        j_3 = 1;
        while j_3 < tlen[i as usize] {
            *packet.offset(j_3 as isize) = (fast_rand() & 255) as u8;
            j_3 += 1;
        }
        out_samples = opus_decode(
            &mut *dec[t as usize],
            packet,
            tlen[i as usize],
            outbuf,
            5760,
            0,
        );
        if out_samples != tret[i as usize] {
            _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 364);
        }
        i += 1;
    }
    println!("  dec[{:3}] pre-selected random packets OK.", t);
    free(decbak as *mut core::ffi::c_void);
    t = 0;
    while t < 5 * 2 {
        opus_decoder_destroy(dec[t as usize]);
        t += 1;
    }
    println!("  Decoders stopped.");
    err = 0;
    i = 0;
    while i < 8 * 2 {
        err |= (*outbuf_int.offset(i as isize) as i32 != 32749) as i32;
        i += 1;
    }
    i = 5760 * 2;
    while i < (5760 + 8) * 2 {
        err |= (*outbuf.offset(i as isize) as i32 != 32749) as i32;
        i += 1;
    }
    if err != 0 {
        _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 376);
    }
    free(outbuf_int as *mut core::ffi::c_void);
    free(packet as *mut core::ffi::c_void);
    0
}
pub unsafe fn test_soft_clip() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: [f32; 1024] = [0.; 1024];
    let mut s: [f32; 8] = [
        0 as f32, 0 as f32, 0 as f32, 0 as f32, 0 as f32, 0 as f32, 0 as f32, 0 as f32,
    ];
    println!("  Testing opus_pcm_soft_clip... ");
    i = 0;
    while i < 1024 {
        j = 0;
        while j < 1024 {
            x[j as usize] = (j & 255) as f32 * (1 as f32 / 32.0f32) - 4.0f32;
            j += 1;
        }
        opus_pcm_soft_clip(
            &mut *x.as_mut_ptr().offset(i as isize),
            1024 - i,
            1,
            s.as_mut_ptr(),
        );
        j = i;
        while j < 1024 {
            if x[j as usize] > 1.0f32 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 399);
            }
            if x[j as usize] < -1.0f32 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 400);
            }
            j += 1;
        }
        i += 1;
    }
    i = 1;
    while i < 9 {
        j = 0;
        while j < 1024 {
            x[j as usize] = (j & 255) as f32 * (1 as f32 / 32.0f32) - 4.0f32;
            j += 1;
        }
        opus_pcm_soft_clip(x.as_mut_ptr(), 1024 / i, i, s.as_mut_ptr());
        j = 0;
        while j < 1024 / i * i {
            if x[j as usize] > 1.0f32 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 412);
            }
            if x[j as usize] < -1.0f32 {
                _test_failed(b"tests/test_opus_decode.c\0" as *const u8 as *const i8, 413);
            }
            j += 1;
        }
        i += 1;
    }
    opus_pcm_soft_clip(x.as_mut_ptr(), 0, 1, s.as_mut_ptr());
    opus_pcm_soft_clip(x.as_mut_ptr(), 1, 0, s.as_mut_ptr());
    opus_pcm_soft_clip(x.as_mut_ptr(), 1, 1, std::ptr::null_mut::<f32>());
    opus_pcm_soft_clip(x.as_mut_ptr(), 1, -1, s.as_mut_ptr());
    opus_pcm_soft_clip(x.as_mut_ptr(), -1, 1, s.as_mut_ptr());
    opus_pcm_soft_clip(std::ptr::null_mut::<f32>(), 1, 1, s.as_mut_ptr());
    println!("OK.");
}

// make dummy arguments
// rust's test harness has its own arguments and will handle them itself
// not sure of the best way to pass arguments except modifying the code rn...
// provide a fixed seed 42
const DUMMY_ARGS: &[&str] = &["test_opus_decode", "42"];

unsafe fn main_0() -> i32 {
    let mut args = DUMMY_ARGS.into_iter().map(|v| v.to_string()); // std::env::args();
    let _argv0 = args.next().unwrap();

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
                let mut v = [0; 4];
                getrandom::getrandom(&mut v).unwrap();
                let v = u32::from_le_bytes(v);

                eprintln!("Using random seed: {}", v);
                v
            }
        },
    };

    Rz = iseed;
    Rw = Rz;

    let oversion = opus_get_version_string();

    eprintln!(
        "Testing {} decoder. Random seed: {} ({:4X})",
        oversion,
        iseed,
        (fast_rand() % 65535)
    );
    test_decoder_code0(std::env::var("TEST_OPUS_NOFUZZ").is_ok());
    test_soft_clip();
    0
}

#[test]
fn test_opus_decode() {
    assert_eq!(unsafe { main_0() }, 0, "Test returned a non-zero exit code");
}
