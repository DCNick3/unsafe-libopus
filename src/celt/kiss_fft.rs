#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:38"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "65:9"]
pub struct kiss_fft_cpx {
    pub r: libc::c_float,
    pub i: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "70:9"]
pub struct kiss_twiddle_cpx {
    pub r: libc::c_float,
    pub i: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "81:16"]
pub struct arch_fft_state {
    pub is_supported: libc::c_int,
    pub priv_0: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "86:16"]
pub struct kiss_fft_state {
    pub nfft: libc::c_int,
    pub scale: opus_val16,
    pub shift: libc::c_int,
    pub factors: [i16; 16],
    pub bitrev: *const i16,
    pub twiddles: *const kiss_twiddle_cpx,
    pub arch_fft: *mut arch_fft_state,
}
pub use self::arch_h::opus_val16;
pub use self::stddef_h::size_t;
use crate::celt::celt::celt_fatal;

#[c2rust::src_loc = "48:1"]
unsafe fn kf_bfly2(mut Fout: *mut kiss_fft_cpx, m: libc::c_int, N: libc::c_int) {
    let mut Fout2: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut i: libc::c_int = 0;
    let mut tw: opus_val16 = 0.;
    tw = 0.7071067812f32;
    if !(m == 4 as libc::c_int) {
        celt_fatal(
            b"assertion failed: m==4\0" as *const u8 as *const libc::c_char,
            b"celt/kiss_fft.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < N {
        let mut t: kiss_fft_cpx = kiss_fft_cpx { r: 0., i: 0. };
        Fout2 = Fout.offset(4 as libc::c_int as isize);
        t = *Fout2.offset(0 as libc::c_int as isize);
        (*Fout2.offset(0 as libc::c_int as isize)).r =
            (*Fout.offset(0 as libc::c_int as isize)).r - t.r;
        (*Fout2.offset(0 as libc::c_int as isize)).i =
            (*Fout.offset(0 as libc::c_int as isize)).i - t.i;
        (*Fout.offset(0 as libc::c_int as isize)).r += t.r;
        (*Fout.offset(0 as libc::c_int as isize)).i += t.i;
        t.r = ((*Fout2.offset(1 as libc::c_int as isize)).r
            + (*Fout2.offset(1 as libc::c_int as isize)).i)
            * tw;
        t.i = ((*Fout2.offset(1 as libc::c_int as isize)).i
            - (*Fout2.offset(1 as libc::c_int as isize)).r)
            * tw;
        (*Fout2.offset(1 as libc::c_int as isize)).r =
            (*Fout.offset(1 as libc::c_int as isize)).r - t.r;
        (*Fout2.offset(1 as libc::c_int as isize)).i =
            (*Fout.offset(1 as libc::c_int as isize)).i - t.i;
        (*Fout.offset(1 as libc::c_int as isize)).r += t.r;
        (*Fout.offset(1 as libc::c_int as isize)).i += t.i;
        t.r = (*Fout2.offset(2 as libc::c_int as isize)).i;
        t.i = -(*Fout2.offset(2 as libc::c_int as isize)).r;
        (*Fout2.offset(2 as libc::c_int as isize)).r =
            (*Fout.offset(2 as libc::c_int as isize)).r - t.r;
        (*Fout2.offset(2 as libc::c_int as isize)).i =
            (*Fout.offset(2 as libc::c_int as isize)).i - t.i;
        (*Fout.offset(2 as libc::c_int as isize)).r += t.r;
        (*Fout.offset(2 as libc::c_int as isize)).i += t.i;
        t.r = ((*Fout2.offset(3 as libc::c_int as isize)).i
            - (*Fout2.offset(3 as libc::c_int as isize)).r)
            * tw;
        t.i = -((*Fout2.offset(3 as libc::c_int as isize)).i
            + (*Fout2.offset(3 as libc::c_int as isize)).r)
            * tw;
        (*Fout2.offset(3 as libc::c_int as isize)).r =
            (*Fout.offset(3 as libc::c_int as isize)).r - t.r;
        (*Fout2.offset(3 as libc::c_int as isize)).i =
            (*Fout.offset(3 as libc::c_int as isize)).i - t.i;
        (*Fout.offset(3 as libc::c_int as isize)).r += t.r;
        (*Fout.offset(3 as libc::c_int as isize)).i += t.i;
        Fout = Fout.offset(8 as libc::c_int as isize);
        i += 1;
    }
}
#[c2rust::src_loc = "104:1"]
unsafe fn kf_bfly4(
    mut Fout: *mut kiss_fft_cpx,
    fstride: size_t,
    st: *const kiss_fft_state,
    m: libc::c_int,
    N: libc::c_int,
    mm: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if m == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < N {
            let mut scratch0: kiss_fft_cpx = kiss_fft_cpx { r: 0., i: 0. };
            let mut scratch1: kiss_fft_cpx = kiss_fft_cpx { r: 0., i: 0. };
            scratch0.r = (*Fout).r - (*Fout.offset(2 as libc::c_int as isize)).r;
            scratch0.i = (*Fout).i - (*Fout.offset(2 as libc::c_int as isize)).i;
            (*Fout).r += (*Fout.offset(2 as libc::c_int as isize)).r;
            (*Fout).i += (*Fout.offset(2 as libc::c_int as isize)).i;
            scratch1.r = (*Fout.offset(1 as libc::c_int as isize)).r
                + (*Fout.offset(3 as libc::c_int as isize)).r;
            scratch1.i = (*Fout.offset(1 as libc::c_int as isize)).i
                + (*Fout.offset(3 as libc::c_int as isize)).i;
            (*Fout.offset(2 as libc::c_int as isize)).r = (*Fout).r - scratch1.r;
            (*Fout.offset(2 as libc::c_int as isize)).i = (*Fout).i - scratch1.i;
            (*Fout).r += scratch1.r;
            (*Fout).i += scratch1.i;
            scratch1.r = (*Fout.offset(1 as libc::c_int as isize)).r
                - (*Fout.offset(3 as libc::c_int as isize)).r;
            scratch1.i = (*Fout.offset(1 as libc::c_int as isize)).i
                - (*Fout.offset(3 as libc::c_int as isize)).i;
            (*Fout.offset(1 as libc::c_int as isize)).r = scratch0.r + scratch1.i;
            (*Fout.offset(1 as libc::c_int as isize)).i = scratch0.i - scratch1.r;
            (*Fout.offset(3 as libc::c_int as isize)).r = scratch0.r - scratch1.i;
            (*Fout.offset(3 as libc::c_int as isize)).i = scratch0.i + scratch1.r;
            Fout = Fout.offset(4 as libc::c_int as isize);
            i += 1;
        }
    } else {
        let mut j: libc::c_int = 0;
        let mut scratch: [kiss_fft_cpx; 6] = [kiss_fft_cpx { r: 0., i: 0. }; 6];
        let mut tw1: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
        let mut tw2: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
        let mut tw3: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
        let m2: libc::c_int = 2 as libc::c_int * m;
        let m3: libc::c_int = 3 as libc::c_int * m;
        let Fout_beg: *mut kiss_fft_cpx = Fout;
        i = 0 as libc::c_int;
        while i < N {
            Fout = Fout_beg.offset((i * mm) as isize);
            tw1 = (*st).twiddles;
            tw2 = tw1;
            tw3 = tw2;
            j = 0 as libc::c_int;
            while j < m {
                scratch[0 as libc::c_int as usize].r = (*Fout.offset(m as isize)).r * (*tw1).r
                    - (*Fout.offset(m as isize)).i * (*tw1).i;
                scratch[0 as libc::c_int as usize].i = (*Fout.offset(m as isize)).r * (*tw1).i
                    + (*Fout.offset(m as isize)).i * (*tw1).r;
                scratch[1 as libc::c_int as usize].r = (*Fout.offset(m2 as isize)).r * (*tw2).r
                    - (*Fout.offset(m2 as isize)).i * (*tw2).i;
                scratch[1 as libc::c_int as usize].i = (*Fout.offset(m2 as isize)).r * (*tw2).i
                    + (*Fout.offset(m2 as isize)).i * (*tw2).r;
                scratch[2 as libc::c_int as usize].r = (*Fout.offset(m3 as isize)).r * (*tw3).r
                    - (*Fout.offset(m3 as isize)).i * (*tw3).i;
                scratch[2 as libc::c_int as usize].i = (*Fout.offset(m3 as isize)).r * (*tw3).i
                    + (*Fout.offset(m3 as isize)).i * (*tw3).r;
                scratch[5 as libc::c_int as usize].r =
                    (*Fout).r - scratch[1 as libc::c_int as usize].r;
                scratch[5 as libc::c_int as usize].i =
                    (*Fout).i - scratch[1 as libc::c_int as usize].i;
                (*Fout).r += scratch[1 as libc::c_int as usize].r;
                (*Fout).i += scratch[1 as libc::c_int as usize].i;
                scratch[3 as libc::c_int as usize].r =
                    scratch[0 as libc::c_int as usize].r + scratch[2 as libc::c_int as usize].r;
                scratch[3 as libc::c_int as usize].i =
                    scratch[0 as libc::c_int as usize].i + scratch[2 as libc::c_int as usize].i;
                scratch[4 as libc::c_int as usize].r =
                    scratch[0 as libc::c_int as usize].r - scratch[2 as libc::c_int as usize].r;
                scratch[4 as libc::c_int as usize].i =
                    scratch[0 as libc::c_int as usize].i - scratch[2 as libc::c_int as usize].i;
                (*Fout.offset(m2 as isize)).r = (*Fout).r - scratch[3 as libc::c_int as usize].r;
                (*Fout.offset(m2 as isize)).i = (*Fout).i - scratch[3 as libc::c_int as usize].i;
                tw1 = tw1.offset(fstride as isize);
                tw2 = tw2.offset(fstride.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
                tw3 = tw3.offset(fstride.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize);
                (*Fout).r += scratch[3 as libc::c_int as usize].r;
                (*Fout).i += scratch[3 as libc::c_int as usize].i;
                (*Fout.offset(m as isize)).r =
                    scratch[5 as libc::c_int as usize].r + scratch[4 as libc::c_int as usize].i;
                (*Fout.offset(m as isize)).i =
                    scratch[5 as libc::c_int as usize].i - scratch[4 as libc::c_int as usize].r;
                (*Fout.offset(m3 as isize)).r =
                    scratch[5 as libc::c_int as usize].r - scratch[4 as libc::c_int as usize].i;
                (*Fout.offset(m3 as isize)).i =
                    scratch[5 as libc::c_int as usize].i + scratch[4 as libc::c_int as usize].r;
                Fout = Fout.offset(1);
                j += 1;
            }
            i += 1;
        }
    };
}
#[c2rust::src_loc = "176:1"]
unsafe fn kf_bfly3(
    mut Fout: *mut kiss_fft_cpx,
    fstride: size_t,
    st: *const kiss_fft_state,
    m: libc::c_int,
    N: libc::c_int,
    mm: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: size_t = 0;
    let m2: size_t = (2 as libc::c_int * m) as size_t;
    let mut tw1: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
    let mut tw2: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
    let mut scratch: [kiss_fft_cpx; 5] = [kiss_fft_cpx { r: 0., i: 0. }; 5];
    let mut epi3: kiss_twiddle_cpx = kiss_twiddle_cpx { r: 0., i: 0. };
    let Fout_beg: *mut kiss_fft_cpx = Fout;
    epi3 = *((*st).twiddles).offset(fstride.wrapping_mul(m as libc::c_ulong) as isize);
    i = 0 as libc::c_int;
    while i < N {
        Fout = Fout_beg.offset((i * mm) as isize);
        tw2 = (*st).twiddles;
        tw1 = tw2;
        k = m as size_t;
        loop {
            scratch[1 as libc::c_int as usize].r =
                (*Fout.offset(m as isize)).r * (*tw1).r - (*Fout.offset(m as isize)).i * (*tw1).i;
            scratch[1 as libc::c_int as usize].i =
                (*Fout.offset(m as isize)).r * (*tw1).i + (*Fout.offset(m as isize)).i * (*tw1).r;
            scratch[2 as libc::c_int as usize].r =
                (*Fout.offset(m2 as isize)).r * (*tw2).r - (*Fout.offset(m2 as isize)).i * (*tw2).i;
            scratch[2 as libc::c_int as usize].i =
                (*Fout.offset(m2 as isize)).r * (*tw2).i + (*Fout.offset(m2 as isize)).i * (*tw2).r;
            scratch[3 as libc::c_int as usize].r =
                scratch[1 as libc::c_int as usize].r + scratch[2 as libc::c_int as usize].r;
            scratch[3 as libc::c_int as usize].i =
                scratch[1 as libc::c_int as usize].i + scratch[2 as libc::c_int as usize].i;
            scratch[0 as libc::c_int as usize].r =
                scratch[1 as libc::c_int as usize].r - scratch[2 as libc::c_int as usize].r;
            scratch[0 as libc::c_int as usize].i =
                scratch[1 as libc::c_int as usize].i - scratch[2 as libc::c_int as usize].i;
            tw1 = tw1.offset(fstride as isize);
            tw2 = tw2.offset(fstride.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize);
            (*Fout.offset(m as isize)).r =
                (*Fout).r - scratch[3 as libc::c_int as usize].r * 0.5f32;
            (*Fout.offset(m as isize)).i =
                (*Fout).i - scratch[3 as libc::c_int as usize].i * 0.5f32;
            scratch[0 as libc::c_int as usize].r *= epi3.i;
            scratch[0 as libc::c_int as usize].i *= epi3.i;
            (*Fout).r += scratch[3 as libc::c_int as usize].r;
            (*Fout).i += scratch[3 as libc::c_int as usize].i;
            (*Fout.offset(m2 as isize)).r =
                (*Fout.offset(m as isize)).r + scratch[0 as libc::c_int as usize].i;
            (*Fout.offset(m2 as isize)).i =
                (*Fout.offset(m as isize)).i - scratch[0 as libc::c_int as usize].r;
            (*Fout.offset(m as isize)).r =
                (*Fout.offset(m as isize)).r - scratch[0 as libc::c_int as usize].i;
            (*Fout.offset(m as isize)).i =
                (*Fout.offset(m as isize)).i + scratch[0 as libc::c_int as usize].r;
            Fout = Fout.offset(1);
            k = k.wrapping_sub(1);
            if !(k != 0) {
                break;
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "235:1"]
unsafe fn kf_bfly5(
    mut Fout: *mut kiss_fft_cpx,
    fstride: size_t,
    st: *const kiss_fft_state,
    m: libc::c_int,
    N: libc::c_int,
    mm: libc::c_int,
) {
    let mut Fout0: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout1: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout2: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout3: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut Fout4: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut i: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut scratch: [kiss_fft_cpx; 13] = [kiss_fft_cpx { r: 0., i: 0. }; 13];
    let mut tw: *const kiss_twiddle_cpx = 0 as *const kiss_twiddle_cpx;
    let mut ya: kiss_twiddle_cpx = kiss_twiddle_cpx { r: 0., i: 0. };
    let mut yb: kiss_twiddle_cpx = kiss_twiddle_cpx { r: 0., i: 0. };
    let Fout_beg: *mut kiss_fft_cpx = Fout;
    ya = *((*st).twiddles).offset(fstride.wrapping_mul(m as libc::c_ulong) as isize);
    yb = *((*st).twiddles).offset(
        fstride
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(m as libc::c_ulong) as isize,
    );
    tw = (*st).twiddles;
    i = 0 as libc::c_int;
    while i < N {
        Fout = Fout_beg.offset((i * mm) as isize);
        Fout0 = Fout;
        Fout1 = Fout0.offset(m as isize);
        Fout2 = Fout0.offset((2 as libc::c_int * m) as isize);
        Fout3 = Fout0.offset((3 as libc::c_int * m) as isize);
        Fout4 = Fout0.offset((4 as libc::c_int * m) as isize);
        u = 0 as libc::c_int;
        while u < m {
            scratch[0 as libc::c_int as usize] = *Fout0;
            scratch[1 as libc::c_int as usize].r = (*Fout1).r
                * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).r
                - (*Fout1).i * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).i;
            scratch[1 as libc::c_int as usize].i = (*Fout1).r
                * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).i
                + (*Fout1).i * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).r;
            scratch[2 as libc::c_int as usize].r = (*Fout2).r
                * (*tw.offset(
                    ((2 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                ))
                .r
                - (*Fout2).i
                    * (*tw.offset(
                        ((2 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                    ))
                    .i;
            scratch[2 as libc::c_int as usize].i = (*Fout2).r
                * (*tw.offset(
                    ((2 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                ))
                .i
                + (*Fout2).i
                    * (*tw.offset(
                        ((2 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                    ))
                    .r;
            scratch[3 as libc::c_int as usize].r = (*Fout3).r
                * (*tw.offset(
                    ((3 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                ))
                .r
                - (*Fout3).i
                    * (*tw.offset(
                        ((3 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                    ))
                    .i;
            scratch[3 as libc::c_int as usize].i = (*Fout3).r
                * (*tw.offset(
                    ((3 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                ))
                .i
                + (*Fout3).i
                    * (*tw.offset(
                        ((3 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                    ))
                    .r;
            scratch[4 as libc::c_int as usize].r = (*Fout4).r
                * (*tw.offset(
                    ((4 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                ))
                .r
                - (*Fout4).i
                    * (*tw.offset(
                        ((4 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                    ))
                    .i;
            scratch[4 as libc::c_int as usize].i = (*Fout4).r
                * (*tw.offset(
                    ((4 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                ))
                .i
                + (*Fout4).i
                    * (*tw.offset(
                        ((4 as libc::c_int * u) as libc::c_ulong).wrapping_mul(fstride) as isize,
                    ))
                    .r;
            scratch[7 as libc::c_int as usize].r =
                scratch[1 as libc::c_int as usize].r + scratch[4 as libc::c_int as usize].r;
            scratch[7 as libc::c_int as usize].i =
                scratch[1 as libc::c_int as usize].i + scratch[4 as libc::c_int as usize].i;
            scratch[10 as libc::c_int as usize].r =
                scratch[1 as libc::c_int as usize].r - scratch[4 as libc::c_int as usize].r;
            scratch[10 as libc::c_int as usize].i =
                scratch[1 as libc::c_int as usize].i - scratch[4 as libc::c_int as usize].i;
            scratch[8 as libc::c_int as usize].r =
                scratch[2 as libc::c_int as usize].r + scratch[3 as libc::c_int as usize].r;
            scratch[8 as libc::c_int as usize].i =
                scratch[2 as libc::c_int as usize].i + scratch[3 as libc::c_int as usize].i;
            scratch[9 as libc::c_int as usize].r =
                scratch[2 as libc::c_int as usize].r - scratch[3 as libc::c_int as usize].r;
            scratch[9 as libc::c_int as usize].i =
                scratch[2 as libc::c_int as usize].i - scratch[3 as libc::c_int as usize].i;
            (*Fout0).r = (*Fout0).r
                + (scratch[7 as libc::c_int as usize].r + scratch[8 as libc::c_int as usize].r);
            (*Fout0).i = (*Fout0).i
                + (scratch[7 as libc::c_int as usize].i + scratch[8 as libc::c_int as usize].i);
            scratch[5 as libc::c_int as usize].r = scratch[0 as libc::c_int as usize].r
                + (scratch[7 as libc::c_int as usize].r * ya.r
                    + scratch[8 as libc::c_int as usize].r * yb.r);
            scratch[5 as libc::c_int as usize].i = scratch[0 as libc::c_int as usize].i
                + (scratch[7 as libc::c_int as usize].i * ya.r
                    + scratch[8 as libc::c_int as usize].i * yb.r);
            scratch[6 as libc::c_int as usize].r = scratch[10 as libc::c_int as usize].i * ya.i
                + scratch[9 as libc::c_int as usize].i * yb.i;
            scratch[6 as libc::c_int as usize].i = -(scratch[10 as libc::c_int as usize].r * ya.i
                + scratch[9 as libc::c_int as usize].r * yb.i);
            (*Fout1).r =
                scratch[5 as libc::c_int as usize].r - scratch[6 as libc::c_int as usize].r;
            (*Fout1).i =
                scratch[5 as libc::c_int as usize].i - scratch[6 as libc::c_int as usize].i;
            (*Fout4).r =
                scratch[5 as libc::c_int as usize].r + scratch[6 as libc::c_int as usize].r;
            (*Fout4).i =
                scratch[5 as libc::c_int as usize].i + scratch[6 as libc::c_int as usize].i;
            scratch[11 as libc::c_int as usize].r = scratch[0 as libc::c_int as usize].r
                + (scratch[7 as libc::c_int as usize].r * yb.r
                    + scratch[8 as libc::c_int as usize].r * ya.r);
            scratch[11 as libc::c_int as usize].i = scratch[0 as libc::c_int as usize].i
                + (scratch[7 as libc::c_int as usize].i * yb.r
                    + scratch[8 as libc::c_int as usize].i * ya.r);
            scratch[12 as libc::c_int as usize].r = scratch[9 as libc::c_int as usize].i * ya.i
                - scratch[10 as libc::c_int as usize].i * yb.i;
            scratch[12 as libc::c_int as usize].i = scratch[10 as libc::c_int as usize].r * yb.i
                - scratch[9 as libc::c_int as usize].r * ya.i;
            (*Fout2).r =
                scratch[11 as libc::c_int as usize].r + scratch[12 as libc::c_int as usize].r;
            (*Fout2).i =
                scratch[11 as libc::c_int as usize].i + scratch[12 as libc::c_int as usize].i;
            (*Fout3).r =
                scratch[11 as libc::c_int as usize].r - scratch[12 as libc::c_int as usize].r;
            (*Fout3).i =
                scratch[11 as libc::c_int as usize].i - scratch[12 as libc::c_int as usize].i;
            Fout0 = Fout0.offset(1);
            Fout1 = Fout1.offset(1);
            Fout2 = Fout2.offset(1);
            Fout3 = Fout3.offset(1);
            Fout4 = Fout4.offset(1);
            u += 1;
        }
        i += 1;
    }
}
#[c2rust::src_loc = "521:1"]
pub unsafe fn opus_fft_impl(st: *const kiss_fft_state, fout: *mut kiss_fft_cpx) {
    let mut m2: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut L: libc::c_int = 0;
    let mut fstride: [libc::c_int; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    shift = if (*st).shift > 0 as libc::c_int {
        (*st).shift
    } else {
        0 as libc::c_int
    };
    fstride[0 as libc::c_int as usize] = 1 as libc::c_int;
    L = 0 as libc::c_int;
    loop {
        p = (*st).factors[(2 as libc::c_int * L) as usize] as libc::c_int;
        m = (*st).factors[(2 as libc::c_int * L + 1 as libc::c_int) as usize] as libc::c_int;
        fstride[(L + 1 as libc::c_int) as usize] = fstride[L as usize] * p;
        L += 1;
        if !(m != 1 as libc::c_int) {
            break;
        }
    }
    m = (*st).factors[(2 as libc::c_int * L - 1 as libc::c_int) as usize] as libc::c_int;
    i = L - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if i != 0 as libc::c_int {
            m2 = (*st).factors[(2 as libc::c_int * i - 1 as libc::c_int) as usize] as libc::c_int;
        } else {
            m2 = 1 as libc::c_int;
        }
        match (*st).factors[(2 as libc::c_int * i) as usize] as libc::c_int {
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
#[c2rust::src_loc = "569:1"]
pub unsafe fn opus_fft_c(
    st: *const kiss_fft_state,
    fin: *const kiss_fft_cpx,
    fout: *mut kiss_fft_cpx,
) {
    let mut i: libc::c_int = 0;
    let mut scale: opus_val16 = 0.;
    scale = (*st).scale;
    if !(fin != fout as *const kiss_fft_cpx) {
        celt_fatal(
            b"assertion failed: fin != fout\nIn-place FFT not supported\0" as *const u8
                as *const libc::c_char,
            b"celt/kiss_fft.c\0" as *const u8 as *const libc::c_char,
            580 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < (*st).nfft {
        let x: kiss_fft_cpx = *fin.offset(i as isize);
        (*fout.offset(*((*st).bitrev).offset(i as isize) as isize)).r = scale * x.r;
        (*fout.offset(*((*st).bitrev).offset(i as isize) as isize)).i = scale * x.i;
        i += 1;
    }
    opus_fft_impl(st, fout);
}
#[c2rust::src_loc = "592:1"]
pub unsafe fn opus_ifft_c(
    st: *const kiss_fft_state,
    fin: *const kiss_fft_cpx,
    fout: *mut kiss_fft_cpx,
) {
    let mut i: libc::c_int = 0;
    if !(fin != fout as *const kiss_fft_cpx) {
        celt_fatal(
            b"assertion failed: fin != fout\nIn-place FFT not supported\0" as *const u8
                as *const libc::c_char,
            b"celt/kiss_fft.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < (*st).nfft {
        *fout.offset(*((*st).bitrev).offset(i as isize) as isize) = *fin.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*st).nfft {
        (*fout.offset(i as isize)).i = -(*fout.offset(i as isize)).i;
        i += 1;
    }
    opus_fft_impl(st, fout);
    i = 0 as libc::c_int;
    while i < (*st).nfft {
        (*fout.offset(i as isize)).i = -(*fout.offset(i as isize)).i;
        i += 1;
    }
}
