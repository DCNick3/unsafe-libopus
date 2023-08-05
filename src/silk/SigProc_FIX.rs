#[inline]
pub unsafe fn silk_ROR32(a32: i32, rot: i32) -> i32 {
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
pub unsafe fn silk_max_16(a: i16, b: i16) -> i16 {
    return (if a as i32 > b as i32 {
        a as i32
    } else {
        b as i32
    }) as i16;
}

#[inline]
pub unsafe fn silk_min_32(a: i32, b: i32) -> i32 {
    return if a < b { a } else { b };
}
#[inline]
pub unsafe fn silk_max_32(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b };
}

#[inline]
pub unsafe fn silk_min_int(a: i32, b: i32) -> i32 {
    return if a < b { a } else { b };
}
#[inline]
pub unsafe fn silk_max_int(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b };
}
