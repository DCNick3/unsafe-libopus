pub mod stddef_h {
    pub type size_t = u64;
}
pub mod arch_h {
    pub type opus_val16 = f32;
}

use num_traits::Zero;
pub type kiss_fft_cpx = num_complex::Complex32;
pub type kiss_twiddle_cpx = num_complex::Complex32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kiss_fft_state {
    pub nfft: i32,
    pub scale: opus_val16,
    pub shift: i32,
    pub factors: [i16; 16],
    pub bitrev: *const i16,
    pub twiddles: *const kiss_twiddle_cpx,
}

pub use self::arch_h::opus_val16;
pub use self::stddef_h::size_t;
use crate::celt::celt::celt_fatal;

unsafe fn kf_bfly2(mut Fout: *mut kiss_fft_cpx, m: i32, N: i32) {
    let tw: opus_val16 = std::f32::consts::FRAC_1_SQRT_2;
    /* We know that m==4 here because the radix-2 is just after a radix-4 */
    assert_eq!(m, 4);
    for _ in 0..N {
        let Fout2 = Fout.offset(4);
        let t = *Fout2.offset(0);
        (*Fout2.offset(0)) = (*Fout.offset(0)) - t;
        (*Fout.offset(0)) += t;

        let t = kiss_fft_cpx::new(
            ((*Fout2.offset(1)).re + (*Fout2.offset(1)).im) * tw,
            ((*Fout2.offset(1)).im - (*Fout2.offset(1)).re) * tw,
        );
        (*Fout2.offset(1)) = (*Fout.offset(1)) - t;
        (*Fout.offset(1)) += t;

        let t = kiss_fft_cpx::new((*Fout2.offset(2)).im, -(*Fout2.offset(2)).re);
        (*Fout2.offset(2)) = (*Fout.offset(2)) - t;
        (*Fout.offset(2)) += t;

        let t = kiss_fft_cpx::new(
            ((*Fout2.offset(3)).im - (*Fout2.offset(3)).re) * tw,
            -((*Fout2.offset(3)).im + (*Fout2.offset(3)).re) * tw,
        );
        (*Fout2.offset(3)) = (*Fout.offset(3)) - t;
        (*Fout.offset(3)) += t;

        Fout = Fout.offset(8);
    }
}
unsafe fn kf_bfly4(
    mut Fout: *mut kiss_fft_cpx,
    fstride: size_t,
    st: *const kiss_fft_state,
    m: i32,
    N: i32,
    mm: i32,
) {
    if m == 1 {
        /* Degenerate case where all the twiddles are 1. */
        for _ in 0..N {
            let mut scratch0: kiss_fft_cpx = kiss_fft_cpx::zero();
            let mut scratch1: kiss_fft_cpx = kiss_fft_cpx::zero();

            scratch0 = (*Fout) - (*Fout.offset(2));
            (*Fout) += *Fout.offset(2);
            scratch1 = (*Fout.offset(1)) + (*Fout.offset(3));
            (*Fout.offset(2)) = (*Fout) - scratch1;
            (*Fout) += scratch1;
            scratch1 = (*Fout.offset(1)) - (*Fout.offset(3));

            (*Fout.offset(1)).re = scratch0.re + scratch1.im;
            (*Fout.offset(1)).im = scratch0.im - scratch1.re;
            (*Fout.offset(3)).re = scratch0.re - scratch1.im;
            (*Fout.offset(3)).im = scratch0.im + scratch1.re;
            Fout = Fout.offset(4);
        }
    } else {
        let mut scratch: [kiss_fft_cpx; 6] = [kiss_fft_cpx::zero(); 6];
        let mut tw1: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
        let mut tw2: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
        let mut tw3: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
        let m2: i32 = 2 as i32 * m;
        let m3: i32 = 3 as i32 * m;
        let Fout_beg: *mut kiss_fft_cpx = Fout;

        for i in 0..N {
            Fout = Fout_beg.offset((i * mm) as isize);
            tw1 = (*st).twiddles;
            tw2 = tw1;
            tw3 = tw2;
            /* m is guaranteed to be a multiple of 4. */
            for _ in 0..m {
                scratch[0] = *Fout.offset(m as isize) * (*tw1);
                scratch[1] = *Fout.offset(m2 as isize) * (*tw2);
                scratch[2] = *Fout.offset(m3 as isize) * (*tw3);

                scratch[5] = (*Fout) - scratch[1];
                (*Fout) += scratch[1];
                scratch[3] = scratch[0] + scratch[2];
                scratch[4] = scratch[0] - scratch[2];
                (*Fout.offset(m2 as isize)) = (*Fout) - scratch[3];
                tw1 = tw1.offset(fstride as isize);
                tw2 = tw2.offset(fstride.wrapping_mul(2) as isize);
                tw3 = tw3.offset(fstride.wrapping_mul(3) as isize);
                (*Fout) += scratch[3];

                (*Fout.offset(m as isize)).re = scratch[5].re + scratch[4].im;
                (*Fout.offset(m as isize)).im = scratch[5].im - scratch[4].re;
                (*Fout.offset(m3 as isize)).re = scratch[5].re - scratch[4].im;
                (*Fout.offset(m3 as isize)).im = scratch[5].im + scratch[4].re;
                Fout = Fout.offset(1);
            }
        }
    };
}
unsafe fn kf_bfly3(
    mut Fout: *mut kiss_fft_cpx,
    fstride: size_t,
    st: *const kiss_fft_state,
    m: i32,
    N: i32,
    mm: i32,
) {
    let mut k: usize = 0;
    let m2: usize = 2 * m as usize;
    let mut tw1: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
    let mut tw2: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
    let mut scratch: [kiss_fft_cpx; 5] = [kiss_fft_cpx::zero(); 5];
    let Fout_beg: *mut kiss_fft_cpx = Fout;
    let epi3 = *((*st).twiddles).offset(fstride.wrapping_mul(m as u64) as isize);
    for i in 0..N {
        Fout = Fout_beg.offset((i * mm) as isize);
        tw2 = (*st).twiddles;
        tw1 = tw2;
        /* For non-custom modes, m is guaranteed to be a multiple of 4. */
        k = m as usize;
        loop {
            scratch[1] = *Fout.offset(m as isize) * (*tw1);
            scratch[2] = *Fout.offset(m2 as isize) * (*tw2);

            scratch[3] = scratch[1] + scratch[2];
            scratch[0] = scratch[1] - scratch[2];
            tw1 = tw1.offset(fstride as isize);
            tw2 = tw2.offset(fstride.wrapping_mul(2) as isize);

            (*Fout.offset(m as isize)) = (*Fout) - scratch[3] * 0.5f32;

            scratch[0] *= epi3.im;

            (*Fout) += scratch[3];

            (*Fout.offset(m2 as isize)).re = (*Fout.offset(m as isize)).re + scratch[0].im;
            (*Fout.offset(m2 as isize)).im = (*Fout.offset(m as isize)).im - scratch[0].re;

            (*Fout.offset(m as isize)).re = (*Fout.offset(m as isize)).re - scratch[0].im;
            (*Fout.offset(m as isize)).im = (*Fout.offset(m as isize)).im + scratch[0].re;

            Fout = Fout.offset(1);

            k -= 1;
            if k == 0 {
                break;
            }
        }
    }
}
unsafe fn kf_bfly5(
    mut Fout: *mut kiss_fft_cpx,
    fstride: size_t,
    st: *const kiss_fft_state,
    m: i32,
    N: i32,
    mm: i32,
) {
    let mut Fout0: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout1: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout2: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout3: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout4: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut scratch: [kiss_fft_cpx; 13] = [kiss_fft_cpx::zero(); 13];
    let mut tw: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
    let Fout_beg: *mut kiss_fft_cpx = Fout;
    let ya = *((*st).twiddles).offset(fstride.wrapping_mul(m as u64) as isize);
    let yb = *((*st).twiddles)
        .offset(fstride.wrapping_mul(2 as i32 as u64).wrapping_mul(m as u64) as isize);
    tw = (*st).twiddles;
    for i in 0..N {
        Fout = Fout_beg.offset((i * mm) as isize);
        Fout0 = Fout;
        Fout1 = Fout0.offset(m as isize);
        Fout2 = Fout0.offset(2 * m as isize);
        Fout3 = Fout0.offset(3 * m as isize);
        Fout4 = Fout0.offset(4 * m as isize);

        /* For non-custom modes, m is guaranteed to be a multiple of 4. */
        for u in 0..m as u64 {
            scratch[0] = *Fout0;

            scratch[1] = *Fout1 * (*tw.offset((u).wrapping_mul(fstride) as isize));
            scratch[2] = *Fout2 * (*tw.offset((2 * u).wrapping_mul(fstride) as isize));
            scratch[3] = *Fout3 * (*tw.offset((3 * u).wrapping_mul(fstride) as isize));
            scratch[4] = *Fout4 * (*tw.offset((4 * u).wrapping_mul(fstride) as isize));

            scratch[7] = scratch[1] + scratch[4];
            scratch[10] = scratch[1] - scratch[4];
            scratch[8] = scratch[2] + scratch[3];
            scratch[9] = scratch[2] - scratch[3];

            (*Fout0) = (*Fout0) + (scratch[7] + scratch[8]);

            scratch[5].re = scratch[0].re + (scratch[7].re * ya.re + scratch[8].re * yb.re);
            scratch[5].im = scratch[0].im + (scratch[7].im * ya.re + scratch[8].im * yb.re);

            scratch[6].re = scratch[10].im * ya.im + scratch[9].im * yb.im;
            scratch[6].im = -(scratch[10].re * ya.im + scratch[9].re * yb.im);

            (*Fout1) = scratch[5] - scratch[6];
            (*Fout4) = scratch[5] + scratch[6];

            scratch[11].re = scratch[0].re + (scratch[7].re * yb.re + scratch[8].re * ya.re);
            scratch[11].im = scratch[0].im + (scratch[7].im * yb.re + scratch[8].im * ya.re);
            scratch[12].re = scratch[9].im * ya.im - scratch[10].im * yb.im;
            scratch[12].im = scratch[10].re * yb.im - scratch[9].re * ya.im;

            (*Fout2) = scratch[11] + scratch[12];
            (*Fout3) = scratch[11] - scratch[12];

            Fout0 = Fout0.offset(1);
            Fout1 = Fout1.offset(1);
            Fout2 = Fout2.offset(1);
            Fout3 = Fout3.offset(1);
            Fout4 = Fout4.offset(1);
        }
    }
}
pub unsafe fn opus_fft_impl(st: *const kiss_fft_state, fout: *mut kiss_fft_cpx) {
    let mut m2: i32 = 0;
    let mut m: i32 = 0;
    let mut p: i32 = 0;
    let mut L: i32 = 0;
    let mut fstride: [i32; 8] = [0; 8];
    let mut i: i32 = 0;
    let mut shift: i32 = 0;
    shift = if (*st).shift > 0 as i32 {
        (*st).shift
    } else {
        0 as i32
    };
    fstride[0] = 1 as i32;
    L = 0 as i32;
    loop {
        p = (*st).factors[(2 as i32 * L) as usize] as i32;
        m = (*st).factors[(2 as i32 * L + 1 as i32) as usize] as i32;
        fstride[(L + 1 as i32) as usize] = fstride[L as usize] * p;
        L += 1;
        if !(m != 1 as i32) {
            break;
        }
    }
    m = (*st).factors[(2 as i32 * L - 1 as i32) as usize] as i32;
    i = L - 1 as i32;
    while i >= 0 as i32 {
        if i != 0 as i32 {
            m2 = (*st).factors[(2 as i32 * i - 1 as i32) as usize] as i32;
        } else {
            m2 = 1 as i32;
        }
        match (*st).factors[(2 as i32 * i) as usize] as i32 {
            2 => {
                kf_bfly2(fout, m, fstride[i as usize]);
            }
            4 => {
                kf_bfly4(
                    fout,
                    (fstride[i as usize] << shift) as size_t,
                    st,
                    m,
                    fstride[i as usize],
                    m2,
                );
            }
            3 => {
                kf_bfly3(
                    fout,
                    (fstride[i as usize] << shift) as size_t,
                    st,
                    m,
                    fstride[i as usize],
                    m2,
                );
            }
            5 => {
                kf_bfly5(
                    fout,
                    (fstride[i as usize] << shift) as size_t,
                    st,
                    m,
                    fstride[i as usize],
                    m2,
                );
            }
            _ => {}
        }
        m = m2;
        i -= 1;
    }
}
pub unsafe fn opus_fft_c(
    st: *const kiss_fft_state,
    fin: *const kiss_fft_cpx,
    fout: *mut kiss_fft_cpx,
) {
    let mut i: i32 = 0;
    let mut scale: opus_val16 = 0.;
    scale = (*st).scale;
    if !(fin != fout as *const kiss_fft_cpx) {
        celt_fatal(
            b"assertion failed: fin != fout\nIn-place FFT not supported\0" as *const u8
                as *const i8,
            b"celt/kiss_fft.c\0" as *const u8 as *const i8,
            580 as i32,
        );
    }
    i = 0 as i32;
    while i < (*st).nfft {
        let x: kiss_fft_cpx = *fin.offset(i as isize);
        (*fout.offset(*((*st).bitrev).offset(i as isize) as isize)) = scale * x;
        i += 1;
    }
    opus_fft_impl(st, fout);
}
