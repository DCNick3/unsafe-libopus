use crate::silk::macros::EC_CLZ0;
use ::libc;

use std::f32::consts::PI;

#[c2rust::src_loc = "50:9"]
pub const cA: libc::c_float = 0.43157974f32;
#[c2rust::src_loc = "51:9"]
pub const cB: libc::c_float = 0.67848403f32;
#[c2rust::src_loc = "52:9"]
pub const cC: libc::c_float = 0.08595542f32;
#[c2rust::src_loc = "53:9"]
pub const cE: libc::c_float = PI / 2 as libc::c_int as libc::c_float;

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

#[inline]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn fast_atan2f(y: libc::c_float, x: libc::c_float) -> libc::c_float {
    let mut x2: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    x2 = x * x;
    y2 = y * y;
    if x2 + y2 < 1e-18f32 {
        return 0 as libc::c_int as libc::c_float;
    }
    if x2 < y2 {
        let den: libc::c_float = (y2 + cB * x2) * (y2 + cC * x2);
        return -x * y * (y2 + cA * x2) / den
            + (if y < 0 as libc::c_int as libc::c_float {
                -cE
            } else {
                cE
            });
    } else {
        let den_0: libc::c_float = (x2 + cB * y2) * (x2 + cC * y2);
        return x * y * (x2 + cA * y2) / den_0
            + (if y < 0 as libc::c_int as libc::c_float {
                -cE
            } else {
                cE
            })
            - (if x * y < 0 as libc::c_int as libc::c_float {
                -cE
            } else {
                cE
            });
    };
}

pub type opus_val16 = libc::c_float;
pub type opus_val32 = libc::c_float;

#[inline]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn celt_maxabs16(x: *const opus_val16, len: libc::c_int) -> opus_val32 {
    let mut i: libc::c_int = 0;
    let mut maxval: opus_val16 = 0 as libc::c_int as opus_val16;
    let mut minval: opus_val16 = 0 as libc::c_int as opus_val16;
    i = 0 as libc::c_int;
    while i < len {
        maxval = if maxval > *x.offset(i as isize) {
            maxval
        } else {
            *x.offset(i as isize)
        };
        minval = if minval < *x.offset(i as isize) {
            minval
        } else {
            *x.offset(i as isize)
        };
        i += 1;
    }
    return if maxval > -minval { maxval } else { -minval };
}
