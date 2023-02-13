use crate::silk::macros::EC_CLZ0;

use std::f32::consts::PI;

pub const cA: f32 = 0.43157974f32;
pub const cB: f32 = 0.67848403f32;
pub const cC: f32 = 0.08595542f32;
pub const cE: f32 = PI / 2 as i32 as f32;

pub unsafe fn isqrt32(mut _val: u32) -> u32 {
    let mut b: u32 = 0;
    let mut g: u32 = 0;
    let mut bshift: i32 = 0;
    g = 0 as i32 as u32;
    bshift = EC_CLZ0 - _val.leading_zeros() as i32 - 1 as i32 >> 1 as i32;
    b = (1 as u32) << bshift;
    loop {
        let mut t: u32 = 0;
        t = (g << 1 as i32).wrapping_add(b) << bshift;
        if t <= _val {
            g = g.wrapping_add(b);
            _val = (_val as u32).wrapping_sub(t) as u32 as u32;
        }
        b >>= 1 as i32;
        bshift -= 1;
        if !(bshift >= 0 as i32) {
            break;
        }
    }
    return g;
}

#[inline]
pub unsafe fn fast_atan2f(y: f32, x: f32) -> f32 {
    let mut x2: f32 = 0.;
    let mut y2: f32 = 0.;
    x2 = x * x;
    y2 = y * y;
    if x2 + y2 < 1e-18f32 {
        return 0 as i32 as f32;
    }
    if x2 < y2 {
        let den: f32 = (y2 + cB * x2) * (y2 + cC * x2);
        return -x * y * (y2 + cA * x2) / den + (if y < 0 as i32 as f32 { -cE } else { cE });
    } else {
        let den_0: f32 = (x2 + cB * y2) * (x2 + cC * y2);
        return x * y * (x2 + cA * y2) / den_0 + (if y < 0 as i32 as f32 { -cE } else { cE })
            - (if x * y < 0 as i32 as f32 { -cE } else { cE });
    };
}

pub type opus_val16 = f32;
pub type opus_val32 = f32;

#[inline]
pub unsafe fn celt_maxabs16(x: *const opus_val16, len: i32) -> opus_val32 {
    let mut i: i32 = 0;
    let mut maxval: opus_val16 = 0 as i32 as opus_val16;
    let mut minval: opus_val16 = 0 as i32 as opus_val16;
    i = 0 as i32;
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
