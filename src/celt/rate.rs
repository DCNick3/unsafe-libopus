pub mod arch_h {
    pub type opus_val16 = f32;
}
pub const FINE_OFFSET: i32 = 21 as i32;
pub const MAX_FINE_BITS: i32 = 8 as i32;
#[inline]
pub unsafe fn get_pulses(i: i32) -> i32 {
    return if i < 8 as i32 {
        i
    } else {
        8 as i32 + (i & 7 as i32) << (i >> 3 as i32) - 1 as i32
    };
}
#[inline]
pub unsafe fn bits2pulses(m: *const OpusCustomMode, band: i32, mut LM: i32, mut bits: i32) -> i32 {
    let mut i: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut cache: *const u8 = 0 as *const u8;
    LM += 1;
    cache = ((*m).cache.bits)
        .offset(*((*m).cache.index).offset((LM * (*m).nbEBands + band) as isize) as i32 as isize);
    lo = 0 as i32;
    hi = *cache.offset(0 as i32 as isize) as i32;
    bits -= 1;
    i = 0 as i32;
    while i < LOG_MAX_PSEUDO {
        let mid: i32 = lo + hi + 1 as i32 >> 1 as i32;
        if *cache.offset(mid as isize) as i32 >= bits {
            hi = mid;
        } else {
            lo = mid;
        }
        i += 1;
    }
    if bits
        - (if lo == 0 as i32 {
            -(1 as i32)
        } else {
            *cache.offset(lo as isize) as i32
        })
        <= *cache.offset(hi as isize) as i32 - bits
    {
        return lo;
    } else {
        return hi;
    };
}
pub const LOG_MAX_PSEUDO: i32 = 6 as i32;
#[inline]
pub unsafe fn pulses2bits(m: *const OpusCustomMode, band: i32, mut LM: i32, pulses: i32) -> i32 {
    let mut cache: *const u8 = 0 as *const u8;
    LM += 1;
    cache = ((*m).cache.bits)
        .offset(*((*m).cache.index).offset((LM * (*m).nbEBands + band) as isize) as i32 as isize);
    return if pulses == 0 as i32 {
        0 as i32
    } else {
        *cache.offset(pulses as isize) as i32 + 1 as i32
    };
}
pub const QTHETA_OFFSET_TWOPHASE: i32 = 16 as i32;
pub const QTHETA_OFFSET: i32 = 4 as i32;

pub use self::arch_h::opus_val16;
use crate::celt::celt::celt_fatal;
use crate::celt::entcode::{celt_udiv, ec_ctx, BITRES};
use crate::celt::entdec::{ec_dec_bit_logp, ec_dec_uint};
use crate::celt::entenc::{ec_enc_bit_logp, ec_enc_uint};
use crate::celt::modes::OpusCustomMode;

static mut LOG2_FRAC_TABLE: [u8; 24] = [
    0 as i32 as u8,
    8 as i32 as u8,
    13 as i32 as u8,
    16 as i32 as u8,
    19 as i32 as u8,
    21 as i32 as u8,
    23 as i32 as u8,
    24 as i32 as u8,
    26 as i32 as u8,
    27 as i32 as u8,
    28 as i32 as u8,
    29 as i32 as u8,
    30 as i32 as u8,
    31 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    33 as i32 as u8,
    34 as i32 as u8,
    34 as i32 as u8,
    35 as i32 as u8,
    36 as i32 as u8,
    36 as i32 as u8,
    37 as i32 as u8,
    37 as i32 as u8,
];
pub const ALLOC_STEPS: i32 = 6 as i32;
#[inline]
unsafe fn interp_bits2pulses(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    skip_start: i32,
    bits1: *const i32,
    bits2: *const i32,
    thresh: *const i32,
    cap: *const i32,
    mut total: i32,
    mut _balance: *mut i32,
    skip_rsv: i32,
    intensity: *mut i32,
    mut intensity_rsv: i32,
    dual_stereo: *mut i32,
    mut dual_stereo_rsv: i32,
    bits: *mut i32,
    ebits: *mut i32,
    fine_priority: *mut i32,
    C: i32,
    LM: i32,
    ec: *mut ec_ctx,
    encode: i32,
    prev: i32,
    signalBandwidth: i32,
) -> i32 {
    let mut psum: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut logM: i32 = 0;
    let mut stereo: i32 = 0;
    let mut codedBands: i32 = -(1 as i32);
    let mut alloc_floor: i32 = 0;
    let mut left: i32 = 0;
    let mut percoeff: i32 = 0;
    let mut done: i32 = 0;
    let mut balance: i32 = 0;
    alloc_floor = C << BITRES;
    stereo = (C > 1 as i32) as i32;
    logM = LM << BITRES;
    lo = 0 as i32;
    hi = (1 as i32) << ALLOC_STEPS;
    i = 0 as i32;
    while i < ALLOC_STEPS {
        let mid: i32 = lo + hi >> 1 as i32;
        psum = 0 as i32;
        done = 0 as i32;
        j = end;
        loop {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 > start) {
                break;
            }
            let tmp: i32 =
                *bits1.offset(j as isize) + (mid * *bits2.offset(j as isize) >> ALLOC_STEPS);
            if tmp >= *thresh.offset(j as isize) || done != 0 {
                done = 1 as i32;
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
    psum = 0 as i32;
    done = 0 as i32;
    j = end;
    loop {
        let fresh1 = j;
        j = j - 1;
        if !(fresh1 > start) {
            break;
        }
        let mut tmp_0: i32 =
            *bits1.offset(j as isize) + (lo * *bits2.offset(j as isize) >> ALLOC_STEPS);
        if tmp_0 < *thresh.offset(j as isize) && done == 0 {
            if tmp_0 >= alloc_floor {
                tmp_0 = alloc_floor;
            } else {
                tmp_0 = 0 as i32;
            }
        } else {
            done = 1 as i32;
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
        let mut band_width: i32 = 0;
        let mut band_bits: i32 = 0;
        let mut rem: i32 = 0;
        j = codedBands - 1 as i32;
        if j <= skip_start {
            total += skip_rsv;
            break;
        } else {
            left = total - psum;
            percoeff = celt_udiv(
                left as u32,
                (*((*m).eBands).offset(codedBands as isize) as i32
                    - *((*m).eBands).offset(start as isize) as i32) as u32,
            ) as i32;
            left -= (*((*m).eBands).offset(codedBands as isize) as i32
                - *((*m).eBands).offset(start as isize) as i32)
                * percoeff;
            rem = if left
                - (*((*m).eBands).offset(j as isize) as i32
                    - *((*m).eBands).offset(start as isize) as i32)
                > 0 as i32
            {
                left - (*((*m).eBands).offset(j as isize) as i32
                    - *((*m).eBands).offset(start as isize) as i32)
            } else {
                0 as i32
            };
            band_width = *((*m).eBands).offset(codedBands as isize) as i32
                - *((*m).eBands).offset(j as isize) as i32;
            band_bits = *bits.offset(j as isize) + percoeff * band_width + rem;
            if band_bits
                >= (if *thresh.offset(j as isize) > alloc_floor + ((1 as i32) << 3 as i32) {
                    *thresh.offset(j as isize)
                } else {
                    alloc_floor + ((1 as i32) << 3 as i32)
                })
            {
                if encode != 0 {
                    let mut depth_threshold: i32 = 0;
                    if codedBands > 17 as i32 {
                        depth_threshold = if j < prev { 7 as i32 } else { 9 as i32 };
                    } else {
                        depth_threshold = 0 as i32;
                    }
                    if codedBands <= start + 2 as i32
                        || band_bits > depth_threshold * band_width << LM << BITRES >> 4 as i32
                            && j <= signalBandwidth
                    {
                        ec_enc_bit_logp(ec, 1 as i32, 1 as i32 as u32);
                        break;
                    } else {
                        ec_enc_bit_logp(ec, 0 as i32, 1 as i32 as u32);
                    }
                } else if ec_dec_bit_logp(ec, 1 as i32 as u32) != 0 {
                    break;
                }
                psum += (1 as i32) << BITRES;
                band_bits -= (1 as i32) << BITRES;
            }
            psum -= *bits.offset(j as isize) + intensity_rsv;
            if intensity_rsv > 0 as i32 {
                intensity_rsv = LOG2_FRAC_TABLE[(j - start) as usize] as i32;
            }
            psum += intensity_rsv;
            if band_bits >= alloc_floor {
                psum += alloc_floor;
                *bits.offset(j as isize) = alloc_floor;
            } else {
                *bits.offset(j as isize) = 0 as i32;
            }
            codedBands -= 1;
        }
    }
    if !(codedBands > start) {
        celt_fatal(
            b"assertion failed: codedBands > start\0" as *const u8 as *const i8,
            b"celt/rate.c\0" as *const u8 as *const i8,
            391 as i32,
        );
    }
    if intensity_rsv > 0 as i32 {
        if encode != 0 {
            *intensity = if *intensity < codedBands {
                *intensity
            } else {
                codedBands
            };
            ec_enc_uint(
                ec,
                (*intensity - start) as u32,
                (codedBands + 1 as i32 - start) as u32,
            );
        } else {
            *intensity = (start as u32)
                .wrapping_add(ec_dec_uint(ec, (codedBands + 1 as i32 - start) as u32))
                as i32;
        }
    } else {
        *intensity = 0 as i32;
    }
    if *intensity <= start {
        total += dual_stereo_rsv;
        dual_stereo_rsv = 0 as i32;
    }
    if dual_stereo_rsv > 0 as i32 {
        if encode != 0 {
            ec_enc_bit_logp(ec, *dual_stereo, 1 as i32 as u32);
        } else {
            *dual_stereo = ec_dec_bit_logp(ec, 1 as i32 as u32);
        }
    } else {
        *dual_stereo = 0 as i32;
    }
    left = total - psum;
    percoeff = celt_udiv(
        left as u32,
        (*((*m).eBands).offset(codedBands as isize) as i32
            - *((*m).eBands).offset(start as isize) as i32) as u32,
    ) as i32;
    left -= (*((*m).eBands).offset(codedBands as isize) as i32
        - *((*m).eBands).offset(start as isize) as i32)
        * percoeff;
    j = start;
    while j < codedBands {
        *bits.offset(j as isize) += percoeff
            * (*((*m).eBands).offset((j + 1 as i32) as isize) as i32
                - *((*m).eBands).offset(j as isize) as i32);
        j += 1;
    }
    j = start;
    while j < codedBands {
        let tmp_1: i32 = if left
            < *((*m).eBands).offset((j + 1 as i32) as isize) as i32
                - *((*m).eBands).offset(j as isize) as i32
        {
            left
        } else {
            *((*m).eBands).offset((j + 1 as i32) as isize) as i32
                - *((*m).eBands).offset(j as isize) as i32
        };
        *bits.offset(j as isize) += tmp_1;
        left -= tmp_1;
        j += 1;
    }
    balance = 0 as i32;
    j = start;
    while j < codedBands {
        let mut N0: i32 = 0;
        let mut N: i32 = 0;
        let mut den: i32 = 0;
        let mut offset: i32 = 0;
        let mut NClogN: i32 = 0;
        let mut excess: i32 = 0;
        let mut bit: i32 = 0;
        if !(*bits.offset(j as isize) >= 0 as i32) {
            celt_fatal(
                b"assertion failed: bits[j] >= 0\0" as *const u8 as *const i8,
                b"celt/rate.c\0" as *const u8 as *const i8,
                442 as i32,
            );
        }
        N0 = *((*m).eBands).offset((j + 1 as i32) as isize) as i32
            - *((*m).eBands).offset(j as isize) as i32;
        N = N0 << LM;
        bit = *bits.offset(j as isize) + balance;
        if N > 1 as i32 {
            excess = if bit - *cap.offset(j as isize) > 0 as i32 {
                bit - *cap.offset(j as isize)
            } else {
                0 as i32
            };
            *bits.offset(j as isize) = bit - excess;
            den = C * N
                + (if C == 2 as i32 && N > 2 as i32 && *dual_stereo == 0 && j < *intensity {
                    1 as i32
                } else {
                    0 as i32
                });
            NClogN = den * (*((*m).logN).offset(j as isize) as i32 + logM);
            offset = (NClogN >> 1 as i32) - den * FINE_OFFSET;
            if N == 2 as i32 {
                offset += den << BITRES >> 2 as i32;
            }
            if *bits.offset(j as isize) + offset < (den * 2 as i32) << BITRES {
                offset += NClogN >> 2 as i32;
            } else if *bits.offset(j as isize) + offset < (den * 3 as i32) << BITRES {
                offset += NClogN >> 3 as i32;
            }
            *ebits.offset(j as isize) =
                if 0 as i32 > *bits.offset(j as isize) + offset + (den << 3 as i32 - 1 as i32) {
                    0 as i32
                } else {
                    *bits.offset(j as isize) + offset + (den << 3 as i32 - 1 as i32)
                };
            *ebits.offset(j as isize) =
                (celt_udiv(*ebits.offset(j as isize) as u32, den as u32) >> BITRES) as i32;
            if C * *ebits.offset(j as isize) > *bits.offset(j as isize) >> BITRES {
                *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> BITRES;
            }
            *ebits.offset(j as isize) = if *ebits.offset(j as isize) < 8 as i32 {
                *ebits.offset(j as isize)
            } else {
                8 as i32
            };
            *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) * (den << BITRES)
                >= *bits.offset(j as isize) + offset)
                as i32;
            *bits.offset(j as isize) -= C * *ebits.offset(j as isize) << BITRES;
        } else {
            excess = if 0 as i32 > bit - (C << 3 as i32) {
                0 as i32
            } else {
                bit - (C << 3 as i32)
            };
            *bits.offset(j as isize) = bit - excess;
            *ebits.offset(j as isize) = 0 as i32;
            *fine_priority.offset(j as isize) = 1 as i32;
        }
        if excess > 0 as i32 {
            let mut extra_fine: i32 = 0;
            let mut extra_bits: i32 = 0;
            extra_fine = if (excess >> stereo + 3 as i32) < 8 as i32 - *ebits.offset(j as isize) {
                excess >> stereo + 3 as i32
            } else {
                8 as i32 - *ebits.offset(j as isize)
            };
            *ebits.offset(j as isize) += extra_fine;
            extra_bits = extra_fine * C << BITRES;
            *fine_priority.offset(j as isize) = (extra_bits >= excess - balance) as i32;
            excess -= extra_bits;
        }
        balance = excess;
        if !(*bits.offset(j as isize) >= 0 as i32) {
            celt_fatal(
                b"assertion failed: bits[j] >= 0\0" as *const u8 as *const i8,
                b"celt/rate.c\0" as *const u8 as *const i8,
                513 as i32,
            );
        }
        if !(*ebits.offset(j as isize) >= 0 as i32) {
            celt_fatal(
                b"assertion failed: ebits[j] >= 0\0" as *const u8 as *const i8,
                b"celt/rate.c\0" as *const u8 as *const i8,
                514 as i32,
            );
        }
        j += 1;
    }
    *_balance = balance;
    while j < end {
        *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> BITRES;
        if !(C * *ebits.offset(j as isize) << 3 as i32 == *bits.offset(j as isize)) {
            celt_fatal(
                b"assertion failed: C*ebits[j]<<BITRES == bits[j]\0" as *const u8 as *const i8,
                b"celt/rate.c\0" as *const u8 as *const i8,
                524 as i32,
            );
        }
        *bits.offset(j as isize) = 0 as i32;
        *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) < 1 as i32) as i32;
        j += 1;
    }
    return codedBands;
}
pub unsafe fn clt_compute_allocation(
    m: *const OpusCustomMode,
    start: i32,
    end: i32,
    offsets: *const i32,
    cap: *const i32,
    alloc_trim: i32,
    intensity: *mut i32,
    dual_stereo: *mut i32,
    mut total: i32,
    balance: *mut i32,
    pulses: *mut i32,
    ebits: *mut i32,
    fine_priority: *mut i32,
    C: i32,
    LM: i32,
    ec: *mut ec_ctx,
    encode: i32,
    prev: i32,
    signalBandwidth: i32,
) -> i32 {
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut len: i32 = 0;
    let mut j: i32 = 0;
    let mut codedBands: i32 = 0;
    let mut skip_start: i32 = 0;
    let mut skip_rsv: i32 = 0;
    let mut intensity_rsv: i32 = 0;
    let mut dual_stereo_rsv: i32 = 0;
    total = if total > 0 as i32 { total } else { 0 as i32 };
    len = (*m).nbEBands;
    skip_start = start;
    skip_rsv = if total >= (1 as i32) << BITRES {
        (1 as i32) << BITRES
    } else {
        0 as i32
    };
    total -= skip_rsv;
    dual_stereo_rsv = 0 as i32;
    intensity_rsv = dual_stereo_rsv;
    if C == 2 as i32 {
        intensity_rsv = LOG2_FRAC_TABLE[(end - start) as usize] as i32;
        if intensity_rsv > total {
            intensity_rsv = 0 as i32;
        } else {
            total -= intensity_rsv;
            dual_stereo_rsv = if total >= (1 as i32) << BITRES {
                (1 as i32) << BITRES
            } else {
                0 as i32
            };
            total -= dual_stereo_rsv;
        }
    }
    let vla = len as usize;
    let mut bits1: Vec<i32> = ::std::vec::from_elem(0, vla);
    let vla_0 = len as usize;
    let mut bits2: Vec<i32> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = len as usize;
    let mut thresh: Vec<i32> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = len as usize;
    let mut trim_offset: Vec<i32> = ::std::vec::from_elem(0, vla_2);
    j = start;
    while j < end {
        *thresh.as_mut_ptr().offset(j as isize) = if C << 3 as i32
            > 3 as i32
                * (*((*m).eBands).offset((j + 1 as i32) as isize) as i32
                    - *((*m).eBands).offset(j as isize) as i32)
                << LM
                << 3 as i32
                >> 4 as i32
        {
            C << 3 as i32
        } else {
            3 as i32
                * (*((*m).eBands).offset((j + 1 as i32) as isize) as i32
                    - *((*m).eBands).offset(j as isize) as i32)
                << LM
                << 3 as i32
                >> 4 as i32
        };
        *trim_offset.as_mut_ptr().offset(j as isize) = C
            * (*((*m).eBands).offset((j + 1 as i32) as isize) as i32
                - *((*m).eBands).offset(j as isize) as i32)
            * (alloc_trim - 5 as i32 - LM)
            * (end - j - 1 as i32)
            * ((1 as i32) << LM + BITRES)
            >> 6 as i32;
        if (*((*m).eBands).offset((j + 1 as i32) as isize) as i32
            - *((*m).eBands).offset(j as isize) as i32)
            << LM
            == 1 as i32
        {
            *trim_offset.as_mut_ptr().offset(j as isize) -= C << BITRES;
        }
        j += 1;
    }
    lo = 1 as i32;
    hi = (*m).nbAllocVectors - 1 as i32;
    loop {
        let mut done: i32 = 0 as i32;
        let mut psum: i32 = 0 as i32;
        let mid: i32 = lo + hi >> 1 as i32;
        j = end;
        loop {
            let fresh2 = j;
            j = j - 1;
            if !(fresh2 > start) {
                break;
            }
            let mut bitsj: i32 = 0;
            let N: i32 = *((*m).eBands).offset((j + 1 as i32) as isize) as i32
                - *((*m).eBands).offset(j as isize) as i32;
            bitsj = (C * N * *((*m).allocVectors).offset((mid * len + j) as isize) as i32) << LM
                >> 2 as i32;
            if bitsj > 0 as i32 {
                bitsj = if 0 as i32 > bitsj + *trim_offset.as_mut_ptr().offset(j as isize) {
                    0 as i32
                } else {
                    bitsj + *trim_offset.as_mut_ptr().offset(j as isize)
                };
            }
            bitsj += *offsets.offset(j as isize);
            if bitsj >= *thresh.as_mut_ptr().offset(j as isize) || done != 0 {
                done = 1 as i32;
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
            hi = mid - 1 as i32;
        } else {
            lo = mid + 1 as i32;
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
        let mut bits1j: i32 = 0;
        let mut bits2j: i32 = 0;
        let N_0: i32 = *((*m).eBands).offset((j + 1 as i32) as isize) as i32
            - *((*m).eBands).offset(j as isize) as i32;
        bits1j = (C * N_0 * *((*m).allocVectors).offset((lo * len + j) as isize) as i32) << LM
            >> 2 as i32;
        bits2j = if hi >= (*m).nbAllocVectors {
            *cap.offset(j as isize)
        } else {
            (C * N_0 * *((*m).allocVectors).offset((hi * len + j) as isize) as i32) << LM
                >> 2 as i32
        };
        if bits1j > 0 as i32 {
            bits1j = if 0 as i32 > bits1j + *trim_offset.as_mut_ptr().offset(j as isize) {
                0 as i32
            } else {
                bits1j + *trim_offset.as_mut_ptr().offset(j as isize)
            };
        }
        if bits2j > 0 as i32 {
            bits2j = if 0 as i32 > bits2j + *trim_offset.as_mut_ptr().offset(j as isize) {
                0 as i32
            } else {
                bits2j + *trim_offset.as_mut_ptr().offset(j as isize)
            };
        }
        if lo > 0 as i32 {
            bits1j += *offsets.offset(j as isize);
        }
        bits2j += *offsets.offset(j as isize);
        if *offsets.offset(j as isize) > 0 as i32 {
            skip_start = j;
        }
        bits2j = if 0 as i32 > bits2j - bits1j {
            0 as i32
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
