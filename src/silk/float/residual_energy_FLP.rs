use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:32"]
pub mod main_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "178:1"]
        pub fn silk_LPC_analysis_filter_FLP(
            r_LPC: *mut libc::c_float,
            PredCoef: *const libc::c_float,
            s: *const libc::c_float,
            length: libc::c_int,
            Order: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "134:1"]
        pub fn silk_energy_FLP(data: *const libc::c_float, dataSize: libc::c_int)
            -> libc::c_double;
    }
}
use self::main_FLP_h::silk_LPC_analysis_filter_FLP;
use self::SigProc_FLP_h::silk_energy_FLP;
use crate::celt::celt::celt_fatal;
use crate::silk::define::MAX_NB_SUBFR;
#[c2rust::src_loc = "34:9"]
pub const MAX_ITERATIONS_RESIDUAL_NRG: libc::c_int = 10 as libc::c_int;
#[c2rust::src_loc = "35:9"]
pub const REGULARIZATION_FACTOR: libc::c_float = 1e-8f32;
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_residual_energy_covar_FLP(
    c: *const libc::c_float,
    wXX: *mut libc::c_float,
    wXx: *const libc::c_float,
    wxx: libc::c_float,
    D: libc::c_int,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp: libc::c_float = 0.;
    let mut nrg: libc::c_float = 0.0f32;
    let mut regularization: libc::c_float = 0.;
    if !(D >= 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: D >= 0\0" as *const u8 as *const libc::c_char,
            b"silk/float/residual_energy_FLP.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
        );
    }
    regularization = REGULARIZATION_FACTOR
        * (*wXX.offset(0 as libc::c_int as isize)
            + *wXX.offset((D * D - 1 as libc::c_int) as isize));
    k = 0 as libc::c_int;
    while k < MAX_ITERATIONS_RESIDUAL_NRG {
        nrg = wxx;
        tmp = 0.0f32;
        i = 0 as libc::c_int;
        while i < D {
            tmp += *wXx.offset(i as isize) * *c.offset(i as isize);
            i += 1;
        }
        nrg -= 2.0f32 * tmp;
        i = 0 as libc::c_int;
        while i < D {
            tmp = 0.0f32;
            j = i + 1 as libc::c_int;
            while j < D {
                tmp += *wXX.offset((i + D * j) as isize) * *c.offset(j as isize);
                j += 1;
            }
            nrg += *c.offset(i as isize)
                * (2.0f32 * tmp + *wXX.offset((i + D * i) as isize) * *c.offset(i as isize));
            i += 1;
        }
        if nrg > 0 as libc::c_int as libc::c_float {
            break;
        }
        i = 0 as libc::c_int;
        while i < D {
            *wXX.offset((i + D * i) as isize) += regularization;
            i += 1;
        }
        regularization *= 2.0f32;
        k += 1;
    }
    if k == MAX_ITERATIONS_RESIDUAL_NRG {
        nrg = 1.0f32;
    }
    return nrg;
}
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn silk_residual_energy_FLP(
    nrgs: *mut libc::c_float,
    x: *const libc::c_float,
    a: *mut [libc::c_float; 16],
    gains: *const libc::c_float,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
    LPC_order: libc::c_int,
) {
    let mut shift: libc::c_int = 0;
    let mut LPC_res_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut LPC_res: [libc::c_float; 192] = [0.; 192];
    LPC_res_ptr = LPC_res.as_mut_ptr().offset(LPC_order as isize);
    shift = LPC_order + subfr_length;
    silk_LPC_analysis_filter_FLP(
        LPC_res.as_mut_ptr(),
        (*a.offset(0 as libc::c_int as isize)).as_mut_ptr() as *const libc::c_float,
        x.offset((0 as libc::c_int * shift) as isize),
        2 as libc::c_int * shift,
        LPC_order,
    );
    *nrgs.offset(0 as libc::c_int as isize) = ((*gains.offset(0 as libc::c_int as isize)
        * *gains.offset(0 as libc::c_int as isize))
        as libc::c_double
        * silk_energy_FLP(
            LPC_res_ptr.offset((0 as libc::c_int * shift) as isize),
            subfr_length,
        )) as libc::c_float;
    *nrgs.offset(1 as libc::c_int as isize) = ((*gains.offset(1 as libc::c_int as isize)
        * *gains.offset(1 as libc::c_int as isize))
        as libc::c_double
        * silk_energy_FLP(
            LPC_res_ptr.offset((1 as libc::c_int * shift) as isize),
            subfr_length,
        )) as libc::c_float;
    if nb_subfr == MAX_NB_SUBFR {
        silk_LPC_analysis_filter_FLP(
            LPC_res.as_mut_ptr(),
            (*a.offset(1 as libc::c_int as isize)).as_mut_ptr() as *const libc::c_float,
            x.offset((2 as libc::c_int * shift) as isize),
            2 as libc::c_int * shift,
            LPC_order,
        );
        *nrgs.offset(2 as libc::c_int as isize) = ((*gains.offset(2 as libc::c_int as isize)
            * *gains.offset(2 as libc::c_int as isize))
            as libc::c_double
            * silk_energy_FLP(
                LPC_res_ptr.offset((0 as libc::c_int * shift) as isize),
                subfr_length,
            )) as libc::c_float;
        *nrgs.offset(3 as libc::c_int as isize) = ((*gains.offset(3 as libc::c_int as isize)
            * *gains.offset(3 as libc::c_int as isize))
            as libc::c_double
            * silk_energy_FLP(
                LPC_res_ptr.offset((1 as libc::c_int * shift) as isize),
                subfr_length,
            )) as libc::c_float;
    }
}
