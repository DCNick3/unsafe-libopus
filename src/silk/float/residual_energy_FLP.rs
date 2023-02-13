use crate::celt::celt::celt_fatal;
use crate::silk::define::MAX_NB_SUBFR;
use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP;

#[c2rust::src_loc = "34:9"]
pub const MAX_ITERATIONS_RESIDUAL_NRG: i32 = 10 as i32;
#[c2rust::src_loc = "35:9"]
pub const REGULARIZATION_FACTOR: f32 = 1e-8f32;
#[c2rust::src_loc = "38:1"]
pub unsafe fn silk_residual_energy_covar_FLP(
    c: *const f32,
    wXX: *mut f32,
    wXx: *const f32,
    wxx: f32,
    D: i32,
) -> f32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut tmp: f32 = 0.;
    let mut nrg: f32 = 0.0f32;
    let mut regularization: f32 = 0.;
    if !(D >= 0 as i32) {
        celt_fatal(
            b"assertion failed: D >= 0\0" as *const u8 as *const i8,
            b"silk/float/residual_energy_FLP.c\0" as *const u8 as *const i8,
            50 as i32,
        );
    }
    regularization = REGULARIZATION_FACTOR
        * (*wXX.offset(0 as i32 as isize) + *wXX.offset((D * D - 1 as i32) as isize));
    k = 0 as i32;
    while k < MAX_ITERATIONS_RESIDUAL_NRG {
        nrg = wxx;
        tmp = 0.0f32;
        i = 0 as i32;
        while i < D {
            tmp += *wXx.offset(i as isize) * *c.offset(i as isize);
            i += 1;
        }
        nrg -= 2.0f32 * tmp;
        i = 0 as i32;
        while i < D {
            tmp = 0.0f32;
            j = i + 1 as i32;
            while j < D {
                tmp += *wXX.offset((i + D * j) as isize) * *c.offset(j as isize);
                j += 1;
            }
            nrg += *c.offset(i as isize)
                * (2.0f32 * tmp + *wXX.offset((i + D * i) as isize) * *c.offset(i as isize));
            i += 1;
        }
        if nrg > 0 as i32 as f32 {
            break;
        }
        i = 0 as i32;
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
#[c2rust::src_loc = "91:1"]
pub unsafe fn silk_residual_energy_FLP(
    nrgs: *mut f32,
    x: *const f32,
    a: *mut [f32; 16],
    gains: *const f32,
    subfr_length: i32,
    nb_subfr: i32,
    LPC_order: i32,
) {
    let mut shift: i32 = 0;
    let mut LPC_res_ptr: *mut f32 = 0 as *mut f32;
    let mut LPC_res: [f32; 192] = [0.; 192];
    LPC_res_ptr = LPC_res.as_mut_ptr().offset(LPC_order as isize);
    shift = LPC_order + subfr_length;
    silk_LPC_analysis_filter_FLP(
        LPC_res.as_mut_ptr(),
        (*a.offset(0 as i32 as isize)).as_mut_ptr() as *const f32,
        x.offset((0 as i32 * shift) as isize),
        2 as i32 * shift,
        LPC_order,
    );
    *nrgs.offset(0 as i32 as isize) = ((*gains.offset(0 as i32 as isize)
        * *gains.offset(0 as i32 as isize)) as f64
        * silk_energy_FLP(
            LPC_res_ptr.offset((0 as i32 * shift) as isize),
            subfr_length,
        )) as f32;
    *nrgs.offset(1 as i32 as isize) = ((*gains.offset(1 as i32 as isize)
        * *gains.offset(1 as i32 as isize)) as f64
        * silk_energy_FLP(
            LPC_res_ptr.offset((1 as i32 * shift) as isize),
            subfr_length,
        )) as f32;
    if nb_subfr == MAX_NB_SUBFR {
        silk_LPC_analysis_filter_FLP(
            LPC_res.as_mut_ptr(),
            (*a.offset(1 as i32 as isize)).as_mut_ptr() as *const f32,
            x.offset((2 as i32 * shift) as isize),
            2 as i32 * shift,
            LPC_order,
        );
        *nrgs.offset(2 as i32 as isize) = ((*gains.offset(2 as i32 as isize)
            * *gains.offset(2 as i32 as isize)) as f64
            * silk_energy_FLP(
                LPC_res_ptr.offset((0 as i32 * shift) as isize),
                subfr_length,
            )) as f32;
        *nrgs.offset(3 as i32 as isize) = ((*gains.offset(3 as i32 as isize)
            * *gains.offset(3 as i32 as isize)) as f64
            * silk_energy_FLP(
                LPC_res_ptr.offset((1 as i32 * shift) as isize),
                subfr_length,
            )) as f32;
    }
}
