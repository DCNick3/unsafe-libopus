use crate::celt::kiss_fft::{kiss_fft_cpx, kiss_fft_state, opus_fft_impl};
use num_traits::Zero;

pub mod arch_h {
    pub type opus_val16 = f32;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdct_lookup {
    pub n: i32,
    pub maxshift: i32,
    pub kfft: [*const kiss_fft_state; 4],
    pub trig: *const f32,
}
pub use self::arch_h::opus_val16;

pub unsafe fn clt_mdct_forward_c(
    l: *const mdct_lookup,
    in_0: *mut f32,
    out: *mut f32,
    window: *const opus_val16,
    overlap: i32,
    shift: i32,
    stride: i32,
    _arch: i32,
) {
    let mut i: i32 = 0;
    let mut N: i32 = 0;
    let mut N2: i32 = 0;
    let mut N4: i32 = 0;
    let st: *const kiss_fft_state = (*l).kfft[shift as usize];
    let mut trig: *const f32 = 0 as *const f32;
    let mut scale: opus_val16 = 0.;
    scale = (*st).scale;
    N = (*l).n;
    trig = (*l).trig;
    i = 0 as i32;
    while i < shift {
        N >>= 1 as i32;
        trig = trig.offset(N as isize);
        i += 1;
    }
    N2 = N >> 1 as i32;
    N4 = N >> 2 as i32;
    let vla = N2 as usize;
    let mut f: Vec<f32> = ::std::vec::from_elem(0., vla);
    let vla_0 = N4 as usize;
    let mut f2: Vec<kiss_fft_cpx> = ::std::vec::from_elem(kiss_fft_cpx::zero(), vla_0);
    let mut xp1: *const f32 = in_0.offset((overlap >> 1 as i32) as isize);
    let mut xp2: *const f32 = in_0
        .offset(N2 as isize)
        .offset(-(1 as i32 as isize))
        .offset((overlap >> 1 as i32) as isize);
    let mut yp: *mut f32 = f.as_mut_ptr();
    let mut wp1: *const opus_val16 = window.offset((overlap >> 1 as i32) as isize);
    let mut wp2: *const opus_val16 = window
        .offset((overlap >> 1 as i32) as isize)
        .offset(-(1 as i32 as isize));
    i = 0 as i32;
    while i < overlap + 3 as i32 >> 2 as i32 {
        let fresh0 = yp;
        yp = yp.offset(1);
        *fresh0 = *wp2 * *xp1.offset(N2 as isize) + *wp1 * *xp2;
        let fresh1 = yp;
        yp = yp.offset(1);
        *fresh1 = *wp1 * *xp1 - *wp2 * *xp2.offset(-N2 as isize);
        xp1 = xp1.offset(2 as i32 as isize);
        xp2 = xp2.offset(-(2 as i32 as isize));
        wp1 = wp1.offset(2 as i32 as isize);
        wp2 = wp2.offset(-(2 as i32 as isize));
        i += 1;
    }
    wp1 = window;
    wp2 = window.offset(overlap as isize).offset(-(1 as i32 as isize));
    while i < N4 - (overlap + 3 as i32 >> 2 as i32) {
        let fresh2 = yp;
        yp = yp.offset(1);
        *fresh2 = *xp2;
        let fresh3 = yp;
        yp = yp.offset(1);
        *fresh3 = *xp1;
        xp1 = xp1.offset(2 as i32 as isize);
        xp2 = xp2.offset(-(2 as i32 as isize));
        i += 1;
    }
    while i < N4 {
        let fresh4 = yp;
        yp = yp.offset(1);
        *fresh4 = -(*wp1 * *xp1.offset(-N2 as isize)) + *wp2 * *xp2;
        let fresh5 = yp;
        yp = yp.offset(1);
        *fresh5 = *wp2 * *xp1 + *wp1 * *xp2.offset(N2 as isize);
        xp1 = xp1.offset(2 as i32 as isize);
        xp2 = xp2.offset(-(2 as i32 as isize));
        wp1 = wp1.offset(2 as i32 as isize);
        wp2 = wp2.offset(-(2 as i32 as isize));
        i += 1;
    }
    let mut yp_0: *mut f32 = f.as_mut_ptr();
    let t: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    i = 0 as i32;
    while i < N4 {
        let mut yc: kiss_fft_cpx = kiss_fft_cpx::zero();
        let mut t0: f32 = 0.;
        let mut t1: f32 = 0.;
        let mut re: f32 = 0.;
        let mut im: f32 = 0.;
        let mut yr: f32 = 0.;
        let mut yi: f32 = 0.;
        t0 = *t.offset(i as isize);
        t1 = *t.offset((N4 + i) as isize);
        let fresh6 = yp_0;
        yp_0 = yp_0.offset(1);
        re = *fresh6;
        let fresh7 = yp_0;
        yp_0 = yp_0.offset(1);
        im = *fresh7;
        yr = re * t0 - im * t1;
        yi = im * t0 + re * t1;
        yc.re = yr;
        yc.im = yi;
        yc.re = scale * yc.re;
        yc.im = scale * yc.im;
        *f2.as_mut_ptr()
            .offset(*((*st).bitrev).offset(i as isize) as isize) = yc;
        i += 1;
    }
    opus_fft_impl(st, f2.as_mut_ptr());
    let mut fp: *const kiss_fft_cpx = f2.as_mut_ptr();
    let mut yp1: *mut f32 = out;
    let mut yp2: *mut f32 = out.offset((stride * (N2 - 1 as i32)) as isize);
    let t_0: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    i = 0 as i32;
    while i < N4 {
        let mut yr_0: f32 = 0.;
        let mut yi_0: f32 = 0.;
        yr_0 = (*fp).im * *t_0.offset((N4 + i) as isize) - (*fp).re * *t_0.offset(i as isize);
        yi_0 = (*fp).re * *t_0.offset((N4 + i) as isize) + (*fp).im * *t_0.offset(i as isize);
        *yp1 = yr_0;
        *yp2 = yi_0;
        fp = fp.offset(1);
        yp1 = yp1.offset((2 as i32 * stride) as isize);
        yp2 = yp2.offset(-((2 as i32 * stride) as isize));
        i += 1;
    }
}
pub unsafe fn clt_mdct_backward_c(
    l: *const mdct_lookup,
    in_0: *mut f32,
    out: *mut f32,
    window: *const opus_val16,
    overlap: i32,
    shift: i32,
    stride: i32,
    _arch: i32,
) {
    let mut i: i32 = 0;
    let mut N: i32 = 0;
    let mut N2: i32 = 0;
    let mut N4: i32 = 0;
    let mut trig: *const f32 = 0 as *const f32;
    N = (*l).n;
    trig = (*l).trig;
    i = 0 as i32;
    while i < shift {
        N >>= 1 as i32;
        trig = trig.offset(N as isize);
        i += 1;
    }
    N2 = N >> 1 as i32;
    N4 = N >> 2 as i32;
    let mut xp1: *const f32 = in_0;
    let mut xp2: *const f32 = in_0.offset((stride * (N2 - 1 as i32)) as isize);
    let yp: *mut f32 = out.offset((overlap >> 1 as i32) as isize);
    let t: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    let mut bitrev: *const i16 = (*(*l).kfft[shift as usize]).bitrev;
    i = 0 as i32;
    while i < N4 {
        let mut rev: i32 = 0;
        let mut yr: f32 = 0.;
        let mut yi: f32 = 0.;
        let fresh8 = bitrev;
        bitrev = bitrev.offset(1);
        rev = *fresh8 as i32;
        yr = *xp2 * *t.offset(i as isize) + *xp1 * *t.offset((N4 + i) as isize);
        yi = *xp1 * *t.offset(i as isize) - *xp2 * *t.offset((N4 + i) as isize);
        *yp.offset((2 as i32 * rev + 1 as i32) as isize) = yr;
        *yp.offset((2 as i32 * rev) as isize) = yi;
        xp1 = xp1.offset((2 as i32 * stride) as isize);
        xp2 = xp2.offset(-((2 as i32 * stride) as isize));
        i += 1;
    }
    opus_fft_impl(
        (*l).kfft[shift as usize],
        out.offset((overlap >> 1 as i32) as isize) as *mut kiss_fft_cpx,
    );
    let mut yp0: *mut f32 = out.offset((overlap >> 1 as i32) as isize);
    let mut yp1: *mut f32 = out
        .offset((overlap >> 1 as i32) as isize)
        .offset(N2 as isize)
        .offset(-(2 as i32 as isize));
    let t_0: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    i = 0 as i32;
    while i < N4 + 1 as i32 >> 1 as i32 {
        let mut re: f32 = 0.;
        let mut im: f32 = 0.;
        let mut yr_0: f32 = 0.;
        let mut yi_0: f32 = 0.;
        let mut t0: f32 = 0.;
        let mut t1: f32 = 0.;
        re = *yp0.offset(1 as i32 as isize);
        im = *yp0.offset(0 as i32 as isize);
        t0 = *t_0.offset(i as isize);
        t1 = *t_0.offset((N4 + i) as isize);
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        re = *yp1.offset(1 as i32 as isize);
        im = *yp1.offset(0 as i32 as isize);
        *yp0.offset(0 as i32 as isize) = yr_0;
        *yp1.offset(1 as i32 as isize) = yi_0;
        t0 = *t_0.offset((N4 - i - 1 as i32) as isize);
        t1 = *t_0.offset((N2 - i - 1 as i32) as isize);
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        *yp1.offset(0 as i32 as isize) = yr_0;
        *yp0.offset(1 as i32 as isize) = yi_0;
        yp0 = yp0.offset(2 as i32 as isize);
        yp1 = yp1.offset(-(2 as i32 as isize));
        i += 1;
    }
    let mut xp1_0: *mut f32 = out.offset(overlap as isize).offset(-(1 as i32 as isize));
    let mut yp1_0: *mut f32 = out;
    let mut wp1: *const opus_val16 = window;
    let mut wp2: *const opus_val16 = window.offset(overlap as isize).offset(-(1 as i32 as isize));
    i = 0 as i32;
    while i < overlap / 2 as i32 {
        let mut x1: f32 = 0.;
        let mut x2: f32 = 0.;
        x1 = *xp1_0;
        x2 = *yp1_0;
        let fresh9 = yp1_0;
        yp1_0 = yp1_0.offset(1);
        *fresh9 = *wp2 * x2 - *wp1 * x1;
        let fresh10 = xp1_0;
        xp1_0 = xp1_0.offset(-1);
        *fresh10 = *wp1 * x2 + *wp2 * x1;
        wp1 = wp1.offset(1);
        wp2 = wp2.offset(-1);
        i += 1;
    }
}
