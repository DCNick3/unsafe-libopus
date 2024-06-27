#![forbid(
    // we need this unsafe =(
    // but it's nicer
    // unsafe_code,
    non_camel_case_types,
    // we'll fix this
    // non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]

use crate::celt::kiss_fft::{kiss_fft_state, opus_fft_impl};
use ndarray::{aview1, aview_mut1, azip, s, Axis};
use num_complex::Complex;
use num_traits::Zero as _;
use std::ops::Neg as _;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MdctLookup<'a> {
    pub n: usize,
    pub maxshift: i32,
    pub kfft: [&'a kiss_fft_state<'a>; 4],
    pub trig: &'a [&'a [f32]; 4],
}

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
    l: &MdctLookup,
    input: &[f32],
    out: &mut [f32],
    window: &[f32],
    overlap: usize,
    shift: usize,
    output_stride: usize,
) {
    let st: &kiss_fft_state = l.kfft[shift];
    let scale = st.scale;
    let trig = aview1(l.trig[shift]);
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

pub fn mdct_backward(
    l: &MdctLookup,
    input: &[f32],
    out: &mut [f32],
    window: &[f32],
    overlap: usize,
    shift: usize,
    input_stride: usize,
) {
    let trig = aview1(l.trig[shift]);
    let N = l.n >> shift;
    let N2 = N / 2;
    let N4 = N / 4;

    let O = overlap;
    let O2 = overlap / 2;

    assert_eq!(l.kfft[shift].nfft, N4);

    assert_eq!(window.len(), O);
    assert_eq!(trig.len(), N2);
    let window = aview1(window);

    assert_eq!(input.len(), N2 * input_stride);
    let input = aview1(input);
    // TODO: it would be nice to accept a strided view directly as an `ArrayView1`
    let input = input.slice(s![..N2 * input_stride;input_stride]);

    assert_eq!(out.len(), N2 + O);
    let out = &mut out[..N2 + O];

    let (trig_real, trig_imag) = trig.split_at(Axis(0), N4);

    let outmid_scalar = &mut out[O2..][..N2];

    let (xf, xb) = ndutil::split_interleaving_opposite(input, Axis(0));

    // use the output space temporarily to compute fft there
    let outmid: &mut [Complex<f32>] = bytemuck::cast_slice_mut(outmid_scalar);

    /* Pre-rotate */
    {
        azip!((&xr in xf, &xi in xb, &tr in trig_real, &ti in trig_imag, &rev in l.kfft[shift].bitrev) {
            let t = Complex::new(tr, ti);
            let x = Complex::new(xr, xi);
            let y = x * t;
            outmid[rev as usize] = y;
        });
    }
    opus_fft_impl(l.kfft[shift], outmid);

    let mut outmid = aview_mut1(outmid);

    /* Post-rotate and de-shuffle from both ends of the buffer at once to make
    it in-place. */
    /* Loop to (N4+1)>>1 to handle odd N4. When N4 is odd, the
    middle pair will be computed twice. */
    // additional asserts to maybe help the optimizer remove bounds checks
    assert_eq!(outmid.len(), N4);
    assert_eq!(trig_real.len(), N4);
    assert_eq!(trig_imag.len(), N4);
    for i in 0..(N4 + 1) / 2 {
        // NB: unlike the loops in ctl_mdct_forward_c, the yp0 and yp1 "pointers" are NOT disjoint because they are stepped only by 1
        // so yp0 and yp1 can alias, especially when N4 is odd
        let yp0 = i;
        let yp1 = N4 - i - 1;

        fn swap(Complex { re, im }: Complex<f32>) -> Complex<f32> {
            Complex { re: im, im: re }
        }

        /* We swap real and imag because we're using an FFT instead of an IFFT. */
        let x = swap(outmid[yp0]);
        let t = swap(Complex::new(trig_real[i], trig_imag[i]));
        /* We'd scale up by 2 here, but instead it's done when mixing the windows */
        let y = swap(x * t);

        /* We swap real and imag because we're using an FFT instead of an IFFT. */
        let x = swap(outmid[yp1]);
        outmid[yp0].re = y.re;
        outmid[yp1].im = y.im;

        let t = swap(Complex::new(trig_real[N4 - i - 1], trig_imag[N4 - i - 1]));
        /* We'd scale up by 2 here, but instead it's done when mixing the windows */
        let y = swap(x * t);
        outmid[yp1].re = y.re;
        outmid[yp0].im = y.im;
    }

    /* Mirror on both sides for TDAC */
    {
        let outh = aview_mut1(&mut out[..O]);
        let (mut outhf, mut outhb) = outh.split_at(Axis(0), O2);
        let mut outhb = outhb.slice_mut(s![..;-1]);

        let (wf, wb) = window.split_at(Axis(0), O2);
        let wb = wb.slice(s![..;-1]);

        azip!((of in &mut outhf, ob in &mut outhb, &wf in wf, &wb in wb) {
            let x1 = *ob;
            let x2 = *of;
            *of = wb * x2 - wf * x1;
            *ob = wf * x2 + wb * x1;
        });
    }
}
