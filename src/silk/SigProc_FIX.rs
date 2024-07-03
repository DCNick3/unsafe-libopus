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
pub fn silk_min_int(a: i32, b: i32) -> i32 {
    a.min(b)
}
#[inline]
pub fn silk_max_int(a: i32, b: i32) -> i32 {
    a.max(b)
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
pub fn silk_SAT16(a: i32) -> i32 {
    if a > i16::MAX as i32 {
        i16::MAX as i32
    } else if a < i16::MIN as i32 {
        i16::MIN as i32
    } else {
        a
    }
}

#[inline]
pub fn silk_LIMIT(a: i32, limit1: i32, limit2: i32) -> i32 {
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
