/// max order of the LPC analysis in schur() and k2a()
pub const SILK_MAX_ORDER_LPC: usize = 24;

/// Rotate a32 right by 'rot' bits. Negative rot values result in rotating left. Output is 32bit int.
///
/// Note: contemporary compilers recognize the C expression below and
/// compile it into a 'ror' instruction if available. No need for OPUS_INLINE ASM!
#[inline]
pub fn silk_ROR32(a32: i32, rot: i32) -> i32 {
    let x: u32 = a32 as u32;
    let r: u32 = rot as u32;
    let m: u32 = -rot as u32;
    if rot == 0 {
        return a32;
    } else if rot < 0 {
        return (x << m | x >> (32u32).wrapping_sub(m)) as i32;
    } else {
        return (x << (32u32).wrapping_sub(r) | x >> r) as i32;
    };
}

#[inline]
pub fn silk_SMULTT(a32: i32, b32: i32) -> i32 {
    (a32 >> 16) * (b32 >> 16)
}

#[inline]
pub fn silk_SAT16(a: i32) -> i32 {
    if a > i16::MAX as i32 {
        i16::MAX as i32
    } else if a < i16::MIN as i32 {
        i16::MIN as i32
    } else {
        a
    }
}

/// saturates before shifting
#[inline]
pub fn silk_LSHIFT_SAT32(a: i32, shift: i32) -> i32 {
    silk_LIMIT(a, i32::MIN >> shift, i32::MAX >> shift) << shift
}

#[inline]
pub fn silk_RSHIFT_ROUND(a: i32, shift: i32) -> i32 {
    if shift == 1 {
        ((a) >> 1) + ((a) & 1)
    } else {
        (((a) >> ((shift) - 1)) + 1) >> 1
    }
}

#[inline]
pub fn silk_RSHIFT_ROUND64(a: i64, shift: i32) -> i64 {
    if shift == 1 {
        ((a) >> 1) + ((a) & 1)
    } else {
        (((a) >> ((shift) - 1)) + 1) >> 1
    }
}

/// Macro to convert floating-point constants to fixed-point
macro_rules! SILK_FIX_CONST {
    ($C:expr, $Q:expr) => {
        (($C as f64) * (1i64 << ($Q as i32)) as f64 + 0.5f64) as i32
    };
}
pub(crate) use SILK_FIX_CONST;

#[inline]
pub fn silk_min_int(a: i32, b: i32) -> i32 {
    a.min(b)
}
#[inline]
pub fn silk_max_int(a: i32, b: i32) -> i32 {
    a.max(b)
}

#[inline]
pub fn silk_max_16(a: i16, b: i16) -> i16 {
    a.max(b)
}

#[inline]
pub fn silk_min_32(a: i32, b: i32) -> i32 {
    a.min(b)
}
#[inline]
pub fn silk_max_32(a: i32, b: i32) -> i32 {
    a.max(b)
}

#[inline]
pub fn silk_LIMIT<T: Ord>(a: T, limit1: T, limit2: T) -> T {
    if limit1 > limit2 {
        if a > limit1 {
            limit1
        } else if a < limit2 {
            limit2
        } else {
            a
        }
    } else if a > limit2 {
        limit2
    } else if a < limit1 {
        limit1
    } else {
        a
    }
}

pub const RAND_MULTIPLIER: i32 = 196314165;
pub const RAND_INCREMENT: i32 = 907633515;

/// PSEUDO-RANDOM GENERATOR
///
/// Make sure to store the result as the seed for the next call (also in between
/// frames), otherwise the result won't be random at all. When only using some of the
/// bits, take the most significant bits by right-shifting.
#[inline]
pub fn silk_RAND(seed: i32) -> i32 {
    seed.wrapping_mul(RAND_MULTIPLIER)
        .wrapping_add(RAND_INCREMENT)
}

#[inline]
pub fn silk_SMMUL(a32: i32, b32: i32) -> i32 {
    ((a32 as i64 * b32 as i64) >> 32) as i32
}
