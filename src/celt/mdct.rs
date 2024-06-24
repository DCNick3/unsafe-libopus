use crate::celt::kiss_fft::{kiss_fft_state, opus_fft_impl};
use ndarray::{aview1, aview_mut1, azip, s, Axis};
use num_complex::Complex;
use num_traits::Zero as _;
use std::ops::Neg as _;

pub mod arch_h {
    pub type opus_val16 = f32;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdct_lookup<'a> {
    pub n: usize,
    pub maxshift: i32,
    pub kfft: [&'a kiss_fft_state<'a>; 4],
    pub trig: &'a [&'a [f32]; 4],
}
pub use self::arch_h::opus_val16;

mod ndutil {
    use ndarray::{ArrayView, ArrayViewMut, Axis, Dimension, Slice};
    /// Split an evenly sized dimension into two interleaving views going in different directions
    ///
    /// For example, if the input view is `[a, b, c, d, e, f]`, the output views are `([a, c, e], [f, d, b])`.
    pub fn split_interleaving_opposite<'a, A, D>(
        array: ArrayView<'a, A, D>,
        axis: Axis,
    ) -> (ArrayView<'a, A, D>, ArrayView<'a, A, D>)
    where
        D: Dimension + 'a,
    {
        assert_eq!(array.len() % 2, 0);

        let forward = array.slice_axis(axis, Slice::new(0, None, 2)).raw_view();
        let backward = array.slice_axis(axis, Slice::new(0, None, -2)).raw_view();

        // SAFETY: trust me bro, the lifetimes are fine
        unsafe { (forward.deref_into_view(), backward.deref_into_view()) }
    }

    /// Split an evenly sized dimension into two interleaving views going in different directions
    ///
    /// For example, if the input view is `[a, b, c, d, e, f]`, the output views are `([a, c, e], [f, d, b])`.
    pub fn split_interleaving_opposite_mut<'a, A, D>(
        mut array: ArrayViewMut<A, D>,
        axis: Axis,
    ) -> (ArrayViewMut<A, D>, ArrayViewMut<A, D>)
    where
        D: Dimension,
    {
        assert_eq!(array.len() % 2, 0);

        let forward = array
            .slice_axis_mut(axis, Slice::new(0, None, 2))
            .raw_view_mut();
        let backward = array
            .slice_axis_mut(axis, Slice::new(0, None, -2))
            .raw_view_mut();

        // SAFETY: trust me bro, those views are disjoint
        unsafe {
            (
                forward.deref_into_view_mut(),
                backward.deref_into_view_mut(),
            )
        }
    }
}

pub fn clt_mdct_forward_c(
    l: &mdct_lookup,
    input: &[f32],
    out: &mut [f32],
    window: &[opus_val16],
    overlap: usize,
    shift: usize,
    output_stride: usize,
) {
    let st: &kiss_fft_state = l.kfft[shift as usize];
    let scale = st.scale;
    let trig = aview1(l.trig[shift as usize]);
    let N = l.n >> shift;
    let N2 = N / 2;
    let N4 = N / 4;

    let O = overlap;
    let O2 = overlap / 2;
    let O4 = overlap / 4;

    assert_eq!(window.len(), O);
    assert_eq!(trig.len(), N2);
    let window = aview1(window);

    // TODO: make sure all callers pass the exactly-sized slice
    assert!(input.len() >= N2 + O);
    let input = aview1(&input[..N2 + O]);

    assert!(out.len() >= N2 * output_stride);
    let mut out = aview_mut1(out);
    // TODO: it would be nice to accept a strided view directly as an `ArrayViewMut1`
    let out = out.slice_mut(s![..N2 * output_stride;output_stride]);

    assert_eq!(O % 4, 0);

    // further down we split arrays into chunks.
    // and then split the chunks into two views with split_interleaving_opposite.
    // the chunks are suffixed with h (for head), mid (for middle) and t (for tail).
    // the input chunks are more numerous, so there are also h1, h2, mid, t1, t2 (for two heads and two tails).
    // the views are suffixed with f (for forward) and b (for backward).

    let (trig_real, trig_imag) = trig.split_at(Axis(0), N4);

    let (wh, wt) = window.split_at(Axis(0), O2);
    // [  wh  |  wt  ]
    // [<-O2->|<-O2->]
    // [<-----O----->]

    let (whf, whb) = ndutil::split_interleaving_opposite(wh, Axis(0));
    let (wtf, wtb) = ndutil::split_interleaving_opposite(wt, Axis(0));

    let (xh1, tail) = input.split_at(Axis(0), O2);
    let (xh2, tail) = tail.split_at(Axis(0), O2);
    let (xmid, tail) = tail.split_at(Axis(0), N2 - O);
    let (xt1, xt2) = tail.split_at(Axis(0), O2);
    // [  xh1   |   xh2  |   xmid   |  xt1   |   xt2  ]
    // [<--O2-->|<--O2-->|<--N2-O-->|<--O2-->|<--O2-->]
    // [<--O2-->|<------------N2------------>|<--O2-->]
    // [<-------O------->|<------------N2------------>]
    // [<------------N2------------>|<-------O------->]

    let (xh1f, xh1b) = ndutil::split_interleaving_opposite(xh1, Axis(0));
    let (xh2f, xh2b) = ndutil::split_interleaving_opposite(xh2, Axis(0));
    let (xmidf, xmidb) = ndutil::split_interleaving_opposite(xmid, Axis(0));
    let (xt1f, xt1b) = ndutil::split_interleaving_opposite(xt1, Axis(0));
    let (xt2f, xt2b) = ndutil::split_interleaving_opposite(xt2, Axis(0));

    // TODO: allocate from a custom per-frame allocator?
    let mut f = std::vec::from_elem(Complex::zero(), N4);
    let fv = aview_mut1(&mut f);

    let (mut fh, tail) = fv.split_at(Axis(0), O4);
    let (mut fmid, mut ft) = tail.split_at(Axis(0), N4 - O2);
    // [  fh  |   fmid   |  ft  ]
    // [<-O4->|<--N4-O2->|<-O4->]
    // [<----------N4---------->]

    /* Consider the input to be composed of four blocks: [a, b, c, d] */
    /* Window, shuffle, fold */
    {
        // doing two passes because ndarray limits us to only 6 variables (that's sooo small :sob:)
        azip!((y in &mut fh, &w1 in wtf, &w2 in whb, &x1_N2 in xt2f, &x2 in &xt1b) {
            y.re = w2 * x1_N2 + w1 * x2;
        });
        azip!((y in &mut fh, &w1 in wtf, &w2 in whb, &x1 in xh2f, &x2_N2 in xh1b) {
            y.im = w1 * x1 - w2 * x2_N2;
        });

        azip!((y in &mut fmid, &x1 in xmidf, &x2 in xmidb) {
            y.re = x2;
            y.im = x1;
        });

        // doing two passes because ndarray limits us to only 6 variables (that's sooo small :sob:)
        azip!((y in &mut ft, &w1 in whf, &w2 in wtb, &x1_N2 in xh1f, &x2 in &xh2b) {
            y.re = -(w1 * x1_N2) + w2 * x2;
        });
        azip!((y in &mut ft, &w1 in whf, &w2 in wtb, &x1 in xt1f, &x2_N2 in xt2b) {
            y.im = w2 * x1 + w1 * x2_N2;
        });
    }

    // TODO: allocate from a custom per-frame allocator?
    let mut f2 = std::vec::from_elem(Complex::zero(), N4);

    /* Pre-rotation */
    {
        azip!((index i, &y in aview1(&f), &tr in trig_real, &ri in trig_imag) {
            let t = Complex::new(tr, ri);
            let yc = scale * (y * t);
            f2[st.bitrev[i] as usize] = yc;
        });
    }

    /* N/4 complex FFT, does not downscale anymore */
    opus_fft_impl(st, &mut f2);

    /* Post-rotate */
    {
        let (mut yp1, mut yp2) = ndutil::split_interleaving_opposite_mut(out, Axis(0));

        let fp = aview1(&f2);

        azip!((y1 in &mut yp1, y2 in &mut yp2, f in fp, &tr in trig_real, &ti in trig_imag) {
            let t = Complex::new(tr, ti);
            let yc = (f * t).conj().neg();
            *y1 = yc.re;
            *y2 = yc.im;
        });
    }
}
pub unsafe fn clt_mdct_backward_c(
    l: &mdct_lookup,
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
    let trig = l.trig[shift as usize];
    N = (l.n >> shift) as _;
    N2 = N >> 1;
    N4 = N >> 2;
    let mut xp1: *const f32 = in_0;
    let mut xp2: *const f32 = in_0.offset((stride * (N2 - 1)) as isize);
    let yp: *mut f32 = out.offset((overlap >> 1) as isize);
    let t = trig;
    let mut bitrev: *const i16 = l.kfft[shift as usize].bitrev.as_ptr();
    i = 0;
    while i < N4 {
        let mut rev: i32 = 0;
        let mut yr: f32 = 0.;
        let mut yi: f32 = 0.;
        let fresh8 = bitrev;
        bitrev = bitrev.offset(1);
        rev = *fresh8 as i32;
        yr = *xp2 * t[i as usize] + *xp1 * t[(N4 + i) as usize];
        yi = *xp1 * t[i as usize] - *xp2 * t[(N4 + i) as usize];
        *yp.offset((2 * rev + 1) as isize) = yr;
        *yp.offset((2 * rev) as isize) = yi;
        xp1 = xp1.offset((2 * stride) as isize);
        xp2 = xp2.offset(-((2 * stride) as isize));
        i += 1;
    }
    opus_fft_impl(
        l.kfft[shift as usize],
        std::slice::from_raw_parts_mut(
            out.offset((overlap >> 1) as isize) as *mut Complex<f32>,
            l.kfft[shift as usize].nfft,
        ),
    );
    let mut yp0: *mut f32 = out.offset((overlap >> 1) as isize);
    let mut yp1: *mut f32 = out
        .offset((overlap >> 1) as isize)
        .offset(N2 as isize)
        .offset(-(2 as isize));
    let t = trig;
    i = 0;
    while i < N4 + 1 >> 1 {
        let mut re: f32 = 0.;
        let mut im: f32 = 0.;
        let mut yr_0: f32 = 0.;
        let mut yi_0: f32 = 0.;
        let mut t0: f32 = 0.;
        let mut t1: f32 = 0.;
        re = *yp0.offset(1 as isize);
        im = *yp0.offset(0 as isize);
        t0 = t[i as usize];
        t1 = t[(N4 + i) as usize];
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        re = *yp1.offset(1 as isize);
        im = *yp1.offset(0 as isize);
        *yp0.offset(0 as isize) = yr_0;
        *yp1.offset(1 as isize) = yi_0;
        t0 = t[(N4 - i - 1) as usize];
        t1 = t[(N2 - i - 1) as usize];
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        *yp1.offset(0 as isize) = yr_0;
        *yp0.offset(1 as isize) = yi_0;
        yp0 = yp0.offset(2 as isize);
        yp1 = yp1.offset(-(2 as isize));
        i += 1;
    }
    let mut xp1_0: *mut f32 = out.offset(overlap as isize).offset(-(1 as isize));
    let mut yp1_0: *mut f32 = out;
    let mut wp1: *const opus_val16 = window;
    let mut wp2: *const opus_val16 = window.offset(overlap as isize).offset(-(1 as isize));
    i = 0;
    while i < overlap / 2 {
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
