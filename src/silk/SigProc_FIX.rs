#[inline]
#[c2rust::src_loc = "398:1"]
pub unsafe extern "C" fn silk_ROR32(a32: i32, rot: libc::c_int) -> i32 {
    let x: u32 = a32 as u32;
    let r: u32 = rot as u32;
    let m: u32 = -rot as u32;
    if rot == 0 as libc::c_int {
        return a32;
    } else if rot < 0 as libc::c_int {
        return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m)) as i32;
    } else {
        return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r) as i32;
    };
}

#[inline]
#[c2rust::src_loc = "568:1"]
pub unsafe extern "C" fn silk_max_16(a: i16, b: i16) -> i16 {
    return (if a as libc::c_int > b as libc::c_int {
        a as libc::c_int
    } else {
        b as libc::c_int
    }) as i16;
}

#[inline]
#[c2rust::src_loc = "554:1"]
pub unsafe extern "C" fn silk_min_32(a: i32, b: i32) -> i32 {
    return if a < b { a } else { b };
}
#[inline]
#[c2rust::src_loc = "572:1"]
pub unsafe extern "C" fn silk_max_32(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b };
}

#[inline]
#[c2rust::src_loc = "546:1"]
pub unsafe extern "C" fn silk_min_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
#[c2rust::src_loc = "564:1"]
pub unsafe extern "C" fn silk_max_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
