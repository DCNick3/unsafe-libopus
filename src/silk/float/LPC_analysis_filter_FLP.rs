use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
use self::arch_h::celt_fatal;
use self::string_h::memset;
#[inline]
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn silk_LPC_analysis_filter16_FLP(
    mut r_LPC: *mut libc::c_float,
    mut PredCoef: *const libc::c_float,
    mut s: *const libc::c_float,
    length: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut LPC_pred: libc::c_float = 0.;
    let mut s_ptr: *const libc::c_float = 0 as *const libc::c_float;
    ix = 16 as libc::c_int;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as libc::c_int) as isize) as *const libc::c_float;
        LPC_pred = *s_ptr.offset(0 as libc::c_int as isize)
            * *PredCoef.offset(0 as libc::c_int as isize)
            + *s_ptr.offset(-(1 as libc::c_int) as isize)
                * *PredCoef.offset(1 as libc::c_int as isize)
            + *s_ptr.offset(-(2 as libc::c_int) as isize)
                * *PredCoef.offset(2 as libc::c_int as isize)
            + *s_ptr.offset(-(3 as libc::c_int) as isize)
                * *PredCoef.offset(3 as libc::c_int as isize)
            + *s_ptr.offset(-(4 as libc::c_int) as isize)
                * *PredCoef.offset(4 as libc::c_int as isize)
            + *s_ptr.offset(-(5 as libc::c_int) as isize)
                * *PredCoef.offset(5 as libc::c_int as isize)
            + *s_ptr.offset(-(6 as libc::c_int) as isize)
                * *PredCoef.offset(6 as libc::c_int as isize)
            + *s_ptr.offset(-(7 as libc::c_int) as isize)
                * *PredCoef.offset(7 as libc::c_int as isize)
            + *s_ptr.offset(-(8 as libc::c_int) as isize)
                * *PredCoef.offset(8 as libc::c_int as isize)
            + *s_ptr.offset(-(9 as libc::c_int) as isize)
                * *PredCoef.offset(9 as libc::c_int as isize)
            + *s_ptr.offset(-(10 as libc::c_int) as isize)
                * *PredCoef.offset(10 as libc::c_int as isize)
            + *s_ptr.offset(-(11 as libc::c_int) as isize)
                * *PredCoef.offset(11 as libc::c_int as isize)
            + *s_ptr.offset(-(12 as libc::c_int) as isize)
                * *PredCoef.offset(12 as libc::c_int as isize)
            + *s_ptr.offset(-(13 as libc::c_int) as isize)
                * *PredCoef.offset(13 as libc::c_int as isize)
            + *s_ptr.offset(-(14 as libc::c_int) as isize)
                * *PredCoef.offset(14 as libc::c_int as isize)
            + *s_ptr.offset(-(15 as libc::c_int) as isize)
                * *PredCoef.offset(15 as libc::c_int as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as libc::c_int as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn silk_LPC_analysis_filter12_FLP(
    mut r_LPC: *mut libc::c_float,
    mut PredCoef: *const libc::c_float,
    mut s: *const libc::c_float,
    length: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut LPC_pred: libc::c_float = 0.;
    let mut s_ptr: *const libc::c_float = 0 as *const libc::c_float;
    ix = 12 as libc::c_int;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as libc::c_int) as isize) as *const libc::c_float;
        LPC_pred = *s_ptr.offset(0 as libc::c_int as isize)
            * *PredCoef.offset(0 as libc::c_int as isize)
            + *s_ptr.offset(-(1 as libc::c_int) as isize)
                * *PredCoef.offset(1 as libc::c_int as isize)
            + *s_ptr.offset(-(2 as libc::c_int) as isize)
                * *PredCoef.offset(2 as libc::c_int as isize)
            + *s_ptr.offset(-(3 as libc::c_int) as isize)
                * *PredCoef.offset(3 as libc::c_int as isize)
            + *s_ptr.offset(-(4 as libc::c_int) as isize)
                * *PredCoef.offset(4 as libc::c_int as isize)
            + *s_ptr.offset(-(5 as libc::c_int) as isize)
                * *PredCoef.offset(5 as libc::c_int as isize)
            + *s_ptr.offset(-(6 as libc::c_int) as isize)
                * *PredCoef.offset(6 as libc::c_int as isize)
            + *s_ptr.offset(-(7 as libc::c_int) as isize)
                * *PredCoef.offset(7 as libc::c_int as isize)
            + *s_ptr.offset(-(8 as libc::c_int) as isize)
                * *PredCoef.offset(8 as libc::c_int as isize)
            + *s_ptr.offset(-(9 as libc::c_int) as isize)
                * *PredCoef.offset(9 as libc::c_int as isize)
            + *s_ptr.offset(-(10 as libc::c_int) as isize)
                * *PredCoef.offset(10 as libc::c_int as isize)
            + *s_ptr.offset(-(11 as libc::c_int) as isize)
                * *PredCoef.offset(11 as libc::c_int as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as libc::c_int as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "115:1"]
unsafe extern "C" fn silk_LPC_analysis_filter10_FLP(
    mut r_LPC: *mut libc::c_float,
    mut PredCoef: *const libc::c_float,
    mut s: *const libc::c_float,
    length: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut LPC_pred: libc::c_float = 0.;
    let mut s_ptr: *const libc::c_float = 0 as *const libc::c_float;
    ix = 10 as libc::c_int;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as libc::c_int) as isize) as *const libc::c_float;
        LPC_pred = *s_ptr.offset(0 as libc::c_int as isize)
            * *PredCoef.offset(0 as libc::c_int as isize)
            + *s_ptr.offset(-(1 as libc::c_int) as isize)
                * *PredCoef.offset(1 as libc::c_int as isize)
            + *s_ptr.offset(-(2 as libc::c_int) as isize)
                * *PredCoef.offset(2 as libc::c_int as isize)
            + *s_ptr.offset(-(3 as libc::c_int) as isize)
                * *PredCoef.offset(3 as libc::c_int as isize)
            + *s_ptr.offset(-(4 as libc::c_int) as isize)
                * *PredCoef.offset(4 as libc::c_int as isize)
            + *s_ptr.offset(-(5 as libc::c_int) as isize)
                * *PredCoef.offset(5 as libc::c_int as isize)
            + *s_ptr.offset(-(6 as libc::c_int) as isize)
                * *PredCoef.offset(6 as libc::c_int as isize)
            + *s_ptr.offset(-(7 as libc::c_int) as isize)
                * *PredCoef.offset(7 as libc::c_int as isize)
            + *s_ptr.offset(-(8 as libc::c_int) as isize)
                * *PredCoef.offset(8 as libc::c_int as isize)
            + *s_ptr.offset(-(9 as libc::c_int) as isize)
                * *PredCoef.offset(9 as libc::c_int as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as libc::c_int as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "147:1"]
unsafe extern "C" fn silk_LPC_analysis_filter8_FLP(
    mut r_LPC: *mut libc::c_float,
    mut PredCoef: *const libc::c_float,
    mut s: *const libc::c_float,
    length: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut LPC_pred: libc::c_float = 0.;
    let mut s_ptr: *const libc::c_float = 0 as *const libc::c_float;
    ix = 8 as libc::c_int;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as libc::c_int) as isize) as *const libc::c_float;
        LPC_pred = *s_ptr.offset(0 as libc::c_int as isize)
            * *PredCoef.offset(0 as libc::c_int as isize)
            + *s_ptr.offset(-(1 as libc::c_int) as isize)
                * *PredCoef.offset(1 as libc::c_int as isize)
            + *s_ptr.offset(-(2 as libc::c_int) as isize)
                * *PredCoef.offset(2 as libc::c_int as isize)
            + *s_ptr.offset(-(3 as libc::c_int) as isize)
                * *PredCoef.offset(3 as libc::c_int as isize)
            + *s_ptr.offset(-(4 as libc::c_int) as isize)
                * *PredCoef.offset(4 as libc::c_int as isize)
            + *s_ptr.offset(-(5 as libc::c_int) as isize)
                * *PredCoef.offset(5 as libc::c_int as isize)
            + *s_ptr.offset(-(6 as libc::c_int) as isize)
                * *PredCoef.offset(6 as libc::c_int as isize)
            + *s_ptr.offset(-(7 as libc::c_int) as isize)
                * *PredCoef.offset(7 as libc::c_int as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as libc::c_int as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "177:1"]
unsafe extern "C" fn silk_LPC_analysis_filter6_FLP(
    mut r_LPC: *mut libc::c_float,
    mut PredCoef: *const libc::c_float,
    mut s: *const libc::c_float,
    length: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut LPC_pred: libc::c_float = 0.;
    let mut s_ptr: *const libc::c_float = 0 as *const libc::c_float;
    ix = 6 as libc::c_int;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as libc::c_int) as isize) as *const libc::c_float;
        LPC_pred = *s_ptr.offset(0 as libc::c_int as isize)
            * *PredCoef.offset(0 as libc::c_int as isize)
            + *s_ptr.offset(-(1 as libc::c_int) as isize)
                * *PredCoef.offset(1 as libc::c_int as isize)
            + *s_ptr.offset(-(2 as libc::c_int) as isize)
                * *PredCoef.offset(2 as libc::c_int as isize)
            + *s_ptr.offset(-(3 as libc::c_int) as isize)
                * *PredCoef.offset(3 as libc::c_int as isize)
            + *s_ptr.offset(-(4 as libc::c_int) as isize)
                * *PredCoef.offset(4 as libc::c_int as isize)
            + *s_ptr.offset(-(5 as libc::c_int) as isize)
                * *PredCoef.offset(5 as libc::c_int as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as libc::c_int as isize) - LPC_pred;
        ix += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "210:1"]
pub unsafe extern "C" fn silk_LPC_analysis_filter_FLP(
    mut r_LPC: *mut libc::c_float,
    mut PredCoef: *const libc::c_float,
    mut s: *const libc::c_float,
    length: libc::c_int,
    Order: libc::c_int,
) {
    if !(Order <= length) {
        celt_fatal(
            b"assertion failed: Order <= length\0" as *const u8 as *const libc::c_char,
            b"silk/float/LPC_analysis_filter_FLP.c\0" as *const u8
                as *const libc::c_char,
            218 as libc::c_int,
        );
    }
    match Order {
        6 => {
            silk_LPC_analysis_filter6_FLP(r_LPC, PredCoef, s, length);
        }
        8 => {
            silk_LPC_analysis_filter8_FLP(r_LPC, PredCoef, s, length);
        }
        10 => {
            silk_LPC_analysis_filter10_FLP(r_LPC, PredCoef, s, length);
        }
        12 => {
            silk_LPC_analysis_filter12_FLP(r_LPC, PredCoef, s, length);
        }
        16 => {
            silk_LPC_analysis_filter16_FLP(r_LPC, PredCoef, s, length);
        }
        _ => {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/float/LPC_analysis_filter_FLP.c\0" as *const u8
                        as *const libc::c_char,
                    242 as libc::c_int,
                );
            }
        }
    }
    memset(
        r_LPC as *mut libc::c_void,
        0 as libc::c_int,
        (Order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
}
