use crate::silk::macros::EC_CLZ0;
use ::libc;

#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn isqrt32(mut _val: u32) -> libc::c_uint {
    let mut b: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    let mut bshift: libc::c_int = 0;
    g = 0 as libc::c_int as libc::c_uint;
    bshift = EC_CLZ0 - _val.leading_zeros() as i32 - 1 as libc::c_int >> 1 as libc::c_int;
    b = (1 as libc::c_uint) << bshift;
    loop {
        let mut t: u32 = 0;
        t = (g << 1 as libc::c_int).wrapping_add(b) << bshift;
        if t <= _val {
            g = g.wrapping_add(b);
            _val = (_val as libc::c_uint).wrapping_sub(t) as u32 as u32;
        }
        b >>= 1 as libc::c_int;
        bshift -= 1;
        if !(bshift >= 0 as libc::c_int) {
            break;
        }
    }
    return g;
}
