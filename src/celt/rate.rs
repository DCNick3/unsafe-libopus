#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
}
#[c2rust::src_loc = "39:9"]
pub const FINE_OFFSET: libc::c_int = 21 as libc::c_int;
#[c2rust::src_loc = "37:9"]
pub const MAX_FINE_BITS: libc::c_int = 8 as libc::c_int;
#[inline]
#[c2rust::src_loc = "48:1"]
pub unsafe fn get_pulses(i: libc::c_int) -> libc::c_int {
    return if i < 8 as libc::c_int {
        i
    } else {
        8 as libc::c_int + (i & 7 as libc::c_int) << (i >> 3 as libc::c_int) - 1 as libc::c_int
    };
}
#[inline]
#[c2rust::src_loc = "53:1"]
pub unsafe fn bits2pulses(
    m: *const OpusCustomMode,
    band: libc::c_int,
    mut LM: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut cache: *const libc::c_uchar = 0 as *const libc::c_uchar;
    LM += 1;
    cache = ((*m).cache.bits).offset(
        *((*m).cache.index).offset((LM * (*m).nbEBands + band) as isize) as libc::c_int as isize,
    );
    lo = 0 as libc::c_int;
    hi = *cache.offset(0 as libc::c_int as isize) as libc::c_int;
    bits -= 1;
    i = 0 as libc::c_int;
    while i < LOG_MAX_PSEUDO {
        let mid: libc::c_int = lo + hi + 1 as libc::c_int >> 1 as libc::c_int;
        if *cache.offset(mid as isize) as libc::c_int >= bits {
            hi = mid;
        } else {
            lo = mid;
        }
        i += 1;
    }
    if bits
        - (if lo == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            *cache.offset(lo as isize) as libc::c_int
        })
        <= *cache.offset(hi as isize) as libc::c_int - bits
    {
        return lo;
    } else {
        return hi;
    };
}
#[c2rust::src_loc = "33:9"]
pub const LOG_MAX_PSEUDO: libc::c_int = 6 as libc::c_int;
#[inline]
#[c2rust::src_loc = "80:1"]
pub unsafe fn pulses2bits(
    m: *const OpusCustomMode,
    band: libc::c_int,
    mut LM: libc::c_int,
    pulses: libc::c_int,
) -> libc::c_int {
    let mut cache: *const libc::c_uchar = 0 as *const libc::c_uchar;
    LM += 1;
    cache = ((*m).cache.bits).offset(
        *((*m).cache.index).offset((LM * (*m).nbEBands + band) as isize) as libc::c_int as isize,
    );
    return if pulses == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *cache.offset(pulses as isize) as libc::c_int + 1 as libc::c_int
    };
}
#[c2rust::src_loc = "41:9"]
pub const QTHETA_OFFSET_TWOPHASE: libc::c_int = 16 as libc::c_int;
#[c2rust::src_loc = "40:9"]
pub const QTHETA_OFFSET: libc::c_int = 4 as libc::c_int;

pub use self::arch_h::opus_val16;
use crate::celt::celt::celt_fatal;
use crate::celt::entcode::{celt_udiv, ec_ctx, BITRES};
use crate::celt::entdec::{ec_dec_bit_logp, ec_dec_uint};
use crate::celt::entenc::{ec_enc_bit_logp, ec_enc_uint};
use crate::celt::modes::OpusCustomMode;

#[c2rust::src_loc = "42:28"]
static mut LOG2_FRAC_TABLE: [libc::c_uchar; 24] = [
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
];
#[c2rust::src_loc = "246:9"]
pub const ALLOC_STEPS: libc::c_int = 6 as libc::c_int;
#[inline]
#[c2rust::src_loc = "248:1"]
unsafe fn interp_bits2pulses(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    skip_start: libc::c_int,
    bits1: *const libc::c_int,
    bits2: *const libc::c_int,
    thresh: *const libc::c_int,
    cap: *const libc::c_int,
    mut total: i32,
    mut _balance: *mut i32,
    skip_rsv: libc::c_int,
    intensity: *mut libc::c_int,
    mut intensity_rsv: libc::c_int,
    dual_stereo: *mut libc::c_int,
    mut dual_stereo_rsv: libc::c_int,
    bits: *mut libc::c_int,
    ebits: *mut libc::c_int,
    fine_priority: *mut libc::c_int,
    C: libc::c_int,
    LM: libc::c_int,
    ec: *mut ec_ctx,
    encode: libc::c_int,
    prev: libc::c_int,
    signalBandwidth: libc::c_int,
) -> libc::c_int {
    let mut psum: i32 = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut logM: libc::c_int = 0;
    let mut stereo: libc::c_int = 0;
    let mut codedBands: libc::c_int = -(1 as libc::c_int);
    let mut alloc_floor: libc::c_int = 0;
    let mut left: i32 = 0;
    let mut percoeff: i32 = 0;
    let mut done: libc::c_int = 0;
    let mut balance: i32 = 0;
    alloc_floor = C << BITRES;
    stereo = (C > 1 as libc::c_int) as libc::c_int;
    logM = LM << BITRES;
    lo = 0 as libc::c_int;
    hi = (1 as libc::c_int) << ALLOC_STEPS;
    i = 0 as libc::c_int;
    while i < ALLOC_STEPS {
        let mid: libc::c_int = lo + hi >> 1 as libc::c_int;
        psum = 0 as libc::c_int;
        done = 0 as libc::c_int;
        j = end;
        loop {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 > start) {
                break;
            }
            let tmp: libc::c_int =
                *bits1.offset(j as isize) + (mid * *bits2.offset(j as isize) >> ALLOC_STEPS);
            if tmp >= *thresh.offset(j as isize) || done != 0 {
                done = 1 as libc::c_int;
                psum += if tmp < *cap.offset(j as isize) {
                    tmp
                } else {
                    *cap.offset(j as isize)
                };
            } else if tmp >= alloc_floor {
                psum += alloc_floor;
            }
        }
        if psum > total {
            hi = mid;
        } else {
            lo = mid;
        }
        i += 1;
    }
    psum = 0 as libc::c_int;
    done = 0 as libc::c_int;
    j = end;
    loop {
        let fresh1 = j;
        j = j - 1;
        if !(fresh1 > start) {
            break;
        }
        let mut tmp_0: libc::c_int =
            *bits1.offset(j as isize) + (lo * *bits2.offset(j as isize) >> ALLOC_STEPS);
        if tmp_0 < *thresh.offset(j as isize) && done == 0 {
            if tmp_0 >= alloc_floor {
                tmp_0 = alloc_floor;
            } else {
                tmp_0 = 0 as libc::c_int;
            }
        } else {
            done = 1 as libc::c_int;
        }
        tmp_0 = if tmp_0 < *cap.offset(j as isize) {
            tmp_0
        } else {
            *cap.offset(j as isize)
        };
        *bits.offset(j as isize) = tmp_0;
        psum += tmp_0;
    }
    codedBands = end;
    loop {
        let mut band_width: libc::c_int = 0;
        let mut band_bits: libc::c_int = 0;
        let mut rem: libc::c_int = 0;
        j = codedBands - 1 as libc::c_int;
        if j <= skip_start {
            total += skip_rsv;
            break;
        } else {
            left = total - psum;
            percoeff = celt_udiv(
                left as u32,
                (*((*m).eBands).offset(codedBands as isize) as libc::c_int
                    - *((*m).eBands).offset(start as isize) as libc::c_int) as u32,
            ) as i32;
            left -= (*((*m).eBands).offset(codedBands as isize) as libc::c_int
                - *((*m).eBands).offset(start as isize) as libc::c_int)
                * percoeff;
            rem = if left
                - (*((*m).eBands).offset(j as isize) as libc::c_int
                    - *((*m).eBands).offset(start as isize) as libc::c_int)
                > 0 as libc::c_int
            {
                left - (*((*m).eBands).offset(j as isize) as libc::c_int
                    - *((*m).eBands).offset(start as isize) as libc::c_int)
            } else {
                0 as libc::c_int
            };
            band_width = *((*m).eBands).offset(codedBands as isize) as libc::c_int
                - *((*m).eBands).offset(j as isize) as libc::c_int;
            band_bits = *bits.offset(j as isize) + percoeff * band_width + rem;
            if band_bits
                >= (if *thresh.offset(j as isize)
                    > alloc_floor + ((1 as libc::c_int) << 3 as libc::c_int)
                {
                    *thresh.offset(j as isize)
                } else {
                    alloc_floor + ((1 as libc::c_int) << 3 as libc::c_int)
                })
            {
                if encode != 0 {
                    let mut depth_threshold: libc::c_int = 0;
                    if codedBands > 17 as libc::c_int {
                        depth_threshold = if j < prev {
                            7 as libc::c_int
                        } else {
                            9 as libc::c_int
                        };
                    } else {
                        depth_threshold = 0 as libc::c_int;
                    }
                    if codedBands <= start + 2 as libc::c_int
                        || band_bits
                            > depth_threshold * band_width << LM << BITRES >> 4 as libc::c_int
                            && j <= signalBandwidth
                    {
                        ec_enc_bit_logp(ec, 1 as libc::c_int, 1 as libc::c_int as libc::c_uint);
                        break;
                    } else {
                        ec_enc_bit_logp(ec, 0 as libc::c_int, 1 as libc::c_int as libc::c_uint);
                    }
                } else if ec_dec_bit_logp(ec, 1 as libc::c_int as libc::c_uint) != 0 {
                    break;
                }
                psum += (1 as libc::c_int) << BITRES;
                band_bits -= (1 as libc::c_int) << BITRES;
            }
            psum -= *bits.offset(j as isize) + intensity_rsv;
            if intensity_rsv > 0 as libc::c_int {
                intensity_rsv = LOG2_FRAC_TABLE[(j - start) as usize] as libc::c_int;
            }
            psum += intensity_rsv;
            if band_bits >= alloc_floor {
                psum += alloc_floor;
                *bits.offset(j as isize) = alloc_floor;
            } else {
                *bits.offset(j as isize) = 0 as libc::c_int;
            }
            codedBands -= 1;
        }
    }
    if !(codedBands > start) {
        celt_fatal(
            b"assertion failed: codedBands > start\0" as *const u8 as *const libc::c_char,
            b"celt/rate.c\0" as *const u8 as *const libc::c_char,
            391 as libc::c_int,
        );
    }
    if intensity_rsv > 0 as libc::c_int {
        if encode != 0 {
            *intensity = if *intensity < codedBands {
                *intensity
            } else {
                codedBands
            };
            ec_enc_uint(
                ec,
                (*intensity - start) as u32,
                (codedBands + 1 as libc::c_int - start) as u32,
            );
        } else {
            *intensity = (start as libc::c_uint).wrapping_add(ec_dec_uint(
                ec,
                (codedBands + 1 as libc::c_int - start) as u32,
            )) as libc::c_int;
        }
    } else {
        *intensity = 0 as libc::c_int;
    }
    if *intensity <= start {
        total += dual_stereo_rsv;
        dual_stereo_rsv = 0 as libc::c_int;
    }
    if dual_stereo_rsv > 0 as libc::c_int {
        if encode != 0 {
            ec_enc_bit_logp(ec, *dual_stereo, 1 as libc::c_int as libc::c_uint);
        } else {
            *dual_stereo = ec_dec_bit_logp(ec, 1 as libc::c_int as libc::c_uint);
        }
    } else {
        *dual_stereo = 0 as libc::c_int;
    }
    left = total - psum;
    percoeff = celt_udiv(
        left as u32,
        (*((*m).eBands).offset(codedBands as isize) as libc::c_int
            - *((*m).eBands).offset(start as isize) as libc::c_int) as u32,
    ) as i32;
    left -= (*((*m).eBands).offset(codedBands as isize) as libc::c_int
        - *((*m).eBands).offset(start as isize) as libc::c_int)
        * percoeff;
    j = start;
    while j < codedBands {
        *bits.offset(j as isize) += percoeff
            * (*((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *((*m).eBands).offset(j as isize) as libc::c_int);
        j += 1;
    }
    j = start;
    while j < codedBands {
        let tmp_1: libc::c_int = if left
            < *((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *((*m).eBands).offset(j as isize) as libc::c_int
        {
            left
        } else {
            *((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *((*m).eBands).offset(j as isize) as libc::c_int
        };
        *bits.offset(j as isize) += tmp_1;
        left -= tmp_1;
        j += 1;
    }
    balance = 0 as libc::c_int;
    j = start;
    while j < codedBands {
        let mut N0: libc::c_int = 0;
        let mut N: libc::c_int = 0;
        let mut den: libc::c_int = 0;
        let mut offset: libc::c_int = 0;
        let mut NClogN: libc::c_int = 0;
        let mut excess: i32 = 0;
        let mut bit: i32 = 0;
        if !(*bits.offset(j as isize) >= 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: bits[j] >= 0\0" as *const u8 as *const libc::c_char,
                b"celt/rate.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int,
            );
        }
        N0 = *((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(j as isize) as libc::c_int;
        N = N0 << LM;
        bit = *bits.offset(j as isize) + balance;
        if N > 1 as libc::c_int {
            excess = if bit - *cap.offset(j as isize) > 0 as libc::c_int {
                bit - *cap.offset(j as isize)
            } else {
                0 as libc::c_int
            };
            *bits.offset(j as isize) = bit - excess;
            den = C * N
                + (if C == 2 as libc::c_int
                    && N > 2 as libc::c_int
                    && *dual_stereo == 0
                    && j < *intensity
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                });
            NClogN = den * (*((*m).logN).offset(j as isize) as libc::c_int + logM);
            offset = (NClogN >> 1 as libc::c_int) - den * FINE_OFFSET;
            if N == 2 as libc::c_int {
                offset += den << BITRES >> 2 as libc::c_int;
            }
            if *bits.offset(j as isize) + offset < (den * 2 as libc::c_int) << BITRES {
                offset += NClogN >> 2 as libc::c_int;
            } else if *bits.offset(j as isize) + offset < (den * 3 as libc::c_int) << BITRES {
                offset += NClogN >> 3 as libc::c_int;
            }
            *ebits.offset(j as isize) = if 0 as libc::c_int
                > *bits.offset(j as isize) + offset + (den << 3 as libc::c_int - 1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                *bits.offset(j as isize) + offset + (den << 3 as libc::c_int - 1 as libc::c_int)
            };
            *ebits.offset(j as isize) =
                (celt_udiv(*ebits.offset(j as isize) as u32, den as u32) >> BITRES) as libc::c_int;
            if C * *ebits.offset(j as isize) > *bits.offset(j as isize) >> BITRES {
                *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> BITRES;
            }
            *ebits.offset(j as isize) = if *ebits.offset(j as isize) < 8 as libc::c_int {
                *ebits.offset(j as isize)
            } else {
                8 as libc::c_int
            };
            *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) * (den << BITRES)
                >= *bits.offset(j as isize) + offset)
                as libc::c_int;
            *bits.offset(j as isize) -= C * *ebits.offset(j as isize) << BITRES;
        } else {
            excess = if 0 as libc::c_int > bit - (C << 3 as libc::c_int) {
                0 as libc::c_int
            } else {
                bit - (C << 3 as libc::c_int)
            };
            *bits.offset(j as isize) = bit - excess;
            *ebits.offset(j as isize) = 0 as libc::c_int;
            *fine_priority.offset(j as isize) = 1 as libc::c_int;
        }
        if excess > 0 as libc::c_int {
            let mut extra_fine: libc::c_int = 0;
            let mut extra_bits: libc::c_int = 0;
            extra_fine = if (excess >> stereo + 3 as libc::c_int)
                < 8 as libc::c_int - *ebits.offset(j as isize)
            {
                excess >> stereo + 3 as libc::c_int
            } else {
                8 as libc::c_int - *ebits.offset(j as isize)
            };
            *ebits.offset(j as isize) += extra_fine;
            extra_bits = extra_fine * C << BITRES;
            *fine_priority.offset(j as isize) = (extra_bits >= excess - balance) as libc::c_int;
            excess -= extra_bits;
        }
        balance = excess;
        if !(*bits.offset(j as isize) >= 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: bits[j] >= 0\0" as *const u8 as *const libc::c_char,
                b"celt/rate.c\0" as *const u8 as *const libc::c_char,
                513 as libc::c_int,
            );
        }
        if !(*ebits.offset(j as isize) >= 0 as libc::c_int) {
            celt_fatal(
                b"assertion failed: ebits[j] >= 0\0" as *const u8 as *const libc::c_char,
                b"celt/rate.c\0" as *const u8 as *const libc::c_char,
                514 as libc::c_int,
            );
        }
        j += 1;
    }
    *_balance = balance;
    while j < end {
        *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> BITRES;
        if !(C * *ebits.offset(j as isize) << 3 as libc::c_int == *bits.offset(j as isize)) {
            celt_fatal(
                b"assertion failed: C*ebits[j]<<BITRES == bits[j]\0" as *const u8
                    as *const libc::c_char,
                b"celt/rate.c\0" as *const u8 as *const libc::c_char,
                524 as libc::c_int,
            );
        }
        *bits.offset(j as isize) = 0 as libc::c_int;
        *fine_priority.offset(j as isize) =
            (*ebits.offset(j as isize) < 1 as libc::c_int) as libc::c_int;
        j += 1;
    }
    return codedBands;
}
#[c2rust::src_loc = "532:1"]
pub unsafe fn clt_compute_allocation(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    offsets: *const libc::c_int,
    cap: *const libc::c_int,
    alloc_trim: libc::c_int,
    intensity: *mut libc::c_int,
    dual_stereo: *mut libc::c_int,
    mut total: i32,
    balance: *mut i32,
    pulses: *mut libc::c_int,
    ebits: *mut libc::c_int,
    fine_priority: *mut libc::c_int,
    C: libc::c_int,
    LM: libc::c_int,
    ec: *mut ec_ctx,
    encode: libc::c_int,
    prev: libc::c_int,
    signalBandwidth: libc::c_int,
) -> libc::c_int {
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut codedBands: libc::c_int = 0;
    let mut skip_start: libc::c_int = 0;
    let mut skip_rsv: libc::c_int = 0;
    let mut intensity_rsv: libc::c_int = 0;
    let mut dual_stereo_rsv: libc::c_int = 0;
    total = if total > 0 as libc::c_int {
        total
    } else {
        0 as libc::c_int
    };
    len = (*m).nbEBands;
    skip_start = start;
    skip_rsv = if total >= (1 as libc::c_int) << BITRES {
        (1 as libc::c_int) << BITRES
    } else {
        0 as libc::c_int
    };
    total -= skip_rsv;
    dual_stereo_rsv = 0 as libc::c_int;
    intensity_rsv = dual_stereo_rsv;
    if C == 2 as libc::c_int {
        intensity_rsv = LOG2_FRAC_TABLE[(end - start) as usize] as libc::c_int;
        if intensity_rsv > total {
            intensity_rsv = 0 as libc::c_int;
        } else {
            total -= intensity_rsv;
            dual_stereo_rsv = if total >= (1 as libc::c_int) << BITRES {
                (1 as libc::c_int) << BITRES
            } else {
                0 as libc::c_int
            };
            total -= dual_stereo_rsv;
        }
    }
    let vla = len as usize;
    let mut bits1: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = len as usize;
    let mut bits2: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = len as usize;
    let mut thresh: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = len as usize;
    let mut trim_offset: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_2);
    j = start;
    while j < end {
        *thresh.as_mut_ptr().offset(j as isize) = if C << 3 as libc::c_int
            > 3 as libc::c_int
                * (*((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
                    - *((*m).eBands).offset(j as isize) as libc::c_int)
                << LM
                << 3 as libc::c_int
                >> 4 as libc::c_int
        {
            C << 3 as libc::c_int
        } else {
            3 as libc::c_int
                * (*((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
                    - *((*m).eBands).offset(j as isize) as libc::c_int)
                << LM
                << 3 as libc::c_int
                >> 4 as libc::c_int
        };
        *trim_offset.as_mut_ptr().offset(j as isize) = C
            * (*((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
                - *((*m).eBands).offset(j as isize) as libc::c_int)
            * (alloc_trim - 5 as libc::c_int - LM)
            * (end - j - 1 as libc::c_int)
            * ((1 as libc::c_int) << LM + BITRES)
            >> 6 as libc::c_int;
        if (*((*m).eBands).offset((j + 1 as libc::c_int) as isize) as libc::c_int
            - *((*m).eBands).offset(j as isize) as libc::c_int)
            << LM
            == 1 as libc::c_int
        {
            *trim_offset.as_mut_ptr().offset(j as isize) -= C << BITRES;
        }
        j += 1;
    }
    lo = 1 as libc::c_int;
    hi = (*m).nbAllocVectors - 1 as libc::c_int;
    loop {
        let mut done: libc::c_int = 0 as libc::c_int;
        let mut psum: libc::c_int = 0 as libc::c_int;
        let mid: libc::c_int = lo + hi >> 1 as libc::c_int;
        j = end;
        loop {
            let fresh2 = j;
            j = j - 1;
            if !(fresh2 > start) {
                break;
            }
            let mut bitsj: libc::c_int = 0;
            let N: libc::c_int = *((*m).eBands).offset((j + 1 as libc::c_int) as isize)
                as libc::c_int
                - *((*m).eBands).offset(j as isize) as libc::c_int;
            bitsj = (C * N * *((*m).allocVectors).offset((mid * len + j) as isize) as libc::c_int)
                << LM
                >> 2 as libc::c_int;
            if bitsj > 0 as libc::c_int {
                bitsj = if 0 as libc::c_int > bitsj + *trim_offset.as_mut_ptr().offset(j as isize) {
                    0 as libc::c_int
                } else {
                    bitsj + *trim_offset.as_mut_ptr().offset(j as isize)
                };
            }
            bitsj += *offsets.offset(j as isize);
            if bitsj >= *thresh.as_mut_ptr().offset(j as isize) || done != 0 {
                done = 1 as libc::c_int;
                psum += if bitsj < *cap.offset(j as isize) {
                    bitsj
                } else {
                    *cap.offset(j as isize)
                };
            } else if bitsj >= C << BITRES {
                psum += C << BITRES;
            }
        }
        if psum > total {
            hi = mid - 1 as libc::c_int;
        } else {
            lo = mid + 1 as libc::c_int;
        }
        if !(lo <= hi) {
            break;
        }
    }
    let fresh3 = lo;
    lo = lo - 1;
    hi = fresh3;
    j = start;
    while j < end {
        let mut bits1j: libc::c_int = 0;
        let mut bits2j: libc::c_int = 0;
        let N_0: libc::c_int = *((*m).eBands).offset((j + 1 as libc::c_int) as isize)
            as libc::c_int
            - *((*m).eBands).offset(j as isize) as libc::c_int;
        bits1j = (C * N_0 * *((*m).allocVectors).offset((lo * len + j) as isize) as libc::c_int)
            << LM
            >> 2 as libc::c_int;
        bits2j = if hi >= (*m).nbAllocVectors {
            *cap.offset(j as isize)
        } else {
            (C * N_0 * *((*m).allocVectors).offset((hi * len + j) as isize) as libc::c_int) << LM
                >> 2 as libc::c_int
        };
        if bits1j > 0 as libc::c_int {
            bits1j = if 0 as libc::c_int > bits1j + *trim_offset.as_mut_ptr().offset(j as isize) {
                0 as libc::c_int
            } else {
                bits1j + *trim_offset.as_mut_ptr().offset(j as isize)
            };
        }
        if bits2j > 0 as libc::c_int {
            bits2j = if 0 as libc::c_int > bits2j + *trim_offset.as_mut_ptr().offset(j as isize) {
                0 as libc::c_int
            } else {
                bits2j + *trim_offset.as_mut_ptr().offset(j as isize)
            };
        }
        if lo > 0 as libc::c_int {
            bits1j += *offsets.offset(j as isize);
        }
        bits2j += *offsets.offset(j as isize);
        if *offsets.offset(j as isize) > 0 as libc::c_int {
            skip_start = j;
        }
        bits2j = if 0 as libc::c_int > bits2j - bits1j {
            0 as libc::c_int
        } else {
            bits2j - bits1j
        };
        *bits1.as_mut_ptr().offset(j as isize) = bits1j;
        *bits2.as_mut_ptr().offset(j as isize) = bits2j;
        j += 1;
    }
    codedBands = interp_bits2pulses(
        m,
        start,
        end,
        skip_start,
        bits1.as_mut_ptr(),
        bits2.as_mut_ptr(),
        thresh.as_mut_ptr(),
        cap,
        total,
        balance,
        skip_rsv,
        intensity,
        intensity_rsv,
        dual_stereo,
        dual_stereo_rsv,
        pulses,
        ebits,
        fine_priority,
        C,
        LM,
        ec,
        encode,
        prev,
        signalBandwidth,
    );
    return codedBands;
}
