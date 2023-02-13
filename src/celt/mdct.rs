use crate::celt::kiss_fft::{kiss_fft_cpx, kiss_fft_state, opus_fft_impl};

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:48"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
}

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "49:9"]
pub struct mdct_lookup {
    pub n: libc::c_int,
    pub maxshift: libc::c_int,
    pub kfft: [*const kiss_fft_state; 4],
    pub trig: *const libc::c_float,
}
pub use self::arch_h::opus_val16;

#[c2rust::src_loc = "119:1"]
pub unsafe fn clt_mdct_forward_c(
    l: *const mdct_lookup,
    in_0: *mut libc::c_float,
    out: *mut libc::c_float,
    window: *const opus_val16,
    overlap: libc::c_int,
    shift: libc::c_int,
    stride: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut N2: libc::c_int = 0;
    let mut N4: libc::c_int = 0;
    let st: *const kiss_fft_state = (*l).kfft[shift as usize];
    let mut trig: *const libc::c_float = 0 as *const libc::c_float;
    let mut scale: opus_val16 = 0.;
    scale = (*st).scale;
    N = (*l).n;
    trig = (*l).trig;
    i = 0 as libc::c_int;
    while i < shift {
        N >>= 1 as libc::c_int;
        trig = trig.offset(N as isize);
        i += 1;
    }
    N2 = N >> 1 as libc::c_int;
    N4 = N >> 2 as libc::c_int;
    let vla = N2 as usize;
    let mut f: Vec<libc::c_float> = ::std::vec::from_elem(0., vla);
    let vla_0 = N4 as usize;
    let mut f2: Vec<kiss_fft_cpx> = ::std::vec::from_elem(kiss_fft_cpx { r: 0., i: 0. }, vla_0);
    let mut xp1: *const libc::c_float = in_0.offset((overlap >> 1 as libc::c_int) as isize);
    let mut xp2: *const libc::c_float = in_0
        .offset(N2 as isize)
        .offset(-(1 as libc::c_int as isize))
        .offset((overlap >> 1 as libc::c_int) as isize);
    let mut yp: *mut libc::c_float = f.as_mut_ptr();
    let mut wp1: *const opus_val16 = window.offset((overlap >> 1 as libc::c_int) as isize);
    let mut wp2: *const opus_val16 = window
        .offset((overlap >> 1 as libc::c_int) as isize)
        .offset(-(1 as libc::c_int as isize));
    i = 0 as libc::c_int;
    while i < overlap + 3 as libc::c_int >> 2 as libc::c_int {
        let fresh0 = yp;
        yp = yp.offset(1);
        *fresh0 = *wp2 * *xp1.offset(N2 as isize) + *wp1 * *xp2;
        let fresh1 = yp;
        yp = yp.offset(1);
        *fresh1 = *wp1 * *xp1 - *wp2 * *xp2.offset(-N2 as isize);
        xp1 = xp1.offset(2 as libc::c_int as isize);
        xp2 = xp2.offset(-(2 as libc::c_int as isize));
        wp1 = wp1.offset(2 as libc::c_int as isize);
        wp2 = wp2.offset(-(2 as libc::c_int as isize));
        i += 1;
    }
    wp1 = window;
    wp2 = window
        .offset(overlap as isize)
        .offset(-(1 as libc::c_int as isize));
    while i < N4 - (overlap + 3 as libc::c_int >> 2 as libc::c_int) {
        let fresh2 = yp;
        yp = yp.offset(1);
        *fresh2 = *xp2;
        let fresh3 = yp;
        yp = yp.offset(1);
        *fresh3 = *xp1;
        xp1 = xp1.offset(2 as libc::c_int as isize);
        xp2 = xp2.offset(-(2 as libc::c_int as isize));
        i += 1;
    }
    while i < N4 {
        let fresh4 = yp;
        yp = yp.offset(1);
        *fresh4 = -(*wp1 * *xp1.offset(-N2 as isize)) + *wp2 * *xp2;
        let fresh5 = yp;
        yp = yp.offset(1);
        *fresh5 = *wp2 * *xp1 + *wp1 * *xp2.offset(N2 as isize);
        xp1 = xp1.offset(2 as libc::c_int as isize);
        xp2 = xp2.offset(-(2 as libc::c_int as isize));
        wp1 = wp1.offset(2 as libc::c_int as isize);
        wp2 = wp2.offset(-(2 as libc::c_int as isize));
        i += 1;
    }
    let mut yp_0: *mut libc::c_float = f.as_mut_ptr();
    let t: *const libc::c_float = &*trig.offset(0 as libc::c_int as isize) as *const libc::c_float;
    i = 0 as libc::c_int;
    while i < N4 {
        let mut yc: kiss_fft_cpx = kiss_fft_cpx { r: 0., i: 0. };
        let mut t0: libc::c_float = 0.;
        let mut t1: libc::c_float = 0.;
        let mut re: libc::c_float = 0.;
        let mut im: libc::c_float = 0.;
        let mut yr: libc::c_float = 0.;
        let mut yi: libc::c_float = 0.;
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
        yc.r = yr;
        yc.i = yi;
        yc.r = scale * yc.r;
        yc.i = scale * yc.i;
        *f2.as_mut_ptr()
            .offset(*((*st).bitrev).offset(i as isize) as isize) = yc;
        i += 1;
    }
    opus_fft_impl(st, f2.as_mut_ptr());
    let mut fp: *const kiss_fft_cpx = f2.as_mut_ptr();
    let mut yp1: *mut libc::c_float = out;
    let mut yp2: *mut libc::c_float = out.offset((stride * (N2 - 1 as libc::c_int)) as isize);
    let t_0: *const libc::c_float =
        &*trig.offset(0 as libc::c_int as isize) as *const libc::c_float;
    i = 0 as libc::c_int;
    while i < N4 {
        let mut yr_0: libc::c_float = 0.;
        let mut yi_0: libc::c_float = 0.;
        yr_0 = (*fp).i * *t_0.offset((N4 + i) as isize) - (*fp).r * *t_0.offset(i as isize);
        yi_0 = (*fp).r * *t_0.offset((N4 + i) as isize) + (*fp).i * *t_0.offset(i as isize);
        *yp1 = yr_0;
        *yp2 = yi_0;
        fp = fp.offset(1);
        yp1 = yp1.offset((2 as libc::c_int * stride) as isize);
        yp2 = yp2.offset(-((2 as libc::c_int * stride) as isize));
        i += 1;
    }
}
#[c2rust::src_loc = "242:1"]
pub unsafe fn clt_mdct_backward_c(
    l: *const mdct_lookup,
    in_0: *mut libc::c_float,
    out: *mut libc::c_float,
    window: *const opus_val16,
    overlap: libc::c_int,
    shift: libc::c_int,
    stride: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut N2: libc::c_int = 0;
    let mut N4: libc::c_int = 0;
    let mut trig: *const libc::c_float = 0 as *const libc::c_float;
    N = (*l).n;
    trig = (*l).trig;
    i = 0 as libc::c_int;
    while i < shift {
        N >>= 1 as libc::c_int;
        trig = trig.offset(N as isize);
        i += 1;
    }
    N2 = N >> 1 as libc::c_int;
    N4 = N >> 2 as libc::c_int;
    let mut xp1: *const libc::c_float = in_0;
    let mut xp2: *const libc::c_float = in_0.offset((stride * (N2 - 1 as libc::c_int)) as isize);
    let yp: *mut libc::c_float = out.offset((overlap >> 1 as libc::c_int) as isize);
    let t: *const libc::c_float = &*trig.offset(0 as libc::c_int as isize) as *const libc::c_float;
    let mut bitrev: *const i16 = (*(*l).kfft[shift as usize]).bitrev;
    i = 0 as libc::c_int;
    while i < N4 {
        let mut rev: libc::c_int = 0;
        let mut yr: libc::c_float = 0.;
        let mut yi: libc::c_float = 0.;
        let fresh8 = bitrev;
        bitrev = bitrev.offset(1);
        rev = *fresh8 as libc::c_int;
        yr = *xp2 * *t.offset(i as isize) + *xp1 * *t.offset((N4 + i) as isize);
        yi = *xp1 * *t.offset(i as isize) - *xp2 * *t.offset((N4 + i) as isize);
        *yp.offset((2 as libc::c_int * rev + 1 as libc::c_int) as isize) = yr;
        *yp.offset((2 as libc::c_int * rev) as isize) = yi;
        xp1 = xp1.offset((2 as libc::c_int * stride) as isize);
        xp2 = xp2.offset(-((2 as libc::c_int * stride) as isize));
        i += 1;
    }
    opus_fft_impl(
        (*l).kfft[shift as usize],
        out.offset((overlap >> 1 as libc::c_int) as isize) as *mut kiss_fft_cpx,
    );
    let mut yp0: *mut libc::c_float = out.offset((overlap >> 1 as libc::c_int) as isize);
    let mut yp1: *mut libc::c_float = out
        .offset((overlap >> 1 as libc::c_int) as isize)
        .offset(N2 as isize)
        .offset(-(2 as libc::c_int as isize));
    let t_0: *const libc::c_float =
        &*trig.offset(0 as libc::c_int as isize) as *const libc::c_float;
    i = 0 as libc::c_int;
    while i < N4 + 1 as libc::c_int >> 1 as libc::c_int {
        let mut re: libc::c_float = 0.;
        let mut im: libc::c_float = 0.;
        let mut yr_0: libc::c_float = 0.;
        let mut yi_0: libc::c_float = 0.;
        let mut t0: libc::c_float = 0.;
        let mut t1: libc::c_float = 0.;
        re = *yp0.offset(1 as libc::c_int as isize);
        im = *yp0.offset(0 as libc::c_int as isize);
        t0 = *t_0.offset(i as isize);
        t1 = *t_0.offset((N4 + i) as isize);
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        re = *yp1.offset(1 as libc::c_int as isize);
        im = *yp1.offset(0 as libc::c_int as isize);
        *yp0.offset(0 as libc::c_int as isize) = yr_0;
        *yp1.offset(1 as libc::c_int as isize) = yi_0;
        t0 = *t_0.offset((N4 - i - 1 as libc::c_int) as isize);
        t1 = *t_0.offset((N2 - i - 1 as libc::c_int) as isize);
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        *yp1.offset(0 as libc::c_int as isize) = yr_0;
        *yp0.offset(1 as libc::c_int as isize) = yi_0;
        yp0 = yp0.offset(2 as libc::c_int as isize);
        yp1 = yp1.offset(-(2 as libc::c_int as isize));
        i += 1;
    }
    let mut xp1_0: *mut libc::c_float = out
        .offset(overlap as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut yp1_0: *mut libc::c_float = out;
    let mut wp1: *const opus_val16 = window;
    let mut wp2: *const opus_val16 = window
        .offset(overlap as isize)
        .offset(-(1 as libc::c_int as isize));
    i = 0 as libc::c_int;
    while i < overlap / 2 as libc::c_int {
        let mut x1: libc::c_float = 0.;
        let mut x2: libc::c_float = 0.;
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
