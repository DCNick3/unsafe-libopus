use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:38"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:38"]
pub mod opus_types_h {
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:38"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "183:1"]
    pub type celt_sig = libc::c_float;
    #[c2rust::src_loc = "203:9"]
    pub const Q15ONE: libc::c_float = 1.0f32;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:38"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:38"]
pub mod entcode_h {
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe extern "C" fn celt_udiv(
        n: opus_uint32,
        d: opus_uint32,
    ) -> opus_uint32 {
        return n.wrapping_div(d);
    }
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:38"]
pub mod pitch_h {
    #[inline]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn dual_inner_prod_c(
        x: *const opus_val16,
        y01: *const opus_val16,
        y02: *const opus_val16,
        N: libc::c_int,
        xy1: *mut opus_val32,
        xy2: *mut opus_val32,
    ) {
        let mut i: libc::c_int = 0;
        let mut xy01: opus_val32 = 0 as libc::c_int as opus_val32;
        let mut xy02: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy01 = xy01 + *x.offset(i as isize) * *y01.offset(i as isize);
            xy02 = xy02 + *x.offset(i as isize) * *y02.offset(i as isize);
            i += 1;
        }
        *xy1 = xy01;
        *xy2 = xy02;
    }
    #[c2rust::src_loc = "189:10"]
    pub const celt_pitch_xcorr: unsafe extern "C" fn(
        *const opus_val16,
        *const opus_val16,
        *mut opus_val32,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> () = celt_pitch_xcorr_c;
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn xcorr_kernel_c(
        mut x: *const opus_val16,
        mut y: *const opus_val16,
        sum: *mut opus_val32,
        len: libc::c_int,
    ) {
        let mut j: libc::c_int = 0;
        let mut y_0: opus_val16 = 0.;
        let mut y_1: opus_val16 = 0.;
        let mut y_2: opus_val16 = 0.;
        let mut y_3: opus_val16 = 0.;
        if !(len >= 3 as libc::c_int) {
            celt_fatal(
                b"assertion failed: len>=3\0" as *const u8 as *const libc::c_char,
                b"./celt/pitch.h\0" as *const u8 as *const libc::c_char,
                69 as libc::c_int,
            );
        }
        y_3 = 0 as libc::c_int as opus_val16;
        let fresh0 = y;
        y = y.offset(1);
        y_0 = *fresh0;
        let fresh1 = y;
        y = y.offset(1);
        y_1 = *fresh1;
        let fresh2 = y;
        y = y.offset(1);
        y_2 = *fresh2;
        j = 0 as libc::c_int;
        while j < len - 3 as libc::c_int {
            let mut tmp: opus_val16 = 0.;
            let fresh3 = x;
            x = x.offset(1);
            tmp = *fresh3;
            let fresh4 = y;
            y = y.offset(1);
            y_3 = *fresh4;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp * y_0;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp * y_1;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp * y_2;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp * y_3;
            let fresh5 = x;
            x = x.offset(1);
            tmp = *fresh5;
            let fresh6 = y;
            y = y.offset(1);
            y_0 = *fresh6;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp * y_1;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp * y_2;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp * y_3;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp * y_0;
            let fresh7 = x;
            x = x.offset(1);
            tmp = *fresh7;
            let fresh8 = y;
            y = y.offset(1);
            y_1 = *fresh8;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp * y_2;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp * y_3;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp * y_0;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp * y_1;
            let fresh9 = x;
            x = x.offset(1);
            tmp = *fresh9;
            let fresh10 = y;
            y = y.offset(1);
            y_2 = *fresh10;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp * y_3;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp * y_0;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp * y_1;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp * y_2;
            j += 4 as libc::c_int;
        }
        let fresh11 = j;
        j = j + 1;
        if fresh11 < len {
            let fresh12 = x;
            x = x.offset(1);
            let tmp_0: opus_val16 = *fresh12;
            let fresh13 = y;
            y = y.offset(1);
            y_3 = *fresh13;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp_0 * y_0;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp_0 * y_1;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp_0 * y_2;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp_0 * y_3;
        }
        let fresh14 = j;
        j = j + 1;
        if fresh14 < len {
            let fresh15 = x;
            x = x.offset(1);
            let tmp_1: opus_val16 = *fresh15;
            let fresh16 = y;
            y = y.offset(1);
            y_0 = *fresh16;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp_1 * y_1;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp_1 * y_2;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp_1 * y_3;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp_1 * y_0;
        }
        if j < len {
            let fresh17 = x;
            x = x.offset(1);
            let tmp_2: opus_val16 = *fresh17;
            let fresh18 = y;
            y = y.offset(1);
            y_1 = *fresh18;
            *sum
                .offset(
                    0 as libc::c_int as isize,
                ) = *sum.offset(0 as libc::c_int as isize) + tmp_2 * y_2;
            *sum
                .offset(
                    1 as libc::c_int as isize,
                ) = *sum.offset(1 as libc::c_int as isize) + tmp_2 * y_3;
            *sum
                .offset(
                    2 as libc::c_int as isize,
                ) = *sum.offset(2 as libc::c_int as isize) + tmp_2 * y_0;
            *sum
                .offset(
                    3 as libc::c_int as isize,
                ) = *sum.offset(3 as libc::c_int as isize) + tmp_2 * y_1;
        }
    }
    #[inline]
    #[c2rust::src_loc = "159:1"]
    pub unsafe extern "C" fn celt_inner_prod_c(
        x: *const opus_val16,
        y: *const opus_val16,
        N: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1;
        }
        return xy;
    }
    use super::arch_h::{opus_val16, opus_val32, celt_fatal};
    use super::celt_pitch_xcorr_c;
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/stdlib.h:38"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "861:12"]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/celt_lpc.h:43"]
pub mod celt_lpc_h {
    use super::arch_h::{opus_val16, opus_val32};
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn _celt_lpc(_lpc: *mut opus_val16, ac: *const opus_val32, p: libc::c_int);
        #[c2rust::src_loc = "63:1"]
        pub fn _celt_autocorr(
            x: *const opus_val16,
            ac: *mut opus_val32,
            window: *const opus_val16,
            overlap: libc::c_int,
            lag: libc::c_int,
            n: libc::c_int,
            arch: libc::c_int,
        ) -> libc::c_int;
    }
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::opus_uint32;
pub use self::arch_h::{opus_val16, opus_val32, celt_sig, Q15ONE, celt_fatal};
use self::mathcalls_h::sqrt;
pub use self::entcode_h::celt_udiv;
pub use self::pitch_h::{
    dual_inner_prod_c, celt_pitch_xcorr, xcorr_kernel_c, celt_inner_prod_c,
};
pub use self::stddef_h::NULL;
use self::stdlib_h::abs;
use self::celt_lpc_h::{_celt_lpc, _celt_autocorr};
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn find_best_pitch(
    xcorr: *mut opus_val32,
    y: *mut opus_val16,
    len: libc::c_int,
    max_pitch: libc::c_int,
    best_pitch: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut Syy: opus_val32 = 1 as libc::c_int as opus_val32;
    let mut best_num: [opus_val16; 2] = [0.; 2];
    let mut best_den: [opus_val32; 2] = [0.; 2];
    best_num[0 as libc::c_int as usize] = -(1 as libc::c_int) as opus_val16;
    best_num[1 as libc::c_int as usize] = -(1 as libc::c_int) as opus_val16;
    best_den[0 as libc::c_int as usize] = 0 as libc::c_int as opus_val32;
    best_den[1 as libc::c_int as usize] = 0 as libc::c_int as opus_val32;
    *best_pitch.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *best_pitch.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    j = 0 as libc::c_int;
    while j < len {
        Syy = Syy + *y.offset(j as isize) * *y.offset(j as isize);
        j += 1;
    }
    i = 0 as libc::c_int;
    while i < max_pitch {
        if *xcorr.offset(i as isize) > 0 as libc::c_int as libc::c_float {
            let mut num: opus_val16 = 0.;
            let mut xcorr16: opus_val32 = 0.;
            xcorr16 = *xcorr.offset(i as isize);
            xcorr16 *= 1e-12f32;
            num = xcorr16 * xcorr16;
            if num * best_den[1 as libc::c_int as usize]
                > best_num[1 as libc::c_int as usize] * Syy
            {
                if num * best_den[0 as libc::c_int as usize]
                    > best_num[0 as libc::c_int as usize] * Syy
                {
                    best_num[1 as libc::c_int
                        as usize] = best_num[0 as libc::c_int as usize];
                    best_den[1 as libc::c_int
                        as usize] = best_den[0 as libc::c_int as usize];
                    *best_pitch
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *best_pitch.offset(0 as libc::c_int as isize);
                    best_num[0 as libc::c_int as usize] = num;
                    best_den[0 as libc::c_int as usize] = Syy;
                    *best_pitch.offset(0 as libc::c_int as isize) = i;
                } else {
                    best_num[1 as libc::c_int as usize] = num;
                    best_den[1 as libc::c_int as usize] = Syy;
                    *best_pitch.offset(1 as libc::c_int as isize) = i;
                }
            }
        }
        Syy
            += *y.offset((i + len) as isize) * *y.offset((i + len) as isize)
                - *y.offset(i as isize) * *y.offset(i as isize);
        Syy = if 1 as libc::c_int as libc::c_float > Syy {
            1 as libc::c_int as libc::c_float
        } else {
            Syy
        };
        i += 1;
    }
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn celt_fir5(
    x: *mut opus_val16,
    num: *const opus_val16,
    N: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut num0: opus_val16 = 0.;
    let mut num1: opus_val16 = 0.;
    let mut num2: opus_val16 = 0.;
    let mut num3: opus_val16 = 0.;
    let mut num4: opus_val16 = 0.;
    let mut mem0: opus_val32 = 0.;
    let mut mem1: opus_val32 = 0.;
    let mut mem2: opus_val32 = 0.;
    let mut mem3: opus_val32 = 0.;
    let mut mem4: opus_val32 = 0.;
    num0 = *num.offset(0 as libc::c_int as isize);
    num1 = *num.offset(1 as libc::c_int as isize);
    num2 = *num.offset(2 as libc::c_int as isize);
    num3 = *num.offset(3 as libc::c_int as isize);
    num4 = *num.offset(4 as libc::c_int as isize);
    mem0 = 0 as libc::c_int as opus_val32;
    mem1 = 0 as libc::c_int as opus_val32;
    mem2 = 0 as libc::c_int as opus_val32;
    mem3 = 0 as libc::c_int as opus_val32;
    mem4 = 0 as libc::c_int as opus_val32;
    i = 0 as libc::c_int;
    while i < N {
        let mut sum: opus_val32 = *x.offset(i as isize);
        sum = sum + num0 * mem0;
        sum = sum + num1 * mem1;
        sum = sum + num2 * mem2;
        sum = sum + num3 * mem3;
        sum = sum + num4 * mem4;
        mem4 = mem3;
        mem3 = mem2;
        mem2 = mem1;
        mem1 = mem0;
        mem0 = *x.offset(i as isize);
        *x.offset(i as isize) = sum;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn pitch_downsample(
    x: *mut *mut celt_sig,
    x_lp: *mut opus_val16,
    len: libc::c_int,
    C: libc::c_int,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ac: [opus_val32; 5] = [0.; 5];
    let mut tmp: opus_val16 = Q15ONE;
    let mut lpc: [opus_val16; 4] = [0.; 4];
    let mut lpc2: [opus_val16; 5] = [0.; 5];
    let c1: opus_val16 = 0.8f32;
    i = 1 as libc::c_int;
    while i < len >> 1 as libc::c_int {
        *x_lp
            .offset(
                i as isize,
            ) = 0.5f32
            * (0.5f32
                * (*(*x.offset(0 as libc::c_int as isize))
                    .offset((2 as libc::c_int * i - 1 as libc::c_int) as isize)
                    + *(*x.offset(0 as libc::c_int as isize))
                        .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize))
                + *(*x.offset(0 as libc::c_int as isize))
                    .offset((2 as libc::c_int * i) as isize));
        i += 1;
    }
    *x_lp
        .offset(
            0 as libc::c_int as isize,
        ) = 0.5f32
        * (0.5f32
            * *(*x.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            + *(*x.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize));
    if C == 2 as libc::c_int {
        i = 1 as libc::c_int;
        while i < len >> 1 as libc::c_int {
            let ref mut fresh19 = *x_lp.offset(i as isize);
            *fresh19
                += 0.5f32
                    * (0.5f32
                        * (*(*x.offset(1 as libc::c_int as isize))
                            .offset((2 as libc::c_int * i - 1 as libc::c_int) as isize)
                            + *(*x.offset(1 as libc::c_int as isize))
                                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize))
                        + *(*x.offset(1 as libc::c_int as isize))
                            .offset((2 as libc::c_int * i) as isize));
            i += 1;
        }
        let ref mut fresh20 = *x_lp.offset(0 as libc::c_int as isize);
        *fresh20
            += 0.5f32
                * (0.5f32
                    * *(*x.offset(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize)
                    + *(*x.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize));
    }
    _celt_autocorr(
        x_lp,
        ac.as_mut_ptr(),
        NULL as *const opus_val16,
        0 as libc::c_int,
        4 as libc::c_int,
        len >> 1 as libc::c_int,
        arch,
    );
    ac[0 as libc::c_int as usize] *= 1.0001f32;
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        ac[i as usize]
            -= ac[i as usize] * (0.008f32 * i as libc::c_float)
                * (0.008f32 * i as libc::c_float);
        i += 1;
    }
    _celt_lpc(lpc.as_mut_ptr(), ac.as_mut_ptr(), 4 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        tmp = 0.9f32 * tmp;
        lpc[i as usize] = lpc[i as usize] * tmp;
        i += 1;
    }
    lpc2[0 as libc::c_int as usize] = lpc[0 as libc::c_int as usize] + 0.8f32;
    lpc2[1 as libc::c_int
        as usize] = lpc[1 as libc::c_int as usize] + c1 * lpc[0 as libc::c_int as usize];
    lpc2[2 as libc::c_int
        as usize] = lpc[2 as libc::c_int as usize] + c1 * lpc[1 as libc::c_int as usize];
    lpc2[3 as libc::c_int
        as usize] = lpc[3 as libc::c_int as usize] + c1 * lpc[2 as libc::c_int as usize];
    lpc2[4 as libc::c_int as usize] = c1 * lpc[3 as libc::c_int as usize];
    celt_fir5(x_lp, lpc2.as_mut_ptr(), len >> 1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "214:1"]
pub unsafe extern "C" fn celt_pitch_xcorr_c(
    mut _x: *const opus_val16,
    mut _y: *const opus_val16,
    xcorr: *mut opus_val32,
    len: libc::c_int,
    max_pitch: libc::c_int,
    _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if !(max_pitch > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: max_pitch>0\0" as *const u8 as *const libc::c_char,
            b"celt/pitch.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < max_pitch - 3 as libc::c_int {
        let mut sum: [opus_val32; 4] = [
            0 as libc::c_int as opus_val32,
            0 as libc::c_int as opus_val32,
            0 as libc::c_int as opus_val32,
            0 as libc::c_int as opus_val32,
        ];
        xcorr_kernel_c(_x, _y.offset(i as isize), sum.as_mut_ptr(), len);
        *xcorr.offset(i as isize) = sum[0 as libc::c_int as usize];
        *xcorr.offset((i + 1 as libc::c_int) as isize) = sum[1 as libc::c_int as usize];
        *xcorr.offset((i + 2 as libc::c_int) as isize) = sum[2 as libc::c_int as usize];
        *xcorr.offset((i + 3 as libc::c_int) as isize) = sum[3 as libc::c_int as usize];
        i += 4 as libc::c_int;
    }
    while i < max_pitch {
        let mut sum_0: opus_val32 = 0.;
        sum_0 = celt_inner_prod_c(_x, _y.offset(i as isize), len);
        *xcorr.offset(i as isize) = sum_0;
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "284:1"]
pub unsafe extern "C" fn pitch_search(
    x_lp: *const opus_val16,
    y: *mut opus_val16,
    len: libc::c_int,
    max_pitch: libc::c_int,
    pitch: *mut libc::c_int,
    arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lag: libc::c_int = 0;
    let mut best_pitch: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let mut offset: libc::c_int = 0;
    if !(len > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: len>0\0" as *const u8 as *const libc::c_char,
            b"celt/pitch.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
        );
    }
    if !(max_pitch > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: max_pitch>0\0" as *const u8 as *const libc::c_char,
            b"celt/pitch.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
        );
    }
    lag = len + max_pitch;
    let vla = (len >> 2 as libc::c_int) as usize;
    let mut x_lp4: Vec::<opus_val16> = ::std::vec::from_elem(0., vla);
    let vla_0 = (lag >> 2 as libc::c_int) as usize;
    let mut y_lp4: Vec::<opus_val16> = ::std::vec::from_elem(0., vla_0);
    let vla_1 = (max_pitch >> 1 as libc::c_int) as usize;
    let mut xcorr: Vec::<opus_val32> = ::std::vec::from_elem(0., vla_1);
    j = 0 as libc::c_int;
    while j < len >> 2 as libc::c_int {
        *x_lp4
            .as_mut_ptr()
            .offset(j as isize) = *x_lp.offset((2 as libc::c_int * j) as isize);
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < lag >> 2 as libc::c_int {
        *y_lp4
            .as_mut_ptr()
            .offset(j as isize) = *y.offset((2 as libc::c_int * j) as isize);
        j += 1;
    }
    celt_pitch_xcorr_c(
        x_lp4.as_mut_ptr(),
        y_lp4.as_mut_ptr(),
        xcorr.as_mut_ptr(),
        len >> 2 as libc::c_int,
        max_pitch >> 2 as libc::c_int,
        arch,
    );
    find_best_pitch(
        xcorr.as_mut_ptr(),
        y_lp4.as_mut_ptr(),
        len >> 2 as libc::c_int,
        max_pitch >> 2 as libc::c_int,
        best_pitch.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < max_pitch >> 1 as libc::c_int {
        let mut sum: opus_val32 = 0.;
        *xcorr.as_mut_ptr().offset(i as isize) = 0 as libc::c_int as opus_val32;
        if !(abs(i - 2 as libc::c_int * best_pitch[0 as libc::c_int as usize])
            > 2 as libc::c_int
            && abs(i - 2 as libc::c_int * best_pitch[1 as libc::c_int as usize])
                > 2 as libc::c_int)
        {
            sum = celt_inner_prod_c(x_lp, y.offset(i as isize), len >> 1 as libc::c_int);
            *xcorr
                .as_mut_ptr()
                .offset(
                    i as isize,
                ) = if -(1 as libc::c_int) as libc::c_float > sum {
                -(1 as libc::c_int) as libc::c_float
            } else {
                sum
            };
        }
        i += 1;
    }
    find_best_pitch(
        xcorr.as_mut_ptr(),
        y,
        len >> 1 as libc::c_int,
        max_pitch >> 1 as libc::c_int,
        best_pitch.as_mut_ptr(),
    );
    if best_pitch[0 as libc::c_int as usize] > 0 as libc::c_int
        && best_pitch[0 as libc::c_int as usize]
            < (max_pitch >> 1 as libc::c_int) - 1 as libc::c_int
    {
        let mut a: opus_val32 = 0.;
        let mut b: opus_val32 = 0.;
        let mut c: opus_val32 = 0.;
        a = *xcorr
            .as_mut_ptr()
            .offset((best_pitch[0 as libc::c_int as usize] - 1 as libc::c_int) as isize);
        b = *xcorr.as_mut_ptr().offset(best_pitch[0 as libc::c_int as usize] as isize);
        c = *xcorr
            .as_mut_ptr()
            .offset((best_pitch[0 as libc::c_int as usize] + 1 as libc::c_int) as isize);
        if c - a > 0.7f32 * (b - a) {
            offset = 1 as libc::c_int;
        } else if a - c > 0.7f32 * (b - c) {
            offset = -(1 as libc::c_int);
        } else {
            offset = 0 as libc::c_int;
        }
    } else {
        offset = 0 as libc::c_int;
    }
    *pitch = 2 as libc::c_int * best_pitch[0 as libc::c_int as usize] - offset;
}
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn compute_pitch_gain(
    xy: opus_val32,
    xx: opus_val32,
    yy: opus_val32,
) -> opus_val16 {
    return xy
        / sqrt((1 as libc::c_int as libc::c_float + xx * yy) as libc::c_double)
            as libc::c_float;
}
#[c2rust::src_loc = "430:18"]
static mut second_check: [libc::c_int; 16] = [
    0 as libc::c_int,
    0 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    5 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    5 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
];
#[no_mangle]
#[c2rust::src_loc = "431:1"]
pub unsafe extern "C" fn remove_doubling(
    mut x: *mut opus_val16,
    mut maxperiod: libc::c_int,
    mut minperiod: libc::c_int,
    mut N: libc::c_int,
    T0_: *mut libc::c_int,
    mut prev_period: libc::c_int,
    prev_gain: opus_val16,
    _arch: libc::c_int,
) -> opus_val16 {
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut T: libc::c_int = 0;
    let mut T0: libc::c_int = 0;
    let mut g: opus_val16 = 0.;
    let mut g0: opus_val16 = 0.;
    let mut pg: opus_val16 = 0.;
    let mut xy: opus_val32 = 0.;
    let mut xx: opus_val32 = 0.;
    let mut yy: opus_val32 = 0.;
    let mut xy2: opus_val32 = 0.;
    let mut xcorr: [opus_val32; 3] = [0.; 3];
    let mut best_xy: opus_val32 = 0.;
    let mut best_yy: opus_val32 = 0.;
    let mut offset: libc::c_int = 0;
    let mut minperiod0: libc::c_int = 0;
    minperiod0 = minperiod;
    maxperiod /= 2 as libc::c_int;
    minperiod /= 2 as libc::c_int;
    *T0_ /= 2 as libc::c_int;
    prev_period /= 2 as libc::c_int;
    N /= 2 as libc::c_int;
    x = x.offset(maxperiod as isize);
    if *T0_ >= maxperiod {
        *T0_ = maxperiod - 1 as libc::c_int;
    }
    T0 = *T0_;
    T = T0;
    let vla = (maxperiod + 1 as libc::c_int) as usize;
    let mut yy_lookup: Vec::<opus_val32> = ::std::vec::from_elem(0., vla);
    dual_inner_prod_c(x, x, x.offset(-(T0 as isize)), N, &mut xx, &mut xy);
    *yy_lookup.as_mut_ptr().offset(0 as libc::c_int as isize) = xx;
    yy = xx;
    i = 1 as libc::c_int;
    while i <= maxperiod {
        yy = yy + *x.offset(-i as isize) * *x.offset(-i as isize)
            - *x.offset((N - i) as isize) * *x.offset((N - i) as isize);
        *yy_lookup
            .as_mut_ptr()
            .offset(
                i as isize,
            ) = if 0 as libc::c_int as libc::c_float > yy {
            0 as libc::c_int as libc::c_float
        } else {
            yy
        };
        i += 1;
    }
    yy = *yy_lookup.as_mut_ptr().offset(T0 as isize);
    best_xy = xy;
    best_yy = yy;
    g0 = compute_pitch_gain(xy, xx, yy);
    g = g0;
    k = 2 as libc::c_int;
    while k <= 15 as libc::c_int {
        let mut T1: libc::c_int = 0;
        let mut T1b: libc::c_int = 0;
        let mut g1: opus_val16 = 0.;
        let mut cont: opus_val16 = 0 as libc::c_int as opus_val16;
        let mut thresh: opus_val16 = 0.;
        T1 = celt_udiv(
            (2 as libc::c_int * T0 + k) as opus_uint32,
            (2 as libc::c_int * k) as opus_uint32,
        ) as libc::c_int;
        if T1 < minperiod {
            break;
        }
        if k == 2 as libc::c_int {
            if T1 + T0 > maxperiod {
                T1b = T0;
            } else {
                T1b = T0 + T1;
            }
        } else {
            T1b = celt_udiv(
                (2 as libc::c_int * second_check[k as usize] * T0 + k) as opus_uint32,
                (2 as libc::c_int * k) as opus_uint32,
            ) as libc::c_int;
        }
        dual_inner_prod_c(
            x,
            &mut *x.offset(-T1 as isize),
            &mut *x.offset(-T1b as isize),
            N,
            &mut xy,
            &mut xy2,
        );
        xy = 0.5f32 * (xy + xy2);
        yy = 0.5f32
            * (*yy_lookup.as_mut_ptr().offset(T1 as isize)
                + *yy_lookup.as_mut_ptr().offset(T1b as isize));
        g1 = compute_pitch_gain(xy, xx, yy);
        if abs(T1 - prev_period) <= 1 as libc::c_int {
            cont = prev_gain;
        } else if abs(T1 - prev_period) <= 2 as libc::c_int
            && 5 as libc::c_int * k * k < T0
        {
            cont = 0.5f32 * prev_gain;
        } else {
            cont = 0 as libc::c_int as opus_val16;
        }
        thresh = if 0.3f32 > 0.7f32 * g0 - cont { 0.3f32 } else { 0.7f32 * g0 - cont };
        if T1 < 3 as libc::c_int * minperiod {
            thresh = if 0.4f32 > 0.85f32 * g0 - cont {
                0.4f32
            } else {
                0.85f32 * g0 - cont
            };
        } else if T1 < 2 as libc::c_int * minperiod {
            thresh = if 0.5f32 > 0.9f32 * g0 - cont {
                0.5f32
            } else {
                0.9f32 * g0 - cont
            };
        }
        if g1 > thresh {
            best_xy = xy;
            best_yy = yy;
            T = T1;
            g = g1;
        }
        k += 1;
    }
    best_xy = if 0 as libc::c_int as libc::c_float > best_xy {
        0 as libc::c_int as libc::c_float
    } else {
        best_xy
    };
    if best_yy <= best_xy {
        pg = Q15ONE;
    } else {
        pg = best_xy / (best_yy + 1 as libc::c_int as libc::c_float);
    }
    k = 0 as libc::c_int;
    while k < 3 as libc::c_int {
        xcorr[k
            as usize] = celt_inner_prod_c(
            x,
            x.offset(-((T + k - 1 as libc::c_int) as isize)),
            N,
        );
        k += 1;
    }
    if xcorr[2 as libc::c_int as usize] - xcorr[0 as libc::c_int as usize]
        > 0.7f32 * (xcorr[1 as libc::c_int as usize] - xcorr[0 as libc::c_int as usize])
    {
        offset = 1 as libc::c_int;
    } else if xcorr[0 as libc::c_int as usize] - xcorr[2 as libc::c_int as usize]
        > 0.7f32 * (xcorr[1 as libc::c_int as usize] - xcorr[2 as libc::c_int as usize])
    {
        offset = -(1 as libc::c_int);
    } else {
        offset = 0 as libc::c_int;
    }
    if pg > g {
        pg = g;
    }
    *T0_ = 2 as libc::c_int * T + offset;
    if *T0_ < minperiod0 {
        *T0_ = minperiod0;
    }
    return pg;
}
