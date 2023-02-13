use crate::celt::celt::celt_fatal;
use crate::externs::memset;
#[inline]
#[c2rust::src_loc = "43:1"]
unsafe fn silk_LPC_analysis_filter16_FLP(
    r_LPC: *mut f32,
    PredCoef: *const f32,
    s: *const f32,
    length: i32,
) {
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 16 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize)
            + *s_ptr.offset(-(8 as i32) as isize) * *PredCoef.offset(8 as i32 as isize)
            + *s_ptr.offset(-(9 as i32) as isize) * *PredCoef.offset(9 as i32 as isize)
            + *s_ptr.offset(-(10 as i32) as isize) * *PredCoef.offset(10 as i32 as isize)
            + *s_ptr.offset(-(11 as i32) as isize) * *PredCoef.offset(11 as i32 as isize)
            + *s_ptr.offset(-(12 as i32) as isize) * *PredCoef.offset(12 as i32 as isize)
            + *s_ptr.offset(-(13 as i32) as isize) * *PredCoef.offset(13 as i32 as isize)
            + *s_ptr.offset(-(14 as i32) as isize) * *PredCoef.offset(14 as i32 as isize)
            + *s_ptr.offset(-(15 as i32) as isize) * *PredCoef.offset(15 as i32 as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "81:1"]
unsafe fn silk_LPC_analysis_filter12_FLP(
    r_LPC: *mut f32,
    PredCoef: *const f32,
    s: *const f32,
    length: i32,
) {
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 12 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize)
            + *s_ptr.offset(-(8 as i32) as isize) * *PredCoef.offset(8 as i32 as isize)
            + *s_ptr.offset(-(9 as i32) as isize) * *PredCoef.offset(9 as i32 as isize)
            + *s_ptr.offset(-(10 as i32) as isize) * *PredCoef.offset(10 as i32 as isize)
            + *s_ptr.offset(-(11 as i32) as isize) * *PredCoef.offset(11 as i32 as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "115:1"]
unsafe fn silk_LPC_analysis_filter10_FLP(
    r_LPC: *mut f32,
    PredCoef: *const f32,
    s: *const f32,
    length: i32,
) {
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 10 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize)
            + *s_ptr.offset(-(8 as i32) as isize) * *PredCoef.offset(8 as i32 as isize)
            + *s_ptr.offset(-(9 as i32) as isize) * *PredCoef.offset(9 as i32 as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "147:1"]
unsafe fn silk_LPC_analysis_filter8_FLP(
    r_LPC: *mut f32,
    PredCoef: *const f32,
    s: *const f32,
    length: i32,
) {
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 8 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1;
    }
}
#[inline]
#[c2rust::src_loc = "177:1"]
unsafe fn silk_LPC_analysis_filter6_FLP(
    r_LPC: *mut f32,
    PredCoef: *const f32,
    s: *const f32,
    length: i32,
) {
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 6 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize);
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1;
    }
}
#[c2rust::src_loc = "210:1"]
pub unsafe fn silk_LPC_analysis_filter_FLP(
    r_LPC: *mut f32,
    PredCoef: *const f32,
    s: *const f32,
    length: i32,
    Order: i32,
) {
    if !(Order <= length) {
        celt_fatal(
            b"assertion failed: Order <= length\0" as *const u8 as *const i8,
            b"silk/float/LPC_analysis_filter_FLP.c\0" as *const u8 as *const i8,
            218 as i32,
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
            if 0 as i32 == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const i8,
                    b"silk/float/LPC_analysis_filter_FLP.c\0" as *const u8 as *const i8,
                    242 as i32,
                );
            }
        }
    }
    memset(
        r_LPC as *mut core::ffi::c_void,
        0 as i32,
        (Order as u64).wrapping_mul(::core::mem::size_of::<f32>() as u64),
    );
}
